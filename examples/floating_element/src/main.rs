use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{
    theme,
    widget::{Button, Column, Container, Scrollable, Text},
    Element, Length, Sandbox, Settings, Theme,
};

use iced_aw::floating_element::Anchor;
use iced_aw::{helpers::floating_element, Icon, ICON_FONT};

fn main() -> iced::Result {
    FloatingElementExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

struct FloatingElementExample {
    lines: Vec<String>,
}

impl Sandbox for FloatingElementExample {
    type Message = Message;

    fn new() -> Self {
        FloatingElementExample { lines: Vec::new() }
    }

    fn title(&self) -> String {
        String::from("FloatingButton example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => self.lines.push("This is a newly added line.".into()),
        }
    }

    fn view(&self) -> Element<Message> {
        let scrollable_content = self.lines.iter().enumerate().fold(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10),
            |scroll, (i, line)| scroll.push(Text::new(format!("{}. {}", i + 1, line))),
        );
        let scrollable_content = Scrollable::new(scrollable_content);

        let content = floating_element(
            Container::new(scrollable_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .max_width(400)
                .max_height(600)
                .style(theme::Container::Box),
            || {
                Button::new(
                    Text::new(Icon::Plus.to_string())
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .font(ICON_FONT)
                        .size(39),
                )
                //.style(iced_aw::style::button::Primary),
                .on_press(Message::ButtonPressed)
                .padding(5)
                .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
                    theme::Button::Primary,
                ))))
                .into()
            },
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
        appearance.border_radius = 200.0;

        appearance
    }
}
