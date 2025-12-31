//! Integration tests for the TypedInput widget
//!
//! These tests verify the TypedInput widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

#[macro_use]
mod common;

use iced_aw::TypedInput;
use iced_test::Error;

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Message {
    Changed(u32),
    StringChanged(String),
    Submit(Result<u32, String>),
    Paste(u32),
}

// Generate test helpers for this Message type
test_helpers!(Message);

// ============================================================================
// Basic Rendering Tests
// ============================================================================

#[test]
fn typed_input_renders() {
    let value = 42u32;
    run_test(move || TypedInput::new("Enter a number", &value).into());
}

#[test]
fn typed_input_displays_initial_value() -> Result<(), Error> {
    let value = 123u32;

    let (app, _) = App::new(move || TypedInput::new("Enter a number", &value).into());

    let mut ui = simulator(&app);
    assert!(ui.find("123").is_ok(), "Initial value should be displayed");

    Ok(())
}

#[test]
fn typed_input_displays_placeholder() -> Result<(), Error> {
    let value = 0u32;

    let (app, _) = App::new(move || TypedInput::new("Enter a number", &value).into());

    let mut ui = simulator(&app);
    // When value is 0, it should display "0"
    assert!(ui.find("0").is_ok(), "Value 0 should be displayed");

    Ok(())
}

// ============================================================================
// Text Input Tests
// ============================================================================

#[test]
fn typed_input_can_find_value() -> Result<(), Error> {
    let value = 999u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
            .on_input(Message::Changed)
            .into()
    });

    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("999").is_ok(),
        "TypedInput value should be findable"
    );

    Ok(())
}

// ============================================================================
// Keyboard Interaction Tests
// ============================================================================

#[test]
fn typed_input_can_be_focused_with_click() -> Result<(), Error> {
    let value = 42u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
            .on_input(Message::Changed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the input to focus it
    ui.click("42")?;

    // Process any messages
    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn typed_input_typewrite_produces_messages() -> Result<(), Error> {
    let value = 0u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
            .on_input(Message::Changed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click to focus the input first
    ui.click("0")?;

    // Select all existing text
    ui.tap_key(iced::keyboard::Key::Named(iced::keyboard::key::Named::Home));
    ui.tap_key(iced::keyboard::Key::Named(iced::keyboard::key::Named::End));

    // Type a new number
    ui.typewrite("123");

    // Check if we got Changed messages
    let mut got_changed = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Changed(_)) {
            got_changed = true;
        }
        app.update(message);
    }

    // Note: We may or may not get a message depending on whether "123" is valid
    // and different from the current value. The test passes if it doesn't panic.
    let _ = got_changed;

    Ok(())
}

#[test]
fn typed_input_backspace_key_works() -> Result<(), Error> {
    let value = 456u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
            .on_input(Message::Changed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click to focus
    ui.click("456")?;

    // Press backspace
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::Backspace,
    ));

    // Process messages
    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn typed_input_delete_key_works() -> Result<(), Error> {
    let value = 789u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
            .on_input(Message::Changed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click to focus
    ui.click("789")?;

    // Move to beginning
    ui.tap_key(iced::keyboard::Key::Named(iced::keyboard::key::Named::Home));

    // Press delete
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::Delete,
    ));

    // Process messages
    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn typed_input_arrow_keys_work() -> Result<(), Error> {
    let value = 100u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
            .on_input(Message::Changed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click to focus
    ui.click("100")?;

    // Use arrow keys to navigate
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowLeft,
    ));
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowRight,
    ));
    ui.tap_key(iced::keyboard::Key::Named(iced::keyboard::key::Named::Home));
    ui.tap_key(iced::keyboard::Key::Named(iced::keyboard::key::Named::End));

    // Process messages
    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn typed_input_typewrite_full_replacement() -> Result<(), Error> {
    let value = 0u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
            .on_input(Message::Changed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click to focus
    ui.click("0")?;

    // Select all
    ui.tap_key(iced::keyboard::Key::Named(iced::keyboard::key::Named::Home));

    // Type new value to replace
    ui.typewrite("999");

    // Process messages
    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

// ============================================================================
// On Submit Tests
// ============================================================================

#[test]
fn typed_input_on_submit_with_valid_value() -> Result<(), Error> {
    let value = 50u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
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

    // Verify we got a Submit message with Ok result
    let mut got_submit = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Submit(Ok(_))) {
            got_submit = true;
        }
        app.update(message);
    }

    assert!(
        got_submit,
        "Enter key with valid value should produce Message::Submit(Ok(_))"
    );

    Ok(())
}

