use iced::widget::PickList;
use iced::{
    widget::{column, container},
    Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::Spinner;
use std::fmt::{Display, Formatter};

struct SpinnerExample {
    theme: ThemeSelection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ThemeSelection {
    Dark,
    Light,
}

impl Display for ThemeSelection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeSelection::Dark => write!(f, "Dark"),
            ThemeSelection::Light => write!(f, "Light"),
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    ThemeChanged(ThemeSelection),
}

const AVAILABLE_THEMES: [ThemeSelection; 2] = [ThemeSelection::Light, ThemeSelection::Dark];

impl Application for SpinnerExample {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                theme: ThemeSelection::Light,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Spinner")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            container(Spinner::new())
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y(),
            PickList::new(
                AVAILABLE_THEMES.as_slice(),
                Some(self.theme),
                Message::ThemeChanged
            ),
        ]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        match self.theme {
            ThemeSelection::Dark => Theme::Dark,
            ThemeSelection::Light => Theme::Light,
        }
    }
}

fn main() -> iced::Result {
    SpinnerExample::run(Settings::default())
}
