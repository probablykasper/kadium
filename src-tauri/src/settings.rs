use crate::data::{write_atomically, AppPaths};
use crate::throw;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum VersionedSettings {
  V1(Settings),
}
impl Default for VersionedSettings {
  fn default() -> Self {
    Self::V1(Settings {
      api_key: "".to_string(),
      max_concurrent_requests: 5,
      channels: Vec::new(),
    })
  }
}
impl VersionedSettings {
  pub fn unwrap(&mut self) -> &mut Settings {
    match self {
      VersionedSettings::V1(user_data) => user_data,
    }
  }
  pub fn unwrap_ref(&self) -> &Settings {
    match self {
      VersionedSettings::V1(user_data) => user_data,
    }
  }
  pub fn load(paths: &AppPaths) -> Result<Self, String> {
    let mut settings_file = match File::open(&paths.settings_file) {
      Ok(file) => file,
      Err(e) => throw!("{}", e.to_string()),
    };
    let mut json_str = String::new();
    match settings_file.read_to_string(&mut json_str) {
      Ok(_) => {}
      Err(err) => throw!("Error reading file: {}", err),
    };
    match serde_json::from_str(&mut json_str) {
      Ok(settings) => Ok(settings),
      Err(err) => {
        throw!("Error parsing file: {}", err.to_string());
      }
    }
  }
  pub fn save(&self, paths: &AppPaths) -> Result<(), String> {
    let mut json = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut ser = serde_json::Serializer::with_formatter(&mut json, formatter);
    match self.serialize(&mut ser) {
      Ok(_) => {}
      Err(e) => throw!("Error saving content: {}", e.to_string()),
    }
    match write_atomically(&paths.settings_file, &json) {
      Ok(_) => {}
      Err(e) => throw!("Error saving: {}", e.to_string()),
    }
    Ok(())
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
  pub id: String,
  pub name: String,
  pub icon: String,
  pub uploads_playlist_id: String,
  pub from_time: i64,
  pub refresh_rate_ms: u64,
  pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
  pub api_key: String,
  pub max_concurrent_requests: u32,
  pub channels: Vec<Channel>,
}
impl Settings {
  pub fn wrap(self) -> VersionedSettings {
    VersionedSettings::V1(self)
  }
}

pub mod v1 {
  pub use super::{Channel, Settings};
}

#[allow(non_snake_case)]
pub mod yt_email_notifier {
  use crate::settings::v1;
  use crate::throw;
  use serde::{Deserialize, Serialize};
  use std::fs::File;
  use std::io::Read;
  use std::path::PathBuf;

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Channel {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub uploadsPlaylistId: String,
    pub fromTime: i64,
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Instance {
    pub email: String,
    pub minutesBetweenRefreshes: String,
    pub channels: Vec<Channel>,
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct Settings {
    pub apiKey: String,
    pub fromEmail: String,
    pub unreadErrors: bool,
    pub maxConcurrentRequests: u32,
    pub instances: Vec<Instance>,
  }
  fn load_settings(file_path: PathBuf) -> Result<Settings, String> {
    let mut file = match File::open(&file_path) {
      Ok(file) => file,
      Err(e) => throw!("Error opening file: {}", e.to_string()),
    };
    let mut json_str = String::new();
    match file.read_to_string(&mut json_str) {
      Ok(_) => {}
      Err(err) => throw!("Error reading file: {}", err),
    };
    let settings: Settings = match serde_json::from_str(&mut json_str) {
      Ok(v) => v,
      Err(err) => {
        throw!("Error parsing file: {}", err.to_string());
      }
    };
    Ok(settings)
  }
  fn convert(settings: Settings) -> v1::Settings {
    v1::Settings {
      api_key: settings.apiKey,
      max_concurrent_requests: settings.maxConcurrentRequests,
      channels: {
        let mut channels = Vec::new();
        for v1_instance in settings.instances.iter() {
          for v1_channel in v1_instance.channels.iter() {
            let refresh_rate_mins = v1_instance.minutesBetweenRefreshes.parse().unwrap_or(60.0);
            channels.push(v1::Channel {
              id: v1_channel.id.clone(),
              name: v1_channel.name.clone(),
              icon: v1_channel.icon.clone(),
              uploads_playlist_id: v1_channel.uploadsPlaylistId.clone(),
              from_time: v1_channel.fromTime,
              refresh_rate_ms: refresh_rate_mins as u64 * 60 * 1000,
              tags: vec![v1_instance.email.clone()],
            });
          }
        }
        channels
      },
    }
  }
  fn app_dir() -> PathBuf {
    let data_dir = tauri::api::path::data_dir().expect("No data dir");
    data_dir.join("YouTube Email Notifier")
  }
  pub fn can_import() -> bool {
    if cfg!(target_os = "macos") {
      app_dir().exists()
    } else {
      false
    }
  }
  pub struct ImportedStuff {
    pub settings: v1::Settings,
    pub update_note: String,
  }
  pub fn import() -> Result<ImportedStuff, String> {
    let app_dir = app_dir();
    let settings = match load_settings(app_dir.join("settings.json")) {
      Ok(settings) => settings,
      Err(err) => throw!("Error migrating v1 settings: {}", err),
    };
    let imported_user_data = convert(settings);
    Ok(ImportedStuff{
      settings: imported_user_data,
      update_note: "Your old settings and data has been imported.\n\
        \n\
        To enable \"Launch on Startup\", open System Preferences, go to Users & Groups > Login Items and add YouTube Email Notifier.".to_string(),
    })
  }
}
