//! Integration tests for the DropDown widget
//!
//! These tests verify the DropDown widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

#[macro_use]
mod common;

use iced::{Length, Theme};
use iced_aw::DropDown;
use iced_test::Error;
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

// Generate test helpers for this Message type
test_helpers!(Message);

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
    process_messages(ui, &mut app);

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
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Expand),
        "Clicking button should produce Message::Expand",
    );

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

    // Snapshot testing: verify visual rendering matches baseline
    assert_snapshot_matches(&mut ui, "tests/snapshots/drop_down_expanded_shows_overlay")?;

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
    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::SelectOption(_)));
    let selected_option = messages.first().and_then(|m| {
        if let Message::SelectOption(option) = m {
            Some(*option)
        } else {
            None
        }
    });

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

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::SelectOption(_)));
    let selected_option = messages.first().and_then(|m| {
        if let Message::SelectOption(option) = m {
            Some(*option)
        } else {
            None
        }
    });

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

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::SelectOption(_)));
    let selected_option = messages.first().and_then(|m| {
        if let Message::SelectOption(option) = m {
            Some(*option)
        } else {
            None
        }
    });

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

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::SelectOption(_)));
    let selected = messages.first().and_then(|m| {
        if let Message::SelectOption(opt) = m {
            Some(*opt)
        } else {
            None
        }
    });

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

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::SelectOption(_)));
    let selected = messages.first().and_then(|m| {
        if let Message::SelectOption(opt) = m {
            Some(*opt)
        } else {
            None
        }
    });

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

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::SelectOption(_)));
    let selected = messages.first().and_then(|m| {
        if let Message::SelectOption(opt) = m {
            Some(*opt)
        } else {
            None
        }
    });

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

    // Snapshot testing: verify visual rendering matches baseline
    assert_snapshot_matches(
        &mut ui,
        "tests/snapshots/drop_down_menu_with_text_and_buttons",
    )?;

    // Verify all options are visible
    assert!(ui.find("Option A").is_ok(), "Option A should be visible");
    assert!(ui.find("Option B").is_ok(), "Option B should be visible");
    assert!(ui.find("Option C").is_ok(), "Option C should be visible");

    // Click Option B
    ui.click("Option B")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::SelectOption(_)));
    let selected = messages.first().and_then(|m| {
        if let Message::SelectOption(opt) = m {
            Some(*opt)
        } else {
            None
        }
    });

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

    // Snapshot testing: verify visual rendering matches baseline
    assert_snapshot_matches(&mut ui, "tests/snapshots/drop_down_with_rows_of_choices")?;

    // Verify all colors are visible
    assert!(ui.find("Red").is_ok(), "Red should be visible");
    assert!(ui.find("Green").is_ok(), "Green should be visible");
    assert!(ui.find("Blue").is_ok(), "Blue should be visible");
    assert!(ui.find("Yellow").is_ok(), "Yellow should be visible");

    // Click Blue (in second row)
    ui.click("Blue")?;

    let messages = collect_messages(ui, &mut app, |m| matches!(m, Message::SelectOption(_)));
    let selected = messages.first().and_then(|m| {
        if let Message::SelectOption(opt) = m {
            Some(*opt)
        } else {
            None
        }
    });

    assert_eq!(selected, Some(3), "Should select Blue");

    Ok(())
}

// ============================================================================
// Keyboard Dismiss Tests
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
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Dismiss),
        "Escape key should trigger dismiss message",
    );

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Dismiss on Outside Click
// ============================================================================

#[test]
fn drop_down_left_click_outside_triggers_dismiss() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("Overlay");
        DropDown::new(underlay, overlay, true)
            .on_dismiss(Message::Dismiss)
            .into()
    });

    let mut ui = simulator(&app);

    // Simulate left click outside
    let pos = outside_position();
    simulate_left_click_at(&mut ui, pos.x, pos.y);

    // Verify we got Message::Dismiss
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Dismiss),
        "Left click outside overlay should trigger dismiss message",
    );

    Ok(())
}

#[test]
fn drop_down_right_click_outside_triggers_dismiss() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("Overlay");
        DropDown::new(underlay, overlay, true)
            .on_dismiss(Message::Dismiss)
            .into()
    });

    let mut ui = simulator(&app);

    // Simulate right click outside
    let pos = outside_position();
    simulate_right_click_at(&mut ui, pos.x, pos.y);

    // Verify we got Message::Dismiss
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Dismiss),
        "Right click outside overlay should trigger dismiss message",
    );

    Ok(())
}

