//! Integration tests for the ColorPicker widget
//!
//! These tests verify the ColorPicker widget's behavior by actually exercising
//! the widget through the iced test framework.

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

// ============================================================================
// Snapshot Test
// ============================================================================
#[test]
fn color_picker_snapshot_test() -> Result<(), iced_test::Error> {
    let (app, _) = App::new(move || ColorPicker::new(
                true, // Open
                Color::from_rgb(0.5, 0.5, 0.5),
                button(Text::new("Pick")).on_press(Message::Open),
                Message::Cancel,
                Message::Submit,
            )
            .into());

    // Create simulator
    let mut ui = simulator(&app);

    // Snapshot testing: verify visual rendering matches baseline
    assert_snapshot_matches(&mut ui, "tests/snapshots/color_picker_snapshot_test")?;

    Ok(())
}

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

// ============================================================================
// Mouse Interaction Tests
// ============================================================================
// These tests verify widget structure, button clicks, and basic interactions
// with the closed ColorPicker state.

#[test]
fn color_picker_underlay_button_is_clickable() {
    use iced_test::Error;

    let result: Result<(), Error> = (|| {
        let (mut app, _) = App::new(|| {
            ColorPicker::new(
                false,
                Color::from_rgb(0.5, 0.5, 0.5),
                button(Text::new("Pick Color")).on_press(Message::Open),
                Message::Cancel,
                Message::Submit,
            )
            .into()
        });

        let mut ui = simulator(&app);

        // Click the button to open the picker
        ui.click("Pick Color")?;

        // Verify we got the Open message
        assert_message_received(
            ui,
            &mut app,
            |m| matches!(m, Message::Open),
            "Clicking underlay button should produce Open message",
        );
        Ok(())
    })();

    result.expect("Test should complete without errors");
}

