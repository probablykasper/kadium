pub fn load_data() -> Result<Option<v2::Data>, String> {
  v2::load_from_v1()
}

mod v2 {
  use crate::migration::v1;
  use crate::settings::{Channel, Group, Settings};
  pub struct Data {
    pub settings: Settings,
  }
  pub fn load_from_v1() -> Result<Option<Data>, String> {
    match v1::load_data() {
      Ok(Some(v1_data)) => Ok(Some(data_from_v1(v1_data))),
      Ok(None) => Ok(None),
      Err(e) => Err(e),
    }
  }
  fn data_from_v1(v1_data: v1::Data) -> Data {
    Data {
      settings: Settings {
        api_key: v1_data.settings.apiKey,
        from_email: v1_data.settings.fromEmail,
        unread_errors: v1_data.settings.unreadErrors,
        max_concurrent_requests: v1_data.settings.maxConcurrentRequests,
        groups: v1_data
          .settings
          .instances
          .into_iter()
          .map(|v1_instance| Group {
            email: v1_instance.email,
            minutes_between_refreshes: v1_instance.minutesBetweenRefreshes.parse().unwrap_or(60.0),
            channels: v1_instance
              .channels
              .into_iter()
              .map(|v1_channel| Channel {
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
}

#[allow(non_snake_case)]
mod v1 {
  use crate::throw;
  use serde::{Deserialize, Serialize};
  use serde_json::Value;
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
  pub struct Data {
    pub settings: Settings,
  }
  pub fn load_data() -> Result<Option<Data>, String> {
    if cfg!(target_os = "macos") {
      let data_dir = tauri::api::path::data_dir().expect("No data dir");
      let app_dir = data_dir.join("YouTube Email Notifier");

      if data_dir.exists() {
        let settings = match load_settings(app_dir.join("settings.json")) {
          Ok(settings) => settings,
          Err(err) => throw!("Error migrating v1 settings: {}", err),
        };
        return Ok(Some(Data { settings }));
      }
    }
    Ok(None)
  }
}
