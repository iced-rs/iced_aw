//! Integration tests for the TimePicker widget
//!
//! These tests verify the TimePicker widget's behavior and public API
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
use iced_aw::{
    TimePicker,
    time_picker::{Period, Time},
};
use iced_test::{Error, Simulator};
use iced_widget::{button, text::Text};

#[derive(Clone, Debug)]
enum Message {
    Open,
    Cancel,
    Submit(Time),
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
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(false, time, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Open Time Picker").is_ok(),
        "Underlay button text should be findable"
    );

    Ok(())
}

#[test]
fn underlay_button_opens_picker() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };
    let mut show_picker = false;

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(false, time, button, Message::Cancel, Message::Submit).into()
    });

    // Verify the underlay button is clickable and produces Message::Open
    let mut ui = simulator(&app);
    assert!(
        ui.find("Open Time Picker").is_ok(),
        "Underlay button should be findable"
    );

    ui.click("Open Time Picker")?;

    // Process messages to verify we got Message::Open
    for message in ui.into_messages() {
        if matches!(message, Message::Open) {
            show_picker = true;
        }
        app.update(message);
    }

    assert!(show_picker, "Clicking button should produce Message::Open");

    // Verify picker displays when opened (24h format)
    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);
    assert!(
        ui.find("14").is_ok(),
        "Hour should be displayed when picker is open"
    );
    assert!(
        ui.find("30").is_ok(),
        "Minute should be displayed when picker is open"
    );

    Ok(())
}

// ============================================================================
// Digital Clock Tests (Hour/Minute Navigation)
// ============================================================================

#[test]
fn hour_up_arrow_increments_hour() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify initial hour
    assert!(ui.find("14").is_ok(), "Initial hour should be 14");

    // Click the up arrow for hours
    ui.click("\u{e804}")?; // Up arrow
    assert!(
        ui.find("15").is_ok(),
        "Hour should now be 15 after clicking up"
    );

    Ok(())
}

#[test]
fn hour_down_arrow_decrements_hour() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify initial hour
    assert!(ui.find("14").is_ok(), "Initial hour should be 14");

    // Click the down arrow for hours
    ui.click("\u{e801}")?; // Down arrow
    assert!(
        ui.find("13").is_ok(),
        "Hour should now be 13 after clicking down"
    );

    Ok(())
}

#[test]
fn minute_up_arrow_increments_minute() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify initial minute
    assert!(ui.find("30").is_ok(), "Initial minute should be 30");

    // Click on the minute field to focus it, then use keyboard navigation
    ui.click("30")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowUp,
    ));
    assert!(
        ui.find("31").is_ok(),
        "Minute should now be 31 after keyboard up"
    );

    Ok(())
}

#[test]
fn minute_down_arrow_decrements_minute() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify initial minute
    assert!(ui.find("30").is_ok(), "Initial minute should be 30");

    // Click on the minute field to focus it, then use keyboard navigation
    ui.click("30")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowDown,
    ));
    assert!(
        ui.find("29").is_ok(),
        "Minute should now be 29 after keyboard down"
    );

    Ok(())
}

#[test]
fn time_wraps_at_hour_boundaries() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 23,
        minute: 30,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify initial hour is 23
    assert!(ui.find("23").is_ok(), "Initial hour should be 23");

    // Click up arrow to wrap to 00
    ui.click("\u{e804}")?; // Up arrow
    assert!(ui.find("00").is_ok(), "Hour should wrap to 00");

    Ok(())
}

#[test]
fn time_wraps_at_minute_boundaries() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 59,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify initial minute is 59
    assert!(ui.find("59").is_ok(), "Initial minute should be 59");

    // Click on minute field and use keyboard to wrap to 00
    ui.click("59")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowUp,
    ));
    assert!(ui.find("00").is_ok(), "Minute should wrap to 00");

    Ok(())
}

// ============================================================================
// 12-Hour Format Tests
// ============================================================================

#[test]
fn displays_12h_format_correctly() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 2,
        minute: 30,
        period: Period::Pm,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // In 12h format, should show hour as displayed and PM indicator
    assert!(ui.find("02").is_ok(), "Hour should be displayed as 02");
    assert!(ui.find("30").is_ok(), "Minute should be displayed as 30");
    assert!(ui.find("PM").is_ok(), "PM indicator should be displayed");

    Ok(())
}

#[test]
fn displays_am_indicator_correctly() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 9,
        minute: 15,
        period: Period::Am,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    assert!(ui.find("09").is_ok(), "Hour should be displayed as 09");
    assert!(ui.find("15").is_ok(), "Minute should be displayed as 15");
    assert!(ui.find("AM").is_ok(), "AM indicator should be displayed");

    Ok(())
}

// ============================================================================
// Seconds Tests
// ============================================================================

#[test]
fn displays_seconds_when_enabled() -> Result<(), Error> {
    let time = Time::Hms {
        hour: 14,
        minute: 30,
        second: 45,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .show_seconds()
            .into()
    });

    let mut ui = simulator(&app);

    assert!(ui.find("14").is_ok(), "Hour should be displayed");
    assert!(ui.find("30").is_ok(), "Minute should be displayed");
    assert!(ui.find("45").is_ok(), "Second should be displayed");

    Ok(())
}

#[test]
fn second_up_arrow_increments_second() -> Result<(), Error> {
    let time = Time::Hms {
        hour: 14,
        minute: 30,
        second: 45,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .show_seconds()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify initial second
    assert!(ui.find("45").is_ok(), "Initial second should be 45");

    // Click on the second field to focus it, then use keyboard navigation
    ui.click("45")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowUp,
    ));
    assert!(
        ui.find("46").is_ok(),
        "Second should now be 46 after keyboard up"
    );

    Ok(())
}

