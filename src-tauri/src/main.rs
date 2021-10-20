#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::thread;

use tauri::api::{dialog, shell};
use tauri::{
  command, CustomMenuItem, Manager, Menu, MenuItem, Submenu, SystemTray, SystemTrayEvent, Window,
  WindowBuilder, WindowUrl,
};

mod data;
mod migration;
mod settings;

#[command]
fn error_popup_main_thread(msg: impl AsRef<str>) {
  let msg = msg.as_ref().to_string();
  println!("Error: {}", msg);
  let builder = rfd::MessageDialog::new()
    .set_title("Error")
    .set_description(msg.as_ref())
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

fn custom_menu(name: &str) -> CustomMenuItem {
  let c = CustomMenuItem::new(name.to_string(), name);
  return c;
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

  let app_dir = match tauri::api::path::app_dir(ctx.config()) {
    Some(app_dir) => app_dir,
    None => {
      error_popup_main_thread("No app dir");
      panic!("No app dir");
    }
  };
  let loaded_data = match data::ArcData::load(app_dir) {
    Ok(d) => d,
    Err(e) => {
      error_popup_main_thread(e.to_string());
      panic!("{}", e.to_string());
    }
  };

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![])
    .setup(|app| {
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);
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
        .min_inner_size(300.0, 150.0)
        .skip_taskbar(true)
        .fullscreen(false);
      return (win, webview);
    })
    .manage(loaded_data)
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
      "Close" => {
        println!("CLOSE");
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
