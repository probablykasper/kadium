use crate::settings::Settings;
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
  pub settings: Settings,
  pub app_dir: PathBuf,
}

#[command]
pub fn get_settings(data: State<ArcData>) -> Result<Value, String> {
  let data = data.0.lock().unwrap();
  to_json(&data.settings)
}

pub struct ArcData(pub Arc<Mutex<Data>>);

impl ArcData {
  pub fn new(data: Data) -> Self {
    Self(Arc::new(Mutex::new(data)))
  }
}
