// This example demonstrates how to use the `color_picker` widget
//
// It was written by Kaiden42 <gitlab@tinysn.com>

use iced::{
    Alignment, Color, Element, Length,
    widget::{Button, Container, Row, Text},
};

use iced_aw::ICED_AW_FONT_BYTES;
use iced_aw::helpers::color_picker;
fn main() -> iced::Result {
    iced::application(
        ColorPickerExample::default,
        ColorPickerExample::update,
        ColorPickerExample::view,
    )
    .font(ICED_AW_FONT_BYTES)
    .subscription(ColorPickerExample::subscription)
    .run()
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
    Tick,
}

#[derive(Debug)]
struct ColorPickerExample {
    color: Color,
    show_picker: bool,
}

impl Default for ColorPickerExample {
    fn default() -> Self {
        Self {
            color: Color::from_rgba8(0, 0, 0, 1.0),
            show_picker: false,
        }
    }
}
impl ColorPickerExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChooseColor => {
                self.show_picker = true;
            }
            Message::SubmitColor(color) => {
                self.color = color;
                self.show_picker = false;
            }
            Message::CancelColor => {
                self.show_picker = false;
            }
            Message::Tick => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let but = Button::new(Text::new("Set Color")).on_press(Message::ChooseColor);

        let color_picker = color_picker(
            self.show_picker,
            self.color,
            but,
            Message::CancelColor,
            Message::SubmitColor,
        );

        let row = Row::new()
            .align_y(Alignment::Center)
            .spacing(10)
            .push(color_picker)
            .push(Text::new(format!("Color: {:?}", self.color)));

        Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        // We're running this subscription in background to see if any unrelated
        // rerenders can affect the implementation negatively.
        // You do not need a background subscription to use color picker, this
        // is simply test code.
        iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
