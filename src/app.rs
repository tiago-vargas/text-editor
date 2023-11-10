use gtk::prelude::*;
use relm4::prelude::*;
use relm4_components::open_button::{OpenButton, OpenButtonSettings};

mod content;
mod settings;

use relm4_components::open_dialog::OpenDialogSettings;
use settings::Settings;

pub(crate) const APP_ID: &str = "com.github.tiago_vargas.text_editor";

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
    open_button: Controller<OpenButton>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    OpenFile(std::path::PathBuf),
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
        let model = AppModel { content, open_button };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::OpenFile(path) => println!("Open file {path:?}"),  // TODO: Implement actual action
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
