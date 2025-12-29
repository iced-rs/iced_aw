//! Integration tests for the DropDown widget
//!
//! These tests verify the DropDown widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::{Element, Length, Settings, Theme};
use iced_aw::DropDown;
use iced_test::{Error, Simulator};
use iced_widget::{button, text::Text};

#[derive(Clone, Debug)]
enum Message {
    Expand,
    Dismiss,
    SelectOption(usize),
}

// Helper function to create a button with explicit Theme type
fn create_button<'a>(text: &'a str, msg: Message) -> iced_widget::Button<'a, Message, Theme> {
    button(Text::new(text)).on_press(msg)
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
            Message::Expand | Message::Dismiss | Message::SelectOption(_) => {
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
// Underlay Tests
// ============================================================================

#[test]
fn drop_down_can_find_underlay_text() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Click to expand");
        let overlay = Text::new("Overlay content");
        DropDown::new(underlay, overlay, false).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Click to expand").is_ok(),
        "Underlay text should be findable"
    );

    Ok(())
}

#[test]
fn drop_down_underlay_button_is_clickable() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = create_button("Expand Menu", Message::Expand);
        let overlay = Text::new("Overlay content");
        DropDown::new(underlay, overlay, false).into()
    });

    let mut ui = simulator(&app);
    assert!(
        ui.find("Expand Menu").is_ok(),
        "Underlay button should be findable"
    );

    ui.click("Expand Menu")?;

    // Verify we got Message::Expand
    let mut got_expand = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Expand) {
            got_expand = true;
        }
        app.update(message);
    }

    assert!(got_expand, "Clicking button should produce Message::Expand");

    Ok(())
}

#[test]
fn drop_down_collapsed_does_not_show_overlay() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Collapsed");
        let overlay = Text::new("Hidden content");
        DropDown::new(underlay, overlay, false).into()
    });

    let mut ui = simulator(&app);

    // Underlay should be visible
    assert!(ui.find("Collapsed").is_ok(), "Underlay should be visible");

    // Overlay should not be visible when collapsed
    assert!(
        ui.find("Hidden content").is_err(),
        "Overlay should not be visible when collapsed"
    );

    Ok(())
}

// ============================================================================
// Overlay Tests
// ============================================================================

#[test]
fn drop_down_expanded_shows_overlay() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Expanded");
        let overlay = Text::new("Visible content");
        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);

    // Both underlay and overlay should be visible
    assert!(ui.find("Expanded").is_ok(), "Underlay should be visible");
    assert!(
        ui.find("Visible content").is_ok(),
        "Overlay should be visible when expanded"
    );

    Ok(())
}

#[test]
fn drop_down_overlay_button_is_clickable() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = create_button("Select Option", Message::SelectOption(1));
        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);

    // Verify button is visible
    assert!(
        ui.find("Select Option").is_ok(),
        "Overlay button should be visible"
    );

    // Click the button
    ui.click("Select Option")?;

    // Verify we got the correct message
    let mut selected_option: Option<usize> = None;
    for message in ui.into_messages() {
        if let Message::SelectOption(option) = message {
            selected_option = Some(option);
        }
        app.update(message);
    }

    assert_eq!(
        selected_option,
        Some(1),
        "Should receive SelectOption(1) message"
    );

    Ok(())
}

#[test]
fn drop_down_first_button_can_be_clicked() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = create_button("First Choice", Message::SelectOption(1));
        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("First Choice").is_ok(), "Should find first button");

    ui.click("First Choice")?;

    let mut selected_option: Option<usize> = None;
    for message in ui.into_messages() {
        if let Message::SelectOption(option) = message {
            selected_option = Some(option);
        }
        app.update(message);
    }

    assert_eq!(selected_option, Some(1), "Should select first option");

    Ok(())
}

#[test]
fn drop_down_second_button_can_be_clicked() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = create_button("Second Choice", Message::SelectOption(2));
        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);
    assert!(
        ui.find("Second Choice").is_ok(),
        "Should find second button"
    );

    ui.click("Second Choice")?;

    let mut selected_option: Option<usize> = None;
    for message in ui.into_messages() {
        if let Message::SelectOption(option) = message {
            selected_option = Some(option);
        }
        app.update(message);
    }

    assert_eq!(selected_option, Some(2), "Should select second option");

    Ok(())
}

