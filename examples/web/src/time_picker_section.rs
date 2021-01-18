use crate::Section;
use iced::{button, Align, Button, Column, Element, Length, Row, Text};
use iced_aw::{time_picker, TimePicker};

pub struct TimePickerSection {
    time_picker_state: time_picker::State,
    button_state: button::State,
    time: time_picker::Time,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenTimePicker,
    CancelTime,
    SubmitTime(time_picker::Time),
}

impl TimePickerSection {
    pub fn new() -> Self {
        Self {
            time_picker_state: time_picker::State::now(),
            button_state: button::State::new(),
            time: time_picker::Time::default_hm(time_picker::Period::H24),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenTimePicker => self.time_picker_state.show(true),
            Message::CancelTime => self.time_picker_state.show(false),
            Message::SubmitTime(time) => {
                self.time = time;
                self.time_picker_state.show(false);
            },
        }
    }
}

impl Section for TimePickerSection {
    
    type Message = crate::Message;

    fn header(&self) -> String {
        String::from("Time Picker")
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let time_picker = TimePicker::new(
            &mut self.time_picker_state,
            Button::new(&mut self.button_state, Text::new("Pick Time"))
                .on_press(Message::OpenTimePicker),
            Message::CancelTime,
            Message::SubmitTime,
        )
        //.use_24h()
        //.show_seconds()
        ;

        let column: Element<'_, Message> = Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(time_picker)
                    .push(
                        Text::new(format!("Picked time: {}", self.time))
                    )
            ).into();

        column.map(crate::Message::TimePicker)
    }

}