use crate::api::{playlist_items, videos, yt_request};
use crate::{db, settings, throw};
use chrono::{DateTime, FixedOffset};
use iso8601_duration::Duration as IsoDuration;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::async_runtime::Mutex;
use tokio::runtime::Runtime;
use tokio::sync::broadcast;
use tokio::{task, time};

pub struct IntervalInfo {
  pub ms: u64,
  pub channels: Vec<ChannelInfo>,
}
pub struct ChannelInfo {
  pub name: String,
  pub uploads_playlist_id: String,
  pub from_time: i64,
}

pub struct FetcherHandle {
  pub handle: thread::JoinHandle<Result<(), String>>,
  pub stop_sender: broadcast::Sender<()>,
  pub update_counter: Arc<Mutex<u64>>,
}

impl FetcherHandle {
  pub fn stop(&self) {
    // Error can only occur when channel is already closed
    let _ = self.stop_sender.send(());
  }
  pub fn wait_until_stopped(self) -> Result<(), String> {
    match self.handle.join() {
      Ok(result) => result,
      Err(e) => {
        if let Some(e) = e.downcast_ref::<&'static str>() {
          throw!("Interval thread error: {}", e);
        } else {
          throw!("Unknown interval thread error: {:?}", e);
        }
      }
    }
  }
}

pub fn spawn(settings: &settings::Settings, pool: &SqlitePool) -> Option<FetcherHandle> {
  if settings.channels.len() == 0 {
    return None;
  }

  let api_key = settings.api_key.clone();
  let pool = pool.clone();

  let interval_map = new_intervals_map(&settings.channels);
  let interval_infos = to_interval_info_vector(interval_map);

  let (stop_sender, _stop_receiver) = broadcast::channel(1);
  let stop_sender2 = stop_sender.clone();

  let update_counter = Arc::new(Mutex::new(0));
  let update_counter2 = update_counter.clone();

  let tokio_thread = thread::spawn(move || {
    let runtime = match Runtime::new() {
      Ok(r) => r,
      Err(e) => return Err(e.to_string()),
    };

    return runtime.block_on(async {
      // start intervals inside tokio runtime
      return start_intervals(pool, api_key, interval_infos, stop_sender2, update_counter2).await;
    });
  });

  Some(FetcherHandle {
    handle: tokio_thread,
    stop_sender,
    update_counter,
  })
}

pub type IntervalMap = HashMap<u64, IntervalInfo>;
fn to_interval_info_vector(map: IntervalMap) -> Vec<IntervalInfo> {
  map.into_iter().map(|(_ms, info)| info).collect()
}
fn new_intervals_map(channels: &Vec<settings::Channel>) -> IntervalMap {
  let mut intervals_map: IntervalMap = HashMap::new();
  for channel in channels.iter() {
    let default = IntervalInfo {
      ms: channel.refresh_rate,
      channels: Vec::new(),
    };
    let interval_info = intervals_map.entry(channel.refresh_rate).or_insert(default);
    interval_info.channels.push(ChannelInfo {
      name: channel.name.to_string(),
      uploads_playlist_id: channel.uploads_playlist_id.clone(),
      from_time: channel.from_time.clone(),
    });
  }
  intervals_map
}

async fn start_intervals(
  pool: SqlitePool,
  api_key: String,
  interval_infos: Vec<IntervalInfo>,
  stop_sender: broadcast::Sender<()>,
  update_counter: Arc<Mutex<u64>>,
) -> Result<(), String> {
  let mut tasks = Vec::new();
  for interval_info in interval_infos {
    let key = api_key.clone();
    let pool = pool.clone();
    let mut stop_receiver = stop_sender.subscribe();
    let update_counter = update_counter.clone();
    let handle = task::spawn(async move {
      tokio::select! {
        _ = run_interval(pool, key, interval_info, update_counter) => {
          throw!("Interval unexpectedly completed");
        }
        result = stop_receiver.recv() => {
          match result {
            Ok(_) => return Ok(()),
            Err(e) => {
              return Err(e.to_string());
            }
          }
        }
      }
    });
    tasks.push(handle);
  }
  for task in tasks {
    let result = task.await;
    match result {
      Ok(_) => return Ok(()),
      Err(e) => {
        return Err(e.to_string());
      }
    }
  }
  Err("No intervals started".to_string())
}

