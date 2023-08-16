use iced::{
    alignment, font,
    theme::Theme,
    widget::{container, text, Column, Container, Row, Text},
    window, Alignment, Application, Command, Element, Length, Settings,
};

#[derive(Debug)]
enum NumberInputDemo {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
pub struct State {
    value: [NumInput<f32, Message>; 2],
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<(), String>),
    GenericF32Input((usize, NumInputMessage<f32>)),
    FontLoaded(Result<(), font::Error>),
}

fn main() -> iced::Result {
    NumberInputDemo::run(Settings {
        default_text_size: 12.0,
        window: window::Settings {
            size: (500, 400),
            ..Default::default()
        },
        ..Settings::default()
    })
}

mod numberinput;

use numberinput::*;

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
                font::load(iced_aw::graphics::icons::ICON_FONT_BYTES).map(Message::FontLoaded),
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
                    *self = NumberInputDemo::Loaded(State {
                        value: [NumInput::new(27.0), NumInput::new(5.0)],
                    })
                }
            }
            NumberInputDemo::Loaded(State { value }) => {
                if let Message::GenericF32Input((id, val)) = message {
                    value[id].value = val.get_data();
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
                let mut column1 = Column::new();

                for (id, val) in value.iter().enumerate() {
                    let lb_minute = Text::new(format!("Number Input {}:", id));
                    let txt_minute = val.view(id, 1.0, 255.0, 0.5, Message::GenericF32Input, None);

                    column1 = column1.push(
                        Row::new()
                            .spacing(10)
                            .align_items(Alignment::Center)
                            .push(lb_minute)
                            .push(txt_minute),
                    );
                }

                Container::new(column1)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}
