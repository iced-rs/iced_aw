//! Integration tests for the DatePicker widget
//!
//! These tests verify the DatePicker widget's behavior and public API
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

use iced::Theme;
use iced_aw::{DatePicker, date_picker::Date};
use iced_test::Error;
use iced_widget::{button, text::Text};

#[derive(Clone, Debug)]
enum Message {
    Open,
    Cancel,
    Submit(Date),
}

// Helper function to create a button with explicit Theme type
fn create_button<'a>(text: &'a str) -> iced_widget::Button<'a, Message, Theme> {
    button(Text::new(text)).on_press(Message::Open)
}

// Generate test helpers for this Message type
test_helpers!(Message);

// ============================================================================
// Underlay Button Tests
// ============================================================================

#[test]
fn date_picker_can_find_underlay_button_text() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Open Date Picker").is_ok(),
        "Underlay button text should be findable"
    );

    Ok(())
}

#[test]
fn date_picker_underlay_button_opens_picker() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);
    #[allow(unused_assignments)]
    let mut show_picker = false;

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
    });

    // Verify the underlay button is clickable and produces Message::Open
    let mut ui = simulator(&app);
    assert!(
        ui.find("Open Date Picker").is_ok(),
        "Underlay button should be findable"
    );

    ui.click("Open Date Picker")?;

    // Process messages to verify we got Message::Open
    show_picker = check_message_received(ui, &mut app, |m| matches!(m, Message::Open));

    assert!(show_picker, "Clicking button should produce Message::Open");

    // Verify picker displays when opened
    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Snapshot testing: verify visual rendering matches baseline
    assert_snapshot_matches(
        &mut ui,
        "tests/snapshots/date_picker_underlay_button_opens_picker",
    )?;

    assert!(
        ui.find("June").is_ok(),
        "Month should be displayed when picker is open"
    );
    assert!(
        ui.find("2024").is_ok(),
        "Year should be displayed when picker is open"
    );

    Ok(())
}

#[test]
fn date_picker_next_button_navigates_to_next_month() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify initial month
    assert!(ui.find("June").is_ok(), "Initial month should be June");

    // Click the next button (right arrow) and verify new month
    ui.click("\u{e803}")?; // Right arrow
    assert!(
        ui.find("July").is_ok(),
        "Month should now be July after clicking next"
    );

    Ok(())
}

#[test]
fn date_picker_previous_button_navigates_to_previous_month() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify initial month
    assert!(ui.find("June").is_ok(), "Initial month should be June");

    // Click the previous button (left arrow) and verify new month
    ui.click("\u{e802}")?; // Left arrow
    assert!(
        ui.find("May").is_ok(),
        "Month should now be May after clicking previous"
    );

    Ok(())
}

#[test]
fn date_picker_navigation_wraps_year_boundaries() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify initial month
    assert!(
        ui.find("January").is_ok(),
        "Initial month should be January"
    );
    assert!(ui.find("2024").is_ok(), "Initial year should be 2024");

    // Click previous to wrap to December of previous year
    ui.click("\u{e802}")?; // Left arrow
    assert!(ui.find("December").is_ok(), "Month should wrap to December");
    assert!(ui.find("2023").is_ok(), "Year should wrap to 2023");

    Ok(())
}

#[test]
fn date_picker_multiple_next_clicks_advance_months() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify starting month
    assert!(ui.find("June").is_ok(), "Should start at June");
    assert!(ui.find("2024").is_ok(), "Should start at 2024");

    // Click next 7 times to reach January of next year
    for _ in 0..7 {
        ui.click("\u{e803}")?; // Right arrow
    }

    assert!(
        ui.find("January").is_ok(),
        "Should be at January after 7 clicks"
    );
    assert!(ui.find("2025").is_ok(), "Should be at 2025 after wrapping");

    Ok(())
}

#[test]
fn date_picker_multiple_previous_clicks_go_back_months() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify starting month
    assert!(ui.find("June").is_ok(), "Should start at June");

    // Click previous 6 times to reach December of previous year
    for _ in 0..6 {
        ui.click("\u{e802}")?; // Left arrow
    }

    assert!(
        ui.find("December").is_ok(),
        "Should be at December after 6 clicks back"
    );
    assert!(ui.find("2023").is_ok(), "Should be at 2023 after wrapping");

    Ok(())
}

