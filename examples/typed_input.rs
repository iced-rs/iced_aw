// This example demonstrates how to use the typed_input widget
//
// This was written by Ultraxime <36888699+Ultraxime@users.noreply.github.com>

use iced::{
    Alignment, Element, Length,
    widget::{Container, Row, Text},
};
use iced_aw::ICED_AW_FONT_BYTES;
use iced_aw::widgets::typed_input;

#[derive(Default, Debug)]
pub struct TypedInputDemo {
    value: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    TypedInpChanged(f32),
    TypedInpSubmit(Result<f32, String>),
}

fn main() -> iced::Result {
    iced::application(
        TypedInputDemo::default,
        TypedInputDemo::update,
        TypedInputDemo::view,
    )
    .window_size(iced::Size {
        width: 250.0,
        height: 200.0,
    })
    .font(ICED_AW_FONT_BYTES)
    .run()
}

impl TypedInputDemo {
    fn update(&mut self, message: self::Message) {
        match message {
            Message::TypedInpChanged(value) => {
                println!("Value changed to {}", value);
                self.value = value;
            }
            Message::TypedInpSubmit(Ok(value)) => println!("Value submitted: {}", value),
            Message::TypedInpSubmit(Err(text)) => {
                println!("Value submitted while invalid: {}", text)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let lb_minute = Text::new("Typed Input:");
        let txt_minute = typed_input::TypedInput::new("Placeholder", &self.value)
            .on_input(Message::TypedInpChanged)
            .on_submit(Message::TypedInpSubmit);

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
