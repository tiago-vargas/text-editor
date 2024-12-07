use adw::prelude::*;
use relm4::prelude::*;

pub(crate) mod about;

#[derive(Debug)]
pub(crate) enum Modal {
	About,
}

pub(crate) fn show_window(modal: Modal) {
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
