use gtk::prelude::*;
use relm4::{
	actions::{AccelsPlus, RelmAction, RelmActionGroup},
	prelude::*,
};

use super::{modals, Model};
use modals::Modal;

relm4::new_action_group!(pub(crate) AppActions, "app");
relm4::new_stateless_action!(pub(crate) ShowAbout, AppActions, "about");
relm4::new_stateless_action!(pub(crate) Quit, AppActions, "quit");

relm4::new_action_group!(pub(crate) WinActions, "win");
relm4::new_stateless_action!(pub(crate) ShowKeyboardShortcuts, WinActions, "show-help-overlay");

impl Model {
	pub(crate) fn create_actions(
		widgets: &<Self as SimpleComponent>::Widgets,
		_sender: &ComponentSender<Self>,
	) {
		let app = relm4::main_adw_application();

		app.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);

		let mut app_actions = RelmActionGroup::<AppActions>::new();

		let quit = {
			let app = app.clone();
			RelmAction::<Quit>::new_stateless(move |_this| app.quit())
		};
		app.set_accelerators_for_action::<Quit>(&["<primary>q"]);
		app_actions.add_action(quit);

		let show_about = RelmAction::<ShowAbout>::new_stateless(move |_this| {
			modals::show_window(Modal::About);
		});
		app_actions.add_action(show_about);

		app_actions.register_for_widget(&widgets.main_window);
	}
}
