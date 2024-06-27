use iced::{
    widget::{Column, Container, Row, Text},
    Alignment, Element, Length,
};

#[derive(Debug)]
pub struct NumberInputDemo {
    value: [NumInput<f32, Message>; 2],
}

impl Default for NumberInputDemo {
    fn default() -> Self {
        Self {
            value: [NumInput::new(32.0), NumInput::new(12.0)],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    GenericF32Input((usize, NumInputMessage<f32>)),
}

fn main() -> iced::Result {
    iced::application(
        "NumberInput example",
        NumberInputDemo::update,
        NumberInputDemo::view,
    )
    .font(iced_aw::BOOTSTRAP_FONT_BYTES)
    .run()
}

mod numberinput;
use numberinput::*;

impl NumberInputDemo {
    fn update(&mut self, message: self::Message) {
        let Message::GenericF32Input((id, val)) = message;
        self.value[id].value = val.get_data();
    }

    fn view(&self) -> Element<Message> {
        let mut column1 = Column::new();

        for (id, val) in self.value.iter().enumerate() {
            let lb_minute = Text::new(format!("Number Input {}:", id));
            let txt_minute = val.view(id, 1.0, 255.0, 0.5, Message::GenericF32Input, None);

            column1 = column1.push(
                Row::new()
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(lb_minute)
                    .push(txt_minute),
            );
        }

        Container::new(column1)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
