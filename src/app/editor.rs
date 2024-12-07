use std::fs;

use adw::prelude::*;
use gtk::{gio, prelude::FileExt};
use relm4::prelude::*;

pub(crate) struct Editor {
	text_buffer: gtk::TextBuffer,
}

#[derive(Debug)]
pub(crate) enum Input {
	OpenFile(gio::File),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Editor {
	type Init = ();
	type Input = Input;
	type Output = ();

	view! {
		gtk::ScrolledWindow {
			gtk::TextView {
				add_css_class: "monospace",
				set_buffer: Some(&model.text_buffer),
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			text_buffer: gtk::TextBuffer::new(None),
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			Input::OpenFile(file) => {
				let uri = file.path().unwrap();
				let contents = fs::read_to_string(uri).unwrap();
				self.text_buffer.set_text(&contents);
			}
		}
	}
}
