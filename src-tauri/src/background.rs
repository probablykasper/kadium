use crate::api::{playlist_items, videos, yt_request};
use crate::{db, settings, throw};
use chrono::{DateTime, FixedOffset};
use iso8601_duration::Duration as IsoDuration;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::broadcast;
use tokio::{task, time};

pub struct IntervalInfo {
	pub ms: u64,
	pub channels: Vec<ChannelInfo>,
}
pub struct ChannelInfo {
	pub name: String,
	pub uploads_playlist_id: String,
	pub from_time: i64,
}

pub struct BgHandle {
	pub handle: thread::JoinHandle<Result<(), String>>,
	pub stop_sender: broadcast::Sender<()>,
}

impl BgHandle {
	pub fn stop(&self) {
		println!("Stopping tasks if running");
		// Error can only occur when channel is already closed
		let _ = self.stop_sender.send(());
	}
	pub fn wait_until_stopped(self) -> Result<(), String> {
		match self.handle.join() {
			Ok(result) => result,
			Err(e) => {
				if let Some(e) = e.downcast_ref::<&'static str>() {
					throw!("Interval thread error: {}", e);
				} else {
					throw!("Unknown interval thread error: {:?}", e);
				}
			}
		}
	}
}

pub fn spawn_bg(
	settings: &settings::Settings,
	pool: &SqlitePool,
	window: tauri::WebviewWindow,
) -> Option<BgHandle> {
	if settings.check_in_background {
		spawn(settings, pool, false, window)
	} else {
		None
	}
}
pub fn spawn_bg_or_check_now(
	settings: &settings::Settings,
	pool: &SqlitePool,
	window: tauri::WebviewWindow,
) -> Option<BgHandle> {
	if settings.check_in_background {
		spawn(settings, pool, false, window)
	} else {
		spawn(settings, pool, true, window)
	}
}

fn spawn(
	settings: &settings::Settings,
	pool: &SqlitePool,
	run_once: bool,
	window: tauri::WebviewWindow,
) -> Option<BgHandle> {
	if settings.channels.is_empty() {
		return None;
	}

	let interval_map = new_intervals_map(&settings.channels);
	let interval_infos = interval_map.into_values().collect();

	let (stop_sender, _stop_receiver) = broadcast::channel(1);

	let options = IntervalOptions {
		pool: pool.clone(),
		key: settings.api_key_or_default(),
		stop_sender: stop_sender.clone(),
		run_once,
		window,
	};

	let tokio_thread = thread::spawn(move || start(options, interval_infos));

	Some(BgHandle {
		handle: tokio_thread,
		stop_sender,
	})
}

pub type IntervalMap = HashMap<u64, IntervalInfo>;
fn new_intervals_map(channels: &[settings::Channel]) -> IntervalMap {
	let mut intervals_map: IntervalMap = HashMap::new();
	for channel in channels.iter() {
		let default = IntervalInfo {
			ms: channel.refresh_rate_ms,
			channels: Vec::new(),
		};
		let interval_info = intervals_map
			.entry(channel.refresh_rate_ms)
			.or_insert(default);
		interval_info.channels.push(ChannelInfo {
			name: channel.name.to_string(),
			uploads_playlist_id: channel.uploads_playlist_id.clone(),
			from_time: channel.from_time,
		});
	}
	intervals_map
}

#[derive(Clone)]
struct IntervalOptions {
	pool: SqlitePool,
	key: String,
	stop_sender: broadcast::Sender<()>,
	run_once: bool,
	window: tauri::WebviewWindow,
}

#[tokio::main]
async fn start(options: IntervalOptions, interval_infos: Vec<IntervalInfo>) -> Result<(), String> {
	let mut tasks = Vec::new();
	for interval_info in interval_infos {
		let options = options.clone();
		let mut stop_receiver = options.stop_sender.subscribe();
		let handle = task::spawn(async move {
			tokio::select! {
				_ = run(options, interval_info) => Ok(()),
				result = stop_receiver.recv() => {
					match result {
						Ok(_) => Ok(()),
						Err(e) => {
						Err(e.to_string())
						}
					}
				}
			}
		});
		tasks.push(handle);
	}
	for task in tasks {
		match task.await {
			Ok(result) => result?,
			Err(e) => {
				return Err(e.to_string());
			}
		}
	}
	Ok(())
}

async fn run(options: IntervalOptions, interval_info: IntervalInfo) {
	if options.run_once {
		run_interval_once(options, interval_info).await;
	} else {
		run_interval(options, interval_info).await;
	}
}

async fn run_interval_once(options: IntervalOptions, interval_info: IntervalInfo) {
	println!("Start checking once");
	check_channels(&options, &interval_info).await;
	println!("Done checking once");
}

async fn run_interval(options: IntervalOptions, interval_info: IntervalInfo) {
	let mut interval = time::interval(Duration::from_millis(interval_info.ms));
	interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
	loop {
		interval.tick().await;
		println!("Start checking {}ms task", interval_info.ms);
		check_channels(&options, &interval_info).await;
		println!("Done checking {}ms task", interval_info.ms);
	}
}

