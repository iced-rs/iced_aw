use crate::Section;
use iced::{button, Align, Button, Color, Column, Element, Length, Row, Text};
use iced_aw::{color_picker, ColorPicker};

pub struct ColorPickerSection {
    color_picker_state: color_picker::State,
    button_state: button::State,
    color: Color,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenColorPicker,
    CancelColor,
    SubmitColor(Color),
}

impl Section for ColorPickerSection {
    type Message = Message;

    fn new() -> Self {
        Self {
            color_picker_state: color_picker::State::new(),
            button_state: button::State::new(),
            color: Color::default(),
        }
    }

    fn header(&self) -> String {
        String::from("Color Picker")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenColorPicker => self.color_picker_state.show(true),
            Message::CancelColor => self.color_picker_state.show(false),
            Message::SubmitColor(color) => {
                self.color = color;
                self.color_picker_state.show(false);
            }
        }
    }

    fn content(&mut self) -> Element<'_, Self::Message> {
        let color_picker = ColorPicker::new(
            &mut self.color_picker_state,
            Button::new(&mut self.button_state, Text::new("Pick Color"))
                .on_press(Message::OpenColorPicker),
            Message::CancelColor,
            Message::SubmitColor,
        );

        let column = Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .align_items(Align::Center)
                    .spacing(20)
                    .push(color_picker)
                    .push(Text::new(format!("Picked color: {:?}", self.color))),
            );

        column.into()
    }
}
