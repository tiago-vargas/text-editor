use super::AppModel;

use relm4::prelude::*;
use relm4::actions::{AccelsPlus, RelmAction, RelmActionGroup};

impl AppModel {
    pub(crate) fn create_actions(
        widgets: &<Self as SimpleComponent>::Widgets,
        sender: &ComponentSender<Self>
    ) {
        let app = relm4::main_adw_application();

        relm4::new_action_group!(AppActions, "app");
        let mut app_actions = RelmActionGroup::<AppActions>::new();

        relm4::new_stateless_action!(SaveAs, AppActions, "save-as");
        let save_as = {
            let sender = sender.clone();
            RelmAction::<SaveAs>::new_stateless(move |_| {
                sender.input(<Self as SimpleComponent>::Input::ShowSaveDialog);
            })
        };
        app.set_accelerators_for_action::<SaveAs>(&["<primary><Shift>S"]);
        app_actions.add_action(save_as);

        relm4::new_stateless_action!(Save, AppActions, "save");
        let save = {
            let sender = sender.clone();
            RelmAction::<Save>::new_stateless(move |_| {
                sender.input(<Self as SimpleComponent>::Input::SaveCurrentFile);
            })
        };
        app.set_accelerators_for_action::<Save>(&["<primary>S"]);
        app_actions.add_action(save);

        app_actions.register_for_widget(&widgets.main_window);
    }
}
