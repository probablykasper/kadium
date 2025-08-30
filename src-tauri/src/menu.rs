use tauri::{
	menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder},
	App, Manager,
};
use tauri_plugin_opener::OpenerExt;

pub fn manage_menu(app: &App) -> Result<(), Box<dyn std::error::Error>> {
	#[allow(unused_mut)]
	let mut menu = MenuBuilder::new(app);

	#[cfg(target_os = "macos")]
	{
		let app_entry = SubmenuBuilder::new(app, &app.package_info().name.clone())
			.item(&MenuItemBuilder::new("About").build(app)?)
			.item(&PredefinedMenuItem::separator(app)?)
			.item(&MenuItemBuilder::new("Preferences...").build(app)?)
			.item(&PredefinedMenuItem::separator(app)?)
			.item(&MenuItemBuilder::new("Services").build(app)?)
			.item(&PredefinedMenuItem::separator(app)?)
			.item(&MenuItemBuilder::new("Hide").build(app)?)
			.item(&MenuItemBuilder::new("Hide Others").build(app)?)
			.item(&MenuItemBuilder::new("Show All").build(app)?)
			.item(&PredefinedMenuItem::separator(app)?)
			.item(&MenuItemBuilder::new("Quit").build(app)?)
			.build()?;

		menu = menu.item(&app_entry);
	}

	#[allow(unused_mut)]
	let mut file = SubmenuBuilder::new(app, "File")
		.item(
			&MenuItemBuilder::new("Add Channel...")
				.accelerator("CmdOrCtrl+N")
				.build(app)?,
		)
		.item(&MenuItemBuilder::new("Open").build(app)?)
		.item(&MenuItemBuilder::new("Open Channel").build(app)?)
		.item(
			&MenuItemBuilder::new("Archive")
				.accelerator("CmdOrCtrl+Backspace")
				.build(app)?,
		)
		.item(
			&MenuItemBuilder::new("Unarchive")
				.accelerator("Shift+CmdOrCtrl+Backspace")
				.build(app)?,
		)
		.item(&PredefinedMenuItem::separator(app)?)
		.item(&PredefinedMenuItem::close_window(app, "Close".into())?);

	#[cfg(not(target_os = "macos"))]
	{
		file = file
			.item(&MenuItemBuilder::new("Options...").build(app)?)
			.item(&PredefinedMenuItem::separator(app)?);
	}

	let edit = SubmenuBuilder::new(app, "Edit")
		.item(&PredefinedMenuItem::undo(app, "Undo".into())?)
		.item(&PredefinedMenuItem::redo(app, "Redo".into())?)
		.item(&PredefinedMenuItem::separator(app)?)
		.item(&PredefinedMenuItem::cut(app, "Cut".into())?)
		.item(&PredefinedMenuItem::copy(app, "Copy".into())?)
		.item(&PredefinedMenuItem::paste(app, "Paste".into())?)
		.item(&PredefinedMenuItem::separator(app)?)
		.item(&PredefinedMenuItem::select_all(app, "Select All".into())?)
		.item(&PredefinedMenuItem::separator(app)?)
		.item(
			&MenuItemBuilder::new("Find")
				.accelerator("CmdOrCtrl+F")
				.build(app)?,
		)
		.build()?;

	let view = SubmenuBuilder::new(app, "View")
		.item(
			&MenuItemBuilder::new("Show New")
				.accelerator("Alt+CmdOrCtrl+N")
				.build(app)?,
		)
		.item(
			&MenuItemBuilder::new("Show Archived")
				.accelerator("Alt+CmdOrCtrl+E")
				.build(app)?,
		)
		.item(
			&MenuItemBuilder::new("Show All")
				.accelerator("Alt+CmdOrCtrl+A")
				.build(app)?,
		)
		.item(&PredefinedMenuItem::separator(app)?)
		.item(
			&MenuItemBuilder::new("History")
				.accelerator("CmdOrCtrl+Y")
				.build(app)?,
		)
		.item(&PredefinedMenuItem::separator(app)?)
		.item(&PredefinedMenuItem::fullscreen(app, "Fullscreen".into())?)
		.build()?;

	let window = SubmenuBuilder::new(app, "Window")
		.item(&PredefinedMenuItem::minimize(app, "Minimize".into())?)
		// .item(MenuItem::Zoom.into())
		.item(&PredefinedMenuItem::separator(app)?)
		.item(
			&MenuItemBuilder::new("Videos")
				.accelerator("Alt+CmdOrCtrl+1")
				.build(app)?,
		)
		.item(
			&MenuItemBuilder::new("Channels")
				.accelerator("Alt+CmdOrCtrl+2")
				.build(app)?,
		)
		.build()?;

	let help = SubmenuBuilder::new(app, "Help")
		.item(&MenuItemBuilder::new("Get Started").build(app)?)
		.item(&MenuItemBuilder::new("Learn More").build(app)?)
		.build()?;

	let menu_built = menu
		.item(&file.build()?)
		.item(&edit)
		.item(&view)
		.item(&window)
		.item(&help)
		.build()?;

	app.set_menu(menu_built)?;

	app.app_handle()
		.on_menu_event(|app, event| match event.id().0.as_str() {
			"Learn More" => {
				let url = "https://github.com/probablykasper/kadium";
				app.app_handle()
					.opener()
					.open_url(url, None::<&str>)
					.unwrap();
			}
			_ => {}
		});

	Ok(())
}
