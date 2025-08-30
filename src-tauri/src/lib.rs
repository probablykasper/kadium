#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use crate::data::{AppPaths, ArcData, Data};
use crate::settings::VersionedSettings;
use data::UndoHistory;
use tauri::{command, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

mod api;
mod background;
mod data;
mod db;
mod settings;

fn error_popup_main_thread(msg: impl AsRef<str>) {
	let msg = msg.as_ref().to_string();
	let builder = rfd::MessageDialog::new()
		.set_title("Error")
		.set_description(&msg)
		.set_buttons(rfd::MessageButtons::Ok)
		.set_level(rfd::MessageLevel::Error);
	builder.show();
}

#[macro_export]
macro_rules! throw {
	($($arg:tt)*) => {{
		return Err(format!($($arg)*))
	}};
}

#[command]
#[specta::specta]
fn error_popup(app_handle: tauri::AppHandle, msg: String) {
	eprintln!("Error: {}", msg);

	app_handle
		.dialog()
		.message(&msg)
		.kind(MessageDialogKind::Error)
		.title("Error")
		.show(|_| {});
}

/// This can display dialogs, which needs to happen before tauri runs to not panic
fn load_data(paths: &AppPaths) -> Result<VersionedSettings, String> {
	if paths.settings_file.exists() {
		return match settings::VersionedSettings::load(paths) {
			Ok(settings) => Ok(settings),
			Err(e) => Err(e),
		};
	}

	Ok(VersionedSettings::default())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
	let specta_builder =
		tauri_specta::Builder::<tauri::Wry>::new().commands(tauri_specta::collect_commands![
			error_popup,
			data::get_settings,
			data::tags,
			data::set_channels,
			data::add_channel,
			data::set_general_settings,
			data::check_now,
			data::get_history,
			db::get_videos,
			db::archive,
			db::unarchive
		]);

	#[cfg(debug_assertions)]
	specta_builder
		.export(specta_typescript::Typescript::default(), "../bindings.ts")
		.expect("Failed to export typescript bindings");

	let ctx = tauri::generate_context!();

	// macOS "App Nap" periodically pauses our app when it's in the background.
	// We need to prevent that so our intervals are not interrupted.
	#[cfg(target_os = "macos")]
	macos_app_nap::prevent();

	let app_paths = AppPaths::from_tauri_config(ctx.config());

	let mut settings = match load_data(&app_paths) {
		Ok(v) => v,
		Err(e) => {
			error_popup_main_thread(&e);
			panic!("{}", e);
		}
	};

	let pool = match db::init(&app_paths).await {
		Ok(pool) => pool,
		Err(e) => {
			error_popup_main_thread(&e);
			panic!("{}", e);
		}
	};

	let app = tauri::Builder::default()
		.plugin(tauri_plugin_os::init())
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_notification::init())
		.invoke_handler(specta_builder.invoke_handler())
		.setup(move |app| {
			let win = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
				.title("Kadium")
				.inner_size(900.0, 800.0)
				.min_inner_size(400.0, 150.0)
				.decorations(!settings.unwrap_ref().no_window_decorations)
				.theme(Some(tauri::Theme::Dark));

			#[cfg(target_os = "macos")]
			let win = win.title_bar_style(tauri::TitleBarStyle::Transparent);

			let win = win.build().expect("Unable to create window");

			#[cfg(target_os = "macos")]
			{
				// set window background color because Tauri's .background_color() doesn't work
				use cocoa::appkit::NSWindow;
				let nsw = win.ns_window().unwrap() as cocoa::base::id;
				unsafe {
					let bg_color = cocoa::appkit::NSColor::colorWithRed_green_blue_alpha_(
						cocoa::base::nil,
						34.0 / 255.0,
						38.0 / 255.0,
						45.5 / 255.0,
						1.0,
					);
					nsw.setBackgroundColor_(bg_color);
				}
			}

			let data = Data {
				bg_handle: background::spawn_bg(settings.unwrap(), &pool, win.clone()),
				db_pool: pool,
				versioned_settings: settings,
				paths: app_paths,
				window: win.clone(),
				user_history: UndoHistory::new(),
			};
			app.manage(ArcData::new(data));

			Ok(())
		})
		.build(ctx)
		.expect("Error running tauri app");

	app.run(|_app_handle, e| match e {
		tauri::RunEvent::WindowEvent { event, .. } => match event {
			tauri::WindowEvent::CloseRequested { api: _api, .. } => {
				#[cfg(target_os = "macos")]
				{
					_api.prevent_close();
					_app_handle.hide().unwrap();
				}
			}
			_ => {}
		},
		_ => {}
	});
}
