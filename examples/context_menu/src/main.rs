use iced::{
    widget::{column, Button, Container, Row, Text},
    Alignment, Element,
};

use iced_aw::ContextMenu;

fn main() -> iced::Result {
    iced::application(
        "ContextMenu example",
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
                .align_items(Alignment::Center)
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
            column(vec![
                iced::widget::button("Choice 1")
                    .on_press(Message::Choice1)
                    .into(),
                iced::widget::button("Choice 2")
                    .on_press(Message::Choice2)
                    .into(),
                iced::widget::button("Choice 3")
                    .on_press(Message::Choice3)
                    .into(),
                iced::widget::button("Choice 4")
                    .on_press(Message::Choice4)
                    .into(),
            ])
            .into()
        })
        .into()
    }
}
