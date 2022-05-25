#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::data::{AppPaths, ArcData, Data};
use crate::menu::Item as MenuItem;
use crate::settings::yt_email_notifier;
use crate::settings::VersionedSettings;
use std::{env, thread};
use tauri::api::{dialog, shell};
use tauri::{command, CustomMenuItem, Manager, Submenu, Window, WindowBuilder, WindowUrl};

mod api;
mod background;
mod data;
mod db;
mod menu;
mod settings;

fn error_popup_main_thread(msg: impl AsRef<str>) {
  let msg = msg.as_ref().to_string();
  let builder = rfd::MessageDialog::new()
    .set_title("Error")
    .set_description(&msg)
    .set_buttons(rfd::MessageButtons::Ok)
    .set_level(rfd::MessageLevel::Info);
  builder.show();
}

#[macro_export]
macro_rules! throw {
  ($($arg:tt)*) => {{
    return Err(format!($($arg)*))
  }};
}

#[command]
fn error_popup(msg: String, win: Window) {
  eprintln!("Error: {}", msg);
  thread::spawn(move || {
    dialog::message(Some(&win), "Error", msg);
  });
}

fn custom_item(name: &str) -> CustomMenuItem {
  CustomMenuItem::new(name.to_string(), name)
}

/// Note title and message to show asynchronously when/after the app starts
type ImportedNote = Option<(String, String)>;

/// This can display dialogs, which needs to happen before tauri runs to not panic
async fn load_data(paths: &AppPaths) -> Result<(Data, ImportedNote), String> {
  let db_pool_future = db::init(&paths);
  if paths.settings_file.exists() {
    return match settings::VersionedSettings::load(&paths) {
      Ok(mut settings) => {
        let pool = db_pool_future.await?;
        let data = Data {
          bg_handle: background::spawn_bg(&settings.unwrap(), &pool),
          db_pool: pool,
          versioned_settings: settings,
          paths: paths.clone(),
          window_handle: None,
        };
        return Ok((data, None));
      }
      Err(e) => Err(e),
    };
  }

  let will_import = match yt_email_notifier::can_import() {
    true => {
      let msg = "Do you want to import your data from YouTube Email Notifier?";
      let wants_to_import = rfd::MessageDialog::new()
        .set_title("Import")
        .set_description(&msg)
        .set_buttons(rfd::MessageButtons::YesNo)
        .set_level(rfd::MessageLevel::Info)
        .show();
      wants_to_import
    }
    false => false,
  };
  if will_import {
    let imported_stuff = yt_email_notifier::import()?;
    let pool = db_pool_future.await?;
    let rt = background::spawn_bg(&imported_stuff.settings, &pool);
    let versioned_settings = imported_stuff.settings.wrap();
    versioned_settings.save(&paths)?;

    let data = Data {
      bg_handle: rt,
      db_pool: pool,
      versioned_settings,
      paths: paths.clone(),
      window_handle: None,
    };
    let import_note = Some(("Import note".to_string(), imported_stuff.update_note));
    return Ok((data, import_note));
  }

  let mut default_settings = VersionedSettings::default();
  let pool = db_pool_future.await?;
  let data = Data {
    bg_handle: background::spawn_bg(default_settings.unwrap(), &pool),
    db_pool: pool,
    versioned_settings: default_settings,
    paths: paths.clone(),
    window_handle: None,
  };
  Ok((data, None))
}

