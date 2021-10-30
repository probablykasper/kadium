use crate::{settings, throw};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::broadcast;
use tokio::{task, time};

pub struct IntervalInfo {
  pub ms: u64,
  pub channels: Vec<ChannelIntervalInfo>,
}
pub struct ChannelIntervalInfo {
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
    2000,
    IntervalInfo {
      ms: 2000,
      channels: vec![],
    },
  );
  map.insert(
    1000,
    IntervalInfo {
      ms: 1000,
      channels: vec![
        ChannelIntervalInfo {
          uploads_playlist_id: "wawowii".to_string(),
          from_time: 1635617176866,
        },
        ChannelIntervalInfo {
          uploads_playlist_id: "wawowii2".to_string(),
          from_time: 1635617176866,
        },
      ],
    },
  );
  map
}

pub fn spawn(settings: &settings::Settings) -> Option<FetcherHandle> {
  if settings.channels.len() == 0 {
    return None;
  }
  let _interval_map = intervals_map(&settings.channels);
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
      return start_intervals(interval_infos, stop_sender2).await;
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

async fn start_intervals(
  interval_infos: Vec<IntervalInfo>,
  sender: broadcast::Sender<()>,
) -> Result<(), String> {
  let mut tasks = Vec::new();
  for (i, interval_info) in interval_infos.into_iter().enumerate() {
    let mut stop_receiver = sender.subscribe();
    let handle = task::spawn(async move {
      tokio::select! {
        _ = run_interval(i, interval_info) => {
          println!("run_interval done");
          return Ok(());
        }
        result = stop_receiver.recv() => {
          println!("stop_receiver received");
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

async fn run_interval(id: usize, interval_info: IntervalInfo) {
  let mut interval = time::interval(Duration::from_millis(interval_info.ms));
  interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
  let mut run_num = 0;
  loop {
    interval.tick().await;
    println!("Task {}: {}", id, run_num);
    run_num += 1;
  }
}

fn intervals_map(channels: &Vec<settings::Channel>) -> IntervalMap {
  let mut intervals_map: IntervalMap = HashMap::new();
  for channel in channels.iter() {
    let default = IntervalInfo {
      ms: channel.refresh_rate,
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