// ============================================================================
// Multi-Choice Selection Tests
// ============================================================================

#[test]
fn drop_down_with_multiple_choices_first_option() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = create_button("Show Menu", Message::Expand);

        let overlay = Column::new()
            .push(create_button("Choice 1", Message::SelectOption(1)))
            .push(create_button("Choice 2", Message::SelectOption(2)))
            .push(create_button("Choice 3", Message::SelectOption(3)));

        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);

    // Verify all choices are visible
    assert!(ui.find("Choice 1").is_ok(), "Choice 1 should be visible");
    assert!(ui.find("Choice 2").is_ok(), "Choice 2 should be visible");
    assert!(ui.find("Choice 3").is_ok(), "Choice 3 should be visible");

    // Click Choice 1
    ui.click("Choice 1")?;

    let mut selected = None;
    for message in ui.into_messages() {
        if let Message::SelectOption(opt) = message {
            selected = Some(opt);
        }
        app.update(message);
    }

    assert_eq!(selected, Some(1), "Should select Choice 1");

    Ok(())
}

#[test]
fn drop_down_with_multiple_choices_second_option() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = create_button("Show Menu", Message::Expand);

        let overlay = Column::new()
            .push(create_button("Choice 1", Message::SelectOption(1)))
            .push(create_button("Choice 2", Message::SelectOption(2)))
            .push(create_button("Choice 3", Message::SelectOption(3)));

        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);
    ui.click("Choice 2")?;

    let mut selected = None;
    for message in ui.into_messages() {
        if let Message::SelectOption(opt) = message {
            selected = Some(opt);
        }
        app.update(message);
    }

    assert_eq!(selected, Some(2), "Should select Choice 2");

    Ok(())
}

#[test]
fn drop_down_with_multiple_choices_third_option() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = create_button("Show Menu", Message::Expand);

        let overlay = Column::new()
            .push(create_button("Choice 1", Message::SelectOption(1)))
            .push(create_button("Choice 2", Message::SelectOption(2)))
            .push(create_button("Choice 3", Message::SelectOption(3)));

        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);
    ui.click("Choice 3")?;

    let mut selected = None;
    for message in ui.into_messages() {
        if let Message::SelectOption(opt) = message {
            selected = Some(opt);
        }
        app.update(message);
    }

    assert_eq!(selected, Some(3), "Should select Choice 3");

    Ok(())
}

#[test]
fn drop_down_menu_with_text_and_buttons() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Select an option");

        let overlay = Column::new()
            .push(Text::new("Available Options:"))
            .push(create_button("Option A", Message::SelectOption(1)))
            .push(create_button("Option B", Message::SelectOption(2)))
            .push(create_button("Option C", Message::SelectOption(3)))
            .push(Text::new("End of list"));

        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);

    // Verify header and footer text are visible
    assert!(
        ui.find("Available Options:").is_ok(),
        "Header should be visible"
    );
    assert!(ui.find("End of list").is_ok(), "Footer should be visible");

    // Verify all options are visible
    assert!(ui.find("Option A").is_ok(), "Option A should be visible");
    assert!(ui.find("Option B").is_ok(), "Option B should be visible");
    assert!(ui.find("Option C").is_ok(), "Option C should be visible");

    // Click Option B
    ui.click("Option B")?;

    let mut selected = None;
    for message in ui.into_messages() {
        if let Message::SelectOption(opt) = message {
            selected = Some(opt);
        }
        app.update(message);
    }

    assert_eq!(selected, Some(2), "Should select Option B");

    Ok(())
}

#[test]
fn drop_down_with_rows_of_choices() -> Result<(), Error> {
    use iced_widget::{Column, Row};

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Color Picker");

        let overlay = Column::new()
            .push(
                Row::new()
                    .push(create_button("Red", Message::SelectOption(1)))
                    .push(create_button("Green", Message::SelectOption(2))),
            )
            .push(
                Row::new()
                    .push(create_button("Blue", Message::SelectOption(3)))
                    .push(create_button("Yellow", Message::SelectOption(4))),
            );

        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);

    // Verify all colors are visible
    assert!(ui.find("Red").is_ok(), "Red should be visible");
    assert!(ui.find("Green").is_ok(), "Green should be visible");
    assert!(ui.find("Blue").is_ok(), "Blue should be visible");
    assert!(ui.find("Yellow").is_ok(), "Yellow should be visible");

    // Click Blue (in second row)
    ui.click("Blue")?;

    let mut selected = None;
    for message in ui.into_messages() {
        if let Message::SelectOption(opt) = message {
            selected = Some(opt);
        }
        app.update(message);
    }

    assert_eq!(selected, Some(3), "Should select Blue");

    Ok(())
}

