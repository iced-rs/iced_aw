use std::fmt::Display;

use iced::{
    alignment, font,
    widget::{container, text, Button, Column, Container, PickList, Row, Text},
    Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::{NumberInput, Wrap};
use rand::Rng;

fn main() -> iced::Result {
    RandStrings::run(Settings::default())
}

enum RandStrings {
    Loading,
    Loaded(State),
}

struct State {
    vbuttons: Vec<StrButton>,
    hbuttons: Vec<StrButton>,
    spacing: f32,
    line_spacing: f32,
    line_minimal_length: f32,
    align: iced::Alignment,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WrapAlign {
    Start,
    Center,
    End,
}
impl From<iced::Alignment> for WrapAlign {
    fn from(align: iced::Alignment) -> Self {
        match align {
            iced::Alignment::Start => Self::Start,
            iced::Alignment::Center => Self::Center,
            iced::Alignment::End => Self::End,
        }
    }
}
impl From<WrapAlign> for iced::Alignment {
    fn from(wrap_align: WrapAlign) -> Self {
        match wrap_align {
            WrapAlign::Start => Self::Start,
            WrapAlign::Center => Self::Center,
            WrapAlign::End => Self::End,
        }
    }
}
impl Display for WrapAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WrapAlign::Start => f.write_str("start"),
            WrapAlign::Center => f.write_str("center"),
            WrapAlign::End => f.write_str("end"),
        }
    }
}
#[derive(Debug, Clone)]
struct StrButton {
    str: String,
    size: f32,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
enum Message {
    ChangeAlign(WrapAlign),
    ChangeSpacing(f32),
    ChangeLineSpacing(f32),
    ChangeMinimalLength(f32),
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for RandStrings {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        "wrap".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            Self::Loading => {
                if let Message::Loaded(_) = message {
                    *self = Self::Loaded({
                        let mut rng = rand::thread_rng();
                        let data: Vec<StrButton> = (0..45)
                            .map(|s| StrButton {
                                str: s.to_string(),
                                size: rng.gen_range(15.0..50.0),
                            })
                            .collect();
                        State {
                            vbuttons: data.clone(),
                            hbuttons: data,
                            align: iced::Alignment::Start,
                            spacing: 0.0,
                            line_spacing: 0.0,
                            line_minimal_length: 10.0,
                        }
                    })
                }
            }
            Self::Loaded(state) => match message {
                Message::ChangeAlign(align) => {
                    state.align = align.into();
                }
                Message::ChangeSpacing(num) => {
                    state.spacing = num;
                }
                Message::ChangeLineSpacing(num) => {
                    state.line_spacing = num;
                }
                Message::ChangeMinimalLength(num) => {
                    state.line_minimal_length = num;
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            Self::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            Self::Loaded(state) => {
                let vertical = Container::new(
                    state
                        .vbuttons
                        .iter()
                        .fold(Wrap::new_vertical(), |wrap, button| {
                            let StrButton { str, .. } = button;
                            wrap.push(Button::new(Text::new(str.as_str()).size(button.size)))
                        })
                        .align_items(state.align)
                        .spacing(state.spacing)
                        .line_spacing(state.line_spacing)
                        .line_minimal_length(state.line_minimal_length),
                )
                .width(iced::Length::FillPortion(5));
                let horizontal = Container::new(
                    state
                        .hbuttons
                        .iter()
                        .fold(Wrap::new(), |wrap, button| {
                            let StrButton { str, .. } = button;
                            wrap.push(Button::new(Text::new(str.as_str()).size(button.size)))
                        })
                        .align_items(state.align)
                        .spacing(state.spacing)
                        .line_spacing(state.line_spacing)
                        .line_minimal_length(state.line_minimal_length),
                )
                .width(iced::Length::FillPortion(5));
                let align_picklist = PickList::new(
                    vec![WrapAlign::Start, WrapAlign::Center, WrapAlign::End],
                    Some(Into::<WrapAlign>::into(state.align)),
                    Message::ChangeAlign,
                );
                let spacing_input =
                    Column::new()
                        .push(Text::new("spacing"))
                        .push(NumberInput::new(
                            state.spacing,
                            0.0..500.0,
                            Message::ChangeSpacing,
                        ));
                let line_spacing_input =
                    Column::new()
                        .push(Text::new("line spacing"))
                        .push(NumberInput::new(
                            state.line_spacing,
                            0.0..500.0,
                            Message::ChangeLineSpacing,
                        ));
                let line_minimal_length_input = Column::new()
                    .push(Text::new("line minimal length"))
                    .push(NumberInput::new(
                        state.line_minimal_length,
                        0.0..999.9,
                        Message::ChangeMinimalLength,
                    ));
                let ctrls = Column::new()
                    .push(align_picklist)
                    .push(spacing_input)
                    .push(line_spacing_input)
                    .push(line_minimal_length_input)
                    .height(iced::Length::Shrink)
                    .align_items(iced::Alignment::Center);

                Row::new()
                    .push(ctrls)
                    .push(vertical)
                    .push(horizontal)
                    .into()
            }
        }
    }
}