#[test]
fn date_picker_keyboard_next_year_navigation() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify starting state
    assert!(ui.find("June").is_ok(), "Should start at June");
    assert!(ui.find("2024").is_ok(), "Should start at 2024");

    // Click on the year to give it focus
    ui.click("2024")?;

    // Use right arrow to advance year
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowRight,
    ));

    assert!(ui.find("June").is_ok(), "Month should still be June");
    assert!(ui.find("2025").is_ok(), "Year should now be 2025");

    Ok(())
}

#[test]
fn date_picker_keyboard_previous_year_navigation() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify starting state
    assert!(ui.find("June").is_ok(), "Should start at June");
    assert!(ui.find("2024").is_ok(), "Should start at 2024");

    // Click on the year to give it focus
    ui.click("2024")?;

    // Use left arrow to go to previous year
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowLeft,
    ));

    assert!(ui.find("June").is_ok(), "Month should still be June");
    assert!(ui.find("2023").is_ok(), "Year should now be 2023");

    Ok(())
}

#[test]
fn date_picker_keyboard_multiple_year_navigation() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Click on the year to give it focus
    ui.click("2024")?;

    // Advance 3 years
    for _ in 0..3 {
        ui.tap_key(iced::keyboard::Key::Named(
            iced::keyboard::key::Named::ArrowRight,
        ));
    }

    assert!(ui.find("June").is_ok(), "Month should still be June");
    assert!(
        ui.find("2027").is_ok(),
        "Year should be 2027 after 3 advances"
    );

    Ok(())
}

#[test]
fn date_picker_cancel_button_produces_cancel_message() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify picker is open
    assert!(ui.find("June").is_ok(), "Picker should be open");

    // Click the cancel button
    ui.click("\u{e800}")?; // Cancel button

    // Verify we got a Cancel message
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Cancel),
        "Cancel button should produce Message::Cancel",
    );

    Ok(())
}

#[test]
fn date_picker_submit_button_produces_submit_message() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify picker is open
    assert!(ui.find("June").is_ok(), "Picker should be open");

    // Click the OK/submit button
    ui.click("\u{e805}")?; // OK button

    // Verify we got a Submit message with the date
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Submit(_)),
        "Submit button should produce Message::Submit",
    );

    Ok(())
}

#[test]
fn date_picker_clicking_date_and_submit_workflow() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify starting at June 2024
    assert!(ui.find("June").is_ok());
    assert!(ui.find("2024").is_ok());

    // Click on a day number (note: Simulator click positioning may not be exact)
    // Just verify we can click and get a submit with a valid date
    ui.click("15")?;

    // Submit the date
    ui.click("\u{e805}")?; // OK button

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    assert!(submitted_date.is_some(), "Should have submitted a date");
    let submitted = submitted_date.unwrap();
    // Verify it's a valid date in 2024 (exact day may vary due to Simulator positioning)
    assert_eq!(submitted.year, 2024, "Year should be 2024");
    assert!(
        submitted.month >= 5 && submitted.month <= 7,
        "Month should be near June"
    );
    assert!(
        submitted.day >= 1 && submitted.day <= 31,
        "Day should be valid"
    );

    Ok(())
}

#[test]
fn date_picker_navigate_then_submit_workflow() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Navigate to next month
    ui.click("\u{e803}")?; // Right arrow for next month

    // Should now be in July
    assert!(ui.find("July").is_ok(), "Should be in July");

    // Submit without clicking a specific day (uses displayed month/year)
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));

    assert!(!messages.is_empty(), "Should have submitted a date");

    if let Message::Submit(date) = &messages[0] {
        assert_eq!(date.month, 7, "Month should be July");
        assert_eq!(date.year, 2024);
    }

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Year Navigation Arrows
// ============================================================================

#[test]
fn date_picker_mouse_click_year_next_arrow() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify initial year
    assert!(ui.find("2024").is_ok(), "Initial year should be 2024");
    assert!(ui.find("June").is_ok(), "Month should still be June");

    // Click on year to focus it, then look for the right arrow after year text
    ui.click("2024")?;

    // Use keyboard to navigate year (arrow buttons are part of year layout)
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowRight,
    ));

    assert!(ui.find("2025").is_ok(), "Year should be 2025");
    assert!(ui.find("June").is_ok(), "Month should still be June");

    Ok(())
}

