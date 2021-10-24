#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::data::{to_json, AppPaths};
use crate::settings::yt_email_notifier;
use data::{ArcData, Data};
use serde_json::Value;
use settings::VersionedSettings;
use std::thread;
use tauri::api::{dialog, shell};
use tauri::{
  command, CustomMenuItem, Manager, Menu, MenuItem, State, Submenu, SystemTray, SystemTrayEvent,
  Window, WindowBuilder, WindowUrl,
};

mod data;
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

fn custom_menu(name: &str) -> CustomMenuItem {
  let c = CustomMenuItem::new(name.to_string(), name);
  return c;
}

fn load_data(paths: &AppPaths, _win: Window) -> Result<Data, String> {
  if paths.settings_file.exists() {
    return match settings::VersionedSettings::load(&paths) {
      Ok(settings) => Ok(data::Data {
        versioned_settings: settings,
        paths: paths.clone(),
      }),
      Err(e) => Err(e),
    };
  }

  // dont save this yet, otherwise importing popup will not work
  Ok(Data {
    versioned_settings: VersionedSettings::default(),
    paths: paths.clone(),
  })
}

#[command]
fn maybe_ask_for_import(data: State<ArcData>, win: Window) -> Result<Value, String> {
  let mut data = data.0.lock().unwrap();
  if yt_email_notifier::can_import() {
    let msg = "Do you want to import your data from YouTube Email Notifier?";
    let builder = rfd::MessageDialog::new()
      .set_title("Import")
      .set_description(&msg)
      .set_buttons(rfd::MessageButtons::YesNo)
      .set_level(rfd::MessageLevel::Info);
    let wants_to_import = builder.show();
    println!("wants_to_import{}", wants_to_import);
    if wants_to_import {
      match yt_email_notifier::import()? {
        Some(imported_stuff) => {
          data.versioned_settings = imported_stuff.settings.wrap();
          data.versioned_settings.save(&data.paths)?;

          #[cfg(not(feature = "skip_migration_note"))]
          {
            let win2 = win.clone();
            let note = imported_stuff.update_note.clone();
            thread::spawn(move || {
              dialog::message(Some(&win2), "Data imported", note);
            });
          }

          let settings = data.versioned_settings.unwrap();
          return Ok(to_json(settings)?);
        }
        None => {}
      }
    }
  }
  Ok(Value::Null)
}

fn main() {
  let tray = SystemTray::new();

  let menu = Menu::new()
    .add_submenu(Submenu::new(
      // on macOS first menu is always app name
      "YouTube Email Notifier",
      Menu::new()
        .add_native_item(MenuItem::About("YouTube Email Notifier".to_string()))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit),
    ))
    .add_submenu(Submenu::new(
      "File",
      Menu::new().add_item(custom_menu("Close Window").accelerator("cmdOrControl+W")),
    ))
    .add_submenu(Submenu::new("Edit", {
      let mut menu = Menu::new();
      menu = menu.add_native_item(MenuItem::Undo);
      menu = menu.add_native_item(MenuItem::Redo);
      menu = menu.add_native_item(MenuItem::Separator);
      menu = menu.add_native_item(MenuItem::Cut);
      menu = menu.add_native_item(MenuItem::Copy);
      menu = menu.add_native_item(MenuItem::Paste);
      #[cfg(not(target_os = "macos"))]
      {
        menu = menu.add_native_item(MenuItem::Separator);
      }
      menu = menu.add_native_item(MenuItem::SelectAll);
      menu
    }))
    .add_submenu(Submenu::new(
      "View",
      Menu::new().add_native_item(MenuItem::EnterFullScreen),
    ))
    .add_submenu(Submenu::new(
      "Window",
      Menu::new()
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Zoom),
    ))
    .add_submenu(Submenu::new(
      "Help",
      Menu::new().add_item(custom_menu("Learn More")),
    ));

  let ctx = tauri::generate_context!();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      error_popup,
      data::get_settings,
      data::set_channels,
      maybe_ask_for_import,
    ])
    .setup(|app| {
      // app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      let app_paths = AppPaths::from_tauri_config(&app.config());
      let win = app.get_window("main").expect("get main window");
      let loaded_data = match load_data(&app_paths, win) {
        Ok(d) => d,
        Err(e) => {
          app.manage(data::ArcData::new(Data {
            versioned_settings: VersionedSettings::default(),
            paths: app_paths,
          }));
          error_popup_main_thread(&e);
          panic!("{}", e);
        }
      };
      app.manage(data::ArcData::new(loaded_data));

      Ok(())
    })
    .create_window("main", WindowUrl::default(), |win, webview| {
      let win = win
        .title("YouTube Email Notifier")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(900.0, 800.0)
        .min_inner_size(440.0, 150.0)
        .skip_taskbar(true)
        .fullscreen(false);
      return (win, webview);
    })
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { .. } => {
        let window = app.get_window("main").unwrap();
        let is_visible = window.is_visible().unwrap();
        if is_visible {
          window.hide().unwrap();
        } else {
          window.show().unwrap();
          std::thread::sleep(std::time::Duration::from_millis(5));
          window.set_focus().unwrap();
        }
      }
      _ => {}
    })
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id() {
      "Close Window" => {
        let _ = event.window().hide();
      }
      "learn-more" => {
        shell::open(
          "https://github.com/probablykasper/yt-email-notifier".to_string(),
          None,
        )
        .unwrap();
      }
      _ => {}
    })
    .run(ctx)
    .expect("error while running tauri app");
}

pub fn dialog_sync<S: AsRef<str>>(w: Window, title: S, msg: S) -> bool {
  let (sender, receiver) = std::sync::mpsc::channel();
  let title = title.as_ref().to_string();
  let msg = msg.as_ref().to_string();
  thread::spawn(move || {
    dialog::ask(Some(&w), title, msg, move |res| {
      sender.send(res).unwrap();
    })
  });
  receiver.recv().unwrap_or(false)
}
