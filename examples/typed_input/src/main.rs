use iced::{
    widget::{Container, Row, Text},
    Alignment, Element, Length,
};
use iced_aw::widgets::typed_input;

#[derive(Default, Debug)]
pub struct TypedInputDemo {
    value: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    TypedInpChanged(f32),
}

fn main() -> iced::Result {
    iced::application(
        "Typed Input example",
        TypedInputDemo::update,
        TypedInputDemo::view,
    )
    .window_size(iced::Size {
        width: 250.0,
        height: 200.0,
    })
    .font(iced_aw::BOOTSTRAP_FONT_BYTES)
    .run()
}

impl TypedInputDemo {
    fn update(&mut self, message: self::Message) {
        let Message::TypedInpChanged(val) = message;
        println!("Value changed to {:?}", val);
        self.value = val;
    }

    fn view(&self) -> Element<Message> {
        let lb_minute = Text::new("Typed Input:");
        let txt_minute =
            typed_input::TypedInput::new("Placeholder", &self.value, Message::TypedInpChanged);

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