#[test]
fn second_down_arrow_decrements_second() -> Result<(), Error> {
    let time = Time::Hms {
        hour: 14,
        minute: 30,
        second: 45,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .show_seconds()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify initial second
    assert!(ui.find("45").is_ok(), "Initial second should be 45");

    // Click on the second field to focus it, then use keyboard navigation
    ui.click("45")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowDown,
    ));
    assert!(
        ui.find("44").is_ok(),
        "Second should now be 44 after keyboard down"
    );

    Ok(())
}

// ============================================================================
// Button Tests (Cancel and Submit)
// ============================================================================

#[test]
fn cancel_button_produces_cancel_message() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify picker is open
    assert!(ui.find("14").is_ok(), "Picker should be open");

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
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify picker is open
    assert!(ui.find("14").is_ok(), "Picker should be open");

    // Click the OK/submit button
    ui.click("\u{e805}")?; // OK button

    // Verify we got a Submit message with the time
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
fn submit_returns_correct_time() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Click the submit button
    ui.click("\u{e805}")?; // OK button

    // Verify the submitted time
    let mut submitted_time: Option<Time> = None;
    for message in ui.into_messages() {
        if let Message::Submit(time) = message {
            submitted_time = Some(time);
        }
        app.update(message);
    }

    assert!(submitted_time.is_some(), "Should have submitted a time");
    let submitted = submitted_time.unwrap();

    // Verify it's the correct time
    match submitted {
        Time::Hm {
            hour,
            minute,
            period,
        } => {
            assert_eq!(hour, 14, "Hour should be 14");
            assert_eq!(minute, 30, "Minute should be 30");
            assert_eq!(period, Period::H24, "Period should be H24");
        }
        _ => panic!("Expected Time::Hm variant"),
    }

    Ok(())
}

// ============================================================================
// Workflow Tests
// ============================================================================

#[test]
fn adjust_time_then_submit_workflow() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Adjust hour up using keyboard navigation
    ui.click("14")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowUp,
    ));
    assert!(ui.find("15").is_ok(), "Hour should be 15");

    // Adjust minute up using keyboard navigation
    ui.click("30")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowUp,
    ));
    assert!(ui.find("31").is_ok(), "Minute should be 31");

    // Submit the adjusted time
    ui.click("\u{e805}")?;

    // Verify the submitted time
    let mut submitted_time: Option<Time> = None;
    for message in ui.into_messages() {
        if let Message::Submit(time) = message {
            submitted_time = Some(time);
        }
        app.update(message);
    }

    assert!(submitted_time.is_some(), "Should have submitted a time");
    let submitted = submitted_time.unwrap();

    match submitted {
        Time::Hm {
            hour,
            minute,
            period,
        } => {
            assert_eq!(hour, 15, "Hour should be adjusted to 15");
            assert_eq!(minute, 31, "Minute should be adjusted to 31");
            assert_eq!(period, Period::H24, "Period should be H24");
        }
        _ => panic!("Expected Time::Hm variant"),
    }

    Ok(())
}

#[test]
fn keyboard_navigation_workflow() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 14,
        minute: 30,
        period: Period::H24,
    };

    let (app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit)
            .use_24h()
            .into()
    });

    let mut ui = simulator(&app);

    // Verify starting state
    assert!(ui.find("14").is_ok(), "Should start at hour 14");
    assert!(ui.find("30").is_ok(), "Should start at minute 30");

    // Click on the hour to give it focus
    ui.click("14")?;

    // Use right arrow to advance hour
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowRight,
    ));

    assert!(ui.find("15").is_ok(), "Hour should now be 15");

    // Use left arrow to go back
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowLeft,
    ));

    assert!(ui.find("14").is_ok(), "Hour should be back to 14");

    Ok(())
}

#[test]
fn complete_12h_format_workflow() -> Result<(), Error> {
    let time = Time::Hm {
        hour: 11,
        minute: 45,
        period: Period::Am,
    };

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Time Picker");
        TimePicker::new(true, time, button, Message::Cancel, Message::Submit).into()
    });

    let mut ui = simulator(&app);

    // Verify initial state
    assert!(ui.find("11").is_ok(), "Hour should be 11");
    assert!(ui.find("45").is_ok(), "Minute should be 45");
    assert!(ui.find("AM").is_ok(), "Should show AM");

    // Adjust hour using keyboard navigation
    ui.click("11")?;
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowUp,
    ));
    assert!(ui.find("12").is_ok(), "Hour should be 12");

    // Submit
    ui.click("\u{e805}")?;

    let mut submitted_time: Option<Time> = None;
    for message in ui.into_messages() {
        if let Message::Submit(time) = message {
            submitted_time = Some(time);
        }
        app.update(message);
    }

    assert!(submitted_time.is_some(), "Should have submitted a time");
    let submitted = submitted_time.unwrap();

    match submitted {
        Time::Hm {
            hour,
            minute,
            period,
        } => {
            assert_eq!(hour, 12, "Hour should be 12");
            assert_eq!(minute, 45, "Minute should be 45");
            // When incrementing from 11 AM, we go to 12 PM (noon)
            assert_eq!(period, Period::Pm, "Period should be PM (noon)");
        }
        _ => panic!("Expected Time::Hm variant"),
    }

    Ok(())
}
