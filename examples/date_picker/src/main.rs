use iced::{
    widget::{Button, Container, Row, Text},
    Alignment, Element, Length, Sandbox, Settings,
};
use iced_aw::{date_picker::Date, DatePicker};

fn main() -> iced::Result {
    DatePickerExample::run(Settings::default())
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    ChooseDate,
    SubmitDate(Date),
    CancelDate,
}

struct DatePickerExample {
    date: Date,
    show_picker: bool,
}

impl Sandbox for DatePickerExample {
    type Message = Message;

    fn new() -> Self {
        DatePickerExample {
            date: Date::today(),
            show_picker: false,
        }
    }

    fn title(&self) -> String {
        String::from("DatePicker example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChooseDate => {
                //self.state.reset();
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

    fn view(&self) -> Element<'_, Self::Message> {
        let but = Button::new(Text::new("Set Date")).on_press(Message::ChooseDate);

        let datepicker = DatePicker::new(
            self.show_picker,
            self.date,
            but,
            Message::CancelDate,
            Message::SubmitDate,
        );

        let row = Row::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(datepicker)
            .push(Text::new(format!("Date: {}", self.date,)));

        Container::new(row)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