// ============================================================================
// Dismiss Tests
// ============================================================================

#[test]
fn drop_down_escape_key_triggers_dismiss() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("Overlay");
        DropDown::new(underlay, overlay, true)
            .on_dismiss(Message::Dismiss)
            .into()
    });

    let mut ui = simulator(&app);

    // Press Escape key
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::Escape,
    ));

    // Verify we got Message::Dismiss
    let mut got_dismiss = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Dismiss) {
            got_dismiss = true;
        }
        app.update(message);
    }

    assert!(got_dismiss, "Escape key should trigger dismiss message");

    Ok(())
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn drop_down_with_custom_width() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Custom width");
        let overlay = Text::new("Wide overlay");
        DropDown::new(underlay, overlay, true)
            .width(Length::Fixed(300.0))
            .into()
    });

    let mut ui = simulator(&app);

    // Just verify it renders without errors
    assert!(ui.find("Custom width").is_ok());
    assert!(ui.find("Wide overlay").is_ok());

    Ok(())
}

#[test]
fn drop_down_with_custom_height() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Custom height");
        let overlay = Text::new("Tall overlay");
        DropDown::new(underlay, overlay, true)
            .height(Length::Fixed(200.0))
            .into()
    });

    let mut ui = simulator(&app);

    // Just verify it renders without errors
    assert!(ui.find("Custom height").is_ok());
    assert!(ui.find("Tall overlay").is_ok());

    Ok(())
}

#[test]
fn drop_down_with_bottom_alignment() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Bottom aligned");
        let overlay = Text::new("Below underlay");
        DropDown::new(underlay, overlay, true)
            .alignment(iced_aw::drop_down::Alignment::Bottom)
            .into()
    });

    let mut ui = simulator(&app);

    assert!(ui.find("Bottom aligned").is_ok());
    assert!(ui.find("Below underlay").is_ok());

    Ok(())
}

#[test]
fn drop_down_with_top_alignment() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Top aligned");
        let overlay = Text::new("Above underlay");
        DropDown::new(underlay, overlay, true)
            .alignment(iced_aw::drop_down::Alignment::Top)
            .into()
    });

    let mut ui = simulator(&app);

    assert!(ui.find("Top aligned").is_ok());
    assert!(ui.find("Above underlay").is_ok());

    Ok(())
}

// ============================================================================
// Complex Content Tests
// ============================================================================

#[test]
fn drop_down_with_text_overlay() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("This is overlay text content");
        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);

    // Verify elements are findable
    assert!(ui.find("Menu").is_ok(), "Should find underlay");
    assert!(
        ui.find("This is overlay text content").is_ok(),
        "Should find overlay text"
    );

    Ok(())
}

#[test]
fn drop_down_collapsed_button_triggers_expand() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = create_button("Show Menu", Message::Expand);
        let overlay = Text::new("Options");
        DropDown::new(underlay, overlay, false).into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("Show Menu").is_ok());
    assert!(ui.find("Options").is_err(), "Overlay should be hidden");

    ui.click("Show Menu")?;
    let mut got_expand = false;
    for message in ui.into_messages() {
        if matches!(message, Message::Expand) {
            got_expand = true;
        }
        app.update(message);
    }
    assert!(got_expand, "Should have expanded");

    Ok(())
}

#[test]
fn drop_down_expanded_with_dismiss_shows_content() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("Expanded overlay content");
        DropDown::new(underlay, overlay, true)
            .on_dismiss(Message::Dismiss)
            .into()
    });

    let mut ui = simulator(&app);
    assert!(
        ui.find("Expanded overlay content").is_ok(),
        "Should find overlay content when expanded"
    );

    Ok(())
}
