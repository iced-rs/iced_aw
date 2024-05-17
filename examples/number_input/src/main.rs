use iced::{
    alignment, font,
    theme::Theme,
    widget::{container, text, Container, Row, Text},
    window, Alignment, Application, Command, Element, Length, Settings,
};
use iced_aw::{number_input, style::NumberInputStyles};

#[derive(Debug)]
enum NumberInputDemo {
    Loading,
    Loaded(State),
}

#[derive(Default, Debug)]
pub struct State {
    value: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    NumInpChanged(f32),
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

fn main() -> iced::Result {
    NumberInputDemo::run(Settings {
        default_text_size: iced::Pixels(12.0),
        window: window::Settings {
            size: iced::Size {
                width: 250.0,
                height: 200.0,
            },
            ..Default::default()
        },
        ..Settings::default()
    })
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for NumberInputDemo {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (NumberInputDemo, Command<Message>) {
        (
            NumberInputDemo::Loading,
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Number Input Demo")
    }

    fn update(&mut self, message: self::Message) -> Command<Message> {
        match self {
            NumberInputDemo::Loading => {
                if let Message::Loaded(_) = message {
                    *self = NumberInputDemo::Loaded(State { value: 27.0 })
                }
            }
            NumberInputDemo::Loaded(State { value }) => {
                if let Message::NumInpChanged(val) = message {
                    *value = val;
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self {
            NumberInputDemo::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            NumberInputDemo::Loaded(State { value }) => {
                let lb_minute = Text::new("Number Input:");
                let txt_minute = number_input(*value, 0.0..250.0, Message::NumInpChanged)
                    .style(NumberInputStyles::Default)
                    .step(0.5);

                Container::new(
                    Row::new()
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .push(lb_minute)
                        .push(txt_minute),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            }
        }
    }
}
