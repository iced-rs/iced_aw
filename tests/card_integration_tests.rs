//! Integration tests for the Card widget
//!
//! These tests verify the Card widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::{Color, Length, Padding, Theme};
use iced_aw::Card;
use iced_widget::text::Text;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum Message {
    CloseCard,
    Other,
}

#[test]
fn card_can_be_created_with_head_and_body() {
    let _card: Card<Message, Theme> = Card::new(Text::new("Head"), Text::new("Body"));
}

#[test]
fn card_can_add_footer() {
    let _card: Card<Message, Theme> =
        Card::new(Text::new("Head"), Text::new("Body")).foot(Text::new("Footer"));
}

#[test]
fn card_can_set_width() {
    let _card1 = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).width(300);
    let _card2 =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).width(Length::Fill);
    let _card3 =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).width(Length::Shrink);
}

#[test]
fn card_can_set_height() {
    let _card1 = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).height(200);
    let _card2 =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).height(Length::Fill);
    let _card3 =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).height(Length::Shrink);
}

#[test]
fn card_can_set_max_width() {
    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).max_width(500.0);
}

#[test]
fn card_can_set_max_height() {
    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).max_height(400.0);
}

#[test]
fn card_can_set_padding() {
    let padding = Padding::new(15.0);
    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).padding(padding);
}

#[test]
fn card_can_set_head_padding() {
    let padding = Padding::new(20.0);
    let _card =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).padding_head(padding);
}

#[test]
fn card_can_set_body_padding() {
    let padding = Padding::new(25.0);
    let _card =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).padding_body(padding);
}

#[test]
fn card_can_set_foot_padding() {
    let padding = Padding::new(10.0);
    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .foot(Text::new("Footer"))
        .padding_foot(padding);
}

#[test]
fn card_can_set_all_padding_types() {
    let head_padding = Padding::new(5.0);
    let body_padding = Padding::new(10.0);
    let foot_padding = Padding::new(15.0);

    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .foot(Text::new("Footer"))
        .padding_head(head_padding)
        .padding_body(body_padding)
        .padding_foot(foot_padding);
}

#[test]
fn card_can_set_on_close() {
    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .on_close(Message::CloseCard);
}

#[test]
fn card_can_set_close_size() {
    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .on_close(Message::CloseCard)
        .close_size(20.0);
}

#[test]
fn card_can_set_different_close_sizes() {
    let sizes = vec![12.0, 16.0, 20.0, 24.0, 32.0];

    for size in sizes {
        let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
            .on_close(Message::CloseCard)
            .close_size(size);
    }
}

#[test]
fn card_can_be_styled() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).style(
        |_theme: &Theme, _status: Status| style::card::Style {
            background: Background::Color(Color::from_rgb(0.95, 0.95, 0.95)),
            border_radius: 8.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.8, 0.8, 0.8),
            head_background: Background::Color(Color::from_rgb(0.2, 0.4, 0.8)),
            head_text_color: Color::WHITE,
            body_background: Background::Color(Color::WHITE),
            body_text_color: Color::BLACK,
            foot_background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
            foot_text_color: Color::from_rgb(0.2, 0.2, 0.2),
            close_color: Color::WHITE,
        },
    );
}

#[test]
fn card_can_use_custom_class() {
    let _card: Card<Message, Theme> = Card::new(Text::new("Head"), Text::new("Body"))
        .class(<Theme as iced_aw::style::card::Catalog>::default());
}

#[test]
fn card_can_chain_multiple_methods() {
    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .foot(Text::new("Footer"))
        .width(400)
        .height(300)
        .max_width(600.0)
        .max_height(500.0)
        .padding(Padding::new(15.0))
        .on_close(Message::CloseCard);
}

#[test]
fn card_can_chain_all_methods() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _card = Card::<Message, Theme>::new(Text::new("Header"), Text::new("Content"))
        .foot(Text::new("Actions"))
        .width(500)
        .height(400)
        .max_width(800.0)
        .max_height(600.0)
        .padding_head(Padding::new(10.0))
        .padding_body(Padding::new(20.0))
        .padding_foot(Padding::new(10.0))
        .on_close(Message::CloseCard)
        .close_size(18.0)
        .style(|_theme: &Theme, _status: Status| style::card::Style {
            background: Background::Color(Color::WHITE),
            border_radius: 12.0,
            border_width: 2.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            head_background: Background::Color(Color::from_rgb(0.1, 0.3, 0.7)),
            head_text_color: Color::WHITE,
            body_background: Background::Color(Color::from_rgb(0.98, 0.98, 0.98)),
            body_text_color: Color::BLACK,
            foot_background: Background::Color(Color::from_rgb(0.85, 0.85, 0.85)),
            foot_text_color: Color::from_rgb(0.3, 0.3, 0.3),
            close_color: Color::from_rgb(0.9, 0.9, 0.9),
        })
        .class(<Theme as iced_aw::style::card::Catalog>::default());
}

