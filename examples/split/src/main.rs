use iced::{Container, Length, Sandbox, Settings, Text};
use iced_aw::{split, Split};

fn main() -> iced::Result {
    SplitPaneExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    OnResize(u16),
}

struct SplitPaneExample {
    split_pane: split::State,
}

impl Sandbox for SplitPaneExample {
    type Message = Message;

    fn new() -> Self {
        SplitPaneExample {
            split_pane: split::State::new(None, split::Axis::Vertical),
        }
    }

    fn title(&self) -> String {
        String::from("SplitPane example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OnResize(position) => self.split_pane.set_divider_position(position),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let first = Container::new(
            Text::new("First"),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        let second = Container::new(
            Text::new("Second")
        ).width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        Split::new(&mut self.split_pane, first, second, Message::OnResize).into()
    }
}
