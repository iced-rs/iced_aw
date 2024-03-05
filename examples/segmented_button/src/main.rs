use iced::widget::container;
use iced::widget::{column, row, text};
use iced::{Element, Length, Sandbox, Settings};

use iced_aw::widgets::segmented_button;
use segmented_button::SegmentedButton;

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    selected_radio: Option<Choice>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadioSelected(Choice),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self {
            selected_radio: Some(Choice::A),
        }
    }

    fn title(&self) -> String {
        String::from("Radio - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadioSelected(value) => {
                self.selected_radio = Some(value);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // let selected_radio = Some(Choice::A);

        // i added a row just to demonstrate that anything can be used as a child,
        //in this case instead of A B C you might add icons
        let a = SegmentedButton::new(
            row!(text("HEAVY  "), "A"),
            Choice::A,
            self.selected_radio,
            Message::RadioSelected,
        );

        let b = SegmentedButton::new(
            row!(text("MEDIUM  "), "B"),
            Choice::B,
            self.selected_radio,
            Message::RadioSelected,
        );

        let c = SegmentedButton::new(
            row!(text("LIGHT  "), "C"),
            Choice::C,
            self.selected_radio,
            Message::RadioSelected,
        );
        let content = column![
            row![a, b, c],
            text(self.selected_radio.unwrap().to_string())
        ]
        .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Choice {
    #[default]
    A,
    B,
    C,
}

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Choice::A => "A",
                Choice::B => "B",
                Choice::C => "C",
            }
        )
    }
}
