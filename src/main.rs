use gtk::{gio, prelude::*};
use relm4::{main_application, prelude::*};

mod app;
mod config;

use config::{PKG_DATA_DIR, RESOURCE_PREFIX};

fn main() {
	// Load resources
	let resources = gio::Resource::load(format!("{PKG_DATA_DIR}/resources.gresource"))
		.expect("Should be able to load resources");
	gio::resources_register(&resources);

	// Run the app
	let app = main_application();
	app.set_resource_base_path(Some(RESOURCE_PREFIX));
	let app = RelmApp::from_app(app);
	app.run::<app::Model>(app::Init);
}
