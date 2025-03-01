// This example demonstrates how to use the time_picker widget
//
// It was written by Kaiden42 <gitlab@tinysn.com>

use iced::{
    Alignment, Element, Length,
    widget::{Button, Container, Row, Text},
};
use iced_aw::{TimePicker, time_picker::Time};

fn main() -> iced::Result {
    iced::application(
        "TimePicker example",
        TimePickerExample::update,
        TimePickerExample::view,
    )
    .font(iced_fonts::REQUIRED_FONT_BYTES)
    .run()
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    ChooseTime,
    SubmitTime(Time),
    CancelTime,
}

#[derive(Debug, Default)]
struct TimePickerExample {
    time: Time,
    show_picker: bool,
}

impl TimePickerExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChooseTime => {
                self.show_picker = true;
            }
            Message::SubmitTime(time) => {
                self.time = time;
                self.show_picker = false;
            }
            Message::CancelTime => {
                self.show_picker = false;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let but = Button::new(Text::new("Set Time")).on_press(Message::ChooseTime);

        let timepicker = TimePicker::new(
            self.show_picker,
            self.time,
            but,
            Message::CancelTime,
            Message::SubmitTime,
        )
        .use_24h();

        let row = Row::new()
            .align_y(Alignment::Center)
            .spacing(10)
            .push(timepicker)
            .push(Text::new(format!("Time: {}", self.time)));

        Container::new(row)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
