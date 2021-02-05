use crate::Section;
use iced::{button, Align, Button, Column, Element, Length, Row, Text};
use iced_aw::{date_picker, DatePicker};

pub struct DatePickerSection {
    date_picker_state: date_picker::State,
    button_state: button::State,
    date: (i32, u32, u32),
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenDatePicker,
    CancelDate,
    SubmitDate(i32, u32, u32),
}

impl DatePickerSection {
    pub fn new() -> Self {
        Self {
            date_picker_state: date_picker::State::now(),
            button_state: button::State::new(),
            date: (0, 0, 0),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenDatePicker => self.date_picker_state.show(true),
            Message::CancelDate => self.date_picker_state.show(false),
            Message::SubmitDate(year, month, day) => {
                self.date = (year, month, day);
                self.date_picker_state.show(false);
            }
        }
    }
}

impl Section for DatePickerSection {
    type Message = crate::Message;

    fn header(&self) -> String {
        String::from("Date Picker")
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let date_picker = DatePicker::new(
            &mut self.date_picker_state,
            Button::new(&mut self.button_state, Text::new("Pick Date"))
                .on_press(Message::OpenDatePicker),
            Message::CancelDate,
            Message::SubmitDate,
        );

        let column: Element<'_, Message> = Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(date_picker)
                    .push(Text::new(format!(
                        "Picked date: {:04}-{:02}-{:02}",
                        self.date.0, self.date.1, self.date.2
                    ))),
            )
            .into();

        column.map(crate::Message::DatePicker)
    }
}
