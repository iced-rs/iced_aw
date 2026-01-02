// This example demonstrates how to use the `ContextMenu` widget
//
// It was written by wiiznokes <wiiznokes2@gmail.com>

use iced::{
    Alignment, Background, Border, Element, Length, Theme,
    widget::{Button, Container, Row, Text, column, container},
};

use iced_aw::{ContextMenu, MenuButton};

fn main() -> iced::Result {
    iced::application(
        ContextMenuExample::default,
        ContextMenuExample::update,
        ContextMenuExample::view,
    )
    .run()
}

#[derive(Clone, Debug)]
pub enum Message {
    ButtonClicked,
    Choice1,
    Choice2,
    Choice3,
    Choice4,
}

#[derive(Default)]
struct ContextMenuExample {
    last_message: Option<Message>,
}

impl ContextMenuExample {
    fn update(&mut self, message: Message) {
        self.last_message = Some(message);
    }

    fn view(&self) -> Element<'_, Message> {
        let underlay = Container::new(
            Row::new()
                .spacing(10)
                .align_y(Alignment::Center)
                .push(Button::new(Text::new("right click me!")).on_press(Message::ButtonClicked))
                .push(Text::new(format!(
                    "Last message: {}",
                    match self.last_message.as_ref() {
                        Some(message) => match message {
                            Message::ButtonClicked => "button clicked",
                            Message::Choice1 => "choice 1",
                            Message::Choice2 => "choice 2",
                            Message::Choice3 => "choice 3",
                            Message::Choice4 => "choice 4",
                        },
                        None => "None",
                    }
                ))),
        );

        ContextMenu::new(underlay, || {
            Container::new(
                column(vec![
                    MenuButton::new("Choice 1")
                        .on_press(Message::Choice1)
                        .into(),
                    MenuButton::new("Choice 2")
                        .on_press(Message::Choice2)
                        .into(),
                    MenuButton::new("Choice 3")
                        .on_press(Message::Choice3)
                        .into(),
                    MenuButton::new("Choice 4")
                        .on_press(Message::Choice4)
                        .into(),
                ])
                .spacing(2),
            )
            .padding(4)
            .width(Length::Fixed(150.0))
            .style(|theme: &Theme| container::Style {
                background: Some(Background::Color(theme.palette().background)),
                border: Border {
                    color: theme.palette().text,
                    width: 0.5,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
        })
        .into()
    }
}
