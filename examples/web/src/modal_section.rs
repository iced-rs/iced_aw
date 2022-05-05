use crate::Section;
use iced::{button, Alignment, Button, Column, Element, Length, Text};

pub struct ModalSection {
    open_button: button::State,
}

impl Section for ModalSection {
    type Message = crate::Message;

    fn new() -> Self {
        Self {
            open_button: button::State::new(),
        }
    }

    fn header(&self) -> String {
        String::from("Modal")
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .push(
                Button::new(&mut self.open_button, Text::new("Open modal!"))
                    .on_press(crate::Message::OpenPrimaryModal),
            )
            .into()
    }
}
