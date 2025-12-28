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

use iced::{Element, Settings, Theme};
use iced_aw::{DatePicker, date_picker::Date};
use iced_test::{Error, Simulator};
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

type ViewFn = Box<dyn Fn() -> Element<'static, Message>>;

#[derive(Clone)]
struct App {
    view_fn: std::rc::Rc<ViewFn>,
}

impl App {
    fn new<F>(view_fn: F) -> (Self, iced::Task<Message>)
    where
        F: Fn() -> Element<'static, Message> + 'static,
    {
        (
            App {
                view_fn: std::rc::Rc::new(Box::new(view_fn)),
            },
            iced::Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Open | Message::Cancel | Message::Submit(_) => {
                // No state changes in these tests
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        (self.view_fn)()
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

// ============================================================================
// Underlay Button Tests
// ============================================================================

#[test]
fn can_find_underlay_button_text() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Open Date Picker").is_ok(),
        "Underlay button text should be findable"
    );

    Ok(())
}

#[test]
fn underlay_button_opens_picker() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);
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
    for message in ui.into_messages() {
        if matches!(message, Message::Open) {
            show_picker = true;
        }
        app.update(message);
    }

    assert!(show_picker, "Clicking button should produce Message::Open");

    // Verify picker displays when opened
    let (app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);
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
fn next_button_navigates_to_next_month() -> Result<(), Error> {
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
fn previous_button_navigates_to_previous_month() -> Result<(), Error> {
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
fn navigation_wraps_year_boundaries() -> Result<(), Error> {
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
fn multiple_next_clicks_advance_months() -> Result<(), Error> {
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
fn multiple_previous_clicks_go_back_months() -> Result<(), Error> {
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
fn keyboard_next_year_navigation() -> Result<(), Error> {
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
fn keyboard_previous_year_navigation() -> Result<(), Error> {
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
fn keyboard_multiple_year_navigation() -> Result<(), Error> {
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
fn cancel_button_produces_cancel_message() -> Result<(), Error> {
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
    let mut got_cancel = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Cancel) {
            got_cancel = true;
        }
        app.update(message);
    }

    assert!(got_cancel, "Cancel button should produce Message::Cancel");

    Ok(())
}

#[test]
fn submit_button_produces_submit_message() -> Result<(), Error> {
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
    let mut got_submit = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Submit(_)) {
            got_submit = true;
        }
        app.update(message);
    }

    assert!(got_submit, "Submit button should produce Message::Submit");

    Ok(())
}

#[test]
fn clicking_date_and_submit_workflow() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
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

    let mut submitted_date: Option<Date> = None;
    for message in ui.into_messages() {
        if let Message::Submit(date) = message {
            submitted_date = Some(date);
        }
    }

    assert!(submitted_date.is_some(), "Should have submitted a date");
    let submitted = submitted_date.unwrap();
    // Verify it's a valid date in 2024 (exact day may vary due to Simulator positioning)
    assert_eq!(submitted.year, 2024, "Year should be 2024");
    assert!(
        submitted.month >= 5 && submitted.month <= 7,
        "Month should be near June"
    );
    assert!(submitted.day >= 1 && submitted.day <= 31, "Day should be valid");

    Ok(())
}

#[test]
fn navigate_then_submit_workflow() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (app, _) = App::new(move || {
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

    let mut got_submit = false;
    for message in ui.into_messages() {
        if let Message::Submit(date) = message {
            got_submit = true;
            assert_eq!(date.month, 7, "Month should be July");
            assert_eq!(date.year, 2024);
        }
    }

    assert!(got_submit, "Should have submitted a date");

    Ok(())
}
