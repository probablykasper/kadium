use crate::api::{channels, yt_request};
use crate::settings::{Channel, Settings, VersionedSettings};
use crate::{api, background, throw};
use atomicwrites::{AtomicFile, OverwriteBehavior};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{command, Config, State};
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone)]
pub struct AppPaths {
  pub app_dir: PathBuf,
  pub settings_file: PathBuf,
  pub db: String,
}
impl AppPaths {
  pub fn from_tauri_config(config: &Config) -> Self {
    let app_dir = match env::var("DEVELOPMENT").is_ok() {
      true => env::current_dir().unwrap().join("appdata"),
      false => tauri::api::path::app_data_dir(config).unwrap(),
    };
    AppPaths {
      app_dir: app_dir.clone(),
      settings_file: app_dir.join("Settings.json"),
      db: app_dir.join("Kadium.sqlite").to_string_lossy().to_string(),
    }
  }
}

pub struct Data {
  pub bg_handle: Option<background::BgHandle>,
  pub db_pool: SqlitePool,
  pub versioned_settings: VersionedSettings,
  pub paths: AppPaths,
  pub window: tauri::Window,
}
impl Data {
  pub fn settings(&mut self) -> &mut Settings {
    self.versioned_settings.unwrap()
  }
  pub fn settings_ref(&self) -> &Settings {
    self.versioned_settings.unwrap_ref()
  }
  pub fn restart_background(&mut self) -> Result<(), String> {
    if let Some(bg_handle) = self.bg_handle.take() {
      bg_handle.stop();
      bg_handle.wait_until_stopped()?;
    }
    self.bg_handle = background::spawn_bg(self.settings_ref(), &self.db_pool, self.window.clone());
    Ok(())
  }
  pub fn check_now(&mut self) -> Result<(), String> {
    if let Some(bg_handle) = self.bg_handle.take() {
      bg_handle.stop();
      bg_handle.wait_until_stopped()?;
    }
    self.bg_handle =
      background::spawn_bg_or_check_now(self.settings_ref(), &self.db_pool, self.window.clone());
    Ok(())
  }
  pub fn save_settings(&mut self) -> Result<(), String> {
    self.versioned_settings.save(&self.paths)?;
    self.restart_background()?;
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
  let mut tags: Vec<_> = tags_map.keys().collect();
  tags.sort();
  to_json(&tags)
}

#[command]
pub async fn check_now(data: DataState<'_>) -> Result<(), String> {
  let mut data = data.0.lock().await;
  data.check_now()?;
  Ok(())
}

#[derive(Deserialize)]
pub struct AddChannelOptions {
  pub url: String,
  pub from_time: i64,
  pub refresh_rate_ms: u64,
  pub tags: Vec<String>,
}

fn url_parse_video_id(value: &str) -> Option<String> {
  let url = Url::parse(value).ok()?;
  let host = url.host_str()?;
  if host.ends_with("youtube.com") && url.path().starts_with("/watch") {
    for (key, value) in url.query_pairs() {
      if key == "v" {
        return Some(value.to_string());
      }
    }
  } else if host.ends_with("youtu.be") {
    let first_seg = url.path_segments()?.next()?;
    return Some(first_seg.to_string());
  }
  None
}
fn url_parse_channel_id(value: &str) -> Option<String> {
  let url = Url::parse(value).ok()?;
  let host = url.host_str()?;
  if !host.ends_with("youtube.com") {
    return None;
  }
  let mut path_segments = url.path_segments()?;
  if path_segments.next()? != "channel" {
    return None;
  }
  return Some(path_segments.next()?.to_string());
}

#[command]
pub async fn set_channels(channels: Vec<Channel>, data: DataState<'_>) -> Result<(), String> {
  let mut data = data.0.lock().await;
  data.settings().channels = channels;
  data.save_settings()?;
  Ok(())
}

#[command]
pub async fn add_channel(options: AddChannelOptions, data: DataState<'_>) -> Result<(), String> {
  let mut data = data.0.lock().await;
  let settings = data.settings();
  let invalid = "Invalid URL. You could put in a video URL from the channel".to_string();

  let id = if let Some(video_id) = url_parse_video_id(&options.url) {
    let key = &settings.api_key_or_default();
    api::channel_id_from_video_id(&video_id, &key).await?
  } else if let Some(id) = url_parse_channel_id(&options.url) {
    id
  } else {
    return Err(invalid);
  };

  for channel in &settings.channels {
    if channel.id == id {
      throw!("Channel already exists");
    }
  }

  let url = "https://www.googleapis.com/youtube/v3/channels".to_owned()
    + "?part=contentDetails,id,snippet"
    + "&id="
    + &id;
  let channels = yt_request::<channels::Response>(&url, &settings.api_key_or_default())
    .await
    .map_err(|e| format!("Failed to get channel: {}", e))?;
  let channel = match channels.items.into_iter().next() {
    Some(channel) => channel,
    None => throw!("No channel found"),
  };

  settings.channels.push(Channel {
    id: channel.id,
    name: channel.snippet.title,
    icon: channel.snippet.thumbnails.medium.url,
    uploads_playlist_id: channel.contentDetails.relatedPlaylists.uploads,
    from_time: options.from_time,
    refresh_rate_ms: options.refresh_rate_ms,
    tags: options.tags,
  });
  data.save_settings()?;
  Ok(())
}

#[command]
pub async fn set_general_settings(
  api_key: String,
  max_concurrent_requests: u32,
  check_in_background: bool,
  data: DataState<'_>,
) -> Result<(), String> {
  let mut data = data.0.lock().await;
  data.settings().set_api_key(api_key);
  data.settings().max_concurrent_requests = max_concurrent_requests;
  data.settings().check_in_background = check_in_background;
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
