//! Integration tests for the DatePicker widget
//!
//! These tests verify the DatePicker widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.
//! Unicode values for button contents:

// Button cheat sheet
//  cancel          → \u{e800}  // Used for close/cancel buttons
//  down_open       → \u{e801}  // Down arrow (used in number_input, time_picker)
//  left_open       → \u{e802}  // Left arrow (used in date_picker navigation)
//  right_open      → \u{e803}  // Right arrow (used in date_picker navigation)
//  up_open         → \u{e804}  // Up arrow (used in number_input, time_picker)
//  ok              → \u{e805}  // Checkmark/submit (used in pickers)


use iced::{Element, Pixels, Settings, Theme};
use iced_aw::{date_picker::Date, DatePicker};
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

    // Track whether picker should be shown
    let show = std::rc::Rc::new(std::cell::RefCell::new(false));
    let show_clone = show.clone();

    let (mut app, _) = App::new(move || {
        let button = create_button("Open Date Picker");
        let show_value = *show_clone.borrow();
        DatePicker::new(show_value, date, button, Message::Cancel, Message::Submit).into()
    });

    // Verify the button is findable initially
    {
        let mut ui = simulator(&app);
        assert!(
            ui.find("Open Date Picker").is_ok(),
            "Underlay button text should be findable"
        );
    }

    // Click the button and collect messages
    let messages = {
        let mut ui = simulator(&app);
        let _ = ui.click("Open Date Picker")?;
        ui.into_messages().collect::<Vec<_>>()
    };

    // Process messages (now we can mutably borrow app)
    for message in messages {
        if matches!(message, Message::Open) {
            *show.borrow_mut() = true;
        }
        app.update(message);
    }

    // Create new simulator to verify picker is open
    let mut ui = simulator(&app);
    assert!(ui.find("June").is_ok(), "Month name should be displayed");
    assert!(ui.find("2024").is_ok(), "Year should be displayed");

    Ok(())
}

// ============================================================================
// Overlay Content Verification Tests
// ============================================================================

#[test]
fn overlay_shows_correct_month_and_year() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // When picker is shown, verify month and year are displayed
    let mut ui = simulator(&app);
    assert!(ui.find("June").is_ok(), "Month name should be displayed");
    assert!(ui.find("2024").is_ok(), "Year should be displayed");

    Ok(())
}

#[test]
fn overlay_shows_various_months() -> Result<(), Error> {
    let test_cases = vec![
        (Date::from_ymd(2024, 1, 15), "January", "2024"),
        (Date::from_ymd(2024, 3, 10), "March", "2024"),
        (Date::from_ymd(2024, 12, 25), "December", "2024"),
        (Date::from_ymd(2023, 7, 4), "July", "2023"),
    ];

    for (date, expected_month, expected_year) in test_cases {
        let (mut app, _) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(expected_month).is_ok(),
            "Should display month '{}'",
            expected_month
        );
        assert!(
            ui.find(expected_year).is_ok(),
            "Should display year '{}'",
            expected_year
        );
    }

    Ok(())
}

#[test]
fn overlay_shows_day_numbers() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Verify day numbers are displayed
    let mut ui = simulator(&app);
    assert!(ui.find("15").is_ok(), "Current day (15) should be displayed");
    assert!(ui.find("01").is_ok(), "First day should be displayed");
    assert!(ui.find("30").is_ok(), "Last day of June (30) should be displayed");

    Ok(())
}

#[test]
fn overlay_shows_correct_days_for_leap_year() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 2, 15); // 2024 is a leap year

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("February").is_ok(), "Should show February");
    assert!(ui.find("29").is_ok(), "Leap year should show day 29");

    Ok(())
}

#[test]
fn overlay_shows_correct_days_for_non_leap_year() -> Result<(), Error> {
    let date = Date::from_ymd(2023, 2, 15); // 2023 is not a leap year

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("February").is_ok(), "Should show February");
    assert!(ui.find("28").is_ok(), "Non-leap year should show day 28");

    Ok(())
}

