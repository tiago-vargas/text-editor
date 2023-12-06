use std::cell::Cell;
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
    editors: FactoryVecDeque<editor::Model>,
    open_button: Controller<OpenButton>,
    save_dialog: Controller<SaveDialog>,
    toast: Cell<Option<adw::Toast>>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    OpenFile(PathBuf),
    SaveFile(PathBuf),
    SaveCurrentFile,
    ShowSaveDialog,
    ShowSavedToast,
    NewEditor,
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
                    // pack_start: gtk::Button {},

                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        #[watch] set_title: model.editors.get(0).unwrap().opened_file_name.as_ref()
                            .unwrap_or(&String::from("Untitled")),
                        #[watch] set_subtitle: model.editors.get(0).unwrap().opened_path_string.as_ref()
                            .unwrap_or(&String::from("")),
                    },
                },

                adw::TabBar {
                    set_view: Some(model.editors.widget()),
                },

                adw::ToastOverlay {
                    model.editors.widget(),

                    #[watch] add_toast?: model.toast.take(),
                },

                #[local_ref]
                editor_tabs -> adw::TabView {

                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let editors = FactoryVecDeque::new(adw::TabView::default(), sender.input_sender());
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
            editors,
            open_button,
            save_dialog,
            toast: Cell::new(None),
        };

        let editor_tabs = model.editors.widget();
        let widgets = view_output!();

        Self::load_window_state(&widgets);
        Self::create_actions(&widgets, &sender);

        sender.input(Self::Input::NewEditor);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Self::Input::OpenFile(path) => {
                let contents = std::fs::read_to_string(path.clone());
                match contents {
                    Ok(text) => {
                        self.editors
                            .send(0, editor::Input::SetContent(text));
                        self.editors.send(0, editor::Input::SetOpenedPath(path.clone()));
                        self.editors.send(0, editor::Input::UpdateNameAndPath(path));
                    }
                    Err(error) =>  eprintln!("Error reading file: {}", error),
                }
            }
            Self::Input::SaveCurrentFile => {
                match &self.editors.get(0).unwrap().opened_path {
                    Some(path) => sender.input(Self::Input::SaveFile(path.clone())),
                    None => sender.input(Self::Input::ShowSaveDialog),
                }
            }
            Self::Input::SaveFile(path) => {
                let text = self.editors
                    .get(0)
                    .unwrap()
                    .text();
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
            Self::Input::NewEditor => {
                self.editors.guard().push_back(editor::Init);
            }
            Self::Input::DoNothing => (),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        Self::save_window_state(&widgets);
    }
}
