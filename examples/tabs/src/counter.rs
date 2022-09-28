use iced::{
    widget::{Button, Column, Container, Row, Text},
    Alignment, Element,
};
use iced_aw::tab_bar::TabLabel;

use crate::{Icon, Message, Tab};

#[derive(Debug, Clone)]
pub enum CounterMessage {
    Increase,
    Decrease,
}

pub struct CounterTab {
    value: i32,
}

impl CounterTab {
    pub fn new() -> Self {
        CounterTab { value: 0 }
    }

    pub fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increase => self.value += 1,
            CounterMessage::Decrease => self.value -= 1,
        }
    }
}

impl Tab for CounterTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn tab_label(&self) -> TabLabel {
        //TabLabel::Text(self.title())
        TabLabel::IconText(Icon::Calc.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, CounterMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(Text::new(format!("Count: {}", self.value)).size(32))
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Button::new(Text::new("Decrease")).on_press(CounterMessage::Decrease))
                        .push(
                            Button::new(Text::new("Increase")).on_press(CounterMessage::Increase),
                        ),
                ),
        )
        .into();

        content.map(Message::Counter)
    }
}