#[test]
fn drop_down_middle_click_outside_does_not_trigger_dismiss() -> Result<(), Error> {
    use iced_core::mouse;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("Overlay");
        DropDown::new(underlay, overlay, true)
            .on_dismiss(Message::Dismiss)
            .into()
    });

    let mut ui = simulator(&app);

    // Simulate middle click outside
    let pos = outside_position();
    simulate_mouse_click_at(&mut ui, pos.x, pos.y, mouse::Button::Middle);

    // Verify we did NOT get Message::Dismiss (only left/right buttons trigger dismiss)
    let got_dismiss = check_message_received(ui, &mut app, |m| matches!(m, Message::Dismiss));

    assert!(
        !got_dismiss,
        "Middle click should not trigger dismiss message"
    );

    Ok(())
}

#[test]
fn drop_down_click_outside_without_on_dismiss_does_nothing() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("Overlay");
        // Note: No on_dismiss handler
        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);

    // Simulate left click outside
    let pos = outside_position();
    simulate_left_click_at(&mut ui, pos.x, pos.y);

    // Verify no messages were produced
    let messages: Vec<_> = ui.into_messages().collect();
    assert!(
        messages.is_empty(),
        "Should not produce messages when on_dismiss is not set"
    );

    Ok(())
}

// ============================================================================
// Touch Input Tests
// ============================================================================

#[test]
fn drop_down_touch_outside_triggers_dismiss() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("Overlay");
        DropDown::new(underlay, overlay, true)
            .on_dismiss(Message::Dismiss)
            .into()
    });

    let mut ui = simulator(&app);

    // Simulate touch outside
    let pos = outside_position();
    simulate_touch_at(&mut ui, pos.x, pos.y);

    // Verify we got Message::Dismiss
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Dismiss),
        "Touch outside overlay should trigger dismiss message",
    );

    Ok(())
}

#[test]
fn drop_down_touch_outside_without_on_dismiss_does_nothing() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = Text::new("Overlay");
        // Note: No on_dismiss handler
        DropDown::new(underlay, overlay, true).into()
    });

    let mut ui = simulator(&app);

    // Simulate touch outside
    let pos = outside_position();
    simulate_touch_at(&mut ui, pos.x, pos.y);

    // Verify no messages were produced
    let messages: Vec<_> = ui.into_messages().collect();
    assert!(
        messages.is_empty(),
        "Touch should not produce messages when on_dismiss is not set"
    );

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Click Inside Overlay
// ============================================================================

#[test]
fn drop_down_click_on_overlay_button_does_not_dismiss() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");

        let overlay = Column::new()
            .push(create_button("Option 1", Message::SelectOption(1)))
            .push(create_button("Option 2", Message::SelectOption(2)));

        DropDown::new(underlay, overlay, true)
            .on_dismiss(Message::Dismiss)
            .into()
    });

    let mut ui = simulator(&app);

    // Click on a button inside the overlay
    ui.click("Option 1")?;

    // Check the messages - should get SelectOption, NOT Dismiss
    let all_messages = collect_messages(ui, &mut app, |_| true);
    let got_select = all_messages
        .iter()
        .any(|m| matches!(m, Message::SelectOption(_)));
    let got_dismiss = all_messages.iter().any(|m| matches!(m, Message::Dismiss));

    assert!(got_select, "Should receive SelectOption message");
    assert!(
        !got_dismiss,
        "Should not receive Dismiss when clicking inside overlay"
    );

    Ok(())
}

#[test]
fn drop_down_click_on_underlay_does_not_dismiss() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = create_button("Toggle Menu", Message::Expand);
        let overlay = Text::new("Overlay Content");

        DropDown::new(underlay, overlay, true)
            .on_dismiss(Message::Dismiss)
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the underlay button
    ui.click("Toggle Menu")?;

    // Check the messages - should get Expand, NOT Dismiss
    let all_messages = collect_messages(ui, &mut app, |_| true);
    let got_expand = all_messages.iter().any(|m| matches!(m, Message::Expand));
    let got_dismiss = all_messages.iter().any(|m| matches!(m, Message::Dismiss));

    assert!(got_expand, "Should receive Expand message");
    assert!(
        !got_dismiss,
        "Should not receive Dismiss when clicking on underlay"
    );

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
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Expand),
        "Should have expanded",
    );

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
