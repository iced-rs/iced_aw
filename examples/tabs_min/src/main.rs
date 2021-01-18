use iced::{Column, Element, Sandbox, Settings, Text};
use iced_aw::{Tabs, TabLabel};

fn main() -> iced::Result {
    TabBarExample::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TabSelected(usize),
}

struct TabBarExample {
    active_tab: usize,
}

impl Sandbox for TabBarExample {

    type Message = Message;

    fn new() -> Self {
        TabBarExample {
            active_tab: 0,
        }
    }

    fn title(&self) -> String {
        String::from("TabBar example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(index) => {
                self.active_tab = index
            },
        }
    }
    
    fn view(&mut self) -> Element<Message> {
        Tabs::new(self.active_tab, Message::TabSelected)
            // TODO: Fix error
            .push(
                TabLabel::Text(String::from("One")),
                Text::new("This is tab one")
            )
            .push(
                TabLabel::Text(String::from("Two")),
                Text::new("This is tab two")
            )
            .push(
                TabLabel::Text(String::from("Three")),
                Text::new("This is tab three")
            )
            .into()
    }
}