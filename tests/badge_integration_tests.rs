//! Integration tests for the Badge widget
//!
//! These tests verify the Badge widget's behavior by actually exercising
//! the widget through the iced test framework.

#[macro_use]
mod common;

use iced::{Alignment, Color, Length, Theme};
use iced_aw::Badge;
use iced_widget::text::Text;

// Message type for the tests (unused but required by iced)
#[derive(Clone, Debug)]
enum Message {}

// Generate test helpers for this Message type
test_helpers!(Message);

#[test]
fn badge_renders_with_text() {
    run_test_and_find(|| Badge::new(Text::new("Test Badge")).into(), "Test Badge");
}

#[test]
fn badge_with_padding_renders() {
    run_test_and_find(
        || Badge::new(Text::new("Padded")).padding(10).into(),
        "Padded",
    );
}

#[test]
fn badge_with_width_settings_renders() {
    // Test Fixed width
    run_test_and_find(
        || Badge::new(Text::new("Fixed")).width(100.0).into(),
        "Fixed",
    );

    // Test Fill width
    run_test_and_find(
        || Badge::new(Text::new("Fill")).width(Length::Fill).into(),
        "Fill",
    );

    // Test Shrink width
    run_test_and_find(
        || Badge::new(Text::new("Shrink")).width(Length::Shrink).into(),
        "Shrink",
    );
}

#[test]
fn badge_with_height_settings_renders() {
    // Test Fixed height
    run_test_and_find(
        || Badge::new(Text::new("Fixed")).height(30.0).into(),
        "Fixed",
    );

    // Test Fill height
    run_test_and_find(
        || Badge::new(Text::new("Fill")).height(Length::Fill).into(),
        "Fill",
    );

    // Test Shrink height
    run_test_and_find(
        || {
            Badge::new(Text::new("Shrink"))
                .height(Length::Shrink)
                .into()
        },
        "Shrink",
    );
}

#[test]
fn badge_with_horizontal_alignment_renders() {
    run_test_and_find(
        || {
            Badge::new(Text::new("Start"))
                .align_x(Alignment::Start)
                .into()
        },
        "Start",
    );

    run_test_and_find(
        || {
            Badge::new(Text::new("Center"))
                .align_x(Alignment::Center)
                .into()
        },
        "Center",
    );

    run_test_and_find(
        || Badge::new(Text::new("End")).align_x(Alignment::End).into(),
        "End",
    );
}

#[test]
fn badge_with_vertical_alignment_renders() {
    run_test_and_find(
        || {
            Badge::new(Text::new("Top"))
                .align_y(Alignment::Start)
                .into()
        },
        "Top",
    );

    run_test_and_find(
        || {
            Badge::new(Text::new("Middle"))
                .align_y(Alignment::Center)
                .into()
        },
        "Middle",
    );

    run_test_and_find(
        || {
            Badge::new(Text::new("Bottom"))
                .align_y(Alignment::End)
                .into()
        },
        "Bottom",
    );
}

#[test]
fn badge_with_both_alignments_renders() {
    run_test_and_find(
        || {
            Badge::new(Text::new("Aligned"))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
        },
        "Aligned",
    );
}

#[test]
fn badge_with_custom_style_renders() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    run_test_and_find(
        || {
            Badge::new(Text::new("Styled"))
                .style(|_theme: &Theme, _status: Status| style::badge::Style {
                    background: Background::Color(Color::from_rgb(0.8, 0.2, 0.2)),
                    border_radius: Some(10.0),
                    border_width: 2.0,
                    border_color: Some(Color::BLACK),
                    text_color: Color::WHITE,
                })
                .into()
        },
        "Styled",
    );
}

#[test]
fn badge_with_custom_class_renders() {
    run_test_and_find(
        || {
            Badge::new(Text::new("Classed"))
                .class(<Theme as iced_aw::style::badge::Catalog>::default())
                .into()
        },
        "Classed",
    );
}

