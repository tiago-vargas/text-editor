use adw::prelude::*;
use gtk::gio;
use relm4::prelude::*;

use crate::config::{BUILD_TYPE, RESOURCE_PREFIX};

mod actions;
mod editor;
mod modals;
mod settings;

pub(crate) struct Model {
	editor: Controller<editor::Editor>,
}

#[derive(Debug)]
pub(crate) enum Input {
	ShowOpenFileDialog,
	ShowFileContents(gio::File),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = ();
	type Input = Input;
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
					pack_start = &gtk::Button {
						set_label: "Open",
						set_tooltip_text: Some("Open File"),

						connect_clicked => Input::ShowOpenFileDialog,
					},

					pack_end = &gtk::MenuButton {
						set_icon_name: "open-menu-symbolic",
						set_menu_model: Some(&primary_menu),
						set_tooltip: "Main Menu",
					},
				},

				model.editor.widget(),
			},
		}
	}

	fn init(
		_init: Self::Init,
		window: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let editor = editor::Editor::builder().launch(()).detach();
		let model = Model { editor };

		let widgets = view_output!();

		Self::load_window_state(&widgets);
		Self::create_actions(&widgets, &sender);

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Input::ShowOpenFileDialog => {
				let app = relm4::main_application();
				let main_window = app
					.windows()
					.first()
					.expect(
						"Event should have been triggered by last focused window, thus first item",
					)
					.clone();

				let dialog = gtk::FileDialog::builder()
					.title("Open File")
					.modal(true)
					.build();
				dialog.open(
					Some(&main_window),
					None::<&gio::Cancellable>,
					move |result| {
						if let Ok(selected_file) = result {
							sender.input(Input::ShowFileContents(selected_file));
						}
					},
				)
			}
			Input::ShowFileContents(file) => {
				let _ = self.editor.sender().send(editor::Input::OpenFile(file));
			}
		}
	}

	fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
		Self::save_window_state(widgets);
	}
}
