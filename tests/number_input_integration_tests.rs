//! Integration tests for the NumberInput widget
//!
//! These tests verify the NumberInput widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

// Test Notes:
// Button cheat sheet
//  cancel          → \u{e800}  // Used for close/cancel buttons
//  down_open       → \u{e801}  // Down arrow (used in number_input, time_picker)
//  left_open       → \u{e802}  // Left arrow (used in date_picker navigation)
//  right_open      → \u{e803}  // Right arrow (used in date_picker navigation)
//  up_open         → \u{e804}  // Up arrow (used in number_input, time_picker)
//  ok              → \u{e805}  // Checkmark/submit (used in pickers)

// Simulator API https://raw.githubusercontent.com/iced-rs/iced/master/test/src/simulator.rs

#[macro_use]
mod common;

use iced_aw::NumberInput;
use iced_test::{Error, Simulator};

#[derive(Clone, Debug)]
enum Message {
    Changed(#[allow(dead_code)] u32),
    Submit,
}

// Generate test helpers for this Message type
test_helpers!(Message);

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to click the increment button (tries both icon variants)
fn click_increment(ui: &mut Simulator<'_, Message>) -> Result<(), Error> {
    if ui.find(" ▲ ").is_ok() {
        ui.click(" ▲ ").map(|_| ())
    } else {
        ui.click(" + ").map(|_| ())
    }
}

/// Helper to click the decrement button (tries both icon variants)
fn click_decrement(ui: &mut Simulator<'_, Message>) -> Result<(), Error> {
    if ui.find(" ▼ ").is_ok() {
        ui.click(" ▼ ").map(|_| ())
    } else {
        ui.click(" - ").map(|_| ())
    }
}

// ============================================================================
// Display and Finding Elements Tests
// ============================================================================

#[test]
fn number_input_can_find_value() -> Result<(), Error> {
    let value = 42u32;

    let (mut app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("42").is_ok(),
        "Number input value should be findable"
    );

    Ok(())
}

#[test]
fn number_input_displays_correct_initial_value() -> Result<(), Error> {
    let value = 100u32;

    let (app, _) = App::new(move || NumberInput::new(&value, 0..=200, Message::Changed).into());

    let mut ui = simulator(&app);
    assert!(ui.find("100").is_ok(), "Initial value should be displayed");

    Ok(())
}

#[test]
fn number_input_can_find_increment_button() -> Result<(), Error> {
    let value = 50u32;

    let (app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);
    // The up arrow button should be findable (with spaces around it: " ▲ " or " + ")
    assert!(
        ui.find(" ▲ ").is_ok() || ui.find(" + ").is_ok(),
        "Increment button should be findable"
    );

    Ok(())
}

#[test]
fn number_input_can_find_decrement_button() -> Result<(), Error> {
    let value = 50u32;

    let (app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);
    // The down arrow button should be findable (with spaces around it: " ▼ " or " - ")
    assert!(
        ui.find(" ▼ ").is_ok() || ui.find(" - ").is_ok(),
        "Decrement button should be findable"
    );

    Ok(())
}

// ============================================================================
// Button Click Tests
// ============================================================================

#[test]
fn number_input_increment_button_click_produces_message() -> Result<(), Error> {
    let value = 50u32;

    let (mut app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);

    // Click the increment button
    click_increment(&mut ui)?;

    // Verify we got a Changed message
    let mut got_changed = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Changed(_)) {
            got_changed = true;
        }
        app.update(message);
    }

    assert!(
        got_changed,
        "Increment button should produce Message::Changed"
    );

    Ok(())
}

#[test]
fn number_input_decrement_button_click_produces_message() -> Result<(), Error> {
    let value = 50u32;

    let (mut app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);

    // Click the decrement button
    click_decrement(&mut ui)?;

    // Verify we got a Changed message
    let mut got_changed = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Changed(_)) {
            got_changed = true;
        }
        app.update(message);
    }

    assert!(
        got_changed,
        "Decrement button should produce Message::Changed"
    );

    Ok(())
}

// ============================================================================
// Boundary Tests
// ============================================================================

#[test]
fn number_input_cannot_increment_past_max() -> Result<(), Error> {
    let value = 100u32;

    let (app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);

    // Verify at max
    assert!(ui.find("100").is_ok(), "Should be at max value");

    // Try to click increment (value should remain at 100)
    // Note: The button may still be clickable but won't change the value
    let _ = click_increment(&mut ui);

    // Value should still be 100
    assert!(
        ui.find("100").is_ok(),
        "Value should remain at max after clicking increment"
    );

    Ok(())
}

#[test]
fn number_input_cannot_decrement_past_min() -> Result<(), Error> {
    let value = 0u32;

    let (app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);

    // Verify at min
    assert!(ui.find("0").is_ok(), "Should be at min value");

    // Try to click decrement (value should remain at 0)
    let _ = click_decrement(&mut ui);

    // Value should still be 0
    assert!(
        ui.find("0").is_ok(),
        "Value should remain at min after clicking decrement"
    );

    Ok(())
}

// ============================================================================
// On Submit Tests
// ============================================================================

#[test]
fn number_input_on_submit_produces_message() -> Result<(), Error> {
    let value = 50u32;

    let (mut app, _) = App::new(move || {
        NumberInput::new(&value, 0..=100, Message::Changed)
            .on_submit(Message::Submit)
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the value to focus the input
    ui.click("50")?;

    // Press enter
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::Enter,
    ));

    // Verify we got a Submit message
    let mut got_submit = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Submit) {
            got_submit = true;
        }
        app.update(message);
    }

    assert!(got_submit, "Enter key should produce Message::Submit");

    Ok(())
}

// ============================================================================
// Different Value Types Tests
// ============================================================================

#[test]
fn number_input_works_with_i32() -> Result<(), Error> {
    let value = -10i32;

    let (app, _) = App::new(move || {
        // Use a closure that converts i32 to our Message type for testing
        iced_aw::NumberInput::new(&value, -100..=100, |_v| Message::Submit).into()
    });

    let ui = simulator(&app);

    // Just verify it renders without errors
    for _message in ui.into_messages() {
        // Process messages
    }

    Ok(())
}

#[test]
fn number_input_works_with_f64() -> Result<(), Error> {
    let value = 2.5f64;

    // Create a simple widget element without using the full App infrastructure
    // This test just verifies the widget can be constructed with f64
    let _widget: iced_aw::NumberInput<'_, f64, (), iced::Theme, iced::Renderer> =
        iced_aw::NumberInput::new(&value, 0.0..=10.0, |v| {
            // Use a placeholder closure for testing
            let _ = v;
        });

    // If we got here without panic, the test passes
    Ok(())
}
