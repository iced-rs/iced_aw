use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{
    alignment, font, theme,
    widget::{container, text, Button, Container, PickList, Text},
    Application, Command, Element, Length, Settings, Theme,
};

use iced_aw::core::icons::bootstrap::icon_to_string;
use iced_aw::floating_element::Anchor;
use iced_aw::Bootstrap;
use iced_aw::{helpers::floating_element, BOOTSTRAP_FONT};

fn main() -> iced::Result {
    FloatingElementExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
    LanguageSelected(Language),
}

#[derive(Debug)]
enum FloatingElementExample {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct State {
    selected_language: Option<Language>,
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
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
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
                        selected_language: None,
                    })
                }
            }
            FloatingElementExample::Loaded(State { selected_language }) => match message {
                Message::ButtonPressed => println!("Test 123 added new line"),
                Message::LanguageSelected(language) => *selected_language = Some(language),
                _ => {}
            },
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
            FloatingElementExample::Loaded(State { selected_language }) => {
                let content = floating_element(
                    Container::new(PickList::new(
                        &Language::ALL[..],
                        *selected_language,
                        Message::LanguageSelected,
                    ))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .max_width(400)
                    .max_height(600)
                    .style(theme::Container::Box),
                    Button::new(
                        Text::new(icon_to_string(Bootstrap::Plus))
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
        appearance.border.radius = 25.0.into();

        appearance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    Javascript,
    Other,
}

impl Language {
    const ALL: [Language; 7] = [
        Language::C,
        Language::Elm,
        Language::Ruby,
        Language::Haskell,
        Language::Rust,
        Language::Javascript,
        Language::Other,
    ];
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Rust => "Rust",
                Language::Elm => "Elm",
                Language::Ruby => "Ruby",
                Language::Haskell => "Haskell",
                Language::C => "C",
                Language::Javascript => "Javascript",
                Language::Other => "Some other language",
            }
        )
    }
}
