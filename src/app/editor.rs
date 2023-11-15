use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct Model {
    pub(crate) text_buffer: gtk::TextBuffer,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
    SetContent(String),
}

#[derive(Debug)]
pub(crate) enum Output {}

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
        let model = Self { text_buffer };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::SetContent(text) => {
                self.text_buffer.set_text(&text);
            }
        }
    }
}