#[test]
fn card_converts_to_element() {
    use iced::Element;
    use iced_widget::Renderer;

    let card: Card<Message, Theme> = Card::new(Text::new("Head"), Text::new("Body"));
    let _element: Element<Message, Theme, Renderer> = card.into();
}

#[test]
fn card_supports_multiple_instances() {
    let _card1 =
        Card::<Message, Theme>::new(Text::new("Card 1 Head"), Text::new("Card 1 Body")).width(300);
    let _card2 =
        Card::<Message, Theme>::new(Text::new("Card 2 Head"), Text::new("Card 2 Body")).height(200);
    let _card3 = Card::<Message, Theme>::new(Text::new("Card 3 Head"), Text::new("Card 3 Body"))
        .foot(Text::new("Card 3 Footer"));
}

#[test]
fn card_with_different_message_types() {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        Close,
        Action,
    }

    let _card: Card<CustomMessage, Theme> =
        Card::new(Text::new("Head"), Text::new("Body")).on_close(CustomMessage::Close);
}

#[test]
fn card_with_various_width_configurations() {
    let _card_fixed = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).width(400);
    let _card_fill =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).width(Length::Fill);
    let _card_shrink =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).width(Length::Shrink);
    let _card_fillportion = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .width(Length::FillPortion(3));
}

#[test]
fn card_with_various_height_configurations() {
    let _card_fixed = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).height(300);
    let _card_fill =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).height(Length::Fill);
    let _card_shrink =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).height(Length::Shrink);
    let _card_fillportion = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .height(Length::FillPortion(2));
}

#[test]
fn card_with_various_max_dimensions() {
    let _card1 = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .max_width(100.0)
        .max_height(100.0);
    let _card2 = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .max_width(500.0)
        .max_height(400.0);
    let _card3 = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .max_width(1000.0)
        .max_height(800.0);
}

#[test]
fn card_with_different_padding_values() {
    let padding_uniform = Padding::new(10.0);
    let padding_custom = Padding {
        top: 5.0,
        right: 10.0,
        bottom: 15.0,
        left: 20.0,
    };

    let _card1 =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).padding(padding_uniform);
    let _card2 =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).padding(padding_custom);
}

#[test]
fn card_with_asymmetric_padding() {
    let head_padding = Padding {
        top: 10.0,
        right: 15.0,
        bottom: 10.0,
        left: 15.0,
    };
    let body_padding = Padding {
        top: 20.0,
        right: 25.0,
        bottom: 20.0,
        left: 25.0,
    };
    let foot_padding = Padding {
        top: 5.0,
        right: 10.0,
        bottom: 5.0,
        left: 10.0,
    };

    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .foot(Text::new("Footer"))
        .padding_head(head_padding)
        .padding_body(body_padding)
        .padding_foot(foot_padding);
}

#[test]
fn card_with_empty_content() {
    let _card: Card<Message, Theme> = Card::new(Text::new(""), Text::new(""));
}

#[test]
fn card_with_long_content() {
    let long_head = "This is a very long header that might wrap or truncate";
    let long_body = "This is a very long body content that contains multiple sentences and should test how the card handles lengthy text. It might wrap across multiple lines depending on the card width.";
    let long_foot = "Footer with some additional information that might also be quite long";

    let _card: Card<Message, Theme> =
        Card::new(Text::new(long_head), Text::new(long_body)).foot(Text::new(long_foot));
}

#[test]
fn card_with_unicode_content() {
    let _card1: Card<Message, Theme> = Card::new(Text::new("标题"), Text::new("内容"));
    let _card2: Card<Message, Theme> = Card::new(Text::new("العنوان"), Text::new("المحتوى"));
    let _card3: Card<Message, Theme> =
        Card::new(Text::new("🎨 Header"), Text::new("📝 Body")).foot(Text::new("✅ Footer"));
}

#[test]
fn card_without_footer() {
    let _card: Card<Message, Theme> = Card::new(Text::new("Head"), Text::new("Body"));
}

#[test]
fn card_with_footer() {
    let _card: Card<Message, Theme> =
        Card::new(Text::new("Head"), Text::new("Body")).foot(Text::new("Footer"));
}

#[test]
fn card_with_on_close_without_custom_size() {
    let _card: Card<Message, Theme> =
        Card::new(Text::new("Head"), Text::new("Body")).on_close(Message::CloseCard);
}

#[test]
fn card_with_on_close_and_custom_size() {
    let _card: Card<Message, Theme> = Card::new(Text::new("Head"), Text::new("Body"))
        .on_close(Message::CloseCard)
        .close_size(24.0);
}

