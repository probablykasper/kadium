use crate::api::{channels, yt_request};
use crate::settings::{Channel, Settings, VersionedSettings};
use crate::{api, background, throw};
use atomicwrites::{AtomicFile, OverwriteBehavior};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::convert::TryInto;
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{command, Config, Error, State};
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
			false => dirs::data_local_dir()
				.ok_or(Error::UnknownPath)
				.map(|dir| dir.join(&config.identifier))
				.unwrap(),
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
	pub window: tauri::WebviewWindow,
	pub user_history: UndoHistory,
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
		self.bg_handle =
			background::spawn_bg(self.settings_ref(), &self.db_pool, self.window.clone());
		Ok(())
	}
	pub fn check_now(&mut self) -> Result<(), String> {
		if let Some(bg_handle) = self.bg_handle.take() {
			bg_handle.stop();
			bg_handle.wait_until_stopped()?;
		}
		self.bg_handle = background::spawn_bg_or_check_now(
			self.settings_ref(),
			&self.db_pool,
			self.window.clone(),
		);
		Ok(())
	}
	pub fn save_settings(&mut self) -> Result<(), String> {
		self.versioned_settings.save(&self.paths)?;
		self.restart_background()?;
		Ok(())
	}
}

pub fn ensure_parent_exists(file_path: &Path) -> Result<(), String> {
	if let Some(parent) = file_path.parent() {
		if let Err(e) = std::fs::create_dir_all(parent) {
			throw!("Error creating parent folder: {}", e.to_string());
		}
	}
	Ok(())
}

pub fn write_atomically(file_path: &PathBuf, buf: &[u8]) -> Result<(), String> {
	ensure_parent_exists(file_path)?;
	let af = AtomicFile::new(file_path, OverwriteBehavior::AllowOverwrite);
	match af.write(|f| f.write_all(buf)) {
		Ok(_) => Ok(()),
		Err(e) => Err(e.to_string()),
	}
}

#[command]
#[specta::specta]
pub async fn get_settings(data: DataState<'_>) -> Result<Settings, String> {
	let mut data = data.0.lock().await;
	Ok(data.settings().clone())
}

#[command]
#[specta::specta]
pub async fn tags(data: DataState<'_>) -> Result<Vec<String>, String> {
	let data = data.0.lock().await;
	let mut tags_set: HashSet<String> = HashSet::new();
	for channel in &data.settings_ref().channels {
		for tag in &channel.tags {
			tags_set.insert(tag.clone());
		}
	}
	let mut tags: Vec<_> = tags_set.into_iter().collect();
	tags.sort();
	Ok(tags)
}

#[command]
#[specta::specta]
pub async fn check_now(data: DataState<'_>) -> Result<(), String> {
	let mut data = data.0.lock().await;
	data.check_now()?;
	data.user_history.push(Action::CheckNow);
	Ok(())
}

#[derive(Deserialize, Type)]
pub struct AddChannelOptions {
	pub url: String,
	#[specta(type = i32)] // tauri bigint fix
	pub from_time: i64,
	#[specta(type = u32)] // tauri bigint fix
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
	Some(path_segments.next()?.to_string())
}

fn url_parse_username(value: &str) -> Option<String> {
	let url = Url::parse(value).ok()?;
	let host = url.host_str()?;
	if !host.ends_with("youtube.com") {
		return None;
	}
	let mut path_segments = url.path_segments()?;
	if path_segments.next()? != "user" {
		return None;
	}
	Some(path_segments.next()?.to_string())
}

async fn get_channel_id_from_url(url: &str) -> Result<String, String> {
	let client = reqwest::Client::new();
	let text = client
		.get(url)
		.send()
		.await
		.map_err(|e| format!("API request failed: {}", e))?
		.text()
		.await
		.map_err(|e| format!("API response was not JSON: {}", e))?;

	let html = Html::parse_document(&text);
	let selector = Selector::parse("link[rel='canonical'][href*='youtube.com']").unwrap();
	let link_elements = html.select(&selector).collect::<Vec<_>>();
	if link_elements.len() > 1 {
		return Err("Multiple canonical URL elements in the page".to_string());
	}
	let link_element = link_elements
		.first()
		.ok_or("No canonical URL element in the page")?;
	let canonical_url = link_element
		.attr("href")
		.ok_or("No canonical URL href in the page")?;

	url_parse_channel_id(canonical_url).ok_or(format!("Unexpected canonical URL: {canonical_url}"))
}

async fn get_id_from_url(url: &str, key: &str) -> Result<String, String> {
	if let Some(video_id) = url_parse_video_id(&url) {
		api::channel_id_from_video_id(&video_id, key).await
	} else if let Some(id) = url_parse_channel_id(&url) {
		Ok(id)
	} else if let Some(username) = url_parse_username(&url) {
		api::channel_id_from_username(&username, key).await
	} else {
		get_channel_id_from_url(&url).await.map_err(|e| {
			format!(
				"Invalid URL. You could try a video URL from the channel.\n\n{}",
				e
			)
		})
	}
}

#[command]
#[specta::specta]
pub async fn set_channels(channels: Vec<Channel>, data: DataState<'_>) -> Result<(), String> {
	let mut data = data.0.lock().await;
	data.settings().channels = channels;
	data.save_settings()?;
	data.user_history
		.push(Action::UpdateOrDeleteChannels("".to_string()));
	Ok(())
}

#[command]
#[specta::specta]
pub async fn add_channel(options: AddChannelOptions, data: DataState<'_>) -> Result<(), String> {
	let mut data = data.0.lock().await;
	let settings = data.settings();

	let id = get_id_from_url(&options.url, &settings.api_key_or_default()).await?;

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
		id: channel.id.clone(),
		name: channel.snippet.title,
		icon: channel.snippet.thumbnails.medium.url,
		uploads_playlist_id: channel.contentDetails.relatedPlaylists.uploads,
		from_time: options.from_time,
		refresh_rate_ms: options.refresh_rate_ms,
		tags: options.tags,
	});
	data.save_settings()?;
	data.user_history.push(Action::AddChannel(channel.id));
	Ok(())
}

#[command]
#[specta::specta]
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

#[derive(Serialize, Clone, Type)]
pub struct UndoHistory {
	pub entries: Vec<(u32, Action)>,
}

impl UndoHistory {
	pub fn new() -> Self {
		Self { entries: vec![] }
	}
	pub fn push(&mut self, action: Action) {
		let time: u32 = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_secs()
			.try_into()
			.unwrap();
		self.entries.push((time, action));
		if self.entries.len() > 100 {
			self.entries.remove(0);
		}
	}
}

#[derive(Serialize, Clone, Type)]
pub enum Action {
	CheckNow,
	Archive(String),
	Unarchive(String),
	AddChannel(String),
	UpdateOrDeleteChannels(String),
}

#[command]
#[specta::specta]
pub async fn get_history(data: DataState<'_>) -> Result<UndoHistory, String> {
	let data = data.0.lock().await;
	Ok(data.user_history.clone())
}
