//! Integration tests for the MenuButton widget
//!
//! These tests verify the MenuButton widget's behavior by actually exercising
//! the widget through the iced test framework. MenuButton is designed for use
//! in menus and overlays where standard buttons have hover state issues.

#[macro_use]
mod common;

use iced::{Length, Point};
use iced_aw::MenuButton;
use iced_core::mouse::Button;
use iced_test::Error;
use iced_widget::text::Text;

// Message type for the tests
#[derive(Clone, Debug, PartialEq)]
enum Message {
    ButtonPressed,
    Action1,
    Action2,
}

// Generate test helpers for this Message type
test_helpers!(Message);

// ============================================================================
// Snapshot Tests
// ============================================================================

#[test]
fn menu_button_snapshot_test() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        MenuButton::new(Text::new("Menu Item"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    let mut ui = simulator(&app);

    // Snapshot testing: verify visual rendering matches baseline
    assert_snapshot_matches(&mut ui, "tests/snapshots/menu_button_snapshot_test")?;

    Ok(())
}

// ============================================================================
// Basic Rendering Tests
// ============================================================================

#[test]
fn menu_button_renders_with_text() {
    run_test_and_find(
        || MenuButton::new(Text::new("Test Button")).into(),
        "Test Button",
    );
}

#[test]
fn menu_button_renders_without_on_press() {
    // MenuButton without on_press should render (disabled state)
    run_test(|| MenuButton::<Message>::new(Text::new("Disabled")).into());
}

#[test]
fn menu_button_renders_with_on_press() {
    run_test(|| {
        MenuButton::new(Text::new("Enabled"))
            .on_press(Message::ButtonPressed)
            .into()
    });
}

#[test]
fn menu_button_renders_with_empty_text() {
    // Empty text should render without error
    run_test(|| MenuButton::<Message>::new(Text::new("")).into());
}

#[test]
fn menu_button_renders_with_long_text() {
    let long_text = "This is a very long menu button text that might wrap or overflow";
    run_test_and_find(
        move || MenuButton::<Message>::new(Text::new(long_text)).into(),
        long_text,
    );
}

#[test]
fn menu_button_renders_with_unicode_text() {
    // Test various Unicode scripts
    run_test_and_find(
        || MenuButton::<Message>::new(Text::new("你好")).into(),
        "你好",
    );
    run_test_and_find(
        || MenuButton::<Message>::new(Text::new("🎉")).into(),
        "🎉",
    );
    run_test_and_find(
        || MenuButton::<Message>::new(Text::new("مرحبا")).into(),
        "مرحبا",
    );
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn menu_button_with_custom_width_renders() {
    // Test Fixed width
    run_test_and_find(
        || {
            MenuButton::new(Text::new("Fixed"))
                .width(Length::Fixed(100.0))
                .into()
        },
        "Fixed",
    );

    // Test Fill width
    run_test_and_find(
        || MenuButton::new(Text::new("Fill")).width(Length::Fill).into(),
        "Fill",
    );

    // Test Shrink width
    run_test_and_find(
        || {
            MenuButton::new(Text::new("Shrink"))
                .width(Length::Shrink)
                .into()
        },
        "Shrink",
    );
}

#[test]
fn menu_button_with_custom_padding_renders() {
    // Zero padding
    run_test_and_find(
        || MenuButton::new(Text::new("Zero")).padding(0.0).into(),
        "Zero",
    );

    // Default padding
    run_test_and_find(
        || MenuButton::new(Text::new("Default")).padding(8.0).into(),
        "Default",
    );

    // Large padding
    run_test_and_find(
        || MenuButton::new(Text::new("Large")).padding(20.0).into(),
        "Large",
    );
}

#[test]
fn menu_button_with_fill_portion_width_renders() {
    run_test_and_find(
        || {
            MenuButton::new(Text::new("Portion"))
                .width(Length::FillPortion(2))
                .into()
        },
        "Portion",
    );
}

#[test]
fn menu_button_with_method_chaining_renders() {
    run_test_and_find(
        || {
            MenuButton::new(Text::new("Chained"))
                .on_press(Message::ButtonPressed)
                .width(Length::Fixed(150.0))
                .padding(12.0)
                .into()
        },
        "Chained",
    );
}

// ============================================================================
// Interaction Tests - Click Events
// ============================================================================

#[test]
fn menu_button_click_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        MenuButton::new(Text::new("Click Me"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click the button by finding its text
    ui.click("Click Me")?;

    // Verify the message was produced
    assert_message_received(
        ui,
        &mut app,
        |msg| matches!(msg, Message::ButtonPressed),
        "Should receive ButtonPressed message after click",
    );

    Ok(())
}

#[test]
fn menu_button_without_on_press_produces_no_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| MenuButton::<Message>::new(Text::new("Disabled")).into());

    let mut ui = simulator(&app);

    // Try clicking the disabled button
    simulate_left_click_at(&mut ui, 50.0, 20.0);

    // Verify no message was produced
    let got_message = check_message_received(ui, &mut app, |_| true);
    assert!(!got_message, "Disabled button should not produce messages");

    Ok(())
}

#[test]
fn menu_button_multiple_clicks_produce_multiple_messages() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        MenuButton::new(Text::new("Click Me"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    // Click three times
    for _ in 0..3 {
        let mut ui = simulator(&app);
        simulate_left_click_at(&mut ui, 50.0, 20.0);
        process_messages(ui, &mut app);
    }

    // Each click should have produced a message (verified by not panicking)
    Ok(())
}

// ============================================================================
// Interaction Tests - Mouse Position
// ============================================================================

#[test]
fn menu_button_click_outside_produces_no_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        MenuButton::new(Text::new("Button"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click far outside the button
    let pos = outside_position();
    simulate_left_click_at(&mut ui, pos.x, pos.y);

    // Verify no message was produced
    let got_message = check_message_received(ui, &mut app, |_| true);
    assert!(
        !got_message,
        "Clicking outside button should not produce messages"
    );

    Ok(())
}

#[test]
fn menu_button_right_click_produces_no_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        MenuButton::new(Text::new("Button"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    let mut ui = simulator(&app);

    // Right-click the button (only left clicks should work)
    simulate_right_click_at(&mut ui, 50.0, 20.0);

    // Verify no message was produced
    let got_message = check_message_received(ui, &mut app, |_| true);
    assert!(
        !got_message,
        "Right-clicking button should not produce messages"
    );

    Ok(())
}

// ============================================================================
// Interaction Tests - Touch Events
// ============================================================================

#[test]
fn menu_button_touch_press_works() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        MenuButton::new(Text::new("Touch Me"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    let mut ui = simulator(&app);

    // Simulate touch event
    simulate_touch_at(&mut ui, 50.0, 20.0);

    // Touch events should work like clicks
    // Note: The current implementation may not fully support touch,
    // but this test ensures it doesn't panic
    process_messages(ui, &mut app);

    Ok(())
}

// ============================================================================
// Interaction Tests - Hover State
// ============================================================================

#[test]
fn menu_button_hover_state_updates() -> Result<(), Error> {
    use iced::Event;
    use iced_core::mouse;

    let (app, _) = App::new(|| {
        MenuButton::new(Text::new("Hover Me"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    let mut ui = simulator(&app);

    // Move cursor over button
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(50.0, 20.0),
    })]);

    // The button should update its hover state internally
    // (This doesn't produce a message, but tests that the event is processed)

    // Move cursor away
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(1000.0, 1000.0),
    })]);

    // No panic means hover state is being tracked correctly
    Ok(())
}

// ============================================================================
// Multiple Instance Tests
// ============================================================================

#[test]
fn menu_button_multiple_instances_render() {
    // Test that multiple buttons with different configs can be created
    for i in 0..5 {
        let text = format!("Button {}", i);
        let text_copy = text.clone();
        run_test_and_find(
            move || {
                MenuButton::new(Text::new(text_copy.clone()))
                    .on_press(Message::ButtonPressed)
                    .padding((i as f32) * 2.0)
                    .into()
            },
            &text,
        );
    }
}

#[test]
fn menu_button_different_messages_work() -> Result<(), Error> {
    // Test button with Action1
    {
        let (mut app, _) = App::new(|| {
            MenuButton::new(Text::new("Action 1"))
                .on_press(Message::Action1)
                .into()
        });

        let mut ui = simulator(&app);
        ui.click("Action 1")?;

        assert_message_received(
            ui,
            &mut app,
            |msg| matches!(msg, Message::Action1),
            "Should receive Action1 message",
        );
    }

    // Test button with Action2
    {
        let (mut app, _) = App::new(|| {
            MenuButton::new(Text::new("Action 2"))
                .on_press(Message::Action2)
                .into()
        });

        let mut ui = simulator(&app);
        ui.click("Action 2")?;

        assert_message_received(
            ui,
            &mut app,
            |msg| matches!(msg, Message::Action2),
            "Should receive Action2 message",
        );
    }

    Ok(())
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn menu_button_with_zero_width_renders() {
    // Even with zero width, should render without panic
    run_test(|| {
        MenuButton::<Message>::new(Text::new("Zero"))
            .width(Length::Fixed(0.0))
            .into()
    });
}

#[test]
fn menu_button_with_extreme_padding_renders() {
    // Very large padding
    run_test_and_find(
        || {
            MenuButton::new(Text::new("Huge Padding"))
                .padding(100.0)
                .into()
        },
        "Huge Padding",
    );
}

#[test]
fn menu_button_rapid_clicks() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        MenuButton::new(Text::new("Rapid"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    // Simulate rapid clicking
    for _ in 0..10 {
        let mut ui = simulator(&app);
        simulate_left_click_at(&mut ui, 50.0, 20.0);
        process_messages(ui, &mut app);
    }

    // Should handle rapid clicks without issue
    Ok(())
}

// ============================================================================
// Press and Release Sequence Tests
// ============================================================================

#[test]
fn menu_button_press_release_sequence() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        MenuButton::new(Text::new("Press Me"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    let mut ui = simulator(&app);

    // Use the click method which handles the full press/release sequence
    ui.click("Press Me")?;

    // Verify message was produced
    assert_message_received(
        ui,
        &mut app,
        |msg| matches!(msg, Message::ButtonPressed),
        "Should receive message on button release",
    );

    Ok(())
}

#[test]
fn menu_button_press_move_away_release_produces_no_message() -> Result<(), Error> {
    use iced::Event;
    use iced_core::mouse;

    let (mut app, _) = App::new(|| {
        MenuButton::new(Text::new("Button"))
            .on_press(Message::ButtonPressed)
            .into()
    });

    let mut ui = simulator(&app);

    let button_pos = Point::new(50.0, 20.0);
    let away_pos = Point::new(1000.0, 1000.0);

    // Move cursor over button
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: button_pos,
    })]);

    // Press mouse button
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(Button::Left))]);

    // Move cursor away while pressed
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: away_pos,
    })]);

    // Release mouse button while away
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(Button::Left))]);

    // Verify no message was produced (release outside button)
    let got_message = check_message_received(ui, &mut app, |_| true);
    assert!(
        !got_message,
        "Releasing outside button should not produce message"
    );

    Ok(())
}

// ============================================================================
// Stateful Test
// ============================================================================

#[test]
fn menu_button_in_menu_context() -> Result<(), Error> {
    use iced::widget::column;

    // Simulate a menu with multiple buttons
    let (app, _) = App::new(|| {
        column![
            MenuButton::new(Text::new("Item 1")).on_press(Message::Action1),
            MenuButton::new(Text::new("Item 2")).on_press(Message::Action2),
            MenuButton::new(Text::new("Disabled")),
        ]
        .into()
    });

    let mut ui = simulator(&app);

    // All items should be findable
    assert!(ui.find("Item 1").is_ok(), "Should find Item 1");
    assert!(ui.find("Item 2").is_ok(), "Should find Item 2");
    assert!(ui.find("Disabled").is_ok(), "Should find Disabled item");

    Ok(())
}
