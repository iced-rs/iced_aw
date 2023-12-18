use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{
    alignment, font, theme,
    widget::{container, text, Button, Column, Container, Scrollable, Text},
    Application, Command, Element, Length, Settings, Theme,
};

use iced_aw::floating_element::Anchor;
use iced_aw::graphics::icons::icon_to_string;
use iced_aw::BootstrapIcon;
use iced_aw::{helpers::floating_element, BOOTSTRAP_FONT};

fn main() -> iced::Result {
    FloatingElementExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
enum FloatingElementExample {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct State {
    lines: Vec<String>,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for FloatingElementExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (FloatingElementExample, Command<Message>) {
        (
            FloatingElementExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::graphics::icons::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("FloatingButton example")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            FloatingElementExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = FloatingElementExample::Loaded(State {
                        lines: (0..3000)
                            .map(|_| "This is a newly added line.".into())
                            .collect(),
                    })
                }
            }
            FloatingElementExample::Loaded(State { lines }) => {
                if let Message::ButtonPressed = message {
                    lines.push("This is a newly added line.".into());
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self {
            FloatingElementExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            FloatingElementExample::Loaded(State { lines }) => {
                let scrollable_content = lines.iter().enumerate().fold(
                    Column::new()
                        .width(Length::Fill)
                        .height(Length::Shrink)
                        .padding(10),
                    |scroll, (i, line)| scroll.push(Text::new(format!("{}. {}", i + 1, line))),
                );
                let scrollable_content = Scrollable::new(scrollable_content).height(Length::Fill);

                let content = floating_element(
                    Container::new(scrollable_content)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .max_width(400)
                        .max_height(600)
                        .style(theme::Container::Box),
                    Button::new(
                        Text::new(icon_to_string(BootstrapIcon::Plus))
                            .font(BOOTSTRAP_FONT)
                            .size(35)
                            .line_height(1.0)
                            .shaping(text::Shaping::Advanced),
                    )
                    .on_press(Message::ButtonPressed)
                    .padding(5)
                    .style(theme::Button::Custom(Box::new(
                        CircleButtonStyle::new(theme::Button::Primary),
                    ))),
                )
                .anchor(Anchor::SouthEast)
                .offset(20.0)
                .hide(false);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(10)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}

struct CircleButtonStyle {
    theme: theme::Button,
}

impl CircleButtonStyle {
    pub fn new(theme: theme::Button) -> Self {
        Self { theme }
    }
}

impl button::StyleSheet for CircleButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        let mut appearance = style.active(&self.theme);
        appearance.border_radius = 25.0.into();

        appearance
    }
}
