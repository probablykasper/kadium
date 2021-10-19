#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  api, CustomMenuItem, Manager, Menu, MenuItem, Submenu, SystemTray, SystemTrayEvent,
  WindowBuilder, WindowUrl,
};

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
      "Edit",
      Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::SelectAll),
    ))
    .add_submenu(Submenu::new(
      "Help",
      Menu::new().add_item(CustomMenuItem::new("learn-more", "Learn More")),
    ))
    .add_native_item(MenuItem::Copy);

  let ctx = tauri::generate_context!();
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
      "learn-more" => {
        api::shell::open(
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
