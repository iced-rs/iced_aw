use crate::Section;
use iced::{button, Align, Button, Checkbox, Column, Element, Length, Row, Text};
use iced_aw::{time_picker, TimePicker};

pub struct TimePickerSection {
    time_picker_state: time_picker::State,
    button_state: button::State,
    time: time_picker::Time,
    show_seconds: bool,
    use_24h: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenTimePicker,
    CancelTime,
    SubmitTime(time_picker::Time),
    ToggleSeconds(bool),
    Toggle24h(bool),
}

impl Section for TimePickerSection {
    type Message = Message;

    fn new() -> Self {
        Self {
            time_picker_state: time_picker::State::now(),
            button_state: button::State::new(),
            time: time_picker::Time::default_hm(time_picker::Period::H24),
            show_seconds: false,
            use_24h: false,
        }
    }

    fn header(&self) -> String {
        String::from("Time Picker")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenTimePicker => self.time_picker_state.show(true),
            Message::CancelTime => self.time_picker_state.show(false),
            Message::SubmitTime(time) => {
                self.time = time;
                self.time_picker_state.show(false);
            }
            Message::ToggleSeconds(b) => {
                self.show_seconds = b;
                self.time_picker_state.reset();
            }
            Message::Toggle24h(b) => {
                self.use_24h = b;
                self.time_picker_state.reset();
            }
        }
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let mut time_picker = TimePicker::new(
            &mut self.time_picker_state,
            Button::new(&mut self.button_state, Text::new("Pick Time"))
                .on_press(Message::OpenTimePicker),
            Message::CancelTime,
            Message::SubmitTime,
        )
        //.use_24h()
        //.show_seconds()
        ;

        if self.show_seconds {
            time_picker = time_picker.show_seconds();
        }
        if self.use_24h {
            time_picker = time_picker.use_24h();
        }

        let column = Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .spacing(20)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .spacing(40)
                    .push(Checkbox::new(
                        self.show_seconds,
                        "Show seconds",
                        Message::ToggleSeconds,
                    ))
                    .push(Checkbox::new(
                        self.use_24h,
                        "Use 24h format",
                        Message::Toggle24h,
                    )),
            )
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(time_picker)
                    .push(Text::new(format!("Picked time: {}", self.time))),
            );

        column.into()
    }
}
