use iced::{
    widget::{Button, Container, Row, Text},
    Alignment, Element, Length, Sandbox, Settings,
};
use iced_aw::{time_picker::Time, TimePicker};

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
    show_picker: bool,
}

impl Sandbox for TimePickerExample {
    type Message = Message;

    fn new() -> Self {
        TimePickerExample {
            time: Time::now_hm(true),
            show_picker: false,
        }
    }

    fn title(&self) -> String {
        String::from("TimePicker example")
    }

    fn update(&mut self, message: Self::Message) {
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

    fn view(&self) -> Element<'_, Self::Message> {
        let but = Button::new(Text::new("Set Time")).on_press(Message::ChooseTime);

        let timepicker = TimePicker::new(
            self.show_picker,
            self.time,
            but,
            Message::CancelTime,
            Message::SubmitTime,
        )
        //.show_seconds()
        .use_24h();

        let row = Row::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(timepicker)
            .push(Text::new(format!("Time: {}", self.time)));

        Container::new(row)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
