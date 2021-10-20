use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::migration;
use crate::settings::Settings;
use crate::throw;

pub struct Data {
  settings: Settings,
  app_dir: PathBuf,
}
type UpdateNote = Option<String>;
impl Data {
  pub fn load(app_dir: PathBuf) -> Result<(Self, UpdateNote), String> {
    if app_dir.exists() {
      let settings_file_path = app_dir.join("settings.json");
      let settings_file = match File::open(&settings_file_path) {
        Ok(file) => file,
        Err(e) => throw!("{}", e.to_string()),
      };
      let settings = Settings::load(settings_file)?;
      Ok((Data { settings, app_dir }, None))
    } else {
      let data = match migration::load_migration_data() {
        Ok(Some(migrated_data)) => {
          let data = Data {
            settings: migrated_data.settings,
            app_dir: app_dir,
          };
          (data, Some(migrated_data.update_note))
        }
        Ok(None) => {
          let data = Data {
            settings: Settings::default(),
            app_dir: app_dir,
          };
          (data, None)
        }
        Err(e) => {
          throw!("{}", e.to_string());
        }
      };
      Ok(data)
    }
  }
}

pub struct ArcData(pub Arc<Mutex<Data>>);

impl ArcData {
  pub fn new(data: Data) -> Result<Self, String> {
    Ok(Self(Arc::new(Mutex::new(data))))
  }
}
