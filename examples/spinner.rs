// This example demonstrates how to use the spinner widget
//
// This was written by Iohann Rabeson <irabeson42@gmail.com>

use iced::widget::PickList;
use iced::{
    Element, Length, Theme,
    widget::{column, container},
};
use iced_aw::Spinner;

struct SpinnerExample {
    theme: Theme,
}

impl Default for SpinnerExample {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    ThemeChanged(Theme),
}

impl SpinnerExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            container(Spinner::new())
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            PickList::new(Theme::ALL, Some(&self.theme), Message::ThemeChanged),
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn main() -> iced::Result {
    iced::application(
        "Spinner example",
        SpinnerExample::update,
        SpinnerExample::view,
    )
    .font(iced_fonts::REQUIRED_FONT_BYTES)
    .theme(SpinnerExample::theme)
    .run()
}