async fn run_interval(
  pool: SqlitePool,
  api_key: String,
  interval_info: IntervalInfo,
  update_counter: Arc<Mutex<u64>>,
) {
  let mut interval = time::interval(Duration::from_millis(interval_info.ms));
  interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
  let mut run_num = 0;
  loop {
    interval.tick().await;
    println!("{}ms task: {}", interval_info.ms, run_num);
    for channel in &interval_info.channels {
      match check_channel(&pool, &api_key, &channel).await {
        Ok(did_insert) => {
          if did_insert {
            let mut count = update_counter.lock().await;
            *count = count.clone() + 1;
          }
        }
        Err(e) => {
          println!("{}", e);
          todo!(); // show error to user
        }
      }
    }
    run_num += 1;
  }
}

/// Returns the number of new videos saved
async fn check_channel(
  pool: &SqlitePool,
  api_key: &str,
  channel: &ChannelInfo,
) -> Result<bool, String> {
  println!("Checking {} {}", channel.uploads_playlist_id, channel.name);
  let url = "https://www.googleapis.com/youtube/v3/playlistItems".to_string()
    + "?part=contentDetails"
    + "&maxResults=50"
    + "&playlistId="
    + &channel.uploads_playlist_id;
  let uploads = yt_request::<playlist_items::Response>(&url, api_key)
    .await
    .map_err(|e| format!("Error checking channel \"{}\": {}", channel.name, e))?;

  if uploads.items.len() == 0 {
    return Ok(false); // no channel videos returned
  }

  let existing_ids = db::get_ids(&uploads.items, pool).await?;

  // check which videos are new
  let mut new_ids: Vec<String> = Vec::new();
  for fetched_video in uploads.items {
    let fetched_id = &fetched_video.contentDetails.videoId;
    if existing_ids.contains(fetched_id) {
      continue;
    }

    let published_str = fetched_video.contentDetails.videoPublishedAt;
    let published_time = parse_datetime(&published_str)?.timestamp_millis();
    if published_time < channel.from_time {
      continue;
    }

    new_ids.push(fetched_video.contentDetails.videoId);
  }

  if new_ids.len() == 0 {
    return Ok(false); // no new videos
  }

  // get info about the videos
  println!(
    "Checking videos from {} {}",
    channel.uploads_playlist_id, channel.name
  );
  let url = "https://www.googleapis.com/youtube/v3/videos".to_string()
    + "?part=contentDetails,liveStreamingDetails,snippet"
    + "&id="
    + &new_ids.join(",");
  let videos = yt_request::<videos::Response>(&url, api_key)
    .await
    .map_err(|e| format!("Error checking channel \"{}\": {}", channel.name, e))?;

  let mut videos_to_add: Vec<db::Video> = Vec::new();
  for video in videos.items {
    if let Some(live_streaming_details) = &video.liveStreamingDetails {
      // skip future livestreams. actualStartTime doesn't exist for future streams
      if live_streaming_details.actualStartTime == None {
        continue;
      }
    }
    let publish_time = parse_datetime(&video.snippet.publishedAt)?;
    let duration_ms = parse_absolute_duration(&video.contentDetails.duration)?;
    videos_to_add.push(db::Video {
      id: video.id,
      title: video.snippet.title,
      description: video.snippet.description,
      publishTimeMs: publish_time.timestamp_millis(),
      durationMs: duration_ms,
      thumbnailStandard: video.snippet.thumbnails.standard.is_some(),
      thumbnailMaxres: video.snippet.thumbnails.maxres.is_some(),
      channelId: video.snippet.channelId,
      channelName: video.snippet.channelTitle,
      unread: true,
      archived: false,
    });
  }

  let will_save_videos = videos_to_add.len() >= 1;
  for video in videos_to_add {
    db::insert_video(&video, pool).await?;
  }
  Ok(will_save_videos)
}

pub fn parse_datetime(value: &str) -> Result<DateTime<FixedOffset>, String> {
  match DateTime::parse_from_rfc3339(&value) {
    Ok(datetime) => Ok(datetime),
    Err(e) => throw!("Unexpected video publish date: {}", e),
  }
}
/// Parse a duration that cannot include year or month, because
/// years and months have different lengths depending on what month or year
/// it is.
pub fn parse_absolute_duration(value: &str) -> Result<i64, String> {
  match IsoDuration::parse(&value) {
    Ok(duration) => {
      if duration.month == 0.0 && duration.year == 0.0 {
        let seconds = duration.second as f64
          + duration.minute as f64 as f64 * 60.0
          + duration.hour as f64 * 60.0 * 60.0
          + duration.day as f64 * 60.0 * 60.0 * 24.0;
        let ms = seconds * 1000.0;
        let ms_clamped = ms.max(std::i64::MIN as f64).min(std::i64::MAX as f64);
        let ms_int = ms_clamped.round() as i64;
        Ok(ms_int)
      } else {
        throw!("Cannot parse duration with year or month: {}", value);
      }
    }
    Err(e) => throw!("Unexpected video duration: {}", e),
  }
}
