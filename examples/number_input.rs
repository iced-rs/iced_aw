// This example demonstrates how to use the number input widget
//
// It was written by leang27 <52003343+leang27@users.noreply.github.com>

use iced::{
    widget::{Container, Row, Text},
    Alignment, Element, Length,
};
use iced_aw::number_input;

#[derive(Default, Debug)]
pub struct NumberInputDemo {
    value: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    NumInpChanged(f32),
}

fn main() -> iced::Result {
    iced::application(
        "Number Input example",
        NumberInputDemo::update,
        NumberInputDemo::view,
    )
    .window_size(iced::Size {
        width: 250.0,
        height: 200.0,
    })
    .font(iced_fonts::REQUIRED_FONT_BYTES)
    .run()
}

impl NumberInputDemo {
    fn update(&mut self, message: self::Message) {
        let Message::NumInpChanged(val) = message;
        println!("Value changed to {:?}", val);
        self.value = val;
    }

    fn view(&self) -> Element<Message> {
        let lb_minute = Text::new("Number Input:");
        let txt_minute = number_input(self.value, -10.0..250.0, Message::NumInpChanged)
            .style(number_input::number_input::primary)
            .step(0.5);

        Container::new(
            Row::new()
                .spacing(10)
                .align_y(Alignment::Center)
                .push(lb_minute)
                .push(txt_minute),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}
