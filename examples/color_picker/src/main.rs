use iced::{
    button, Align, Button, Color, Container, Element, Length, Row, Sandbox, Settings, Text,
};

use iced_aw::color_picker::{self, ColorPicker};

fn main() -> iced::Result {
    ColorPickerExample::run(Settings::default())
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
}

struct ColorPickerExample {
    color: Color,
    state: color_picker::State,
    button_state: button::State,
}

impl Sandbox for ColorPickerExample {
    type Message = Message;

    fn new() -> Self {
        ColorPickerExample {
            color: Color::default(),
            state: color_picker::State::new(),
            button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("ColorPicker example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChooseColor => {
                //self.state.reset();
                self.state.show(true);
            }
            Message::SubmitColor(color) => {
                self.color = color;
                self.state.show(false);
            }
            Message::CancelColor => {
                self.state.show(false);
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let but = Button::new(&mut self.button_state, Text::new("Set Color"))
            .on_press(Message::ChooseColor);

        let datepicker = ColorPicker::new(
            &mut self.state,
            but,
            Message::CancelColor,
            Message::SubmitColor,
        );

        let row = Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(datepicker)
            .push(Text::new(format!("Color: {:?}", self.color)));

        Container::new(row)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
