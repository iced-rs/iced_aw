use iced::{
    alignment, font,
    widget::{container, text, Button, Container, Row, Text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::{date_picker::Date, helpers::date_picker};

fn main() -> iced::Result {
    DatePickerExample::run(Settings::default())
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    ChooseDate,
    SubmitDate(Date),
    CancelDate,
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

enum DatePickerExample {
    Loading,
    Loaded(State),
}

struct State {
    date: Date,
    show_picker: bool,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for DatePickerExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (DatePickerExample, Command<Message>) {
        (
            DatePickerExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::graphics::icons::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("DatePicker example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            DatePickerExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = DatePickerExample::Loaded(State {
                        date: Date::today(),
                        show_picker: false,
                    })
                }
            }
            DatePickerExample::Loaded(state) => match message {
                Message::ChooseDate => {
                    state.show_picker = true;
                }
                Message::SubmitDate(date) => {
                    state.date = date;
                    state.show_picker = false;
                }
                Message::CancelDate => {
                    state.show_picker = false;
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            DatePickerExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            DatePickerExample::Loaded(state) => {
                let but = Button::new(Text::new("Set Date")).on_press(Message::ChooseDate);

                let datepicker = date_picker(
                    state.show_picker,
                    state.date,
                    but,
                    Message::CancelDate,
                    Message::SubmitDate,
                );

                let row = Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(datepicker)
                    .push(Text::new(format!("Date: {}", state.date,)));

                Container::new(row)
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }
}