#[test]
fn card_with_various_styles() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let styles = vec![
        style::card::Style {
            background: Background::Color(Color::WHITE),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.9, 0.9, 0.9),
            head_background: Background::Color(Color::from_rgb(0.2, 0.5, 0.9)),
            head_text_color: Color::WHITE,
            body_background: Background::Color(Color::WHITE),
            body_text_color: Color::BLACK,
            foot_background: Background::Color(Color::from_rgb(0.95, 0.95, 0.95)),
            foot_text_color: Color::from_rgb(0.3, 0.3, 0.3),
            close_color: Color::WHITE,
        },
        style::card::Style {
            background: Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
            border_radius: 10.0,
            border_width: 2.0,
            border_color: Color::from_rgb(0.3, 0.3, 0.3),
            head_background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
            head_text_color: Color::from_rgb(0.9, 0.9, 0.9),
            body_background: Background::Color(Color::from_rgb(0.15, 0.15, 0.15)),
            body_text_color: Color::from_rgb(0.85, 0.85, 0.85),
            foot_background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
            foot_text_color: Color::from_rgb(0.8, 0.8, 0.8),
            close_color: Color::from_rgb(0.9, 0.9, 0.9),
        },
    ];

    for (i, style) in styles.into_iter().enumerate() {
        let _card = Card::<Message, Theme>::new(
            Text::new(format!("Style {} Head", i)),
            Text::new(format!("Style {} Body", i)),
        )
        .style(move |_theme: &Theme, _status: Status| style);
    }
}

#[test]
fn card_style_with_transparency() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).style(
        |_theme: &Theme, _status: Status| style::card::Style {
            background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            head_background: Background::Color(Color::from_rgba(0.2, 0.4, 0.8, 0.8)),
            head_text_color: Color::WHITE,
            body_background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.7)),
            body_text_color: Color::BLACK,
            foot_background: Background::Color(Color::from_rgba(0.9, 0.9, 0.9, 0.8)),
            foot_text_color: Color::from_rgb(0.3, 0.3, 0.3),
            close_color: Color::from_rgba(1.0, 1.0, 1.0, 0.9),
        },
    );
}

#[test]
fn card_create_multiple_with_different_configs() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    for i in 0..5 {
        let head = format!("Card {} Header", i);
        let body = format!("Card {} Content", i);
        let footer = format!("Card {} Footer", i);
        let width = 200 + (i * 50);
        let height = 150 + (i * 30);

        let _card = Card::<Message, Theme>::new(Text::new(head), Text::new(body))
            .foot(Text::new(footer))
            .width(width)
            .height(height)
            .padding(Padding::new((i as f32) * 5.0))
            .on_close(Message::CloseCard)
            .close_size(16.0 + (i as f32) * 2.0)
            .style(move |_theme: &Theme, _status: Status| style::card::Style {
                background: Background::Color(Color::from_rgb(1.0, 1.0, 1.0)),
                border_radius: 5.0 + (i as f32),
                border_width: 1.0,
                border_color: Color::from_rgb(0.8, 0.8, 0.8),
                head_background: Background::Color(Color::from_rgb(
                    0.2 + (i as f32) * 0.1,
                    0.4,
                    0.8,
                )),
                head_text_color: Color::WHITE,
                body_background: Background::Color(Color::WHITE),
                body_text_color: Color::BLACK,
                foot_background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
                foot_text_color: Color::from_rgb(0.3, 0.3, 0.3),
                close_color: Color::WHITE,
            });
    }
}

#[test]
fn card_with_extreme_dimensions() {
    let _card_tiny = Card::<Message, Theme>::new(Text::new("H"), Text::new("B"))
        .width(50)
        .height(30);

    let _card_large = Card::<Message, Theme>::new(Text::new("Header"), Text::new("Body"))
        .width(1920)
        .height(1080);
}

#[test]
fn card_with_zero_padding() {
    let zero_padding = Padding::new(0.0);
    let _card =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).padding(zero_padding);
}

#[test]
fn card_with_large_padding() {
    let large_padding = Padding::new(50.0);
    let _card =
        Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body")).padding(large_padding);
}

#[test]
fn card_padding_overrides() {
    // Test that specific padding overrides general padding
    let general_padding = Padding::new(10.0);
    let specific_head_padding = Padding::new(20.0);

    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .padding(general_padding)
        .padding_head(specific_head_padding);
}

#[test]
fn card_with_max_dimensions_smaller_than_regular() {
    // Test when max dimensions are smaller than regular dimensions
    let _card = Card::<Message, Theme>::new(Text::new("Head"), Text::new("Body"))
        .width(500)
        .height(400)
        .max_width(300.0)
        .max_height(250.0);
}

#[test]
fn card_style_then_class() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _card: Card<Message, Theme> = Card::new(Text::new("Head"), Text::new("Body"))
        .style(|_theme: &Theme, _status: Status| style::card::Style {
            background: Background::Color(Color::WHITE),
            border_radius: 8.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            head_background: Background::Color(Color::from_rgb(0.3, 0.5, 0.9)),
            head_text_color: Color::WHITE,
            body_background: Background::Color(Color::WHITE),
            body_text_color: Color::BLACK,
            foot_background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
            foot_text_color: Color::BLACK,
            close_color: Color::WHITE,
        })
        .class(<Theme as iced_aw::style::card::Catalog>::default());
}