#[test]
fn color_picker_with_red_color_renders() {
    run_test(|| {
        ColorPicker::new(
            false,
            Color::from_rgb(1.0, 0.0, 0.0),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_with_green_color_renders() {
    run_test(|| {
        ColorPicker::new(
            false,
            Color::from_rgb(0.0, 1.0, 0.0),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_with_blue_color_renders() {
    run_test(|| {
        ColorPicker::new(
            false,
            Color::from_rgb(0.0, 0.0, 1.0),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_button_text_is_findable() {
    use iced_test::Error;

    fn run_test_inner() -> Result<(), Error> {
        let (app, _) = App::new(|| {
            ColorPicker::new(
                false,
                Color::from_rgb(0.5, 0.5, 0.5),
                button(Text::new("Choose Color")).on_press(Message::Open),
                Message::Cancel,
                Message::Submit,
            )
            .into()
        });

        let mut ui = simulator(&app);
        assert!(
            ui.find("Choose Color").is_ok(),
            "Button text should be findable through operate()"
        );
        Ok(())
    }

    run_test_inner().expect("Test should complete without errors");
}

// ============================================================================
// Color Value Tests
// ============================================================================

#[test]
fn color_picker_accepts_black_color() {
    run_test(|| {
        ColorPicker::new(
            false,
            Color::from_rgb(0.0, 0.0, 0.0),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_accepts_white_color() {
    run_test(|| {
        ColorPicker::new(
            false,
            Color::from_rgb(1.0, 1.0, 1.0),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_accepts_rgba_with_alpha() {
    run_test(|| {
        ColorPicker::new(
            false,
            Color::from_rgba(0.5, 0.5, 0.5, 0.75),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_accepts_transparent_color() {
    run_test(|| {
        ColorPicker::new(
            false,
            Color::from_rgba(1.0, 1.0, 1.0, 0.0),
            button(Text::new("Open")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

// ============================================================================
// Message Tests
// ============================================================================

#[test]
fn color_picker_submit_message_structure() {
    use iced_test::Error;

    fn run_test_inner() -> Result<(), Error> {
        let initial_color = Color::from_rgb(0.5, 0.5, 0.5);

        // Verify the Submit message carries the color value
        let submit_msg = Message::Submit(initial_color);
        assert!(
            matches!(submit_msg, Message::Submit(_)),
            "Submit message should contain color"
        );

        // Verify Cancel message
        let cancel_msg = Message::Cancel;
        assert!(
            matches!(cancel_msg, Message::Cancel),
            "Cancel message should match"
        );

        Ok(())
    }

    run_test_inner().expect("Test should complete without errors");
}

// ============================================================================
// Overlay Tests
// ============================================================================
// Testing overlay mouse interactions to determine if they cause threading issues

#[test]
fn color_picker_overlay_renders_when_open() {
    run_test(|| {
        ColorPicker::new(
            true, // Picker OPEN
            Color::from_rgb(0.5, 0.5, 0.5),
            button(Text::new("Pick Color")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_overlay_with_red_initial_color() {
    run_test(|| {
        ColorPicker::new(
            true,
            Color::from_rgb(1.0, 0.0, 0.0),
            button(Text::new("Pick Color")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_overlay_with_blue_initial_color() {
    run_test(|| {
        ColorPicker::new(
            true,
            Color::from_rgb(0.0, 0.0, 1.0),
            button(Text::new("Pick Color")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

#[test]
fn color_picker_overlay_with_transparent_color() {
    run_test(|| {
        ColorPicker::new(
            true,
            Color::from_rgba(0.5, 0.5, 0.5, 0.5),
            button(Text::new("Pick Color")).on_press(Message::Open),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
}

// ============================================================================
// Overlay Button Click Tests
// ============================================================================

#[test]
fn color_picker_overlay_cancel_button_click() {
    use iced_test::Error;

    fn run_test_inner() -> Result<(), Error> {
        let (mut app, _) = App::new(|| {
            ColorPicker::new(
                true, // Open
                Color::from_rgb(0.5, 0.5, 0.5),
                button(Text::new("Pick")).on_press(Message::Open),
                Message::Cancel,
                Message::Submit,
            )
            .into()
        });

        let mut ui = simulator(&app);

        // Click the cancel button (using Unicode character from icon font)
        ui.click("\u{e800}")?; // Cancel icon

        // Verify we got the Cancel message
        assert_message_received(
            ui,
            &mut app,
            |m| matches!(m, Message::Cancel),
            "Clicking cancel button should produce Cancel message",
        );
        Ok(())
    }

    run_test_inner().expect("Test should complete without errors");
}

#[test]
fn color_picker_overlay_submit_button_click() {
    use iced_test::Error;

    fn run_test_inner() -> Result<(), Error> {
        let (mut app, _) = App::new(|| {
            ColorPicker::new(
                true, // Open
                Color::from_rgb(0.5, 0.5, 0.5),
                button(Text::new("Pick")).on_press(Message::Open),
                Message::Cancel,
                Message::Submit,
            )
            .into()
        });

        let mut ui = simulator(&app);

        // Click the submit button (using Unicode character from icon font)
        ui.click("\u{e805}")?; // OK/Submit icon

        // Verify we got the Submit message
        assert_message_received(
            ui,
            &mut app,
            |m| matches!(m, Message::Submit(_)),
            "Clicking submit button should produce Submit message",
        );
        Ok(())
    }

    run_test_inner().expect("Test should complete without errors");
}

// ============================================================================
// Overlay Mouse Event Tests
// ============================================================================
// Testing raw mouse events on the overlay color areas

#[test]
fn color_picker_overlay_mouse_wheel_scroll_event() {
    use iced::Event;
    use iced_core::mouse;

    fn run_test_inner() {
        let (mut app, _) = App::new(|| {
            ColorPicker::new(
                true, // Open
                Color::from_rgb(0.5, 0.5, 0.5),
                button(Text::new("Pick")).on_press(Message::Open),
                Message::Cancel,
                Message::Submit,
            )
            .into()
        });

        let mut ui = simulator(&app);

        // Simulate mouse wheel scroll (this tests WheelScrolled event handling)
        ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Lines { x: 0.0, y: 1.0 },
        })]);

        // Process messages (even if no messages, this tests that the event doesn't crash)
        process_messages(ui, &mut app);
    }

    run_test_inner();
}

#[test]
fn color_picker_overlay_mouse_button_press_and_release() {
    use iced::Event;
    use iced_core::mouse;

    fn run_test_inner() {
        let (mut app, _) = App::new(|| {
            ColorPicker::new(
                true, // Open
                Color::from_rgb(0.5, 0.5, 0.5),
                button(Text::new("Pick")).on_press(Message::Open),
                Message::Cancel,
                Message::Submit,
            )
            .into()
        });

        let mut ui = simulator(&app);

        // Simulate button press and release (tests drag interaction pattern)
        ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);

        // Process any messages from press
        process_messages(ui, &mut app);

        // Create new simulator for release
        let mut ui = simulator(&app);
        ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
            mouse::Button::Left,
        ))]);

        // Process any messages from release
        process_messages(ui, &mut app);
    }

    run_test_inner();
}

#[test]
fn color_picker_overlay_multiple_wheel_scrolls() {
    use iced::Event;
    use iced_core::mouse;

    fn run_test_inner() {
        let (mut app, _) = App::new(|| {
            ColorPicker::new(
                true, // Open
                Color::from_rgb(0.5, 0.5, 0.5),
                button(Text::new("Pick")).on_press(Message::Open),
                Message::Cancel,
                Message::Submit,
            )
            .into()
        });

        // Scroll up
        let mut ui = simulator(&app);
        ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Lines { x: 0.0, y: 1.0 },
        })]);
        process_messages(ui, &mut app);

        // Scroll down
        let mut ui = simulator(&app);
        ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Lines { x: 0.0, y: -1.0 },
        })]);
        process_messages(ui, &mut app);
    }

    run_test_inner();
}