#[tokio::main]
async fn main() {
  if cfg!(debug_assertions) && env::var("DEVELOPMENT").is_err() {
    eprintln!(
      "Detected debug mode without the DEVELOPMENT environment \
      variable set. Set it using DEVELOPMENT=1. This is explicitly required \
      so you won't forget if you decide to run in release mode"
    );
    panic!();
  }
  let ctx = tauri::generate_context!();

  // macOS "App Nap" periodically pauses our app when it's in the background.
  // We need to prevent that so our intervals are not interrupted.
  #[cfg(target_os = "macos")]
  macos_app_nap::prevent();

  let app_paths = AppPaths::from_tauri_config(&ctx.config());
  let data_load_result = load_data(&app_paths).await;

  let (loaded_data, _note) = match data_load_result {
    Ok(v) => v,
    Err(e) => {
      error_popup_main_thread(&e);
      panic!("{}", e);
    }
  };

  let app = tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      error_popup,
      data::video_update_counter,
      data::get_settings,
      data::tags,
      data::set_channels,
      data::add_channel,
      data::set_general_settings,
      data::check_now,
      db::get_videos,
      db::archive,
      db::unarchive,
    ])
    .setup(move |_app| {
      #[cfg(target_os = "macos")]
      if let Some(note) = _note.clone() {
        thread::spawn(move || {
          dialog::message(Option::<&tauri::Window<tauri::Wry>>::None, note.0, note.1);
        });
      }
      Ok(())
    })
    .setup(|app| {
      let _ = WindowBuilder::new(app, "main", WindowUrl::default())
        .title("Kadium")
        .inner_size(900.0, 800.0)
        .min_inner_size(400.0, 150.0)
        .build()
        .expect("Unable to create window");
      Ok(())
    })
    .manage(ArcData::new(loaded_data))
    .menu(menu::new(vec![
      #[cfg(target_os = "macos")]
      MenuItem::Submenu(Submenu::new(
        &ctx.package_info().name,
        menu::new(vec![
          MenuItem::About(ctx.package_info().name.clone()),
          MenuItem::Separator,
          MenuItem::Custom(custom_item("Preferences...").accelerator("CmdOrCtrl+,")),
          MenuItem::Separator,
          MenuItem::Services,
          MenuItem::Separator,
          MenuItem::Hide,
          MenuItem::HideOthers,
          MenuItem::ShowAll,
          MenuItem::Separator,
          MenuItem::Quit,
        ]),
      )),
      MenuItem::Submenu(Submenu::new(
        "File",
        menu::new(vec![
          MenuItem::Custom(custom_item("Add Channel...").accelerator("CmdOrCtrl+N")),
          MenuItem::Custom(custom_item("Open")),
          MenuItem::Custom(custom_item("Open Channel")),
          MenuItem::Custom(custom_item("Archive").accelerator("CmdOrCtrl+Backspace")),
          MenuItem::Custom(custom_item("Unarchive").accelerator("Shift+CmdOrCtrl+Backspace")),
          MenuItem::Separator,
          #[cfg(not(target_os = "macos"))]
          MenuItem::Custom(custom_item("Options...").accelerator("CmdOrCtrl+,")),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator,
          MenuItem::CloseWindow,
        ]),
      )),
      MenuItem::Submenu(Submenu::new(
        "Edit",
        menu::new(vec![
          MenuItem::Undo,
          MenuItem::Redo,
          MenuItem::Separator,
          MenuItem::Cut,
          MenuItem::Copy,
          MenuItem::Paste,
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator,
          MenuItem::SelectAll,
          MenuItem::Separator,
          MenuItem::Custom(custom_item("Find").accelerator("CmdOrCtrl+F")),
        ]),
      )),
      MenuItem::Submenu(Submenu::new(
        "View",
        menu::new(vec![
          MenuItem::Custom(custom_item("Show New").accelerator("Alt+CmdOrCtrl+N")),
          MenuItem::Custom(custom_item("Show Archived").accelerator("Alt+CmdOrCtrl+E")),
          MenuItem::Custom(custom_item("Show All").accelerator("Alt+CmdOrCtrl+A")),
          MenuItem::Separator,
          MenuItem::EnterFullScreen,
        ]),
      )),
      MenuItem::Submenu(Submenu::new(
        "Window",
        menu::new(vec![
          MenuItem::Minimize,
          MenuItem::Zoom,
          MenuItem::Separator,
          MenuItem::Custom(custom_item("Videos").accelerator("Alt+CmdOrCtrl+1")),
          MenuItem::Custom(custom_item("Channels").accelerator("Alt+CmdOrCtrl+2")),
        ]),
      )),
      MenuItem::Submenu(Submenu::new(
        "Help",
        menu::new(vec![MenuItem::Custom(custom_item("Learn More"))]),
      )),
    ]))
    .on_menu_event(|event| match event.menu_item_id() {
      "Learn More" => {
        let url = "https://github.com/probablykasper/kadium";
        shell::open(&event.window().shell_scope(), url.to_string(), None).unwrap();
      }
      _ => {}
    })
    .build(ctx)
    .expect("Error running tauri app");

  app.run(|_app_handle, e| match e {
    tauri::RunEvent::WindowEvent { event, .. } => match event {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        if cfg!(target_os = "macos") {
          api.prevent_close();
          #[cfg(target_os = "macos")]
          _app_handle.hide().unwrap();
        }
      }
      _ => {}
    },
    _ => {}
  });
}
