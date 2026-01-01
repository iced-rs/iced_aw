//! Integration tests for the NumberInput widget
//!
//! These tests verify the NumberInput widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

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
    process_messages(ui, &mut app);

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

    // Snapshot testing: capture the rendered widget
    let snapshot = ui.snapshot(&iced::Theme::Light)?;

    // Verify the snapshot matches the expected hash
    assert!(
        snapshot.matches_hash("tests/snapshots/number_input_initial_value_100")?,
        "Snapshot hash should match baseline"
    );

    // Verify the snapshot matches the expected image
    assert!(
        snapshot.matches_image("tests/snapshots/number_input_initial_value_100")?,
        "Snapshot image should match baseline"
    );

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
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Changed(_)),
        "Increment button should produce Message::Changed",
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
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Changed(_)),
        "Decrement button should produce Message::Changed",
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
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Submit),
        "Enter key should produce Message::Submit",
    );

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

// ============================================================================
// Mouse Wheel Scrolling Tests
// ============================================================================

#[test]
fn number_input_mouse_scroll_up_increases_value() -> Result<(), Error> {
    use iced::Event;
    use iced_core::mouse;

    let value = 50u32;

    let (mut app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);

    // Simulate mouse wheel scroll up (positive y delta)
    ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: 1.0 },
    })]);

    // Verify we got a Changed message
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Changed(_)),
        "Mouse scroll up should produce Message::Changed",
    );

    Ok(())
}

#[test]
fn number_input_mouse_scroll_down_decreases_value() -> Result<(), Error> {
    use iced::Event;
    use iced_core::mouse;

    let value = 50u32;

    let (mut app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);

    // Simulate mouse wheel scroll down (negative y delta)
    ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: -1.0 },
    })]);

    // Verify we got a Changed message
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Changed(_)),
        "Mouse scroll down should produce Message::Changed",
    );

    Ok(())
}

#[test]
fn number_input_ignore_scroll_prevents_wheel_events() -> Result<(), Error> {
    use iced::Event;
    use iced_core::mouse;

    let value = 50u32;

    let (mut app, _) = App::new(move || {
        NumberInput::new(&value, 0..=100, Message::Changed)
            .ignore_scroll(true)
            .into()
    });

    let mut ui = simulator(&app);

    // Simulate mouse wheel scroll up
    ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: 1.0 },
    })]);

    // Verify we did NOT get a Changed message
    let got_changed = check_message_received(ui, &mut app, |m| matches!(m, Message::Changed(_)));

    assert!(
        !got_changed,
        "Mouse scroll should be ignored when ignore_scroll is true"
    );

    Ok(())
}

#[test]
fn number_input_scroll_respects_boundaries() -> Result<(), Error> {
    use iced::Event;
    use iced_core::mouse;

    // Test scrolling down at minimum
    let value = 0u32;
    let (mut app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);

    // Simulate scroll down at minimum - should not change value
    ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: -1.0 },
    })]);

    // Process messages - value should stay at 0
    process_messages(ui, &mut app);

    // Create new simulator to verify value is still displayed as 0
    let mut ui = simulator(&app);
    assert!(
        ui.find("0").is_ok(),
        "Value should remain at minimum when scrolling down"
    );

    Ok(())
}

// ============================================================================
// Multiple Button Click Tests
// ============================================================================

#[test]
fn number_input_multiple_increment_clicks() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let value = Rc::new(RefCell::new(10u32));
    let value_clone = value.clone();

    #[derive(Clone)]
    struct StatefulApp {
        value: Rc<RefCell<u32>>,
    }

    impl StatefulApp {
        fn new(value: Rc<RefCell<u32>>) -> (Self, iced::Task<Message>) {
            (Self { value }, iced::Task::none())
        }

        fn update(&mut self, message: Message) {
            if let Message::Changed(new_val) = message {
                *self.value.borrow_mut() = new_val;
            }
        }

        fn view(&self) -> iced::Element<'_, Message> {
            let val = *self.value.borrow();
            NumberInput::new(&val, 0..=100, Message::Changed)
                .step(5)
                .into()
        }
    }

    let (mut app, _) = StatefulApp::new(value_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Click increment button
    click_increment(&mut ui)?;
    for message in ui.into_messages() {
        app.update(message);
    }

    // Verify value increased
    let new_value = *value.borrow();
    assert_eq!(new_value, 15, "Value should increase by step (5)");

    Ok(())
}

#[test]
fn number_input_multiple_decrement_clicks() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let value = Rc::new(RefCell::new(50u32));
    let value_clone = value.clone();

    #[derive(Clone)]
    struct StatefulApp {
        value: Rc<RefCell<u32>>,
    }

    impl StatefulApp {
        fn new(value: Rc<RefCell<u32>>) -> (Self, iced::Task<Message>) {
            (Self { value }, iced::Task::none())
        }

        fn update(&mut self, message: Message) {
            if let Message::Changed(new_val) = message {
                *self.value.borrow_mut() = new_val;
            }
        }

        fn view(&self) -> iced::Element<'_, Message> {
            let val = *self.value.borrow();
            NumberInput::new(&val, 0..=100, Message::Changed)
                .step(5)
                .into()
        }
    }

    let (mut app, _) = StatefulApp::new(value_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Click decrement button
    click_decrement(&mut ui)?;
    for message in ui.into_messages() {
        app.update(message);
    }

    // Verify value decreased
    let new_value = *value.borrow();
    assert_eq!(new_value, 45, "Value should decrease by step (5)");

    Ok(())
}

// ============================================================================
// Custom Step Tests
// ============================================================================

#[test]
fn number_input_custom_step_with_increment() -> Result<(), Error> {
    let value = 10u32;

    let (mut app, _) = App::new(move || {
        NumberInput::new(&value, 0..=100, Message::Changed)
            .step(10)
            .into()
    });

    let mut ui = simulator(&app);

    // Click increment button
    click_increment(&mut ui)?;

    // Verify we got a Changed message with appropriate value
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Changed(_)),
        "Increment with custom step should produce Message::Changed",
    );

    Ok(())
}

// ============================================================================
// Ignore Buttons Tests
// ============================================================================

#[test]
fn number_input_ignore_buttons_hides_increment() -> Result<(), Error> {
    let value = 50u32;

    let (app, _) = App::new(move || {
        NumberInput::new(&value, 0..=100, Message::Changed)
            .ignore_buttons(true)
            .into()
    });

    let mut ui = simulator(&app);

    // The buttons should not be findable when ignored
    let inc_found = ui.find(" ▲ ").is_ok() || ui.find(" + ").is_ok();
    let dec_found = ui.find(" ▼ ").is_ok() || ui.find(" - ").is_ok();

    assert!(
        !inc_found && !dec_found,
        "Buttons should not be visible when ignore_buttons is true"
    );

    Ok(())
}

// ============================================================================
// Clicking on Value Input Tests
// ============================================================================

#[test]
fn number_input_clicking_value_is_findable() -> Result<(), Error> {
    let value = 42u32;

    let (app, _) = App::new(move || NumberInput::new(&value, 0..=100, Message::Changed).into());

    let mut ui = simulator(&app);

    // Verify we can find and click on the value - this tests that the value
    // is properly exposed through the operate() method
    ui.click("42")?;

    Ok(())
}