async fn check_channels(options: &IntervalOptions, interval_info: &IntervalInfo) {
	let app = options.window.app_handle();
	let window_visible = match options.window.is_visible() {
		Ok(is_visible) => is_visible,
		Err(e) => {
			eprintln!("{}", e);
			app.notification()
				.builder()
				.title("Failed to check channels")
				.body(e.to_string())
				.show()
				.expect("Unable to show notification");
			return;
		}
	};
	if window_visible {
		let _ = options.window.emit("checking", "");
	}
	for channel in &interval_info.channels {
		match check_channel(options, channel).await {
			Ok(()) => {}
			Err(e) => {
				let title = format!("Error checking {}", channel.name);
				eprintln!("{}: {}", title, e);
				app.notification()
					.builder()
					.title(title)
					.body(e)
					.show()
					.expect("Unable to show notification");
				break;
			}
		}
	}
	if window_visible {
		let _ = options.window.emit("doneChecking", "");
	}
}

/// Returns the number of new videos saved
async fn check_channel(options: &IntervalOptions, channel: &ChannelInfo) -> Result<(), String> {
	println!("Checking {} {}", channel.uploads_playlist_id, channel.name);
	let url = "https://www.googleapis.com/youtube/v3/playlistItems".to_string()
		+ "?part=contentDetails"
		+ "&maxResults=50"
		+ "&playlistId="
		+ &channel.uploads_playlist_id;
	let uploads = yt_request::<playlist_items::Response>(&url, &options.key)
		.await
		.map_err(|e| format!("Failed to get channel: {}", e))?;

	if uploads.items.is_empty() {
		return Ok(()); // no channel videos returned
	}

	let existing_ids = db::get_ids(&uploads.items, &options.pool).await?;

	// check which videos are new
	let mut new_ids: Vec<String> = Vec::new();
	for fetched_video in uploads.items {
		let fetched_id = &fetched_video.contentDetails.videoId;
		if existing_ids.contains(fetched_id) {
			continue;
		}

		let published_str = match fetched_video.contentDetails.videoPublishedAt {
			Some(published_str) => published_str,
			None => {
				// video isn't public
				continue;
			}
		};
		let published_time = parse_datetime(&published_str)?.timestamp_millis();
		if published_time < channel.from_time {
			continue;
		}

		new_ids.push(fetched_video.contentDetails.videoId);
	}

	if new_ids.is_empty() {
		return Ok(()); // no new videos
	}

	// get info about the videos
	println!(
		"Checking videos from {} {}",
		channel.uploads_playlist_id, channel.name
	);
	let url = "https://www.googleapis.com/youtube/v3/videos".to_string()
		+ "?part=contentDetails,liveStreamingDetails,snippet"
		+ "&id="
		+ &new_ids.join(",");
	let videos = yt_request::<videos::Response>(&url, &options.key)
		.await
		.map_err(|e| format!("Failed to get videos: {}", e))?;

	let mut videos_to_add: Vec<db::Video> = Vec::new();
	for video in videos.items {
		let content_details = video.contentDetails.ok_or("No contentDetails")?;
		let duration = match content_details.duration {
			Some(duration) => duration,
			None => {
				// Scheduled live streams don't have a duration
				continue;
			}
		};
		let publish_time = match video.liveStreamingDetails {
			Some(live_streaming_details) => match live_streaming_details.actualStartTime {
				Some(actual_start_time) => parse_datetime(&actual_start_time)?,
				None => {
					// skip future livestreams
					continue;
				}
			},
			None => parse_datetime(&video.snippet.publishedAt)?,
		};
		let duration_ms = parse_absolute_duration(&duration)?;
		videos_to_add.push(db::Video {
			id: video.id,
			title: video.snippet.title,
			description: video.snippet.description,
			publishTimeMs: publish_time.timestamp_millis(),
			durationMs: duration_ms,
			thumbnailStandard: video.snippet.thumbnails.standard.is_some(),
			thumbnailMaxres: video.snippet.thumbnails.maxres.is_some(),
			channelId: video.snippet.channelId,
			channelName: video.snippet.channelTitle,
			unread: true,
			archived: false,
		});
	}

	for video in &videos_to_add {
		db::insert_video(video, &options.pool).await?;
	}
	if !videos_to_add.is_empty() {
		match options.window.emit("refresh", "") {
			Ok(_) => {}
			Err(e) => {
				return Err(format!("Failed to emit refresh: {}", e));
			}
		};
	}
	Ok(())
}

pub fn parse_datetime(value: &str) -> Result<DateTime<FixedOffset>, String> {
	match DateTime::parse_from_rfc3339(value) {
		Ok(datetime) => Ok(datetime),
		Err(e) => throw!("Unexpected video publish date: {}", e),
	}
}
/// Parse a duration that cannot include year or month, because
/// years and months have different lengths depending on what month or year
/// it is.
pub fn parse_absolute_duration(value: &str) -> Result<i64, String> {
	match IsoDuration::parse(value) {
		Ok(duration) => {
			if duration.month == 0.0 && duration.year == 0.0 {
				let seconds = duration.second as f64
					+ duration.minute as f64 * 60.0
					+ duration.hour as f64 * 60.0 * 60.0
					+ duration.day as f64 * 60.0 * 60.0 * 24.0;
				let ms = seconds * 1000.0;
				let ms_clamped = ms.max(std::i64::MIN as f64).min(std::i64::MAX as f64);
				let ms_int = ms_clamped.round() as i64;
				Ok(ms_int)
			} else {
				throw!("Cannot parse duration with year or month: {}", value);
			}
		}
		Err(e) => throw!("Unexpected video duration: {e:?}"),
	}
}
