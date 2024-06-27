use iced::widget::PickList;
use iced::{
    widget::{column, container},
    Element, Length, Theme,
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
    .font(iced_aw::BOOTSTRAP_FONT_BYTES)
    .theme(SpinnerExample::theme)
    .run()
}
