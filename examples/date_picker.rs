// This example demonstrates how to use the [`date_picker`] widget.
//
// It was written by Kaiden42 <gitlab@tinysn.com>

use iced::{
    Alignment, Element, Length,
    widget::{Button, Container, Row, Text},
};
use iced_aw::{date_picker::Date, helpers::date_picker};

fn main() -> iced::Result {
    iced::application(
        "DatePicker example",
        DatePickerExample::update,
        DatePickerExample::view,
    )
    .font(iced_fonts::REQUIRED_FONT_BYTES)
    .run()
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    ChooseDate,
    SubmitDate(Date),
    CancelDate,
}

#[derive(Default)]
struct DatePickerExample {
    date: Date,
    show_picker: bool,
}

impl DatePickerExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChooseDate => {
                self.show_picker = true;
            }
            Message::SubmitDate(date) => {
                self.date = date;
                self.show_picker = false;
            }
            Message::CancelDate => {
                self.show_picker = false;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let but = Button::new(Text::new("Set Date")).on_press(Message::ChooseDate);

        let datepicker = date_picker(
            self.show_picker,
            self.date,
            but,
            Message::CancelDate,
            Message::SubmitDate,
        );

        let row = Row::new()
            .align_y(Alignment::Center)
            .spacing(10)
            .push(datepicker)
            .push(Text::new(format!("Date: {}", self.date,)));

        Container::new(row)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
