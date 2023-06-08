use iced::{
    widget::{container, Button, Container, Row, Text},
    Alignment, Element, Sandbox, Settings,
};

use iced_aw::ContextMenu;

fn main() -> iced::Result {
    ContextMenuExample::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Message {
    OpenModal,
    CloseModal,
}

#[derive(Default)]
struct ContextMenuExample {
    show_modal: bool,
    last_message: Option<Message>,
}

impl Sandbox for ContextMenuExample {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Modal example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenModal => self.show_modal = true,
            Message::CloseModal => self.show_modal = false,
        }
        self.last_message = Some(message)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = Container::new(
            Row::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(Button::new(Text::new("Open modal!")).on_press(Message::OpenModal))
                .push(Text::new(format!(
                    "Last message: {}",
                    match self.last_message.as_ref() {
                        Some(message) => match message {
                            Message::OpenModal => "Modal opened",
                            Message::CloseModal => "Modal closed",
                        },
                        None => "None",
                    }
                ))),
        );

        ContextMenu::new(self.show_modal, content, || {
            container(my_component::MyComponent).into()
        })
        .backdrop(Message::CloseModal)
        .on_esc(Message::CloseModal)
        .into()
    }
}

mod my_component {
    use iced::{
        widget::{container, row, text},
        Element,
    };
    use iced_lazy::{self, Component};

    pub struct MyComponent;

    impl<Message> Component<Message, iced::Renderer> for MyComponent {
        type State = ();
        type Event = ();

        fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
            None
        }

        fn view(&self, _state: &Self::State) -> Element<Self::Event, iced::Renderer> {
            container(row![text("Hello there")]).into()
        }
    }

    impl<'a, Message> From<MyComponent> for Element<'a, Message, iced::Renderer>
    where
        Message: 'a,
    {
        fn from(my_component: MyComponent) -> Self {
            iced_lazy::component(my_component)
        }
    }
}
