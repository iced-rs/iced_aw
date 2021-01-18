use iced::{button, Align, Button, Container, Element, Length, Sandbox, Row, Settings, Text};

use iced_aw::time_picker::{self, Time, TimePicker, Period};

fn main() -> iced::Result {
    TimePickerExample::run(Settings::default())
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    ChooseTime,
    SubmitTime(Time),
    CancelTime,
}

struct TimePickerExample {
    time: Time,
    state: time_picker::State,
    button_state: button::State,
}

impl Sandbox for TimePickerExample {
    
    type Message = Message;

    fn new() -> Self {
        TimePickerExample {
            time: Time::default_hm(Period::H24),
            state: time_picker::State::now(),
            button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("TimePicker example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChooseTime => {
                self.state.reset();
                self.state.show(true);
            },
            Message::SubmitTime(time) => {
                self.time = time;
                self.state.show(false);
            },
            Message::CancelTime => {
                self.state.show(false);
            },
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let but = Button::new(&mut self.button_state, Text::new("Set Time"))
            .on_press(Message::ChooseTime);

        let timepicker = TimePicker::new(
            &mut self.state,
            but,
            Message::CancelTime,
            Message::SubmitTime,
        )
        //.show_seconds()
        .use_24h()
        ;

        let row = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(timepicker)
            .push(
                Text::new(format!("Time: {}", self.time))
            );

        Container::new(row)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}