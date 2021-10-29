use crate::settings;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tokio::{task, time};

pub struct IntervalInfo {
  pub interval: u64,
  pub channels: Vec<ChannelIntervalInfo>,
}
pub struct ChannelIntervalInfo {
  pub uploads_playlist_id: String,
  pub from_time: u64,
}

pub type FetcherHandle = thread::JoinHandle<()>;

pub fn spawn(settings: &settings::Settings) -> Option<FetcherHandle> {
  if settings.channels.len() == 0 {
    return None;
  }
  let _intervals_map = intervals_map(&settings.channels);

  // let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
  // let (handle_tx, handle_rx) = std::sync::mpsc::channel();

  let tokio_thread = thread::spawn(move || {
    let runtime = Runtime::new().expect("Unable to create runtime");
    // handle_tx
    //   .send(runtime.handle().clone())
    //   .expect("Unable to send runtime handle");

    runtime.block_on(async {
      let interval_ids = vec![1, 2];
      let mut intervals = Vec::new();
      for id in interval_ids {
        let handle = task::spawn(async move {
          let mut interval = time::interval(Duration::from_millis(1000));
          let start_time = Instant::now();
          let mut i = 0;
          loop {
            interval.tick().await;
            i += 1;
            let elapsed = Instant::now().duration_since(start_time).as_millis() as f32;
            println!("Task {}: {} / {:.2}s", id, i, elapsed / 1000.0);
          }
        })
        .await;
        intervals.push(handle);
      }
    });
  });

  Some(tokio_thread)
}

fn intervals_map(channels: &Vec<settings::Channel>) -> HashMap<u64, IntervalInfo> {
  let mut intervals_map: HashMap<u64, IntervalInfo> = HashMap::new();
  for channel in channels.iter() {
    let default = IntervalInfo {
      interval: channel.refresh_rate,
      channels: Vec::new(),
    };
    let interval_info = intervals_map.entry(channel.refresh_rate).or_insert(default);
    interval_info.channels.push(ChannelIntervalInfo {
      uploads_playlist_id: channel.uploads_playlist_id.clone(),
      from_time: channel.from_time.clone(),
    });
  }
  intervals_map
}
