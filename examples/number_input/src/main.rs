use iced::{
    widget::{Container, Row, Text},
    window, Alignment, Background, Color, Element, Length, Sandbox, Settings, Theme,
};
use iced_aw::number_input::NumberInput;

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
            .step(0.5)
            .min(1.0)
            .input_style(style::CustomTextInput)
            .style(style::CustomNumInput);

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

mod style {
    use iced::{widget::text_input, Background, Color, Theme};
    use iced_aw::number_input;

    const BACKGROUND: Color = Color::from_rgb(238.0 / 255.0, 238.0 / 255.0, 238.0 / 255.0);
    const FOREGROUND: Color = Color::from_rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0);
    const HOVERED: Color = Color::from_rgb(129.0 / 255.0, 129.0 / 255.0, 129.0 / 255.0);
    const PRIMARY: Color = Color::from_rgb(12.0 / 255.0, 46.0 / 251.0, 179.0 / 255.0);

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CustomNumInput;
    impl number_input::StyleSheet for CustomNumInput {
        type Style = CustomNumInput;

        fn active(&self, style: Self::Style) -> number_input::Appearance {
            number_input::Appearance {
                icon_color: PRIMARY,
                ..number_input::Appearance::default()
            }
        }
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CustomTextInput;
    impl text_input::StyleSheet for CustomTextInput {
        type Style = ();
        fn active(&self, style: Self::Style) -> text_input::Appearance {
            text_input::Appearance {
                background: BACKGROUND.into(),
                border_color: PRIMARY,
                border_width: 1.0,
                border_radius: 5.5,
            }
        }

        fn focused(&self, style: Self::Style) -> text_input::Appearance {
            let active = self.active(style);

            text_input::Appearance {
                background: FOREGROUND.into(),
                ..active
            }
        }

        fn placeholder_color(&self, style: Self::Style) -> Color {
            HOVERED
        }

        fn selection_color(&self, style: Self::Style) -> Color {
            HOVERED
        }

        fn value_color(&self, style: Self::Style) -> Color {
            Color::BLACK
        }
    }
}
