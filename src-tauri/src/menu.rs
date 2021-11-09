#![allow(dead_code)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

#[derive(Debug, Clone)]
pub enum Item {
  None,
  Custom(CustomMenuItem),
  Submenu(Submenu),
  About(String),
  Hide,
  Services,
  HideOthers,
  ShowAll,
  CloseWindow,
  Quit,
  Copy,
  Cut,
  Undo,
  Redo,
  SelectAll,
  Paste,
  EnterFullScreen,
  Minimize,
  Zoom,
  Separator,
}

pub fn new(items: Vec<Item>) -> Menu {
  let mut menu = Menu::new();
  for item in items {
    let item: Item = item.into();
    let menu_item = match item {
      Item::None => continue,
      Item::Custom(custom_menu_item) => {
        menu = menu.add_item(custom_menu_item);
        continue;
      }
      Item::Submenu(submenu) => {
        menu = menu.add_submenu(submenu);
        continue;
      }
      Item::About(name) => MenuItem::About(name),
      Item::Hide => MenuItem::Hide,
      Item::Services => MenuItem::Services,
      Item::HideOthers => MenuItem::HideOthers,
      Item::ShowAll => MenuItem::ShowAll,
      Item::CloseWindow => MenuItem::CloseWindow,
      Item::Quit => MenuItem::Quit,
      Item::Copy => MenuItem::Copy,
      Item::Cut => MenuItem::Cut,
      Item::Undo => MenuItem::Undo,
      Item::Redo => MenuItem::Redo,
      Item::SelectAll => MenuItem::SelectAll,
      Item::Paste => MenuItem::Paste,
      Item::EnterFullScreen => MenuItem::EnterFullScreen,
      Item::Minimize => MenuItem::Minimize,
      Item::Zoom => MenuItem::Zoom,
      Item::Separator => MenuItem::Separator,
    };
    menu = menu.add_native_item(menu_item);
  }
  menu
}

pub fn default_app_submenu(app_name: &str) -> Item {
  #[cfg(target_os = "macos")]
  return Item::Submenu(Submenu::new(
    "Window",
    new(vec![
      Item::About(app_name.to_string()),
      Item::Separator,
      Item::Services,
      Item::Separator,
      Item::Hide,
      Item::HideOthers,
      Item::ShowAll,
      Item::Separator,
      Item::Quit,
    ]),
  ));
  #[cfg(not(target_os = "macos"))]
  return Item::None;
}

pub fn default_file_submenu() -> Item {
  Item::Submenu(Submenu::new(
    "File",
    Menu::new().add_native_item(MenuItem::CloseWindow),
  ))
}

pub fn default_edit_submenu() -> Item {
  Item::Submenu(Submenu::new(
    "Edit",
    new(vec![
      Item::Undo,
      Item::Redo,
      Item::Separator,
      Item::Cut,
      Item::Copy,
      Item::Paste,
      #[cfg(not(target_os = "macos"))]
      Item::Separator,
      Item::SelectAll,
    ]),
  ))
}

pub fn default_view_submenu() -> Item {
  Item::Submenu(Submenu::new("View", new(vec![Item::EnterFullScreen])))
}

pub fn default_window_submenu() -> Item {
  Item::Submenu(Submenu::new(
    "Window",
    new(vec![Item::Minimize, Item::Zoom]),
  ))
}