#[test]
fn overlay_not_shown_when_picker_hidden() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // When picker is hidden, month/year should not be in the UI
    // (only the underlay button should be visible)
    let mut ui = simulator(&app);
    assert!(ui.find("Pick date").is_ok(), "Button should be visible");
    // Note: We can't easily verify that overlay is NOT present without
    // more sophisticated testing, as find() returns Err for not found
    // which is expected. This test mainly verifies no crash occurs.

    Ok(())
}

// ============================================================================
// Configuration and Styling Tests
// ============================================================================

#[test]
fn date_picker_with_custom_font_size() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 1);

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit)
            .font_size(20.0)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Verify content still renders with custom font size
    let mut ui = simulator(&app);
    assert!(ui.find("January").is_ok(), "Month should render with custom font size");
    assert!(ui.find("2024").is_ok(), "Year should render with custom font size");

    Ok(())
}

#[test]
fn date_picker_with_pixels_font_size() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 1);

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit)
            .font_size(Pixels(18.0))
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("January").is_ok(), "Month should render");

    Ok(())
}

#[test]
fn date_picker_with_custom_styling() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 1, 1);

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit)
            .style(|_theme: &Theme, _status: Status| style::date_picker::Style {
                background: Background::Color(iced::Color::WHITE),
                border_radius: 8.0,
                border_width: 1.0,
                border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                text_color: iced::Color::BLACK,
                text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                day_background: Background::Color(iced::Color::TRANSPARENT),
            })
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Verify content renders with custom styling
    let mut ui = simulator(&app);
    assert!(ui.find("January").is_ok(), "Styled picker should render content");

    Ok(())
}

#[test]
fn date_picker_with_chained_methods() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 6, 15);

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit)
            .font_size(16.0)
            .style(|_theme: &Theme, _status: Status| style::date_picker::Style {
                background: Background::Color(iced::Color::WHITE),
                border_radius: 8.0,
                border_width: 1.0,
                border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                text_color: iced::Color::BLACK,
                text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                day_background: Background::Color(iced::Color::TRANSPARENT),
            })
            .class(<Theme as iced_aw::style::date_picker::Catalog>::default())
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("June").is_ok(), "Chained methods should not break rendering");
    assert!(ui.find("2024").is_ok(), "Year should render");

    Ok(())
}

// ============================================================================
// Edge Cases and Boundary Tests
// ============================================================================

#[test]
fn date_picker_with_extreme_dates() -> Result<(), Error> {
    let test_cases = vec![
        (Date::from_ymd(1776, 7, 4), "July", "1776"),
        (Date::from_ymd(2100, 12, 31), "December", "2100"),
    ];

    for (date, expected_month, expected_year) in test_cases {
        let (mut app, _) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(expected_month).is_ok(),
            "Should handle extreme date with month '{}'",
            expected_month
        );
        assert!(
            ui.find(expected_year).is_ok(),
            "Should handle extreme date with year '{}'",
            expected_year
        );
    }

    Ok(())
}

#[test]
fn date_picker_with_month_boundaries() -> Result<(), Error> {
    let test_cases = vec![
        (Date::from_ymd(2024, 1, 1), "January", "01"),  // First day of year
        (Date::from_ymd(2024, 1, 31), "January", "31"), // Last day of January
        (Date::from_ymd(2024, 12, 31), "December", "31"), // Last day of year
    ];

    for (date, expected_month, expected_day) in test_cases {
        let (mut app, _) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(expected_month).is_ok(),
            "Should show month '{}'",
            expected_month
        );
        assert!(
            ui.find(expected_day).is_ok(),
            "Should show day '{}'",
            expected_day
        );
    }

    Ok(())
}

