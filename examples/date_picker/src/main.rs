use iced::{button, Align, Button, Container, Element, Length, Row, Sandbox, Settings, Text};

use iced_aw::date_picker::{self, Date, DatePicker};

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
    state: date_picker::State,
    button_state: button::State,
}

impl Sandbox for DatePickerExample {
    type Message = Message;

    fn new() -> Self {
        DatePickerExample {
            date: Date::default(),
            state: date_picker::State::now(),
            button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("DatePicker example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChooseDate => {
                self.state.reset();
                self.state.show(true);
            }
            Message::SubmitDate(date) => {
                self.date = date;
                self.state.show(false);
            }
            Message::CancelDate => {
                self.state.show(false);
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let but = Button::new(&mut self.button_state, Text::new("Set Date"))
            .on_press(Message::ChooseDate);

        let datepicker = DatePicker::new(
            &mut self.state,
            but,
            Message::CancelDate,
            Message::SubmitDate,
        );

        let row = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(datepicker)
            .push(Text::new(format!(
                "Date: {}", self.date,
            )));

        Container::new(row)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
