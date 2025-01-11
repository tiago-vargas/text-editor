use std::fs;

use gtk::gio;
use relm4::prelude::*;
use source::prelude::*;

pub(crate) struct Editor {
	text_buffer: source::Buffer,
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
			source::View {
				add_css_class: "monospace",
				set_buffer: Some(&model.text_buffer),
				set_show_line_marks: true,
				set_show_line_numbers: true,
				set_show_right_margin: true,
				set_highlight_current_line: true,
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			text_buffer: Default::default(),
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
