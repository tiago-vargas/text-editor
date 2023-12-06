use std::{path::PathBuf, ffi::OsString};

use super::AppInput;

use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct Model {
    pub(crate) text_buffer: gtk::TextBuffer,
    pub(crate) opened_path: Option<PathBuf>,
    pub(crate) opened_path_string: Option<String>,
    pub(crate) opened_file_name: Option<String>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
    SetContent(String),
    SetOpenedPath(PathBuf),
    UpdateNameAndPath(PathBuf),
}

#[derive(Debug)]
pub(crate) enum Output {
    Sync,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for Model {
    type Init = Init;

    type Input = Input;
    type Output = Output;

    type CommandOutput = ();
    type ParentInput = AppInput;
    type ParentWidget = adw::TabView;

    view! {
        gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,

            gtk::TextView {
                set_margin_all: 8,
                set_monospace: true,
                set_buffer: Some(&self.text_buffer),
            }
        }
    }

    fn init_model(
        _payload: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        let text_buffer = gtk::TextBuffer::default();
        Self {
            text_buffer,
            opened_path: None,
            opened_path_string: None,
            opened_file_name: None,
        }
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            Self::Input::SetContent(text) => {
                self.text_buffer.set_text(&text);
            }
            Self::Input::SetOpenedPath(path) => {
                self.opened_path = Some(path.clone());
                sender.input(Self::Input::UpdateNameAndPath(path));
            }
            Self::Input::UpdateNameAndPath(path) => {
                self.opened_path_string = Some(path.clone())
                    .and_then(|p| p.to_str().map(|s| String::from(s)));
                self.opened_file_name = Some(path.clone())
                    .and_then(|p| p.file_name().map(|s| OsString::from(s)))
                    .and_then(|s| s.to_str().map(|s| String::from(s)));
                let _ = sender.output(Self::Output::Sync);
            }
        }
    }

    fn forward_to_parent(output: <Self as FactoryComponent>::Output) -> Option<<Self as FactoryComponent>::ParentInput> {
        match output {
            Self::Output::Sync => Some(AppInput::DoNothing),
        }
    }
}

impl Model {
    pub(crate) fn text(&self) -> gtk::glib::GString {
        let start = self.text_buffer.start_iter();
        let end = self.text_buffer.end_iter();

        self.text_buffer.text(&start, &end, false)
    }
}
