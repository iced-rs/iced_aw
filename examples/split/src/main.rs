use iced::{
    widget::{Container, Text},
    Element, Length, Sandbox, Settings,
};
use iced_aw::{split, Split};

fn main() -> iced::Result {
    SplitPaneExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    OnResize(u16),
}

struct SplitPaneExample {
    divider_position: Option<u16>,
}

impl Sandbox for SplitPaneExample {
    type Message = Message;

    fn new() -> Self {
        SplitPaneExample {
            divider_position: None,
        }
    }

    fn title(&self) -> String {
        String::from("Split example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OnResize(position) => self.divider_position = Some(position),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let first = Container::new(Text::new("First"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let second = Container::new(Text::new("Second"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        Split::new(
            first,
            second,
            self.divider_position,
            split::Axis::Vertical,
            Message::OnResize,
        )
        .into()
    }
}
