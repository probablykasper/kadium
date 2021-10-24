use crate::settings::{Channel, Settings, VersionedSettings};
use crate::throw;
use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{command, Config, State};

#[derive(Clone)]
pub struct AppPaths {
  pub app_dir: PathBuf,
  pub settings_file: PathBuf,
}
impl AppPaths {
  pub fn from_tauri_config(config: &Config) -> Self {
    let app_dir = tauri::api::path::app_dir(config).unwrap();
    AppPaths {
      app_dir: app_dir.clone(),
      settings_file: app_dir.join("settings.json"),
    }
  }
}

pub struct Data {
  pub versioned_settings: VersionedSettings,
  pub paths: AppPaths,
}
impl Data {
  pub fn settings(&mut self) -> &mut Settings {
    self.versioned_settings.unwrap()
  }
  pub fn save_settings(&mut self) -> Result<(), String> {
    self.versioned_settings.save(&self.paths)
  }
}

pub fn to_json<T: Serialize>(data: &T) -> Result<Value, String> {
  match serde_json::to_value(data) {
    Ok(v) => Ok(v),
    Err(e) => throw!("Error serializing {}", e),
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
