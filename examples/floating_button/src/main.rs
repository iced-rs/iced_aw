
use iced::{
    button, scrollable, Button, Element, Container,
    Length, Text, Sandbox, Scrollable, Settings
};

use iced_aw::{floating_button, FloatingButton, Icon, ICON_FONT};

fn main() -> iced::Result {
    FloatingButtonExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

struct FloatingButtonExample {
    button_state: button::State,
    scrollable_state: scrollable::State,
    lines: Vec<String>,
}

impl Sandbox for FloatingButtonExample {
    type Message = Message;

    fn new() -> Self {
        FloatingButtonExample {
            button_state: button::State::new(),
            scrollable_state: scrollable::State::new(),
            lines: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("FloatingButton example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.lines.push("This is a newly added line.".into())
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let scrollable_content = self.lines.iter().enumerate()
            .fold(
                Scrollable::new(&mut self.scrollable_state)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(10),
                |scroll, (i, line)| {
                    scroll.push(Text::new(format!("{}. {}", i+1, line)))
                }
            );

        let content = FloatingButton::new(
            &mut self.button_state,
            Container::new(scrollable_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .max_width(400)
                .max_height(600)
                .style(BorderedContainer),
            |state| Button::new(
                    state,
                    Text::new(Icon::Plus)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .font(ICON_FONT)
                        .size(39)
                )
                //.style(iced_aw::style::button::Primary),
                .on_press(Message::ButtonPressed)
                .padding(5)
                .style(RoundedButton),
        )
        .anchor(floating_button::Anchor::SouthEast)
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

struct RoundedButton;

impl iced::button::StyleSheet for RoundedButton {
    fn active(&self) -> iced::button::Style {
        iced::button::Style {
            border_radius: 25,
            .. iced_aw::style::button::Primary.active()
        }
    }
}

struct BorderedContainer;

impl iced::container::StyleSheet for BorderedContainer {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            border_width: 1,
            border_color: iced::Color::BLACK,
            border_radius: 10,
            .. Default::default()
        }
    }
}