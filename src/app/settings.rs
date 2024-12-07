use gtk::{gio, prelude::*};
use relm4::prelude::*;

use crate::{app, config};

pub(crate) enum WindowSettings {
	Width,
	Height,
	Maximized,
}

impl WindowSettings {
	pub(crate) fn as_str(&self) -> &str {
		match self {
			Self::Width => "window-width",
			Self::Height => "window-height",
			Self::Maximized => "window-maximized",
		}
	}
}

impl app::Model {
	pub(super) fn save_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
		let settings = gio::Settings::new(config::APP_ID);

		let (width, height) = widgets.main_window.default_size();
		settings
			.set_int(WindowSettings::Width.as_str(), width)
			.expect("Key should exist, be writable and be `i32` in the GSchema");
		settings
			.set_int(WindowSettings::Height.as_str(), height)
			.expect("Key should exist, be writable and be `i32` in the GSchema");

		settings
			.set_boolean(
				WindowSettings::Maximized.as_str(),
				widgets.main_window.is_maximized(),
			)
			.expect("Key should exist, be writable and be `bool` in the GSchema");
	}

	pub(super) fn load_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
		let settings = gio::Settings::new(config::APP_ID);

		let width = settings.int(WindowSettings::Width.as_str());
		let height = settings.int(WindowSettings::Height.as_str());
		widgets.main_window.set_default_size(width, height);

		let maximized = settings.boolean(WindowSettings::Maximized.as_str());
		widgets.main_window.set_maximized(maximized);
	}
}