#[test]
fn badge_with_method_chaining_renders() {
    run_test_and_find(
        || {
            Badge::new(Text::new("Chained"))
                .padding(15)
                .width(200)
                .height(60)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
        },
        "Chained",
    );
}

#[test]
fn badge_with_all_methods_chained_renders() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    run_test_and_find(
        || {
            Badge::new(Text::new("Complete"))
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
                    text_color: Color::WHITE,
                })
                .class(<Theme as iced_aw::style::badge::Catalog>::default())
                .into()
        },
        "Complete",
    );
}

#[test]
fn badge_with_empty_text_renders() {
    // Empty text won't be found, but badge should render without error
    run_test(|| Badge::new(Text::new("")).into());
}

#[test]
fn badge_with_long_text_renders() {
    let long_text = "This is a very long badge text that might wrap or overflow";
    run_test_and_find(move || Badge::new(Text::new(long_text)).into(), long_text);
}

#[test]
fn badge_with_unicode_text_renders() {
    run_test_and_find(|| Badge::new(Text::new("你好")).into(), "你好");
    run_test_and_find(|| Badge::new(Text::new("🎉")).into(), "🎉");
    run_test_and_find(|| Badge::new(Text::new("مرحبا")).into(), "مرحبا");
}

#[test]
fn badge_with_extreme_padding_renders() {
    // Zero padding
    run_test_and_find(|| Badge::new(Text::new("Zero")).padding(0).into(), "Zero");

    // Large padding
    run_test_and_find(
        || Badge::new(Text::new("Large")).padding(255).into(),
        "Large",
    );
}

#[test]
fn badge_with_transparent_style_renders() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    run_test_and_find(
        || {
            Badge::new(Text::new("Transparent"))
                .style(|_theme: &Theme, _status: Status| style::badge::Style {
                    background: Background::Color(Color::from_rgba(1.0, 0.0, 0.0, 0.5)),
                    border_radius: Some(8.0),
                    border_width: 1.0,
                    border_color: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.8)),
                    text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.9),
                })
                .into()
        },
        "Transparent",
    );
}

#[test]
fn badge_multiple_instances_render() {
    // Test that multiple badges with different configs can be created
    for i in 0..5 {
        let text = format!("Badge {}", i);
        let text_copy = text.clone();
        run_test_and_find(
            move || {
                Badge::new(Text::new(text_copy.clone()))
                    .padding((i as u16) * 2)
                    .into()
            },
            &text,
        );
    }
}

#[test]
fn badge_with_fill_portion_width_renders() {
    run_test_and_find(
        || {
            Badge::new(Text::new("Portion"))
                .width(Length::FillPortion(2))
                .into()
        },
        "Portion",
    );
}

#[test]
fn badge_with_different_border_styles_renders() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    // No border
    run_test_and_find(
        || {
            Badge::new(Text::new("NoBorder"))
                .style(|_theme: &Theme, _status: Status| style::badge::Style {
                    background: Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
                    border_radius: None,
                    border_width: 0.0,
                    border_color: None,
                    text_color: Color::WHITE,
                })
                .into()
        },
        "NoBorder",
    );

    // Thick border
    run_test_and_find(
        || {
            Badge::new(Text::new("ThickBorder"))
                .style(|_theme: &Theme, _status: Status| style::badge::Style {
                    background: Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
                    border_radius: Some(5.0),
                    border_width: 5.0,
                    border_color: Some(Color::BLACK),
                    text_color: Color::WHITE,
                })
                .into()
        },
        "ThickBorder",
    );

    // Rounded corners
    run_test_and_find(
        || {
            Badge::new(Text::new("Rounded"))
                .style(|_theme: &Theme, _status: Status| style::badge::Style {
                    background: Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
                    border_radius: Some(20.0),
                    border_width: 1.0,
                    border_color: Some(Color::BLACK),
                    text_color: Color::WHITE,
                })
                .into()
        },
        "Rounded",
    );
}
