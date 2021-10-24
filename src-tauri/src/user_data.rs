use crate::settings::Settings;
use crate::throw;
use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum VersionedUserData {
  V1(UserData),
}
impl VersionedUserData {
  pub fn unwrap(self) -> UserData {
    match self {
      VersionedUserData::V1(user_data) => user_data,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
  pub update_note: Option<String>,
  pub settings: Settings,
}

impl UserData {
  pub fn load(app_dir: &PathBuf) -> Result<Self, String> {
    Ok(UserData {
      settings: {
        let mut settings_file = match File::open(&get_settings_file_path(&app_dir)) {
          Ok(file) => file,
          Err(e) => throw!("{}", e.to_string()),
        };
        let mut json_str = String::new();
        match settings_file.read_to_string(&mut json_str) {
          Ok(_) => {}
          Err(err) => throw!("Error reading file: {}", err),
        };
        match serde_json::from_str(&mut json_str) {
          Ok(settings) => settings,
          Err(err) => {
            throw!("Error parsing file: {}", err.to_string());
          }
        }
      },
      update_note: None,
    })
  }
  pub fn save(&self, app_dir: &PathBuf) -> Result<(), String> {
    let mut json = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut ser = serde_json::Serializer::with_formatter(&mut json, formatter);
    match self.settings.serialize(&mut ser) {
      Ok(_) => {}
      Err(e) => throw!("Error saving content: {}", e.to_string()),
    }
    let settings_file_path = get_settings_file_path(&app_dir);
    match write_atomically(settings_file_path, &json) {
      Ok(_) => {}
      Err(e) => throw!("Error saving: {}", e.to_string()),
    }
    Ok(())
  }
}

pub fn write_atomically(file_path: PathBuf, buf: &[u8]) -> Result<(), String> {
  if let Some(parent) = file_path.parent() {
    if let Err(e) = std::fs::create_dir_all(parent) {
      throw!("Error creating parent folder: {}", e.to_string());
    }
  }
  let af = AtomicFile::new(&file_path, AllowOverwrite);
  match af.write(|f| f.write_all(&buf)) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

pub fn get_settings_file_path(app_dir: &PathBuf) -> PathBuf {
  app_dir.join("settings.json")
}

pub mod v1 {
  pub use super::UserData;
  pub use crate::settings;
}

#[allow(non_snake_case)]
pub mod yt_email_notifier {
  use crate::throw;
  use crate::user_data::{v1, VersionedUserData};
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
    pub fromTime: u64,
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
  fn convert(settings: Settings) -> v1::UserData {
    v1::UserData {
      // update_note: "Your old settings and data has been imported.\n\nTo re-enable \"Launch on Startup\", open System Preferences, go to Users & Groups > Login Items and add YouTube Email Notifier.".to_string(),
      update_note:
      Some("Your old settings and data has been imported.\n\
      \n\
      To re-enable \"Launch on Startup\", open System Preferences, go to Users & Groups > Login Items and add YouTube Email Notifier.".to_string()),
      settings: v1::settings::Settings {
        api_key: settings.apiKey,
        max_concurrent_requests: settings.maxConcurrentRequests,
        groups: settings
          .instances
          .into_iter()
          .map(|v1_instance| v1::settings::Group {
            name: v1_instance.email,
            minutes_between_refreshes: v1_instance.minutesBetweenRefreshes.parse().unwrap_or(60.0),
            channels: v1_instance
              .channels
              .into_iter()
              .map(|v1_channel| v1::settings::Channel {
                id: v1_channel.id,
                name: v1_channel.name,
                icon: v1_channel.icon,
                uploads_playlist_id: v1_channel.uploadsPlaylistId,
                from_time: v1_channel.fromTime,
              })
              .collect(),
          })
          .collect(),
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
  pub fn import() -> Result<Option<VersionedUserData>, String> {
    if can_import() {
      let app_dir = app_dir();
      let settings = match load_settings(app_dir.join("settings.json")) {
        Ok(settings) => settings,
        Err(err) => throw!("Error migrating v1 settings: {}", err),
      };
      let imported_user_data = VersionedUserData::V1(convert(settings));
      Ok(Some(imported_user_data))
    } else {
      Ok(None)
    }
  }
}
