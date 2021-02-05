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

impl CardSection {
    pub fn new() -> Self {
        Self {
            primary_card: true,
            secondary_card: true,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::PrimaryCardClose => self.primary_card = false,
            Message::SecondaryCardClose => self.secondary_card = false,
        }
    }
}

impl Section for CardSection {
    type Message = crate::Message;

    fn header(&self) -> String {
        String::from("Card")
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

        let row: Element<'_, Message> = row.into();

        row.map(crate::Message::Card)
    }
}
