use iced::{
    widget::{Container, Text},
    Application, Command, Element, Length, Settings, Theme,
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

impl Application for SplitPaneExample {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: Self::Flags) -> (SplitPaneExample, Command<Message>) {
        (
            SplitPaneExample {
                divider_position: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Split example")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OnResize(position) => self.divider_position = Some(position),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
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
