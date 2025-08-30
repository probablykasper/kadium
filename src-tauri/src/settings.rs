use crate::data::{write_atomically, AppPaths};
use crate::throw;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs::File;
use std::io::Read;

pub fn default_key() -> String {
	let key = vec![
		65, 73, 122, 97, 83, 121, 68, 52, 50, 110, 65, 76, 52, 57, 118, 48, 108, 100, 121, 99, 110,
		100, 49, 78, 79, 113, 71, 111, 114, 54, 54, 95, 56, 107, 108, 83, 78, 102, 48,
	];
	String::from_utf8(key).unwrap()
}

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
			check_in_background: true,
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
		match serde_json::from_str(&json_str) {
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

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct Channel {
	pub id: String,
	pub name: String,
	pub icon: String,
	pub uploads_playlist_id: String,
	#[specta(type = i32)] // tauri bigint fix
	pub from_time: i64,
	#[specta(type = u32)] // tauri bigint fix
	pub refresh_rate_ms: u64,
	pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Settings {
	api_key: String,
	pub max_concurrent_requests: u32,
	pub channels: Vec<Channel>,
	pub check_in_background: bool,
}
impl Settings {
	pub fn wrap(self) -> VersionedSettings {
		VersionedSettings::V1(self)
	}
	pub fn api_key_or_default(&self) -> String {
		if self.api_key == "" {
			default_key()
		} else {
			self.api_key.clone()
		}
	}
	pub fn set_api_key(&mut self, key: String) {
		self.api_key = key;
	}
}