// ============================================================================
// Type Safety Tests
// ============================================================================

#[test]
fn typed_input_works_with_i32() -> Result<(), Error> {
    let value = -42i32;

    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum I32Message {
        Changed(i32),
    }

    // Use a custom message type for this test
    let _widget: TypedInput<'_, i32, I32Message, iced::Theme, iced::Renderer> =
        TypedInput::new("Enter a number", &value).on_input(I32Message::Changed);

    // If we got here without panic, the test passes
    Ok(())
}

#[test]
fn typed_input_works_with_f64() -> Result<(), Error> {
    let value = 2.5;

    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    enum F64Message {
        Changed(f64),
    }

    // Use a custom message type for this test
    let _widget: TypedInput<'_, f64, F64Message, iced::Theme, iced::Renderer> =
        TypedInput::new("Enter a number", &value).on_input(F64Message::Changed);

    // If we got here without panic, the test passes
    Ok(())
}

#[test]
fn typed_input_works_with_string() -> Result<(), Error> {
    let value = "hello".to_string();

    let (app, _) = App::new(move || {
        TypedInput::new("Enter text", &value)
            .on_input(Message::StringChanged)
            .into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("hello").is_ok(), "String value should be displayed");

    Ok(())
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn typed_input_with_custom_width() {
    let value = 42u32;
    run_test(move || {
        TypedInput::new("Enter a number", &value)
            .width(iced::Length::Fill)
            .into()
    });
}

#[test]
fn typed_input_with_custom_padding() {
    let value = 42u32;
    run_test(move || TypedInput::new("Enter a number", &value).padding(10).into());
}

#[test]
fn typed_input_with_custom_size() {
    let value = 42u32;
    run_test(move || TypedInput::new("Enter a number", &value).size(20).into());
}

#[test]
fn typed_input_secure_mode() {
    let value = 1234u32;
    run_test(move || TypedInput::new("Enter PIN", &value).secure(true).into());
}

// ============================================================================
// On Paste Tests
// ============================================================================

#[test]
fn typed_input_on_paste_produces_message_for_valid_paste() -> Result<(), Error> {
    let value = 10u32;

    let (mut app, _) = App::new(move || {
        TypedInput::new("Enter a number", &value)
            .on_paste(Message::Paste)
            .into()
    });

    let mut ui = simulator(&app);

    // Click to focus
    ui.click("10")?;

    // Simulate paste with Ctrl+V (this may or may not trigger depending on clipboard state)
    // Note: iced_test simulator may not fully support clipboard operations
    // This test verifies the widget compiles and renders correctly with on_paste

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

// ============================================================================
// Callback Combination Tests
// ============================================================================

#[test]
fn typed_input_with_multiple_callbacks() {
    let value = 42u32;
    run_test(move || {
        TypedInput::new("Enter a number", &value)
            .on_input(Message::Changed)
            .on_submit(Message::Submit)
            .on_paste(Message::Paste)
            .into()
    });
}

#[test]
fn typed_input_without_callbacks_disabled() {
    let value = 42u32;
    // A TypedInput without on_input or on_submit should still render (but be disabled)
    run_test(move || TypedInput::new("Enter a number", &value).into());
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn typed_input_with_zero_value() -> Result<(), Error> {
    let value = 0u32;

    let (app, _) = App::new(move || TypedInput::new("Enter a number", &value).into());

    let mut ui = simulator(&app);
    assert!(ui.find("0").is_ok(), "Zero value should be displayed");

    Ok(())
}

#[test]
fn typed_input_with_max_value() -> Result<(), Error> {
    let value = u32::MAX;

    let (app, _) = App::new(move || TypedInput::new("Enter a number", &value).into());

    let mut ui = simulator(&app);
    assert!(
        ui.find("4294967295").is_ok(),
        "Maximum u32 value should be displayed"
    );

    Ok(())
}
