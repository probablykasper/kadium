use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

use crate::throw;

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
  pub email: String,
  pub minutes_between_refreshes: f64,
  pub channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
  pub api_key: String,
  pub from_email: String,
  pub unread_errors: bool,
  pub max_concurrent_requests: u32,
  pub groups: Vec<Group>,
}
impl Default for Settings {
  fn default() -> Self {
    Settings {
      api_key: "".to_string(),
      from_email: "".to_string(),
      unread_errors: false,
      max_concurrent_requests: 5,
      groups: Vec::new(),
    }
  }
}
impl Settings {
  pub fn load(mut file: File) -> Result<Self, String> {
    let mut json_str = String::new();
    match file.read_to_string(&mut json_str) {
      Ok(_) => {}
      Err(err) => throw!("Error reading file: {}", err),
    };
    let tax: Self = match serde_json::from_str(&mut json_str) {
      Ok(library) => library,
      Err(err) => {
        throw!("Error parsing file: {}", err.to_string());
      }
    };
    return Ok(tax);
  }
}
