use crate::settings::{Channel, Settings, VersionedSettings};
use crate::throw;
use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{command, State};

pub fn to_json<T: Serialize>(data: &T) -> Result<Value, String> {
  match serde_json::to_value(data) {
    Ok(v) => Ok(v),
    Err(e) => throw!("Error serializing {}", e),
  }
}

pub struct Data {
  pub versioned_settings: VersionedSettings,
  pub app_dir: PathBuf,
}
impl Data {
  pub fn settings(&mut self) -> &mut Settings {
    self.versioned_settings.unwrap()
  }
  pub fn save_settings(&mut self) -> Result<(), String> {
    self.versioned_settings.save(&self.app_dir)
  }
}

#[command]
pub fn get_settings(data: State<ArcData>) -> Result<Value, String> {
  let mut data = data.0.lock().unwrap();
  to_json(&data.settings())
}

#[command]
pub fn set_channels(channels: Vec<Channel>, data: State<ArcData>) -> Result<(), String> {
  let mut data = data.0.lock().unwrap();
  data.settings().channels = channels;
  data.save_settings()?;
  Ok(())
}

pub struct ArcData(pub Arc<Mutex<Data>>);

impl ArcData {
  pub fn new(data: Data) -> Self {
    Self(Arc::new(Mutex::new(data)))
  }
}
