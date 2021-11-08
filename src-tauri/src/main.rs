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
use tauri::{command, CustomMenuItem, Submenu, Window, WindowBuilder, WindowUrl};
use tokio::runtime::Runtime;

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
  println!("Error: {}", msg);
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
          bg_handle: background::spawn(&settings.unwrap(), &pool),
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
    let rt = background::spawn(&imported_stuff.settings, &pool);
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
    bg_handle: background::spawn(default_settings.unwrap(), &pool),
    db_pool: pool,
    versioned_settings: default_settings,
    paths: paths.clone(),
    window_handle: None,
  };
  Ok((data, None))
}

const MAIN_WIN: &str = "main";

fn main() {
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
  macos_app_nap::prevent();

  let app_paths = AppPaths::from_tauri_config(&ctx.config());

  let data_load_result = match Runtime::new() {
    Ok(runtime) => runtime.block_on(async {
      // load data in separate async thread
      // workaround for https://github.com/tauri-apps/tauri/issues/2838
      return load_data(&app_paths).await;
    }),
    Err(e) => Err(e.to_string()),
  };

  let (loaded_data, note) = match data_load_result {
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
      data::set_general_settings,
      db::get_videos,
      db::archive,
      db::unarchive,
    ])
    .setup(move |_app| {
      #[cfg(target_os = "macos")]
      if let Some(note) = note.clone() {
        thread::spawn(move || {
          dialog::message(Option::<&tauri::Window<tauri::Wry>>::None, note.0, note.1);
        });
      }
      Ok(())
    })
    .create_window(MAIN_WIN, WindowUrl::default(), |win, webview| {
      let win = win
        .title("Kadium")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(900.0, 800.0)
        .min_inner_size(400.0, 150.0)
        .fullscreen(false);
      return (win, webview);
    })
    .manage(ArcData::new(loaded_data))
    .menu(menu::new(vec![
      #[cfg(target_os = "macos")]
      MenuItem::Submenu(Submenu::new(
        &ctx.package_info().name,
        menu::new(vec![
          MenuItem::About(ctx.package_info().name.clone()),
          MenuItem::Separator,
          MenuItem::Custom(custom_item("Preferences").accelerator("CmdOrCtrl+,")),
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
      menu::default_file_submenu(),
      menu::default_edit_submenu(),
      menu::default_view_submenu(),
      menu::default_window_submenu(),
      MenuItem::Submenu(Submenu::new(
        "Help",
        menu::new(vec![MenuItem::Custom(custom_item("Learn More"))]),
      )),
    ]))
    .on_menu_event(|event| match event.menu_item_id() {
      "Learn More" => {
        shell::open("https://github.com/probablykasper/kadium".to_string(), None).unwrap();
      }
      _ => {}
    })
    .build(ctx)
    .expect("Error running tauri app");
  app.run(|app_handle, e| match e {
    tauri::Event::CloseRequested { label: _, api, .. } => {
      if cfg!(target_os = "macos") {
        api.prevent_close();
        app_handle.hide().unwrap();
      }
    }
    _ => {}
  });
}
