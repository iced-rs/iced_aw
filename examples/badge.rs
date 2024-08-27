// This example demonstrates how to use the `badge` widget.
//
// It was written by Kaiden42 <gitlab@tinysn.com>

use iced::{
    widget::{Column, Container, Row, Text},
    Alignment, Element, Length,
};

use iced_aw::{helpers::badge, style, style::status::Status};

const BADGE_TEXT_SIZE: u16 = 15;

fn main() -> iced::Result {
    iced::application("Badge example", BadgeExample::update, BadgeExample::view).run()
}

#[derive(Debug, Clone)]
enum Message {}

struct BadgeExample {
    messages: Vec<(String, usize)>,
}

impl Default for BadgeExample {
    fn default() -> Self {
        Self {
            messages: vec![
                ("Charlotte-Jayne Gilpin".to_string(), 20),
                ("Keanu Reeves".to_string(), 42),
                ("Stephen Hawking".to_string(), 21),
            ],
        }
    }
}

impl BadgeExample {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Messages").size(32))
            .spacing(10)
            .max_width(300);

        let content_messages =
            self.messages
                .iter()
                .enumerate()
                .fold(content, |col, (i, (name, count))| {
                    col.push(
                        Row::new()
                            .align_y(Alignment::Center)
                            .push(Text::new(name).width(Length::Fill))
                            .push(
                                badge(Text::new(format!("{count}")).size(BADGE_TEXT_SIZE))
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
                    .push(badge(Text::new("Default")))
                    .push(badge(Text::new("Primary")).style(style::badge::primary))
                    .push(badge(Text::new("Secondary")).style(style::badge::secondary))
                    .push(badge(Text::new("Success")).style(style::badge::success))
                    .push(badge(Text::new("Danger")).style(style::badge::danger)),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .push(badge(Text::new("Warning")).style(style::badge::warning))
                    .push(badge(Text::new("Info")).style(style::badge::info))
                    .push(badge(Text::new("Light")).style(style::badge::light))
                    .push(badge(Text::new("Dark")).style(style::badge::dark))
                    .push(badge(Text::new("White")).style(style::badge::white)),
            );

        Container::new(
            Column::new()
                .spacing(10)
                .align_x(Alignment::Center)
                .push(content_messages)
                .push(content_all),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn predefined_style(index: usize) -> impl Fn(&iced::Theme, Status) -> style::badge::Style {
    match index {
        0 => style::badge::primary,
        1 => style::badge::secondary,
        2 => style::badge::success,
        3 => style::badge::danger,
        4 => style::badge::warning,
        5 => style::badge::info,
        6 => style::badge::light,
        7 => style::badge::dark,
        _ => style::badge::primary,
    }
}
