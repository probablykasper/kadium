use crate::{settings, throw};
use serde::de::DeserializeOwned;
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
  pub from_time: u64,
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
      channels: vec![
        ChannelInfo {
          name: "Bendover Productions".to_string(),
          uploads_playlist_id: "UU9RM-iSvTu1uPJb8X5yp3EQ".to_string(),
          from_time: 1633816800000,
        },
        // ChannelIntervalInfo {
        //   uploads_playlist_id: "wawowii2".to_string(),
        //   from_time: 1635617176866,
        // },
      ],
    },
  );
  map
}

pub fn spawn(settings: &settings::Settings) -> Option<FetcherHandle> {
  if settings.channels.len() == 0 {
    return None;
  }

  let api_key = settings.api_key.clone();

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
      return start_intervals(api_key, interval_infos, stop_sender2).await;
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
  api_key: String,
  interval_infos: Vec<IntervalInfo>,
  sender: broadcast::Sender<()>,
) -> Result<(), String> {
  let mut tasks = Vec::new();
  for interval_info in interval_infos {
    let key = api_key.clone();
    let mut stop_receiver = sender.subscribe();
    let handle = task::spawn(async move {
      tokio::select! {
        _ = run_interval(key, interval_info) => {
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

async fn run_interval(api_key: String, interval_info: IntervalInfo) {
  let mut interval = time::interval(Duration::from_millis(interval_info.ms));
  interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
  let mut run_num = 0;
  loop {
    interval.tick().await;
    println!("{}ms task: {}", interval_info.ms, run_num);
    for channel in &interval_info.channels {
      match check_channel(&api_key, &channel).await {
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

async fn check_channel(api_key: &str, channel: &ChannelInfo) -> Result<(), String> {
  let url = "https://www.googleapis.com/youtube/v3/playlistItems".to_string()
    + "?part=snippet,contentDetails"
    + "&maxResults=50"
    + "&playlistId="
    + &channel.uploads_playlist_id;
  #[derive(serde::Deserialize, Debug)]
  struct Testie {
    yo: String,
  }
  let res = yt_request::<playlist_items::SuccessResponse>(&url, api_key)
    .await
    .map_err(|e| format!("Error checking channel \"{}\": {}", channel.name, e))?;
  println!("{:#?}", res);
  // check if there are any new videos
  // if so, fetch their metadata (duration)
  Ok(())
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

mod playlist_items {
  use serde::Deserialize;

  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/playlistItems/list#properties
  #[derive(Deserialize, Debug)]
  pub struct SuccessResponse {
    pub items: Vec<Item>,
  }
  /// Lists the fields we use only. Documentation:
  /// https://developers.google.com/youtube/v3/docs/playlistItems
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
  pub struct Item {
    pub snippet: Snippet,
    pub content_details: ContentDetails,
  }
  #[derive(Deserialize, Debug)]
  #[serde(rename_all = "camelCase")]
  pub struct Snippet {
    pub title: String,
    pub description: String,
    pub channel_title: String,
    pub thumbnails: Thumbnails,
    pub resource_id: ResourceId,
  }
  #[derive(Deserialize, Debug)]
  pub struct Thumbnails {
    pub default: Option<Thumbnail>,
    pub medium: Option<Thumbnail>,
    pub high: Option<Thumbnail>,
    pub standard: Option<Thumbnail>,
    pub maxres: Option<Thumbnail>,
  }
  #[derive(Deserialize, Debug)]
  pub struct Thumbnail {
    pub url: String,
  }
  #[derive(Deserialize, Debug)]
  #[serde(rename_all = "camelCase")]
  pub struct ResourceId {
    pub video_id: String,
  }
  #[derive(Deserialize, Debug)]
  #[serde(rename_all = "camelCase")]
  pub struct ContentDetails {
    pub video_published_at: String,
  }
}
