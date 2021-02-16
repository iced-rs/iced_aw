use crate::Section;
use iced::{button, Align, Button, Column, Element, Length, Row, Text};
use iced_aw::{
    date_picker::{self, Date},
    DatePicker,
};

pub struct DatePickerSection {
    date_picker_state: date_picker::State,
    button_state: button::State,
    date: Date,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenDatePicker,
    CancelDate,
    SubmitDate(Date),
}

impl Section for DatePickerSection {
    type Message = Message;

    fn new() -> Self {
        Self {
            date_picker_state: date_picker::State::now(),
            button_state: button::State::new(),
            date: Date::default(),
        }
    }

    fn header(&self) -> String {
        String::from("Date Picker")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenDatePicker => self.date_picker_state.show(true),
            Message::CancelDate => self.date_picker_state.show(false),
            Message::SubmitDate(date) => {
                self.date = date;
                self.date_picker_state.show(false);
            }
        }
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let date_picker = DatePicker::new(
            &mut self.date_picker_state,
            Button::new(&mut self.button_state, Text::new("Pick Date"))
                .on_press(Message::OpenDatePicker),
            Message::CancelDate,
            Message::SubmitDate,
        );

        let column = Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(date_picker)
                    .push(Text::new(format!("Picked date: {}", self.date))),
            );

        column.into()
    }
}
