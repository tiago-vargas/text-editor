use std::path::PathBuf;

use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct Model {
    pub(crate) text_buffer: gtk::TextBuffer,
    pub(crate) opened_path_string: Option<String>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
    SetContent(String),
    UpdateNameAndPath(PathBuf),
}

#[derive(Debug)]
pub(crate) enum Output {
    UpdateNameAndPath(PathBuf),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
    type Init = Init;

    type Input = Input;
    type Output = Output;

    view! {
        gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,

            gtk::TextView {
                set_margin_all: 8,
                set_monospace: true,
                set_buffer: Some(&model.text_buffer),
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let text_buffer = gtk::TextBuffer::default();
        let model = Self {
            text_buffer,
            opened_path_string: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Self::Input::SetContent(text) => {
                self.text_buffer.set_text(&text);
            }
            Self::Input::UpdateNameAndPath(path) => {
                self.opened_path_string = Some(path.clone())
                    .and_then(|p| p.to_str().map(|s| String::from(s)));
                let _ = sender.output(Self::Output::UpdateNameAndPath(path));
            }
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
