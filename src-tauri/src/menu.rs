#![allow(dead_code)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

/// An item on the system tray menu.
#[derive(Debug, Clone)]
pub enum Item {
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

pub trait AddDefaultSubmenus {
  fn add_default_app_submenu_if_macos(self, app_name: &str) -> Self;
  fn add_default_file_submenu(self) -> Self;
  fn add_default_edit_submenu(self) -> Self;
  fn add_default_view_submenu(self) -> Self;
  fn add_default_window_submenu(self) -> Self;
}

pub fn default_app_submenu(app_name: &str) -> Option<Submenu> {
  #[cfg(target_os = "macos")]
  return Some(Submenu::new(
    app_name.to_string(),
    Menu::new()
      .add_native_item(MenuItem::About(app_name.to_string()))
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Services)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Hide)
      .add_native_item(MenuItem::HideOthers)
      .add_native_item(MenuItem::ShowAll)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Quit),
  ));
  #[cfg(not(target_os = "macos"))]
  return None;
}

pub fn default_file_submenu() -> Item {
  Item::Submenu(Submenu::new(
    "File",
    Menu::new().add_native_item(MenuItem::CloseWindow),
  ))
}

pub fn default_edit_submenu() -> Item {
  Item::Submenu(Submenu::new("Edit", {
    let mut menu = Menu::new()
      .add_native_item(MenuItem::Undo)
      .add_native_item(MenuItem::Redo)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Cut)
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Paste);
    #[cfg(not(target_os = "macos"))]
    {
      menu = menu.add_native_item(MenuItem::Separator);
    }
    menu = menu.add_native_item(MenuItem::SelectAll);
    // macOS automatically adds "Start Dictation" and "Emoji & Symbols" to
    // the bottom of the Edit menu
    menu
  }))
}

pub fn default_view_submenu() -> Item {
  Item::Submenu(Submenu::new(
    "View",
    Menu::new().add_native_item(MenuItem::EnterFullScreen),
  ))
}

pub fn default_window_submenu() -> Item {
  Item::Submenu(Submenu::new(
    "Window",
    Menu::new()
      .add_native_item(MenuItem::Minimize)
      .add_native_item(MenuItem::Zoom),
  ))
}

impl AddDefaultSubmenus for Menu {
  fn add_default_app_submenu_if_macos(self, app_name: &str) -> Menu {
    #[cfg(target_os = "macos")]
    return self.add_submenu(Submenu::new(
      app_name.to_string(),
      Menu::new()
        .add_native_item(MenuItem::About(app_name.to_string()))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit),
    ));
    #[cfg(not(target_os = "macos"))]
    return self;
  }
  fn add_default_file_submenu(self) -> Menu {
    self.add_submenu(Submenu::new(
      "File",
      Menu::new().add_native_item(MenuItem::CloseWindow),
    ))
  }

  fn add_default_edit_submenu(self) -> Menu {
    self.add_submenu(Submenu::new("Edit", {
      let mut menu = Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste);
      #[cfg(not(target_os = "macos"))]
      {
        menu = menu.add_native_item(MenuItem::Separator);
      }
      menu = menu.add_native_item(MenuItem::SelectAll);
      // macOS automatically adds "Start Dictation" and "Emoji & Symbols" to
      // the bottom of the Edit menu
      menu
    }))
  }

  fn add_default_view_submenu(self) -> Menu {
    self.add_submenu(Submenu::new(
      "View",
      Menu::new().add_native_item(MenuItem::EnterFullScreen),
    ))
  }

  fn add_default_window_submenu(self) -> Menu {
    self.add_submenu(Submenu::new(
      "Window",
      Menu::new()
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Zoom),
    ))
  }
}

pub fn generate_menu(submenus: Vec<Submenu>) -> Menu {
  let mut menu = Menu::new();
  for submenu in submenus {
    menu = menu.add_submenu(submenu);
  }
  menu
}
