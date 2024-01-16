use iced::{
    alignment, font,
    widget::{container, text, Button, Container, Row, Text},
    Alignment, Application, Color, Command, Element, Length, Settings, Theme,
};

use iced_aw::helpers::color_picker;

fn main() -> iced::Result {
    ColorPickerExample::run(Settings::default())
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
enum ColorPickerExample {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct State {
    color: Color,
    show_picker: bool,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for ColorPickerExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ColorPickerExample, Command<Message>) {
        (
            ColorPickerExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::graphics::icons::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("ColorPicker example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            ColorPickerExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = ColorPickerExample::Loaded(State {
                        color: Color::from_rgba(0.5, 0.2, 0.7, 1.0),
                        show_picker: false,
                    })
                }
            }
            ColorPickerExample::Loaded(state) => match message {
                Message::ChooseColor => {
                    state.show_picker = true;
                }
                Message::SubmitColor(color) => {
                    state.color = color;
                    state.show_picker = false;
                }
                Message::CancelColor => {
                    state.show_picker = false;
                }
                _ => {}
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            ColorPickerExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            ColorPickerExample::Loaded(state) => {
                let but = Button::new(Text::new("Set Color")).on_press(Message::ChooseColor);

                let color_picker = color_picker(
                    state.show_picker,
                    state.color,
                    but,
                    Message::CancelColor,
                    Message::SubmitColor,
                );

                let row = Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(color_picker)
                    .push(Text::new(format!("Color: {:?}", state.color)));

                Container::new(row)
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Fill).center_x().center_y()
                    .into()
            }
        }
    }
}
