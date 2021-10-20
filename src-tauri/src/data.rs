use crate::settings::Settings;
use crate::throw;
use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub fn get_settings_file_path(app_dir: &PathBuf) -> PathBuf {
  app_dir.join("settings.json")
}

/// Write
fn write_atomically(file_path: PathBuf, buf: &[u8]) -> Result<(), String> {
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

pub struct Data {
  pub settings: Settings,
  pub app_dir: PathBuf,
}
impl Data {
  pub fn load(app_dir: PathBuf) -> Result<Self, String> {
    let settings_file = match File::open(&get_settings_file_path(&app_dir)) {
      Ok(file) => file,
      Err(e) => throw!("{}", e.to_string()),
    };
    let settings = Settings::load(settings_file)?;
    Ok(Data { settings, app_dir })
  }
  pub fn save_settings(&self) -> Result<(), String> {
    let mut json = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
    let mut ser = serde_json::Serializer::with_formatter(&mut json, formatter);
    match self.settings.serialize(&mut ser) {
      Ok(_) => {}
      Err(e) => throw!("Error saving content: {}", e.to_string()),
    }

    let settings_file_path = get_settings_file_path(&self.app_dir);
    match write_atomically(settings_file_path, &json) {
      Ok(_) => {}
      Err(e) => throw!("Error saving: {}", e.to_string()),
    }
    Ok(())
  }
}

pub struct ArcData(pub Arc<Mutex<Data>>);

impl ArcData {
  pub fn new(data: Data) -> Result<Self, String> {
    Ok(Self(Arc::new(Mutex::new(data))))
  }
}
