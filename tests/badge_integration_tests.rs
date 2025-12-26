//! Integration tests for the Badge widget
//!
//! These tests verify the Badge widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::Settings;
use iced::{Alignment, Color, Element, Length, Theme};
use iced_aw::Badge;
use iced_test::{Error, Simulator};
use iced_widget::text::Text;

#[derive(Clone)]
#[allow(dead_code)]
enum Message {
    DoNothing,
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
#[derive(Default)]
struct App {}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (App::default(), iced::Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DoNothing => {
                // Do nothing
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        Badge::new(Text::new("Test Badge")).into()
    }
}

fn simulator(app: &App) -> Simulator<'_, Message> {
    Simulator::with_settings(
        Settings {
            ..Settings::default()
        },
        app.view(),
    )
}

#[test]
fn badge_operate_test() -> Result<(), Error> {
    let (mut app, _command) = App::new();
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Create simulator again and find text inside Badge
    let mut ui = simulator(&app);

    // Now that Badge implements operate(), the built-in string selector works!
    assert!(
        ui.find("Test Badge").is_ok(),
        "Text inside Badge should be found!"
    );

    Ok(())
}

#[test]
fn badge_can_be_created_with_text() {
    let (app, _command) = App::new();
    let _ui = simulator(&app);

    //let _badge: Badge<Message, Theme> = Badge::new(Text::new("Badge"));
}

#[test]
fn badge_can_be_created_with_different_text_content() {
    let texts = vec!["New", "1", "999+", "Alert", "!"];

    for text in texts {
        let _badge: Badge<Message, Theme> = Badge::new(Text::new(text));
    }
}

#[test]
fn badge_can_set_padding() {
    let badge = Badge::<Message, Theme>::new(Text::new("Text")).padding(10);
    let _ = badge;
}

#[test]
fn badge_can_set_different_padding_values() {
    let padding_values = vec![0, 5, 10, 15, 20, 50, 100];

    for padding in padding_values {
        let _badge = Badge::<Message, Theme>::new(Text::new("Test")).padding(padding);
    }
}

#[test]
fn badge_can_set_width() {
    let _badge1 = Badge::<Message, Theme>::new(Text::new("Text")).width(100);
    let _badge2 = Badge::<Message, Theme>::new(Text::new("Text")).width(Length::Fill);
    let _badge3 = Badge::<Message, Theme>::new(Text::new("Text")).width(Length::Shrink);
}

#[test]
fn badge_can_set_height() {
    let _badge1 = Badge::<Message, Theme>::new(Text::new("Text")).height(50);
    let _badge2 = Badge::<Message, Theme>::new(Text::new("Text")).height(Length::Fill);
    let _badge3 = Badge::<Message, Theme>::new(Text::new("Text")).height(Length::Shrink);
}

#[test]
fn badge_can_set_horizontal_alignment() {
    let _badge_start = Badge::<Message, Theme>::new(Text::new("Text")).align_x(Alignment::Start);
    let _badge_center = Badge::<Message, Theme>::new(Text::new("Text")).align_x(Alignment::Center);
    let _badge_end = Badge::<Message, Theme>::new(Text::new("Text")).align_x(Alignment::End);
}

#[test]
fn badge_can_set_vertical_alignment() {
    let _badge_start = Badge::<Message, Theme>::new(Text::new("Text")).align_y(Alignment::Start);
    let _badge_center = Badge::<Message, Theme>::new(Text::new("Text")).align_y(Alignment::Center);
    let _badge_end = Badge::<Message, Theme>::new(Text::new("Text")).align_y(Alignment::End);
}

#[test]
fn badge_can_set_both_alignments() {
    let _badge = Badge::<Message, Theme>::new(Text::new("Text"))
        .align_x(Alignment::Start)
        .align_y(Alignment::End);
}

#[test]
fn badge_can_be_styled() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _badge = Badge::<Message, Theme>::new(Text::new("Styled")).style(
        |_theme: &Theme, _status: Status| style::badge::Style {
            background: Background::Color(Color::from_rgb(0.8, 0.2, 0.2)),
            border_radius: Some(10.0),
            border_width: 2.0,
            border_color: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
        },
    );
}

#[test]
fn badge_can_use_custom_class() {
    let _badge: Badge<Message, Theme> =
        Badge::new(Text::new("Test")).class(<Theme as iced_aw::style::badge::Catalog>::default());
}

#[test]
fn badge_can_chain_multiple_methods() {
    let _badge = Badge::<Message, Theme>::new(Text::new("Chained"))
        .padding(15)
        .width(200)
        .height(60)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
}

#[test]
fn badge_can_chain_all_methods() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _badge = Badge::<Message, Theme>::new(Text::new("All"))
        .padding(10)
        .width(150)
        .height(40)
        .align_x(Alignment::Start)
        .align_y(Alignment::End)
        .style(|_theme: &Theme, _status: Status| style::badge::Style {
            background: Background::Color(Color::from_rgb(0.3, 0.7, 0.3)),
            border_radius: Some(8.0),
            border_width: 1.0,
            border_color: Some(Color::from_rgb(0.1, 0.1, 0.1)),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
        })
        .class(<Theme as iced_aw::style::badge::Catalog>::default());
}

