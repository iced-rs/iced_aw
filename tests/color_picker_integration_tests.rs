//! Integration tests for the ColorPicker widget
//!
//! These tests verify the ColorPicker widget's behavior by actually exercising
//! the widget through the iced test framework.
//!
//! Note: ColorPicker has known thread-safety issues with overlays in the test
//! framework. These tests are minimal to avoid segfaults in parallel execution.

#[macro_use]
mod common;

use iced::Color;
use iced_aw::ColorPicker;
use iced_widget::button;
use iced_widget::text::Text;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Open,
    Cancel,
    Submit(Color),
}

// Generate test helpers for this Message type
test_helpers!(Message);

#[test]
fn color_picker_closed_renders_underlay_button() {
    let (app, _) = App::new(|| {
        ColorPicker::new(
            false, // Picker closed
            Color::from_rgb(0.5, 0.5, 0.5),
            button(Text::new("Open Picker")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });

    let mut ui = simulator(&app);

    // Verify the underlay button is visible when picker is closed
    assert!(
        ui.find("Open Picker").is_ok(),
        "Underlay button should be visible when picker is closed"
    );
}

#[test]
fn color_picker_closed_state_renders() {
    run_test(|| {
        ColorPicker::new(
            false, // Closed
            Color::from_rgb(0.5, 0.5, 0.5),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}