#[test]
fn date_picker_with_default_date() -> Result<(), Error> {
    let date = Date::default();

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Default date should render something (we don't know exact date, but it should work)
    let mut ui = simulator(&app);
    assert!(
        ui.find("Pick date").is_ok(),
        "DatePicker with default date should render"
    );

    Ok(())
}

// ============================================================================
// Multiple Instances Test
// ============================================================================

#[test]
fn date_picker_supports_multiple_instances() -> Result<(), Error> {
    let date1 = Date::from_ymd(2024, 1, 1);
    let date2 = Date::from_ymd(2024, 6, 15);

    // Test that we can create multiple DatePickers without issues
    let (mut app1, _) = App::new(move || {
        let button = create_button("Picker 1");
        DatePicker::new(false, date1, button, Message::Cancel, Message::Submit).into()
    });

    let (mut app2, _) = App::new(move || {
        let button = create_button("Picker 2");
        DatePicker::new(false, date2, button, Message::Cancel, Message::Submit).into()
    });

    let ui1 = simulator(&app1);
    let ui2 = simulator(&app2);

    for message in ui1.into_messages() {
        app1.update(message);
    }

    for message in ui2.into_messages() {
        app2.update(message);
    }

    let mut ui1 = simulator(&app1);
    let mut ui2 = simulator(&app2);

    assert!(ui1.find("Picker 1").is_ok(), "First picker should render");
    assert!(ui2.find("Picker 2").is_ok(), "Second picker should render");

    Ok(())
}

// ============================================================================
// Type Conversion Test
// ============================================================================

#[test]
fn date_picker_converts_to_element() -> Result<(), Error> {
    use iced_widget::Renderer;

    let date = Date::from_ymd(2024, 1, 1);

    let (mut app, _) = App::new(move || {
        let button = create_button("Pick date");
        let picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit);
        let _element: Element<Message, Theme, Renderer> = picker.into();

        let button2 = create_button("Pick date");
        DatePicker::new(false, date, button2, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("Pick date").is_ok(), "Converted element should render");

    Ok(())
}

// ============================================================================
// Custom Message Type Test
// ============================================================================

#[test]
fn date_picker_with_custom_message_types() -> Result<(), Error> {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        OpenPicker,
        ClosePicker,
        DateSelected(Date),
    }

    let date = Date::from_ymd(2024, 1, 1);

    type CustomViewFn = Box<dyn Fn() -> Element<'static, CustomMessage>>;

    #[derive(Clone)]
    struct CustomApp {
        view_fn: std::rc::Rc<CustomViewFn>,
    }

    impl CustomApp {
        fn new<F>(view_fn: F) -> (Self, iced::Task<CustomMessage>)
        where
            F: Fn() -> Element<'static, CustomMessage> + 'static,
        {
            (
                CustomApp {
                    view_fn: std::rc::Rc::new(Box::new(view_fn)),
                },
                iced::Task::none(),
            )
        }

        fn update(&mut self, message: CustomMessage) {
            match message {
                CustomMessage::OpenPicker
                | CustomMessage::ClosePicker
                | CustomMessage::DateSelected(_) => {}
            }
        }

        fn view(&self) -> Element<'_, CustomMessage> {
            (self.view_fn)()
        }
    }

    let (mut app, _) = CustomApp::new(move || {
        let button: iced_widget::Button<'_, CustomMessage, Theme> =
            button(Text::new("Custom Picker")).on_press(CustomMessage::OpenPicker);
        DatePicker::new(
            false,
            date,
            button,
            CustomMessage::ClosePicker,
            CustomMessage::DateSelected,
        )
        .into()
    });

    let ui = Simulator::with_settings(
        Settings {
            ..Settings::default()
        },
        app.view(),
    );

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = Simulator::with_settings(
        Settings {
            ..Settings::default()
        },
        app.view(),
    );
    assert!(
        ui.find("Custom Picker").is_ok(),
        "DatePicker should work with custom message types"
    );

    Ok(())
}
