#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::data::{AppPaths, ArcData, Data};
use crate::settings::yt_email_notifier;
use crate::settings::VersionedSettings;
use std::{env, thread};
use tauri::api::{dialog, shell};
#[cfg(target_os = "macos")]
use tauri::AboutMetadata;
use tauri::{
  command, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, Window, WindowBuilder,
  WindowUrl,
};

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

/// Note title and message to show asynchronously when/after the app starts
type ImportedNote = Option<(String, String)>;

/// This can display dialogs, which needs to happen before tauri runs to not panic
async fn load_data(paths: &AppPaths) -> Result<(VersionedSettings, ImportedNote), String> {
  if paths.settings_file.exists() {
    return match settings::VersionedSettings::load(&paths) {
      Ok(settings) => Ok((settings, None)),
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
    let versioned_settings = imported_stuff.settings.wrap();
    versioned_settings.save(&paths)?;

    let import_note = Some(("Import note".to_string(), imported_stuff.update_note));
    return Ok((versioned_settings, import_note));
  }

  Ok((VersionedSettings::default(), None))
}

#[tokio::main]
async fn main() {
  let ctx = tauri::generate_context!();

  // macOS "App Nap" periodically pauses our app when it's in the background.
  // We need to prevent that so our intervals are not interrupted.
  #[cfg(target_os = "macos")]
  macos_app_nap::prevent();

  let app_paths = AppPaths::from_tauri_config(&ctx.config());

  let (mut settings, _note) = match load_data(&app_paths).await {
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
    .invoke_handler(tauri::generate_handler![
      error_popup,
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
    .setup(move |app| {
      let win = WindowBuilder::new(app, "main", WindowUrl::default())
        .title("Kadium")
        .inner_size(900.0, 800.0)
        .min_inner_size(400.0, 150.0)
        .build()
        .expect("Unable to create window");
      #[cfg(target_os = "macos")]
      {
        use cocoa::appkit::NSWindow;
        let nsw = win.ns_window().unwrap() as cocoa::base::id;
        unsafe {
          // manual implementation for now (PR https://github.com/tauri-apps/tauri/pull/3965)
          {
            nsw.setTitlebarAppearsTransparent_(cocoa::base::YES);

            // tauri enables fullsizecontentview by default, so disable it
            let mut style_mask = nsw.styleMask();
            style_mask.set(
              cocoa::appkit::NSWindowStyleMask::NSFullSizeContentViewWindowMask,
              false,
            );
            nsw.setStyleMask_(style_mask);
          }

          // set window to always be dark mode
          use cocoa::appkit::NSAppearanceNameVibrantDark;
          use objc::*;
          let appearance: cocoa::base::id = msg_send![
            class!(NSAppearance),
            appearanceNamed: NSAppearanceNameVibrantDark
          ];
          let () = msg_send![nsw, setAppearance: appearance];

          // set window background color
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
      };
      app.manage(ArcData::new(data));

      #[cfg(target_os = "macos")]
      if let Some(note) = _note.clone() {
        dialog::message(Option::Some(&win), note.0, note.1);
      }
      Ok(())
    })
    .menu(Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuEntry::Submenu(Submenu::new(
        &ctx.package_info().name,
        Menu::with_items([
          MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default()).into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("Preferences...", "Preferences...")
            .accelerator("CmdOrCtrl+,")
            .into(),
          MenuItem::Separator.into(),
          MenuItem::Services.into(),
          MenuItem::Separator.into(),
          MenuItem::Hide.into(),
          MenuItem::HideOthers.into(),
          MenuItem::ShowAll.into(),
          MenuItem::Separator.into(),
          MenuItem::Quit.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "File",
        Menu::with_items([
          CustomMenuItem::new("Add Channel...", "Add Channel...")
            .accelerator("CmdOrCtrl+N")
            .into(),
          CustomMenuItem::new("Open", "Open").into(),
          CustomMenuItem::new("Open Channel", "Open Channel").into(),
          CustomMenuItem::new("Archive", "Archive")
            .accelerator("CmdOrCtrl+Backspace")
            .into(),
          CustomMenuItem::new("Unarchive", "Unarchive")
            .accelerator("Shift+CmdOrCtrl+Backspace")
            .into(),
          MenuItem::Separator.into(),
          #[cfg(not(target_os = "macos"))]
          CustomMenuItem::new("Options...", "Options...")
            .accelerator("CmdOrCtrl+,")
            .into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          MenuItem::CloseWindow.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Edit",
        Menu::with_items([
          MenuItem::Undo.into(),
          MenuItem::Redo.into(),
          MenuItem::Separator.into(),
          MenuItem::Cut.into(),
          MenuItem::Copy.into(),
          MenuItem::Paste.into(),
          #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
          MenuItem::SelectAll.into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("Find", "Find")
            .accelerator("CmdOrCtrl+F")
            .into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "View",
        Menu::with_items([
          CustomMenuItem::new("Show New", "Show New")
            .accelerator("Alt+CmdOrCtrl+N")
            .into(),
          CustomMenuItem::new("Show Archived", "Show Archived")
            .accelerator("Alt+CmdOrCtrl+E")
            .into(),
          CustomMenuItem::new("Show All", "Show All")
            .accelerator("Alt+CmdOrCtrl+A")
            .into(),
          MenuItem::Separator.into(),
          MenuItem::EnterFullScreen.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Window",
        Menu::with_items([
          MenuItem::Minimize.into(),
          MenuItem::Zoom.into(),
          MenuItem::Separator.into(),
          CustomMenuItem::new("Videos", "Videos")
            .accelerator("Alt+CmdOrCtrl+1")
            .into(),
          CustomMenuItem::new("Channels", "Channels")
            .accelerator("Alt+CmdOrCtrl+2")
            .into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Help",
        Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]).into(),
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
      tauri::WindowEvent::CloseRequested { api: _api, .. } => {
        #[cfg(target_os = "macos")]
        {
          // hide the application
          // manual for now (PR https://github.com/tauri-apps/tauri/pull/3689)
          _api.prevent_close();
          use objc::*;
          let cls = objc::runtime::Class::get("NSApplication").unwrap();
          let app: cocoa::base::id = unsafe { msg_send![cls, sharedApplication] };
          unsafe { msg_send![app, hide: 0] }
        }
      }
      _ => {}
    },
    _ => {}
  });
}
