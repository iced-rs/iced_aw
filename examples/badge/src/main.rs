use iced::{
    widget::{Column, Container, Row, Text},
    Alignment, Element, Length, Sandbox, Settings,
};

use iced_aw::{style::badge, Badge};

const BADGE_TEXT_SIZE: u16 = 15;

fn main() -> iced::Result {
    BadgeExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {}

struct BadgeExample {
    messages: Vec<(String, usize)>,
}

impl Sandbox for BadgeExample {
    type Message = Message;

    fn new() -> Self {
        BadgeExample {
            messages: vec![
                ("Charlotte-Jayne Gilpin".to_string(), 20),
                ("Keanu Reeves".to_string(), 42),
                ("Stephen Hawking".to_string(), 21),
            ],
        }
    }

    fn title(&self) -> String {
        String::from("Badge example")
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Messages").size(32))
            .spacing(15)
            .max_width(300);

        let content_messages =
            self.messages
                .iter()
                .enumerate()
                .fold(content, |col, (i, (name, count))| {
                    col.push(
                        Row::new()
                            .align_items(Alignment::Center)
                            .push(Text::new(name).width(Length::Fill))
                            .push(
                                Badge::new(Text::new(format!("{}", count)).size(BADGE_TEXT_SIZE))
                                    .style(predefined_style(i)),
                            ),
                    )
                });

        let content_all = Column::new()
            .spacing(10)
            .push(Text::new("All available badge styles:").size(32))
            .push(
                Row::new()
                    .spacing(10)
                    .push(Badge::new(Text::new("Default")))
                    .push(Badge::new(Text::new("Primary")).style(badge::Badge::Primary))
                    .push(Badge::new(Text::new("Secondary")).style(badge::Badge::Secondary))
                    .push(Badge::new(Text::new("Success")).style(badge::Badge::Success))
                    .push(Badge::new(Text::new("Danger")).style(badge::Badge::Danger)),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .push(Badge::new(Text::new("Warning")).style(badge::Badge::Warning))
                    .push(Badge::new(Text::new("Info")).style(badge::Badge::Info))
                    .push(Badge::new(Text::new("Light")).style(badge::Badge::Light))
                    .push(Badge::new(Text::new("Dark")).style(badge::Badge::Dark))
                    .push(Badge::new(Text::new("White")).style(badge::Badge::White)),
            );

        Container::new(
            Column::new()
                .spacing(40)
                .push(content_messages)
                .push(content_all),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn predefined_style(index: usize) -> badge::Badge {
    match index {
        0 => badge::Badge::Primary,
        1 => badge::Badge::Secondary,
        2 => badge::Badge::Success,
        3 => badge::Badge::Danger,
        4 => badge::Badge::Warning,
        5 => badge::Badge::Info,
        6 => badge::Badge::Light,
        7 => badge::Badge::Dark,
        _ => badge::Badge::Default,
    }
}
