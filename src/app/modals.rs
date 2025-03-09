use adw::prelude::*;
use relm4::prelude::*;

mod about;

#[derive(Debug)]
pub enum Modal {
	About,
}

pub fn show_window(modal: &Modal) {
	match modal {
		Modal::About => {
			let app = relm4::main_application();
			let main_window = app
				.windows()
				.first()
				.expect("Event should have been triggered by last focused window, thus first item")
				.clone();

			let about_window = about::Model::builder().launch(about::Init).detach();
			about_window.widget().present(Some(&main_window));
		}
	}
}
