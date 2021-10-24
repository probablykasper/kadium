use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
  pub id: String,
  pub name: String,
  pub icon: String,
  pub uploads_playlist_id: String,
  pub from_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
  pub name: String,
  pub minutes_between_refreshes: f64,
  pub channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
  pub api_key: String,
  pub max_concurrent_requests: u32,
  pub groups: Vec<Group>,
}
impl Default for Settings {
  fn default() -> Self {
    Settings {
      api_key: "".to_string(),
      max_concurrent_requests: 5,
      groups: Vec::new(),
    }
  }
}
