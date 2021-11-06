use crate::settings::{Channel, Settings, VersionedSettings};
use crate::{background, throw};
use atomicwrites::{AtomicFile, OverwriteBehavior};
use serde::Serialize;
use serde_json::Value;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{command, Config, State};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppPaths {
  pub app_dir: PathBuf,
  pub settings_file: PathBuf,
  pub db: String,
}
impl AppPaths {
  pub fn from_tauri_config(config: &Config) -> Self {
    let app_dir = tauri::api::path::app_dir(config).unwrap();
    AppPaths {
      app_dir: app_dir.clone(),
      settings_file: app_dir.join("Settings.json"),
      db: app_dir.join("Kadium.sqlite").to_string_lossy().to_string(),
    }
  }
}

pub struct Data {
  pub bg_handle: Option<background::FetcherHandle>,
  pub db_pool: SqlitePool,
  pub versioned_settings: VersionedSettings,
  pub paths: AppPaths,
  pub window_handle: Option<tauri::Window>,
}
impl Data {
  pub fn settings(&mut self) -> &mut Settings {
    self.versioned_settings.unwrap()
  }
  pub fn settings_ref(&self) -> &Settings {
    self.versioned_settings.unwrap_ref()
  }
  pub fn save_settings(&mut self) -> Result<(), String> {
    self.versioned_settings.save(&self.paths)?;
    if let Some(bg_handle) = self.bg_handle.take() {
      bg_handle.stop();
      bg_handle.wait_until_stopped()?;
    }
    self.bg_handle = background::spawn(self.settings_ref(), &self.db_pool);
    Ok(())
  }
}

pub fn to_json<T: Serialize>(data: &T) -> Result<Value, String> {
  match serde_json::to_value(data) {
    Ok(v) => Ok(v),
    Err(e) => throw!("Error serializing {}", e),
  }
}

pub fn ensure_parent_exists(file_path: &PathBuf) -> Result<(), String> {
  if let Some(parent) = file_path.parent() {
    if let Err(e) = std::fs::create_dir_all(parent) {
      throw!("Error creating parent folder: {}", e.to_string());
    }
  }
  Ok(())
}

pub fn write_atomically(file_path: &PathBuf, buf: &[u8]) -> Result<(), String> {
  ensure_parent_exists(&file_path)?;
  let af = AtomicFile::new(&file_path, OverwriteBehavior::AllowOverwrite);
  match af.write(|f| f.write_all(&buf)) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

#[command]
pub async fn video_update_counter(data: DataState<'_>) -> Result<u64, String> {
  let data = data.0.lock().await;
  match &data.bg_handle {
    Some(bg_handle) => {
      let count = bg_handle.update_counter.lock().await;
      Ok(count.clone())
    }
    None => Ok(0),
  }
}

#[command]
pub async fn get_settings(data: DataState<'_>) -> Result<Value, String> {
  let mut data = data.0.lock().await;
  to_json(&data.settings())
}

#[command]
pub async fn tags(data: DataState<'_>) -> Result<Value, String> {
  let data = data.0.lock().await;
  let mut tags_map: HashMap<&str, ()> = HashMap::new();
  for channel in &data.settings_ref().channels {
    for tag in &channel.tags {
      tags_map.entry(&tag).or_insert(());
    }
  }
  let tags: Vec<_> = tags_map.keys().collect();
  to_json(&tags)
}

#[command]
pub async fn set_channels(channels: Vec<Channel>, data: DataState<'_>) -> Result<(), String> {
  let mut data = data.0.lock().await;
  data.settings().channels = channels;
  data.save_settings()?;
  Ok(())
}

#[command]
pub async fn set_general_settings(
  api_key: String,
  max_concurrent_requests: u32,
  data: DataState<'_>,
) -> Result<(), String> {
  let mut data = data.0.lock().await;
  data.settings().api_key = api_key;
  data.settings().max_concurrent_requests = max_concurrent_requests;
  data.save_settings()?;
  Ok(())
}

pub type DataState<'a> = State<'a, ArcData>;
pub struct ArcData(pub Arc<Mutex<Data>>);
impl ArcData {
  pub fn new(data: Data) -> Self {
    Self(Arc::new(Mutex::new(data)))
  }
}
