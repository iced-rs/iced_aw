use iced::{Align, Column, Container, Element, Length, Row, Sandbox, Settings, Text};

use iced_aw::{Badge, style::{self, badge::StyleSheet}};

const BADGE_TEXT_SIZE: u16 = 15;

fn main() -> iced::Result {
    BadgeExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {

}

struct BadgeExample {
    messages: Vec<(String, usize)>,
}

impl Sandbox for BadgeExample {
    
    type Message = Message;

    fn new() -> Self {
        BadgeExample {
            messages: vec!(
                ("Charlotte-Jayne Gilpin".to_string(), 20),
                ("Keanu Reeves".to_string(), 42),
                ("Stephen Hawking".to_string(), 21),
            )
        }
    }

    fn title(&self) -> String {
        String::from("Badge example")
    }

    fn update(&mut self, _message: Message) {
        
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Messages").size(32))
            .spacing(15)
            .max_width(300);
        
        let content = self.messages.iter()
            .enumerate()
            .fold(
                content,
                |col, (i, (name, count))| {
                    col.push(
                        Row::new()
                            .align_items(Align::Center)
                            .push(
                                Text::new(name)
                                    .width(Length::Fill)
                            )
                            .push(
                                Badge::new(
                                    Text::new(format!("{}", count))
                                        .color(style::colors::SNOW)
                                        .size(BADGE_TEXT_SIZE)
                                )
                                .style(predefined_style(i))
                            )
                    )
                }
            );
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

/// TODO: maybe there is a smarter way
fn predefined_style(index: usize) -> Box<dyn StyleSheet> {
    match index {
        0 => style::badge::Primary.into(),
        1 => style::badge::Secondary.into(),
        2 => style::badge::Success.into(),
        3 => style::badge::Danger.into(),
        4 => style::badge::Warning.into(),
        5 => style::badge::Info.into(),
        6 => style::badge::Light.into(),
        7 => style::badge::Dark.into(),
        _ => style::badge::Default.into(),
    }
}