#[test]
fn badge_converts_to_element() {
    use iced::Element;
    use iced_widget::Renderer;

    let badge: Badge<Message, Theme> = Badge::new(Text::new("Element"));
    let _element: Element<Message, Theme, Renderer> = badge.into();
}

#[test]
fn badge_supports_multiple_instances() {
    let _badge1 = Badge::<Message, Theme>::new(Text::new("First")).padding(5);
    let _badge2 = Badge::<Message, Theme>::new(Text::new("Second")).padding(10);
    let _badge3 = Badge::<Message, Theme>::new(Text::new("Third")).padding(15);
}

#[test]
fn badge_with_different_message_types() {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        BadgeClicked,
        Other,
    }

    let _badge: Badge<CustomMessage, Theme> = Badge::new(Text::new("Custom"));
}

#[test]
fn badge_with_various_width_configurations() {
    let _badge_fixed = Badge::<Message, Theme>::new(Text::new("Fixed")).width(100);
    let _badge_fill = Badge::<Message, Theme>::new(Text::new("Fill")).width(Length::Fill);
    let _badge_shrink = Badge::<Message, Theme>::new(Text::new("Shrink")).width(Length::Shrink);
    let _badge_fillportion =
        Badge::<Message, Theme>::new(Text::new("Portion")).width(Length::FillPortion(2));
}

#[test]
fn badge_with_various_height_configurations() {
    let _badge_fixed = Badge::<Message, Theme>::new(Text::new("Fixed")).height(30);
    let _badge_fill = Badge::<Message, Theme>::new(Text::new("Fill")).height(Length::Fill);
    let _badge_shrink = Badge::<Message, Theme>::new(Text::new("Shrink")).height(Length::Shrink);
    let _badge_fillportion =
        Badge::<Message, Theme>::new(Text::new("Portion")).height(Length::FillPortion(3));
}

#[test]
fn badge_with_extreme_padding_values() {
    let _badge_zero = Badge::<Message, Theme>::new(Text::new("Zero")).padding(0);
    let _badge_large = Badge::<Message, Theme>::new(Text::new("Large")).padding(255);
}

#[test]
fn badge_with_all_alignment_combinations() {
    let h_alignments = vec![Alignment::Start, Alignment::Center, Alignment::End];
    let v_alignments = vec![Alignment::Start, Alignment::Center, Alignment::End];

    for h_align in &h_alignments {
        for v_align in &v_alignments {
            let _badge = Badge::<Message, Theme>::new(Text::new("Aligned"))
                .align_x(*h_align)
                .align_y(*v_align);
        }
    }
}

#[test]
fn badge_with_various_styles() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let styles = vec![
        style::badge::Style {
            background: Background::Color(Color::from_rgb(1.0, 0.0, 0.0)),
            border_radius: Some(5.0),
            border_width: 1.0,
            border_color: Some(Color::BLACK),
            text_color: Color::WHITE,
        },
        style::badge::Style {
            background: Background::Color(Color::from_rgb(0.0, 1.0, 0.0)),
            border_radius: Some(10.0),
            border_width: 2.0,
            border_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
            text_color: Color::from_rgb(0.1, 0.1, 0.1),
        },
        style::badge::Style {
            background: Background::Color(Color::from_rgb(0.0, 0.0, 1.0)),
            border_radius: None,
            border_width: 0.0,
            border_color: None,
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
        },
    ];

    for (i, style) in styles.into_iter().enumerate() {
        let _badge = Badge::<Message, Theme>::new(Text::new(format!("Style {}", i)))
            .style(move |_theme: &Theme, _status: Status| style);
    }
}

#[test]
fn badge_with_empty_text() {
    let _badge: Badge<Message, Theme> = Badge::new(Text::new(""));
}

#[test]
fn badge_with_long_text() {
    let long_text = "This is a very long badge text that might wrap or overflow";
    let _badge: Badge<Message, Theme> = Badge::new(Text::new(long_text));
}

#[test]
fn badge_with_unicode_text() {
    let _badge1: Badge<Message, Theme> = Badge::new(Text::new("你好"));
    let _badge2: Badge<Message, Theme> = Badge::new(Text::new("🎉"));
    let _badge3: Badge<Message, Theme> = Badge::new(Text::new("مرحبا"));
}

#[test]
fn badge_style_with_transparency() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _badge = Badge::<Message, Theme>::new(Text::new("Transparent")).style(
        |_theme: &Theme, _status: Status| style::badge::Style {
            background: Background::Color(Color::from_rgba(1.0, 0.0, 0.0, 0.5)),
            border_radius: Some(8.0),
            border_width: 1.0,
            border_color: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.8)),
            text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.9),
        },
    );
}

#[test]
fn badge_create_multiple_with_different_configs() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    for i in 0..10 {
        let text = format!("Badge {}", i);
        let padding = (i as u16) * 2;
        let width = 50 + (i * 10);

        let _badge = Badge::<Message, Theme>::new(Text::new(text))
            .padding(padding)
            .width(width)
            .align_x(Alignment::Center)
            .style(move |_theme: &Theme, _status: Status| style::badge::Style {
                background: Background::Color(Color::from_rgb(
                    (i as f32) * 0.1,
                    0.5,
                    1.0 - (i as f32) * 0.1,
                )),
                border_radius: Some((i as f32) * 2.0),
                border_width: 1.0,
                border_color: Some(Color::BLACK),
                text_color: Color::WHITE,
            });
    }
}
