use std::fmt::Display;

use iced::{
    button::{self, Button},
    Column, Container, Row, Sandbox, Settings, Text,
};
use iced_aw::{number_input, NumberInput, Wrap};
use rand::Rng;

fn main() -> iced::Result {
    RandStrings::run(Settings::default())
}

struct RandStrings {
    vbuttons: Vec<StrButton>,
    hbuttons: Vec<StrButton>,
    change_align_picklist: iced::pick_list::State<WrapAlign>,
    spacing: u16,
    line_spacing: u16,
    line_minimal_length: u32,
    spacing_input: number_input::State,
    line_spacing_input: number_input::State,
    line_minimal_length_input: number_input::State,
    align: iced::Align,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WrapAlign {
    Start,
    Center,
    End,
}
impl From<iced::Align> for WrapAlign {
    fn from(align: iced::Align) -> Self {
        match align {
            iced::Align::Start => Self::Start,
            iced::Align::Center => Self::Center,
            iced::Align::End => Self::End,
        }
    }
}
impl From<WrapAlign> for iced::Align {
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
    state: button::State,
    str: String,
    size: u16,
}
#[derive(Debug, Clone, Copy)]
enum Message {
    ChangeAlign(WrapAlign),
    ChangeSpacing(u16),
    ChangeLineSpacing(u16),
    ChangeMinimalLength(u32),
}

impl iced::Sandbox for RandStrings {
    type Message = Message;

    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let data: Vec<StrButton> = (0..45)
            .into_iter()
            .map(|s| StrButton {
                state: button::State::new(),
                str: s.to_string(),
                size: rng.gen_range(15..50),
            })
            .collect();
        Self {
            vbuttons: data.clone(),
            hbuttons: data,
            align: iced::Align::Start,
            change_align_picklist: iced::pick_list::State::default(),
            spacing: 0,
            line_spacing: 0,
            line_minimal_length: 10,
            spacing_input: number_input::State::new(),
            line_spacing_input: number_input::State::new(),
            line_minimal_length_input: number_input::State::new(),
        }
    }

    fn title(&self) -> String {
        "wrap".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChangeAlign(align) => {
                self.align = align.into();
            }
            Message::ChangeSpacing(num) => {
                self.spacing = num;
            }
            Message::ChangeLineSpacing(num) => {
                self.line_spacing = num;
            }
            Message::ChangeMinimalLength(num) => {
                self.line_minimal_length = num;
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let RandStrings {
            vbuttons,
            hbuttons,
            change_align_picklist,
            spacing_input,
            line_minimal_length_input,
            line_spacing_input,
            ..
        } = self;
        let vertcal = Container::new(
            vbuttons
                .iter_mut()
                .fold(Wrap::new_vertical(), |wrap, button| {
                    let StrButton { state, str, .. } = button;
                    wrap.push(Button::new(
                        state,
                        Text::new(str.as_str()).size(button.size),
                    ))
                })
                .align_items(self.align)
                .spacing(self.spacing)
                .line_spacing(self.line_spacing)
                .line_minimal_length(self.line_minimal_length),
        )
        .width(iced::Length::FillPortion(5));
        let horizontal = Container::new(
            hbuttons
                .iter_mut()
                .fold(Wrap::new(), |wrap, button| {
                    let StrButton { state, str, .. } = button;
                    wrap.push(Button::new(
                        state,
                        Text::new(str.as_str()).size(button.size),
                    ))
                })
                .align_items(self.align)
                .spacing(self.spacing)
                .line_spacing(self.line_spacing)
                .line_minimal_length(self.line_minimal_length),
        )
        .width(iced::Length::FillPortion(5));
        let align_picklist = iced::PickList::new(
            change_align_picklist,
            vec![WrapAlign::Start, WrapAlign::Center, WrapAlign::End],
            Some(self.align.into()),
            Message::ChangeAlign,
        );
        let spacing_input = Column::new()
            .push(Text::new("spacing"))
            .push(NumberInput::new(
                spacing_input,
                self.spacing,
                500,
                Message::ChangeSpacing,
            ));
        let line_spacing_input =
            Column::new()
                .push(Text::new("line spacing"))
                .push(NumberInput::new(
                    line_spacing_input,
                    self.line_spacing,
                    500,
                    Message::ChangeLineSpacing,
                ));
        let line_minimal_length_input =
            Column::new()
                .push(Text::new("line minimal length"))
                .push(NumberInput::new(
                    line_minimal_length_input,
                    self.line_minimal_length,
                    999,
                    Message::ChangeMinimalLength,
                ));
        let ctrls = Column::new()
            .push(align_picklist)
            .push(spacing_input)
            .push(line_spacing_input)
            .push(line_minimal_length_input)
            .height(iced::Length::Shrink)
            .align_items(iced::Align::Center);

        Row::new().push(ctrls).push(vertcal).push(horizontal).into()
    }
}
