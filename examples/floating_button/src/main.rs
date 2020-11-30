
use iced::{button, Button, Element, Column, Container, Length, Text, Sandbox, Settings};

use iced_aw::{floating_button, FloatingButton};

fn main() -> iced::Result {
    FloatingButtonExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

struct FloatingButtonExample {
    state: button::State,
}

impl Sandbox for FloatingButtonExample {
    type Message = Message;

    fn new() -> Self {
        FloatingButtonExample {
            state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("FloatingButton example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                println!("Hello World!");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        FloatingButton::new(
            Container::new(
                Column::new()
                .push(Text::new("Line"))
                .push(Text::new("Line"))
                .push(Text::new("Line"))
                .push(Text::new("Line"))
                .push(Text::new("Line"))
                .push(Text::new("Line"))
                .push(Text::new("Line"))
                .push(Text::new("Line"))
                .push(Text::new("Line"))
            )
            .width(Length::Fill)
            .height(Length::Fill),
            Button::new(&mut self.state, Text::new("Press Me!"))
                .style(iced_aw::style::button::Primary),
        )
        .on_press(Message::ButtonPressed)
        .anchor(floating_button::Anchor::SouthEast)
        .offset(20.0)
        .hide(false)
        .into()
    }
}