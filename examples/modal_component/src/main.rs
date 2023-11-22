use iced::{
    alignment, font,
    widget::{container, text, Button, Container, Row, Text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

use iced_aw::Modal;

fn main() -> iced::Result {
    ModalExample::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Message {
    OpenModal,
    CloseModal,
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
enum ModalExample {
    Loading,
    Loaded(State),
}

#[derive(Default, Debug)]
struct State {
    show_modal: bool,
    last_message: Option<Message>,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for ModalExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ModalExample, Command<Message>) {
        (
            ModalExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::graphics::icons::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Modal example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            ModalExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = ModalExample::Loaded(State::default())
                }
            }
            ModalExample::Loaded(state) => {
                match message {
                    Message::OpenModal => state.show_modal = true,
                    Message::CloseModal => state.show_modal = false,
                    _ => {}
                }
                state.last_message = Some(message)
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            ModalExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            ModalExample::Loaded(state) => {
                let underlay = Container::new(
                    Row::new()
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .push(Button::new(Text::new("Open modal!")).on_press(Message::OpenModal))
                        .push(Text::new(format!(
                            "Last message: {}",
                            match state.last_message.as_ref() {
                                Some(message) => match message {
                                    Message::OpenModal => "Modal opened",
                                    Message::CloseModal => "Modal closed",
                                    _ => "None",
                                },
                                None => "None",
                            }
                        ))),
                );

                let overlay = if state.show_modal {
                    Some(container(my_component::MyComponent))
                } else {
                    None
                };

                Modal::new(underlay, overlay)
                    .backdrop(Message::CloseModal)
                    .on_esc(Message::CloseModal)
                    .into()
            }
        }
    }
}

mod my_component {
    use iced::{
        widget::{component, container, row, text, Component},
        Element, Renderer,
    };

    pub struct MyComponent;

    impl<Message> Component<Message, Renderer> for MyComponent {
        type State = ();
        type Event = ();

        fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
            None
        }

        fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
            container(row![text("Hello there")]).into()
        }
    }

    impl<'a, Message> From<MyComponent> for Element<'a, Message, Renderer>
    where
        Message: 'a,
    {
        fn from(my_component: MyComponent) -> Self {
            component(my_component)
        }
    }
}
