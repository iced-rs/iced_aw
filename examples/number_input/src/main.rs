use iced::{
    widget::{Container, Row, Text},
    window, Alignment, Element, Length, Sandbox, Settings,
};
use iced_aw::{number_input::NumberInput, style::NumberInputStyles};

#[derive(Default)]
pub struct NumberInputDemo {
    value: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    NumInpChanged(f32),
}

fn main() -> iced::Result {
    NumberInputDemo::run(Settings {
        default_text_size: 14,
        window: window::Settings {
            size: (250, 200),
            ..Default::default()
        },
        ..Settings::default()
    })
}

impl Sandbox for NumberInputDemo {
    type Message = Message;

    fn new() -> Self {
        Self { value: 27.0 }
    }

    fn title(&self) -> String {
        String::from("Number Input Demo")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NumInpChanged(val) => {
                self.value = val;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let lb_minute = Text::new("Number Input:");
        let txt_minute = NumberInput::new(self.value, 255.0, Message::NumInpChanged)
            .style(NumberInputStyles::Default)
            .step(0.5)
            .min(1.0);

        Container::new(
            Row::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(lb_minute)
                .push(txt_minute),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
