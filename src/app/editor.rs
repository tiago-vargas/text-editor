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

impl Model {
    pub(crate) fn text(&self) -> gtk::glib::GString {
        let start = self.text_buffer.start_iter();
        let end = self.text_buffer.end_iter();

        self.text_buffer.text(&start, &end, false)
    }
}

pub(crate) mod editor2 {
    use gtk::prelude::*;
    use relm4::prelude::*;

    pub(crate) struct Model {
        pub(crate) text_buffer: gtk::TextBuffer,
    }

    pub(crate) struct Init;

    #[derive(Debug)]
    pub(crate) enum Input {
        Set(String),
    }

    #[derive(Debug)]
    pub(crate) enum Output {
        Selected(DynamicIndex),
    }

    #[relm4::factory(pub(crate))]
    impl FactoryComponent for Model {
        type Init = Init;
        type Input = Input;
        type Output = Output;

        type CommandOutput = ();
        type ParentInput = crate::app::AppInput;
        type ParentWidget = adw::TabView;

        view! {
            #[root]
            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,

                gtk::TextView {
                    set_margin_all: 8,
                    set_monospace: true,
                    set_buffer: Some(&self.text_buffer),
                }
            },

            #[local_ref]
            returned_widget -> adw::TabPage {
                set_title: "Untitled",

                connect_selected_notify[sender, index] => move |_| {
                    sender.output(Self::Output::Selected(index.clone()));
                },
            },
        }

        fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
            Some(match output {
                // Self::Output::SendFront(index) => AppMsg::SendFront(index),
                Self::Output::Selected(index) => crate::app::AppInput::UpdateSelectedTab(index),
            })
        }

        fn init_model(
            _value: Self::Init,
            _index: &DynamicIndex,
            _sender: FactorySender<Self>
        ) -> Self {
            let text_buffer = gtk::TextBuffer::default();
            Self { text_buffer }
        }

        fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
            match msg {
                Self::Input::Set(content) =>  {}
            }
        }
    }
}