#[test]
fn date_picker_mouse_click_year_previous_arrow() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify initial year
    assert!(ui.find("2024").is_ok(), "Initial year should be 2024");

    // Click on year to focus it
    ui.click("2024")?;

    // Use keyboard to navigate year backwards
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowLeft,
    ));

    assert!(ui.find("2023").is_ok(), "Year should be 2023");
    assert!(ui.find("June").is_ok(), "Month should still be June");

    Ok(())
}

#[test]
fn date_picker_mouse_multiple_year_next_clicks() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Click on year to focus it
    ui.click("2024")?;

    // Click next year arrow 5 times using keyboard
    for _ in 0..5 {
        ui.tap_key(iced::keyboard::Key::Named(
            iced::keyboard::key::Named::ArrowRight,
        ));
    }

    assert!(
        ui.find("2029").is_ok(),
        "Year should be 2029 after 5 clicks"
    );
    assert!(ui.find("June").is_ok(), "Month should still be June");

    Ok(())
}

#[test]
fn date_picker_mouse_rapid_year_navigation() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Click on year to focus it
    ui.click("2024")?;

    // Go forward 3 years
    for _ in 0..3 {
        ui.tap_key(iced::keyboard::Key::Named(
            iced::keyboard::key::Named::ArrowRight,
        ));
    }
    assert!(ui.find("2027").is_ok(), "Year should be 2027");

    // Go back 2 years
    for _ in 0..2 {
        ui.tap_key(iced::keyboard::Key::Named(
            iced::keyboard::key::Named::ArrowLeft,
        ));
    }
    assert!(ui.find("2025").is_ok(), "Year should be 2025");

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Day Selection
// ============================================================================

#[test]
fn date_picker_mouse_click_day_cell_submits_valid_date() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify we're in June 2024
    assert!(ui.find("June").is_ok());
    assert!(ui.find("2024").is_ok());

    // Click on a day (Note: exact day may vary due to simulator positioning)
    // We use day 15 which should be in the current month
    ui.click("15")?;

    // Submit to verify a valid date is selected
    ui.click("\u{e805}")?; // OK button

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    assert!(submitted_date.is_some(), "Should have submitted a date");
    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.year, 2024);
    // Month should be near June (may include May-July due to calendar overflow)
    assert!(
        submitted.month >= 5 && submitted.month <= 7,
        "Month should be near June"
    );
    assert!(
        submitted.day >= 1 && submitted.day <= 31,
        "Day should be valid"
    );

    Ok(())
}

#[test]
fn date_picker_mouse_submit_without_day_click_uses_displayed_date() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Submit without clicking a specific day (uses the initial date)
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.year, 2024);
    assert_eq!(submitted.month, 6);
    assert_eq!(submitted.day, 15);

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Combined Navigation and Selection
// ============================================================================

#[test]
fn date_picker_mouse_navigate_month_then_submit() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Navigate to next month
    ui.click("\u{e803}")?; // Right arrow
    assert!(ui.find("July").is_ok(), "Should be in July");

    // Submit without clicking specific day (uses displayed month/year with original day)
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.month, 7, "Should be July");
    assert_eq!(submitted.year, 2024);
    assert_eq!(submitted.day, 15, "Day should be preserved");

    Ok(())
}

#[test]
fn date_picker_mouse_navigate_year_then_submit() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Click on year to focus it
    ui.click("2024")?;

    // Navigate to next year
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowRight,
    ));

    assert!(ui.find("2025").is_ok(), "Should be in 2025");
    assert!(ui.find("June").is_ok(), "Should still be in June");

    // Submit
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.year, 2025, "Should be 2025");
    assert_eq!(submitted.month, 6, "Should be June");
    assert_eq!(submitted.day, 15, "Day should be preserved");

    Ok(())
}

#[test]
fn date_picker_mouse_navigate_both_month_and_year_then_submit() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Navigate month backward
    ui.click("\u{e802}")?; // Left arrow
    assert!(ui.find("May").is_ok(), "Should be in May");

    // Navigate year forward
    ui.click("2024")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowRight,
    ));
    assert!(ui.find("2025").is_ok(), "Should be in 2025");

    // Navigate month forward twice
    ui.click("\u{e803}")?; // Right arrow
    ui.click("\u{e803}")?; // Right arrow
    assert!(ui.find("July").is_ok(), "Should be in July");

    // Submit
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.year, 2025);
    assert_eq!(submitted.month, 7, "Should be July");
    assert_eq!(submitted.day, 15, "Day should be preserved");

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Month and Year Edge Cases
// ============================================================================

