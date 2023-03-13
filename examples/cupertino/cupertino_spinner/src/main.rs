use iced::{Application, Command, Element, executor, Length, Settings, Theme};
use iced::alignment;
use iced::widget::{column, container, text};
use iced_aw::native::cupertino::cupertino_spinner::CupertinoSpinner;

pub fn main() -> iced::Result {
    Spinner::run(Settings { antialiasing: true, ..Settings::default() })
}

#[derive(Debug, Clone)]
struct State {
    hello: String,
}

enum Spinner {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<State, ()>),
}

impl State {
    async fn load() -> Result<State, ()> {
        println!("Doing stuff...");
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        return Ok(Self { hello: "Loaded!".to_string() });
    }
}

impl Application for Spinner {
    type Executor = executor::Default;
    type Message  = Message;
    type Theme    = Theme;
    type Flags    = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Spinner::Loading, Command::perform(State::load(), Message::Loaded))
    }

    fn title(&self) -> String { String::from("CupertinoSpinner - Iced") }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Spinner::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Spinner::Loaded(State {
                            hello: state.hello,
                        });
                    },

                    _ => ()
                }
            },

            _ => ()

        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self {
            Spinner::Loading => {
                container(
                    CupertinoSpinner::new().width(Length::Fill).height(Length::Fill)
                ).into()
            },

            Spinner::Loaded(state) => {
                container(column![text(&state.hello)
                    .width(Length::Fill)
                    .size(25)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                ])
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_y()
                    .into()
            },
        }
    }

    fn theme(&self) -> Self::Theme { Theme::Light }
}

