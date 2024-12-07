use adw::prelude::*;
use relm4::prelude::*;

use crate::config::{BUILD_TYPE, RESOURCE_PREFIX};

mod actions;
mod content;
mod modals;
mod settings;

pub(crate) struct Model {
	content: Controller<content::Model>,
}

pub(crate) struct Init;

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = ();
	type Output = ();

	menu! {
		primary_menu: {
			section! {
				"Keyboard Shortcuts" => actions::ShowKeyboardShortcuts,
				"About Text Editor" => actions::ShowAbout,
			},
		}
	}

	view! {
		main_window = adw::ApplicationWindow {
			set_title: Some("Text Editor"),

			add_css_class?: if BUILD_TYPE == "debug" { Some("devel") } else { None },

			#[wrap(Some)]
			set_help_overlay: keyboard_shortcuts_window =
				&gtk::Builder::from_resource(
					&format!("{RESOURCE_PREFIX}/app/modals/keyboard-shortcuts-window.ui")
				)
				.object::<gtk::ShortcutsWindow>("help_overlay")
				.unwrap() -> gtk::ShortcutsWindow,

			adw::ToolbarView {
				add_top_bar = &adw::HeaderBar {
					pack_end = &gtk::MenuButton {
						set_icon_name: "open-menu-symbolic",
						set_menu_model: Some(&primary_menu),
						set_tooltip: "Main Menu",
					},
				},

				model.content.widget(),
			},
		}
	}

	fn init(
		_init: Self::Init,
		window: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let content = content::Model::builder().launch(content::Init).detach();
		let model = Model { content };

		let widgets = view_output!();

		Self::load_window_state(&widgets);
		Self::create_actions(&widgets, &sender);

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let () = message;
	}

	fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
		Self::save_window_state(widgets);
	}
}
