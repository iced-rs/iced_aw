//! Integration tests for the DatePicker widget
//!
//! These tests verify the DatePicker widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.
//! Unicode values for button contents:

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

// Button cheat sheet
//  cancel          → \u{e800}  // Used for close/cancel buttons
//  down_open       → \u{e801}  // Down arrow (used in number_input, time_picker)
//  left_open       → \u{e802}  // Left arrow (used in date_picker navigation)
//  right_open      → \u{e803}  // Right arrow (used in date_picker navigation)
//  up_open         → \u{e804}  // Up arrow (used in number_input, time_picker)
//  ok              → \u{e805}  // Checkmark/submit (used in pickers)

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