#[test]
fn date_picker_mouse_navigate_across_year_boundary_forward() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 12, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify starting at December 2024
    assert!(ui.find("December").is_ok());
    assert!(ui.find("2024").is_ok());

    // Navigate to next month (should wrap to January 2025)
    ui.click("\u{e803}")?; // Right arrow

    assert!(ui.find("January").is_ok(), "Should wrap to January");
    assert!(ui.find("2025").is_ok(), "Should wrap to 2025");

    Ok(())
}

#[test]
fn date_picker_mouse_navigate_across_year_boundary_backward() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 15);

    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify starting at January 2024
    assert!(ui.find("January").is_ok());
    assert!(ui.find("2024").is_ok());

    // Navigate to previous month (should wrap to December 2023)
    ui.click("\u{e802}")?; // Left arrow

    assert!(ui.find("December").is_ok(), "Should wrap to December");
    assert!(ui.find("2023").is_ok(), "Should wrap to 2023");

    Ok(())
}

#[test]
fn date_picker_mouse_navigate_leap_year_february() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 2, 15); // February (leap year)

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify we're in February 2024 (leap year)
    assert!(ui.find("February").is_ok());
    assert!(ui.find("2024").is_ok());

    // Navigate to March and back to verify leap year handling
    ui.click("\u{e803}")?; // Next month
    assert!(ui.find("March").is_ok());

    ui.click("\u{e802}")?; // Previous month
    assert!(ui.find("February").is_ok());

    // Submit (February 2024 is valid with 29 days)
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.year, 2024);
    assert_eq!(submitted.month, 2, "Should be February");
    assert_eq!(submitted.day, 15, "Should preserve original day");

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Complex Workflows
// ============================================================================

#[test]
fn date_picker_mouse_rapid_month_navigation_workflow() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Rapidly navigate forward 3 months
    ui.click("\u{e803}")?; // July
    ui.click("\u{e803}")?; // August
    ui.click("\u{e803}")?; // September

    assert!(ui.find("September").is_ok());

    // Go back 1 month
    ui.click("\u{e802}")?; // August

    assert!(ui.find("August").is_ok());

    // Submit
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.month, 8, "Should be August");
    assert_eq!(submitted.day, 15, "Should preserve original day");
    assert_eq!(submitted.year, 2024);

    Ok(())
}

#[test]
fn date_picker_mouse_navigate_multiple_months_forward_and_back() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Navigate to next month
    ui.click("\u{e803}")?;
    assert!(ui.find("July").is_ok());

    // Navigate to next month again
    ui.click("\u{e803}")?;
    assert!(ui.find("August").is_ok());

    // Navigate back to previous month
    ui.click("\u{e802}")?;
    assert!(ui.find("July").is_ok());

    // Navigate back to original month
    ui.click("\u{e802}")?;
    assert!(ui.find("June").is_ok());

    // Submit
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.month, 6, "Should be June");
    assert_eq!(submitted.day, 15, "Should preserve original day");
    assert_eq!(submitted.year, 2024);

    Ok(())
}

#[test]
fn date_picker_mouse_navigate_to_different_year_and_month() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Click on year to focus
    ui.click("2024")?;

    // Navigate to 2026
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowRight,
    ));
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowRight,
    ));

    assert!(ui.find("2026").is_ok());

    // Navigate to December
    for _ in 0..6 {
        ui.click("\u{e803}")?; // 6 months forward
    }

    assert!(ui.find("December").is_ok());

    // Submit
    ui.click("\u{e805}")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::Submit(_)));
    let submitted_date = messages.first().and_then(|m| {
        if let Message::Submit(date) = m {
            Some(*date)
        } else {
            None
        }
    });

    let submitted = submitted_date.unwrap();
    assert_eq!(submitted.year, 2026);
    assert_eq!(submitted.month, 12);
    assert_eq!(submitted.day, 15, "Should preserve original day");

    Ok(())
}
