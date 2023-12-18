use iced::{
    alignment, font,
    widget::{container, text, Button, Container, Row, Text},
    Alignment, Application, Command, Element, Length, Settings, Theme,
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
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
enum TimePickerExample {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct State {
    time: Time,
    show_picker: bool,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for TimePickerExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (TimePickerExample, Command<Message>) {
        (
            TimePickerExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::graphics::icons::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("TimePicker example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            TimePickerExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = TimePickerExample::Loaded(State {
                        time: Time::now_hm(true),
                        show_picker: false,
                    })
                }
            }
            TimePickerExample::Loaded(state) => match message {
                Message::ChooseTime => {
                    state.show_picker = true;
                }
                Message::SubmitTime(time) => {
                    state.time = time;
                    state.show_picker = false;
                }
                Message::CancelTime => {
                    state.show_picker = false;
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            TimePickerExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            TimePickerExample::Loaded(state) => {
                let but = Button::new(Text::new("Set Time")).on_press(Message::ChooseTime);

                let timepicker = TimePicker::new(
                    state.show_picker,
                    state.time,
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
                    .push(Text::new(format!("Time: {}", state.time)));

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
