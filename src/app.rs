use std::cell::Cell;
use std::ffi::OsString;
use std::path::PathBuf;

use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use relm4_components::open_button::{OpenButton, OpenButtonSettings};
use relm4_components::open_dialog::OpenDialogSettings;
use relm4_components::save_dialog::{
    SaveDialog, SaveDialogMsg, SaveDialogResponse, SaveDialogSettings,
};

mod actions;
mod editor;
mod settings;

pub(crate) const APP_ID: &str = "com.github.tiago_vargas.text_editor";

pub(crate) struct AppModel {
    editor: Controller<editor::Model>,
    open_button: Controller<OpenButton>,
    save_dialog: Controller<SaveDialog>,
    opened_path: Option<PathBuf>,
    opened_path_string: Option<String>,
    opened_file_name: Option<String>,
    toast: Cell<Option<adw::Toast>>,
    editors: FactoryVecDeque<editor::editor2::Model>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    OpenFile(PathBuf),
    SaveFile(PathBuf),
    SaveCurrentFile,
    ShowSaveDialog,
    ShowSavedToast,
    UpdateNameAndPath,
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
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_start: model.open_button.widget(),

                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        #[watch] set_title: model.opened_file_name.as_ref()
                            .unwrap_or(&String::from("Untitled")),
                        #[watch] set_subtitle: model.opened_path_string.as_ref()
                            .unwrap_or(&String::from("")),
                    },
                },

                adw::ToastOverlay {
                    model.editor.widget(),

                    #[watch] add_toast?: model.toast.take(),
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let editor = editor::Model::builder()
            .launch(editor::Init)
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
            let editors = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let model = AppModel {
            editor,
            open_button,
            save_dialog,
            opened_path: None,
            opened_path_string: None,
            opened_file_name: None,
            toast: Cell::new(None),
            editors
        };

        let widgets = view_output!();

        Self::load_window_state(&widgets);
        Self::create_actions(&widgets, &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Self::Input::OpenFile(path) => {
                let contents = std::fs::read_to_string(path.clone());
                match contents {
                    Ok(text) => {
                        self.editor
                            .emit(editor::Input::SetContent(text));
                        self.opened_path = Some(path);
                        sender.input(Self::Input::UpdateNameAndPath);
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
                let text = self.editor.model().text();
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
            Self::Input::UpdateNameAndPath => {
                let path = self.opened_path.clone();
                self.opened_file_name = path
                    .and_then(|p| p.file_name().map(|s| OsString::from(s)))
                    .and_then(|s| s.to_str().map(|s| String::from(s)));

                let path = self.opened_path.clone();
                self.opened_path_string = path
                    .and_then(|p| p.to_str().map(|s| String::from(s)));
            }
            Self::Input::DoNothing => (),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        Self::save_window_state(&widgets);
    }
}
