use iced::{
    pure::{
        widget::{Column, Container, Row, Text},
        Element, Sandbox,
    },
    Alignment, Length, Settings,
};

use iced_aw::{
    pure::Badge,
    style::{self, badge::StyleSheet},
};

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
                                Badge::new(
                                    Text::new(format!("{}", count))
                                        .color(style::colors::SNOW)
                                        .size(BADGE_TEXT_SIZE),
                                )
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
                    .push(Badge::new(Text::new("Default")).style(style::badge::Default))
                    .push(
                        Badge::new(Text::new("Primary").color(style::colors::LIGHT))
                            .style(style::badge::Primary),
                    )
                    .push(
                        Badge::new(Text::new("Secondary").color(style::colors::LIGHT))
                            .style(style::badge::Secondary),
                    )
                    .push(
                        Badge::new(Text::new("Success").color(style::colors::LIGHT))
                            .style(style::badge::Success),
                    )
                    .push(
                        Badge::new(Text::new("Danger").color(style::colors::LIGHT))
                            .style(style::badge::Danger),
                    ),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .push(Badge::new(Text::new("Warning")).style(style::badge::Warning))
                    .push(Badge::new(Text::new("Info")).style(style::badge::Info))
                    .push(Badge::new(Text::new("Light")).style(style::badge::Light))
                    .push(
                        Badge::new(Text::new("Dark").color(style::colors::LIGHT))
                            .style(style::badge::Dark),
                    )
                    .push(Badge::new(Text::new("White")).style(style::badge::White)),
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
