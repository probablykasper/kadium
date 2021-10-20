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
impl Data {
  pub fn load(app_dir: PathBuf) -> Result<Self, String> {
    if app_dir.exists() {
      let settings_file_path = app_dir.join("settings.json");
      let settings_file = match File::open(&settings_file_path) {
        Ok(file) => file,
        Err(e) => throw!("{}", e.to_string()),
      };
      let settings = Settings::load(settings_file)?;
      Ok(Data { settings, app_dir })
    } else {
      let data = match migration::load_migration_data() {
        Ok(Some(migrated_data)) => {
          #[cfg(not(feature = "skip_migration_note"))]
          crate::info_popup_main_thread(migrated_data.update_note);
          Data {
            settings: migrated_data.settings,
            app_dir: app_dir,
          }
        }
        Ok(None) => Data {
          settings: Settings::default(),
          app_dir: app_dir,
        },
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
  pub fn load(app_dir: PathBuf) -> Result<Self, String> {
    let data = Data::load(app_dir)?;
    Ok(Self(Arc::new(Mutex::new(data))))
  }
}
