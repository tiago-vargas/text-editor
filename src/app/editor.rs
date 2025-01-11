use std::fs;

use relm4::{factory::FactoryView, prelude::*};
use source::{gio, prelude::*};

pub(crate) struct Editor {
	file: Option<gio::File>,
	title: std::path::PathBuf,
	text_buffer: source::Buffer,
}

type Init = gio::File;

#[derive(Debug)]
pub(crate) enum Input {
	OpenFile(gio::File),
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for Editor {
	type ParentWidget = adw::TabView;
	type Input = Input;
	type Output = ();
	type Init = Init;
	type CommandOutput = ();

	view! {
		#[root]
		gtk::ScrolledWindow {
			source::View {
				add_css_class: "monospace",
				set_buffer: Some(&self.text_buffer),
				set_show_line_marks: true,
				set_show_line_numbers: true,
				set_show_right_margin: true,
				set_highlight_current_line: true,
				set_smart_home_end: source::SmartHomeEndType::Before,
			},
		},

		#[local_ref]
		returned_widget -> adw::TabPage {
			#[watch] set_title: self.title.to_str().unwrap(),
		},
	}

	fn init_model(file: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
		let model = Self {
			file: None,
			title: Default::default(),
			text_buffer: Default::default(),
		};

		sender.input(Input::OpenFile(file));

		model
	}

	fn init_widgets(
		&mut self,
		_index: &DynamicIndex,
		root: Self::Root,
		returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
		_sender: FactorySender<Self>,
	) -> Self::Widgets {
		let widgets = view_output!();
		widgets
	}

	fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
		match message {
			Input::OpenFile(file) => {
				self.file = Some(file);
				let uri = &self
					.file
					.as_ref()
					.unwrap()
					.path()
					.expect("Chosen file should have a path");
				let contents: String = fs::read_to_string(uri).unwrap();
				self.text_buffer.set_text(&contents);
				self.text_buffer.set_language(
					source::LanguageManager::new()
						.guess_language(Some(uri), None)
						.as_ref(),
				);

				self.title = self.file.as_ref().unwrap().basename().unwrap();
			}
		}
	}
}
