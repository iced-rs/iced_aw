use iced_aw::style::input_style;
use iced::{text_input, Sandbox, Settings, Element, TextInput, Length, Container};

fn main() -> iced::Result {
    InputExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    HandlerChangeValue(String),
}

struct InputExample {
    input_state: text_input::State,
    value: String,
}

impl Sandbox for InputExample {
    type Message = Message;

    fn new() -> Self {
        InputExample {
            input_state: text_input::State::new(),
            value: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Input_Style example")
    }

    fn update(&mut self, message: self::Message) {
        match message {
            Message::HandlerChangeValue(val) => {
                self.value = val;
            }
        }
    }

    fn view(&mut self) -> Element<'_, self::Message> {
        let content = TextInput::new(&mut self.input_state,
                                     "default style", &self.value,
                                     Message::HandlerChangeValue)
            .width(Length::Units(180)).padding(5).style(input_style::TextInput);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}
