use gtk::prelude::*;
use relm4::actions::{AccelsPlus, RelmAction, RelmActionGroup};
use relm4::prelude::*;
use relm4_components::open_button::{OpenButton, OpenButtonSettings};
use relm4_components::open_dialog::OpenDialogSettings;
use relm4_components::save_dialog::{
    SaveDialog, SaveDialogMsg, SaveDialogResponse, SaveDialogSettings,
};

mod content;
mod settings;

use settings::Settings;

pub(crate) const APP_ID: &str = "com.github.tiago_vargas.text_editor";

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
    open_button: Controller<OpenButton>,
    save_dialog: Controller<SaveDialog>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    OpenFile(std::path::PathBuf),
    SaveFile(std::path::PathBuf),
    ShowSaveDialog,
    DoNothing,
}

#[derive(Debug)]
pub(crate) enum AppOutput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = AppOutput;

    view! {
        main_window = adw::ApplicationWindow {
            set_title: Some("Text Editor"),
            set_default_width: settings.int(Settings::WindowWidth.as_str()),
            set_default_height: settings.int(Settings::WindowHeight.as_str()),
            set_maximized: settings.boolean(Settings::WindowMaximized.as_str()),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_start: model.open_button.widget(),
                },

                model.content.widget(),
            },
        }
    }

    /// Initialize the UI and model.
    fn init(
        _init: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = gtk::gio::Settings::new(APP_ID);

        let content = content::ContentModel::builder()
            .launch(content::ContentInit)
            .detach();
        let open_button = OpenButton::builder()
            .launch(OpenButtonSettings {
                dialog_settings: OpenDialogSettings::default(),
                text: "Open",
                recently_opened_files: None,
                max_recent_files: 10,
            })
            .forward(sender.input_sender(), Self::Input::OpenFile);
        let save_dialog = SaveDialog::builder()
            .transient_for_native(window)
            .launch(SaveDialogSettings::default())
            .forward(sender.input_sender(), |response| {
                match response {
                    SaveDialogResponse::Accept(path) => Self::Input::SaveFile(path),
                    SaveDialogResponse::Cancel => Self::Input::DoNothing,
                }
            });
        let model = AppModel { content, open_button, save_dialog };

        let widgets = view_output!();

        Self::create_actions(&widgets, &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::OpenFile(path) => {
                let contents = std::fs::read_to_string(path);
                match contents {
                    Ok(text) => {
                        self.content
                            .emit(content::ContentInput::SetContent(text));
                    }
                    Err(error) => {
                        eprintln!("Error reading file: {}", error);
                    }
                }
            }
            Self::Input::SaveFile(path) => {
                let start = self.content.model().text_buffer.start_iter();
                let end = self.content.model().text_buffer.end_iter();
                let text = self.content.model().text_buffer.text(&start, &end, false);
                match std::fs::write(path, text) {
                    Ok(_) => (),
                    Err(error) => eprintln!("Error saving file: {}", error),
                }
            }
            Self::Input::ShowSaveDialog => {
                self.save_dialog
                    .emit(SaveDialogMsg::Save);
            }
            Self::Input::DoNothing => (),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        Self::save_window_state(&widgets);
    }
}

impl AppModel {
    fn save_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
        let settings = gtk::gio::Settings::new(APP_ID);

        let (width, height) = widgets.main_window.default_size();
        let _ = settings.set_int(settings::Settings::WindowWidth.as_str(), width);
        let _ = settings.set_int(settings::Settings::WindowHeight.as_str(), height);

        let _ = settings.set_boolean(
            settings::Settings::WindowMaximized.as_str(),
            widgets.main_window.is_maximized(),
        );
    }

    fn create_actions(
        widgets: &<Self as SimpleComponent>::Widgets,
        sender: &ComponentSender<Self>
    ) {
        let app = relm4::main_adw_application();

        relm4::new_action_group!(AppActions, "app");
        let mut group = RelmActionGroup::<AppActions>::new();

        relm4::new_stateless_action!(SaveAs, AppActions, "save-as");
        let save_as = {
            let sender = sender.clone();
            RelmAction::<SaveAs>::new_stateless(move |_| {
                sender.input(<Self as SimpleComponent>::Input::ShowSaveDialog);
            })
        };
        app.set_accelerators_for_action::<SaveAs>(&["<primary><Shift>S"]);
        group.add_action(save_as);

        group.register_for_widget(&widgets.main_window);
    }
}
