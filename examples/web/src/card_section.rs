use crate::Section;
use iced::{Element, Row, Text};
use iced_aw::Card;

pub struct CardSection {
    primary_card: bool,
    secondary_card: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    PrimaryCardClose,
    SecondaryCardClose,
}

impl Section for CardSection {
    type Message = Message;

    fn new() -> Self {
        Self {
            primary_card: true,
            secondary_card: true,
        }
    }

    fn header(&self) -> String {
        String::from("Card")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::PrimaryCardClose => self.primary_card = false,
            Message::SecondaryCardClose => self.secondary_card = false,
        }
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let mut row = Row::new().spacing(20);

        if self.primary_card {
            row = row.push(
                Card::new(Text::new("Primary"), Text::new("Primary text"))
                    .on_close(Message::PrimaryCardClose)
                    .style(iced_aw::style::card::Primary),
            );
        }

        if self.secondary_card {
            row = row.push(
                Card::new(Text::new("Secondary"), Text::new("Secondary text"))
                    .on_close(Message::SecondaryCardClose)
                    .style(iced_aw::style::card::Secondary),
            );
        }

        row.into()
    }
}
