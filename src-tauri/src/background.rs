use crate::{settings, throw};
use chrono::{DateTime, FixedOffset};
use serde::de::DeserializeOwned;
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
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

fn interval_info_test() -> HashMap<u64, IntervalInfo> {
  let mut map = HashMap::new();
  map.insert(
    2000 * 60 * 60,
    IntervalInfo {
      ms: 2000 * 60 * 60,
      channels: vec![],
    },
  );
  map.insert(
    1000 * 60 * 60,
    IntervalInfo {
      ms: 1000 * 60 * 60,
      channels: vec![ChannelInfo {
        name: "Bendover Productions".to_string(),
        uploads_playlist_id: "UU9RM-iSvTu1uPJb8X5yp3EQ".to_string(),
        from_time: 1623715200000,
      }],
    },
  );
  map
}

pub fn spawn(settings: &settings::Settings, pool: &SqlitePool) -> Option<FetcherHandle> {
  if settings.channels.len() == 0 {
    return None;
  }

  let api_key = settings.api_key.clone();
  let pool = pool.clone();

  let _interval_map = new_intervals_map(&settings.channels);
  let interval_map = interval_info_test();
  let interval_infos = to_interval_info_vector(interval_map);

  let (stop_sender, _stop_receiver) = broadcast::channel(1);
  let stop_sender2 = stop_sender.clone();

  let tokio_thread = thread::spawn(move || {
    let runtime = match Runtime::new() {
      Ok(r) => r,
      Err(e) => return Err(e.to_string()),
    };

    return runtime.block_on(async {
      // start intervals inside tokio runtime
      return start_intervals(pool, api_key, interval_infos, stop_sender2).await;
    });
  });

  Some(FetcherHandle {
    handle: tokio_thread,
    stop_sender,
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
  sender: broadcast::Sender<()>,
) -> Result<(), String> {
  let mut tasks = Vec::new();
  for interval_info in interval_infos {
    let key = api_key.clone();
    let pool = pool.clone();
    let mut stop_receiver = sender.subscribe();
    let handle = task::spawn(async move {
      tokio::select! {
        _ = run_interval(pool, key, interval_info) => {
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

async fn run_interval(pool: SqlitePool, api_key: String, interval_info: IntervalInfo) {
  let mut interval = time::interval(Duration::from_millis(interval_info.ms));
  interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
  let mut run_num = 0;
  loop {
    interval.tick().await;
    println!("{}ms task: {}", interval_info.ms, run_num);
    for channel in &interval_info.channels {
      match check_channel(&pool, &api_key, &channel).await {
        Ok(()) => {}
        Err(e) => {
          println!("{}", e);
          todo!(); // show error to user
        }
      }
    }
    run_num += 1;
  }
}

async fn check_channel(
  pool: &SqlitePool,
  api_key: &str,
  channel: &ChannelInfo,
) -> Result<(), String> {
  let url = "https://www.googleapis.com/youtube/v3/playlistItems".to_string()
    + "?part=contentDetails"
    + "&maxResults=50"
    + "&playlistId="
    + &channel.uploads_playlist_id;
  let uploads = yt_request::<playlist_items::Response>(&url, api_key)
    .await
    .map_err(|e| format!("Error checking channel \"{}\": {}", channel.name, e))?;

  if uploads.items.len() == 0 {
    return Ok(()); // no channel videos returned
  }

  let existing_ids = get_ids(&uploads.items, pool).await?;
  println!("{} existing IDs: {:?}", existing_ids.len(), existing_ids);

  let mut new_ids: Vec<String> = Vec::new();
  for fetched_video in uploads.items {
    let fetched_id = &fetched_video.content_details.video_id;
    if existing_ids.contains(fetched_id) {
      println!("Existing ID: {}", fetched_id);
      continue;
    }

    let published_str = fetched_video.content_details.video_published_at;
    let published_time = parse_datetime(&published_str)?.timestamp_millis();
    if published_time < channel.from_time {
      println!("Too old ID: {}", fetched_id);
      continue;
    }

    new_ids.push(fetched_video.content_details.video_id);
  }

  if new_ids.len() == 0 {
    return Ok(()); // no new videos
  }
  println!("New IDs: {:?}", new_ids);

  let url = "https://www.googleapis.com/youtube/v3/videos".to_string()
    + "?part=contentDetails,liveStreamingDetails,snippet"
    + "&id="
    + &new_ids.join(",");
  let videos = yt_request::<videos::Response>(&url, api_key)
    .await
    .map_err(|e| format!("Error checking channel \"{}\": {}", channel.name, e))?;

  let mut videos_to_add: Vec<videos::Video> = Vec::new();
  for video in videos.items {
    if let Some(live_streaming_details) = &video.live_streaming_details {
      let start_time = &live_streaming_details.scheduled_start_time;
      let start_timestamp = parse_datetime(&start_time)?;
      if start_timestamp > chrono::Utc::now() {
        continue; // skip future livestreams
      }
    }
    videos_to_add.push(video);
  }
  Ok(())
}

async fn get_ids(
  videos: &Vec<playlist_items::Playlist>,
  pool: &SqlitePool,
) -> Result<Vec<String>, String> {
  // let mut id_placeholders = "\"?\"".to_string();
  let mut id_placeholders = "?".to_string();
  for _n in 0..(videos.len() - 1) {
    // id_placeholders.push_str(",\"?\"");
    id_placeholders.push_str(",?");
  }

  let query_str = format!("SELECT id FROM videos WHERE id IN ({});", id_placeholders);
  let mut query = sqlx::query(&query_str);
  for video in videos {
    query = query.bind(&video.content_details.video_id);
  }
  let rows = match query.fetch_all(pool).await {
    Ok(rows) => rows,
    Err(e) => throw!("Unable to get video IDs: {}", e),
  };
  let mut existing_ids: Vec<String> = Vec::new();
  for row in rows {
    match row.try_get(0) {
      Ok(id) => existing_ids.push(id),
      Err(e) => throw!("Unable to get video ID from database row: {}", e),
    };
  }
  Ok(existing_ids)
}

async fn yt_request<T: DeserializeOwned>(url: &str, key: &str) -> Result<T, String> {
  let client = reqwest::Client::new();
  let json: serde_json::Value = client
    .get(url)
    .header("X-Goog-Api-Key", key)
    .send()
    .await
    .map_err(|e| format!("API request failed: {}", e))?
    .json()
    .await
    .map_err(|e| format!("API response was not JSON: {}", e))?;

  match json.get("error") {
    Some(error_obj) => {
      let code = error_obj.get("code").map(|v| v.as_i64()).flatten();
      let code_str = code.map(|n| n.to_string()).unwrap_or_default();
      let message = error_obj.get("message").map(|v| v.as_str()).flatten();
      throw!("API error: {} {}", code_str, message.unwrap_or_default());
    }
    _ => {}
  }
  match serde_json::from_value::<T>(json) {
    Ok(v) => Ok(v),
    Err(e) => {
      throw!("Unexpected API response: {}", e);
    }
  }
}

mod videos {
  use serde::Deserialize;

  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/videos/list#properties
  #[derive(Deserialize, Debug)]
  pub struct Response {
    pub items: Vec<Video>,
  }
  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/videos#properties
  #[derive(Deserialize, Debug)]
  #[serde(rename_all = "camelCase")]
  pub struct Video {
    pub content_details: ContentDetails,
    pub live_streaming_details: Option<LiveStreamingDetails>,
    pub snippet: Snippet,
  }
  #[derive(Deserialize, Debug)]
  pub struct ContentDetails {
    pub duration: String,
  }
  #[derive(Deserialize, Debug)]
  #[serde(rename_all = "camelCase")]
  pub struct LiveStreamingDetails {
    pub scheduled_start_time: String,
  }

  #[derive(Deserialize, Debug)]
  #[serde(rename_all = "camelCase")]
  pub struct Snippet {
    pub channel_id: String,
    pub channel_title: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
  }
  /// default, medium and high always exist:
  /// default 120x90:   https://i.ytimg.com/vi/___ID___/default.jpg
  /// medium 320x180:   https://i.ytimg.com/vi/___ID___/mqdefault.jpg
  /// high 480x360:     https://i.ytimg.com/vi/___ID___/hqdefault.jpg
  /// standard 640x480: https://i.ytimg.com/vi/___ID___/sddefault.jpg
  /// maxres 1280x720:  https://i.ytimg.com/vi/___ID___/maxresdefault.jpg
  #[derive(Deserialize, Debug)]
  pub struct Thumbnails {
    pub standard: Option<Thumbnail>,
    pub maxres: Option<Thumbnail>,
  }
  #[derive(Deserialize, Debug)]
  pub struct Thumbnail {
    pub url: String,
  }
}

mod playlist_items {
  use serde::Deserialize;

  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/playlistItems/list#properties
  #[derive(Deserialize, Debug)]
  pub struct Response {
    pub items: Vec<Playlist>,
  }
  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/playlistItems#properties
  /// weird date situation:
  ///  `snippet.publishedAt` is when the video was added to the uploads playlist.
  ///  `contentDetails.videoPublishedAt` is when the video was published
  ///  Soemtimes these are a few seconds different, other times an hour
  ///  (like with Monstercat). No idea why.
  ///  Additionally, I tried to compare with what YouTube shows:
  ///  - What YouTube shows: 19:56 (should be up to 1 hour inaccurate)
  ///  - publishedAt: 17:31
  ///  - 18:00
  ///  YouTube only shows "9 hours ago", so you'd expect it to be up to
  ///  an hour off... But it's almost 2 hours off, if not 2.5 hours.
  ///  :/
  #[derive(Deserialize, Debug)]
  #[serde(rename_all = "camelCase")]
  pub struct Playlist {
    pub content_details: ContentDetails,
  }
  #[derive(Deserialize, Debug)]
  #[serde(rename_all = "camelCase")]
  pub struct ContentDetails {
    pub video_published_at: String,
    pub video_id: String,
  }
}

pub fn parse_datetime(value: &str) -> Result<DateTime<FixedOffset>, String> {
  match DateTime::parse_from_rfc3339(&value) {
    Ok(datetime) => Ok(datetime),
    Err(e) => throw!("Unexpected video publish date: {}", e),
  }
}
