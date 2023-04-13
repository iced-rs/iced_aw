use iced::{
    widget::{Button, Container, Row, Text},
    Alignment, Color, Element, Length, Sandbox, Settings,
};

use iced_aw::ColorPicker;

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
    show_picker: bool,
}

impl Sandbox for ColorPickerExample {
    type Message = Message;

    fn new() -> Self {
        ColorPickerExample {
            color: Color::from_rgba(0.5, 0.2, 0.7, 1.0),
            show_picker: false,
        }
    }

    fn title(&self) -> String {
        String::from("ColorPicker example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChooseColor => {
                //self.state.reset();
                //self.state.show(true);
                self.show_picker = true;
            }
            Message::SubmitColor(color) => {
                self.color = color;
                //self.state.show(false);
                self.show_picker = false;
            }
            Message::CancelColor => {
                //self.state.show(false);
                self.show_picker = false;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let but = Button::new(Text::new("Set Color")).on_press(Message::ChooseColor);

        let datepicker = ColorPicker::new(
            self.show_picker,
            self.color,
            but,
            Message::CancelColor,
            Message::SubmitColor,
        );

        let row = Row::new()
            .align_items(Alignment::Center)
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
