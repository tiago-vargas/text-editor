use std::cell::Cell;
use std::path::PathBuf;

use gtk::prelude::*;
use relm4::prelude::*;
use relm4_components::open_button::{OpenButton, OpenButtonSettings};
use relm4_components::open_dialog::OpenDialogSettings;
use relm4_components::save_dialog::{
    SaveDialog, SaveDialogMsg, SaveDialogResponse, SaveDialogSettings,
};

mod actions;
mod content;
mod modals;
mod settings;

use settings::Settings;
use modals::*;

pub(crate) const APP_ID: &str = "com.github.tiago_vargas.text_editor";

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
    open_button: Controller<OpenButton>,
    save_dialog: Controller<SaveDialog>,
    opened_path: Option<PathBuf>,
    toast: Cell<Option<adw::Toast>>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    OpenFile(PathBuf),
    SaveFile(PathBuf),
    SaveCurrentFile,
    ShowSaveDialog,
    ShowSavedToast,
    DoNothing,

    ShowPreferencesWindow,
    ShowKeyboardShortcutsWindow,
    ShowHelpWindow,
    ShowAboutWindow,
}

#[derive(Debug)]
pub(crate) enum AppOutput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = AppOutput;

    menu! {
        main_menu: {
            section! {
                "Save" => actions::Save,
                "Save As" => actions::SaveAs,
            },
            section! {  // Standard primary menu items
                "Preferences" => actions::ShowPreferences,
                "Keyboard Shortcuts" => actions::ShowKeyboardShortcuts,
                "Help" => actions::ShowHelp,
                "About App" => actions::ShowAbout,
            },
        }
    }

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

                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&main_menu),
                    },
                },

                adw::ToastOverlay {
                    model.content.widget(),

                    #[watch] add_toast?: model.toast.take(),
                },
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
        let model = AppModel {
            content,
            open_button,
            save_dialog,
            opened_path: None::<PathBuf>,
            toast: Cell::new(None),
        };

        let widgets = view_output!();

        Self::create_actions(&widgets, &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Self::Input::OpenFile(path) => {
                let contents = std::fs::read_to_string(path.clone());
                match contents {
                    Ok(text) => {
                        self.content
                            .emit(content::ContentInput::SetContent(text));
                        self.opened_path = Some(path);
                    }
                    Err(error) =>  eprintln!("Error reading file: {}", error),
                }
            }
            Self::Input::SaveCurrentFile => {
                match &self.opened_path {
                    Some(path) => sender.input(Self::Input::SaveFile(path.clone())),
                    None => sender.input(Self::Input::ShowSaveDialog),
                }
            }
            Self::Input::SaveFile(path) => {
                let start = self.content.model().text_buffer.start_iter();
                let end = self.content.model().text_buffer.end_iter();
                let text = self.content.model().text_buffer.text(&start, &end, false);
                match std::fs::write(path, text) {
                    Ok(_) => sender.input(Self::Input::ShowSavedToast),
                    Err(error) => eprintln!("Error saving file: {}", error),
                }
            }
            Self::Input::ShowSaveDialog => {
                self.save_dialog
                    .emit(SaveDialogMsg::Save);
            }
            Self::Input::ShowSavedToast => {
                let toast = adw::Toast::builder()
                    .title("File saved")
                    .timeout(2)
                    .build();
                self.toast.set(Some(toast));
            }
            Self::Input::DoNothing => (),
            Self::Input::ShowPreferencesWindow => {
                let app = relm4::main_application();
                let main_window = app.windows().first()
                    .expect("Event should have been triggered by last focused window, thus first item")
                    .clone();

                let preferences_window = preferences::Model::builder()
                    .transient_for(&main_window)
                    .launch(preferences::Init)
                    .detach();

                preferences_window.widget().present();
            }
            Self::Input::ShowKeyboardShortcutsWindow => {
                let keyboard_shortcuts_window = keyboard_shortcuts::Model::builder()
                    .launch(keyboard_shortcuts::Init)
                    .detach();
                keyboard_shortcuts_window.widget().present();
            }
            Self::Input::ShowHelpWindow => {
                let help_window = help::Model::builder()
                    .launch(help::Init)
                    .detach();
                help_window.widget().present();
            }
            Self::Input::ShowAboutWindow => {
                let about_window = about::Model::builder()
                    .launch(about::Init)
                    .detach();
                about_window.widget().present();
            }
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
}
