use crate::Section;
use iced::{Align, Column, Element, Row, Text};
use iced_aw::Badge;

pub struct BadgeSection;

impl Section for BadgeSection {
    type Message = crate::Message;

    fn new() -> Self {
        Self {}
    }

    fn header(&self) -> String {
        String::from("Badge")
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .spacing(10)
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .spacing(10)
                    .push(Badge::new(Text::new("Default")).style(iced_aw::style::badge::Default))
                    .push(Badge::new(Text::new("Primary")).style(iced_aw::style::badge::Primary))
                    .push(
                        Badge::new(Text::new("Secondary")).style(iced_aw::style::badge::Secondary),
                    )
                    .push(Badge::new(Text::new("Success")).style(iced_aw::style::badge::Success))
                    .push(Badge::new(Text::new("Danger")).style(iced_aw::style::badge::Danger)),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Badge::new(Text::new("Warning")).style(iced_aw::style::badge::Warning))
                    .push(Badge::new(Text::new("Info")).style(iced_aw::style::badge::Info))
                    .push(Badge::new(Text::new("Dark")).style(iced_aw::style::badge::Dark))
                    .push(Badge::new(Text::new("Light")).style(iced_aw::style::badge::Light))
                    .push(Badge::new(Text::new("White")).style(iced_aw::style::badge::White)),
            )
            .into()
    }
}
