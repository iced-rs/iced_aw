//! Integration tests for the TabBar widget
//!
//! These tests verify the TabBar widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

// Button icons reference:
//  cancel          → \u{e800}  // Used for close/cancel buttons
//  down_open       → \u{e801}  // Down arrow (used in dropdowns)
//  left_open       → \u{e802}  // Left arrow (used in navigation)
//  right_open      → \u{e803}  // Right arrow (used in navigation)
//  up_open         → \u{e804}  // Up arrow (used in increment)
//  ok              → \u{e805}  // Checkmark/submit (used in pickers)

// Simulator API: https://raw.githubusercontent.com/iced-rs/iced/master/test/src/simulator.rs

#[macro_use]
mod common;

use iced_aw::{TabBar, TabLabel};
use iced_test::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum TabId {
    Home,
    Settings,
    Profile,
    About,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Message {
    TabSelected(TabId),
    TabClosed(TabId),
}

// Generate test helpers for this Message type
test_helpers!(Message);

// ============================================================================
// Basic Rendering Tests
// ============================================================================

#[test]
fn tab_bar_renders_with_single_tab() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .into()
    });
}

#[test]
fn tab_bar_renders_with_multiple_tabs() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .push(TabId::Profile, TabLabel::Text("Profile".to_string()))
            .into()
    });
}

#[test]
fn tab_bar_can_find_tab_text() {
    run_test_and_find(
        || {
            TabBar::new(Message::TabSelected)
                .push(TabId::Home, TabLabel::Text("Home".to_string()))
                .into()
        },
        "Home",
    );
}

#[test]
fn tab_bar_can_find_all_tab_texts() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .push(TabId::Profile, TabLabel::Text("Profile".to_string()))
            .into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("Home").is_ok(), "Should find Home tab");
    assert!(ui.find("Settings").is_ok(), "Should find Settings tab");
    assert!(ui.find("Profile").is_ok(), "Should find Profile tab");

    Ok(())
}

// ============================================================================
// Tab Icon Tests
// ============================================================================

#[test]
fn tab_bar_renders_with_icon_tabs() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Icon('🏠'))
            .push(TabId::Settings, TabLabel::Icon('⚙'))
            .into()
    });
}

#[test]
fn tab_bar_can_find_icon_tab() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Icon('🏠'))
            .into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("🏠").is_ok(), "Should find icon tab");

    Ok(())
}

#[test]
fn tab_bar_renders_with_icon_text_tabs() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .push(
                TabId::Settings,
                TabLabel::IconText('⚙', "Settings".to_string()),
            )
            .into()
    });
}

#[test]
fn tab_bar_can_find_icon_text_tab_text() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("Home").is_ok(), "Should find text in icon-text tab");
    assert!(ui.find("🏠").is_ok(), "Should find icon in icon-text tab");

    Ok(())
}

// ============================================================================
// Active Tab Tests
// ============================================================================

#[test]
fn tab_bar_sets_active_tab() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .set_active_tab(&TabId::Settings)
            .into()
    });
}

// ============================================================================
// Close Button Tests
// ============================================================================

#[test]
fn tab_bar_renders_with_close_buttons() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .on_close(Message::TabClosed)
            .into()
    });
}

#[test]
fn tab_bar_can_find_close_button() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .on_close(Message::TabClosed)
            .into()
    });

    let mut ui = simulator(&app);
    // The close button uses the cancel icon (\u{e800})
    assert!(ui.find("\u{e800}").is_ok(), "Should find close button icon");

    Ok(())
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn tab_bar_with_custom_width() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .width(500)
            .into()
    });
}

#[test]
fn tab_bar_with_custom_height() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .height(60)
            .into()
    });
}

#[test]
fn tab_bar_with_custom_icon_size() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Icon('🏠'))
            .icon_size(24.0)
            .into()
    });
}

#[test]
fn tab_bar_with_custom_text_size() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .text_size(20.0)
            .into()
    });
}

#[test]
fn tab_bar_with_custom_close_size() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .on_close(Message::TabClosed)
            .close_size(18.0)
            .into()
    });
}

#[test]
fn tab_bar_with_custom_padding() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .padding(10.0)
            .into()
    });
}

#[test]
fn tab_bar_with_custom_spacing() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .spacing(5.0)
            .into()
    });
}

// ============================================================================
// Interaction Tests
// ============================================================================

#[test]
fn tab_bar_click_tab_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the Settings tab
    ui.click("Settings")?;

    // Verify we got a TabSelected message
    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Settings)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking tab should produce TabSelected message"
    );

    Ok(())
}

#[test]
fn tab_bar_click_close_button_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .on_close(Message::TabClosed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the close button (cancel icon)
    ui.click("\u{e800}")?;

    // Verify we got a TabClosed message
    let mut got_tab_closed = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabClosed(_)) {
            got_tab_closed = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_closed,
        "Clicking close button should produce TabClosed message"
    );

    Ok(())
}

// ============================================================================
// Mixed Tab Types Tests
// ============================================================================

#[test]
fn tab_bar_with_mixed_tab_types() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Icon('⚙'))
            .push(
                TabId::Profile,
                TabLabel::IconText('👤', "Profile".to_string()),
            )
            .into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("Home").is_ok(), "Should find text tab");
    assert!(ui.find("⚙").is_ok(), "Should find icon tab");
    assert!(ui.find("Profile").is_ok(), "Should find icon-text tab text");
    assert!(ui.find("👤").is_ok(), "Should find icon-text tab icon");

    Ok(())
}

// ============================================================================
// Empty and Edge Cases
// ============================================================================

#[test]
fn tab_bar_with_no_tabs() {
    run_test(|| TabBar::<Message, TabId>::new(Message::TabSelected).into());
}

#[test]
fn tab_bar_with_empty_text() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("".to_string()))
            .into()
    });
}

#[test]
fn tab_bar_with_unicode_text() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("主页".to_string()))
            .push(TabId::Settings, TabLabel::Text("设置".to_string()))
            .into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("主页").is_ok(), "Should find Chinese text tab");
    assert!(ui.find("设置").is_ok(), "Should find Chinese text tab");

    Ok(())
}

// ============================================================================
// Multiple Tab Clicks
// ============================================================================

#[test]
fn tab_bar_multiple_tab_clicks() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .push(TabId::Profile, TabLabel::Text("Profile".to_string()))
            .into()
    });

    // Click on Settings tab
    let mut ui = simulator(&app);
    ui.click("Settings")?;

    let mut got_settings = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Settings)) {
            got_settings = true;
        }
        app.update(message);
    }

    assert!(got_settings, "Should receive Settings tab selection");

    // Click on Profile tab
    let mut ui = simulator(&app);
    ui.click("Profile")?;

    let mut got_profile = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Profile)) {
            got_profile = true;
        }
        app.update(message);
    }

    assert!(got_profile, "Should receive Profile tab selection");

    Ok(())
}

// ============================================================================
// Tab Width Configuration
// ============================================================================

#[test]
fn tab_bar_with_fixed_tab_width() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .tab_width(iced::Length::Fixed(150.0))
            .into()
    });
}

#[test]
fn tab_bar_with_fill_tab_width() {
    run_test(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .tab_width(iced::Length::Fill)
            .into()
    });
}

// ============================================================================
// Mouse Input Tests
// ============================================================================

#[test]
fn tab_bar_click_icon_tab_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Icon('🏠'))
            .push(TabId::Settings, TabLabel::Icon('⚙'))
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the Settings icon tab
    ui.click("⚙")?;

    // Verify we got a TabSelected message
    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Settings)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking icon tab should produce TabSelected message"
    );

    Ok(())
}

#[test]
fn tab_bar_click_icon_text_tab_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .push(
                TabId::Settings,
                TabLabel::IconText('⚙', "Settings".to_string()),
            )
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the Settings icon-text tab by clicking on text
    ui.click("Settings")?;

    // Verify we got a TabSelected message
    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Settings)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking icon-text tab should produce TabSelected message"
    );

    Ok(())
}

#[test]
fn tab_bar_click_icon_text_tab_by_icon_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .push(
                TabId::Settings,
                TabLabel::IconText('⚙', "Settings".to_string()),
            )
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the Settings icon-text tab by clicking on icon
    ui.click("⚙")?;

    // Verify we got a TabSelected message
    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Settings)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking icon in icon-text tab should produce TabSelected message"
    );

    Ok(())
}

#[test]
fn tab_bar_click_active_tab_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Text("Settings".to_string()))
            .set_active_tab(&TabId::Home)
            .into()
    });

    let mut ui = simulator(&app);

    // Click on the currently active Home tab
    ui.click("Home")?;

    // Verify we got a TabSelected message even for the active tab
    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Home)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking active tab should still produce TabSelected message"
    );

    Ok(())
}

#[test]
fn tab_bar_click_close_button_on_icon_tab_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Icon('🏠'))
            .push(TabId::Settings, TabLabel::Icon('⚙'))
            .on_close(Message::TabClosed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click on a close button
    ui.click("\u{e800}")?;

    // Verify we got a TabClosed message
    let mut got_tab_closed = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabClosed(_)) {
            got_tab_closed = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_closed,
        "Clicking close button on icon tab should produce TabClosed message"
    );

    Ok(())
}

#[test]
fn tab_bar_click_close_button_on_icon_text_tab_produces_message() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .push(
                TabId::Settings,
                TabLabel::IconText('⚙', "Settings".to_string()),
            )
            .on_close(Message::TabClosed)
            .into()
    });

    let mut ui = simulator(&app);

    // Click on a close button
    ui.click("\u{e800}")?;

    // Verify we got a TabClosed message
    let mut got_tab_closed = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabClosed(_)) {
            got_tab_closed = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_closed,
        "Clicking close button on icon-text tab should produce TabClosed message"
    );

    Ok(())
}

#[test]
fn tab_bar_click_mixed_tab_types() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::Text("Home".to_string()))
            .push(TabId::Settings, TabLabel::Icon('⚙'))
            .push(
                TabId::Profile,
                TabLabel::IconText('👤', "Profile".to_string()),
            )
            .into()
    });

    // Click on text tab
    let mut ui = simulator(&app);
    ui.click("Home")?;

    let mut got_home = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Home)) {
            got_home = true;
        }
        app.update(message);
    }

    assert!(got_home, "Should receive Home tab selection");

    // Click on icon tab
    let mut ui = simulator(&app);
    ui.click("⚙")?;

    let mut got_settings = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Settings)) {
            got_settings = true;
        }
        app.update(message);
    }

    assert!(got_settings, "Should receive Settings tab selection");

    // Click on icon-text tab
    let mut ui = simulator(&app);
    ui.click("Profile")?;

    let mut got_profile = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Profile)) {
            got_profile = true;
        }
        app.update(message);
    }

    assert!(got_profile, "Should receive Profile tab selection");

    Ok(())
}

#[test]
fn tab_bar_icon_text_position_top_click() -> Result<(), Error> {
    use iced_aw::tab_bar::Position;

    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .set_position(Position::Top)
            .into()
    });

    let mut ui = simulator(&app);
    ui.click("Home")?;

    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Home)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking icon-text tab with icon on top should produce message"
    );

    Ok(())
}

#[test]
fn tab_bar_icon_text_position_right_click() -> Result<(), Error> {
    use iced_aw::tab_bar::Position;

    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .set_position(Position::Right)
            .into()
    });

    let mut ui = simulator(&app);
    ui.click("Home")?;

    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Home)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking icon-text tab with icon on right should produce message"
    );

    Ok(())
}

#[test]
fn tab_bar_icon_text_position_bottom_click() -> Result<(), Error> {
    use iced_aw::tab_bar::Position;

    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .set_position(Position::Bottom)
            .into()
    });

    let mut ui = simulator(&app);
    ui.click("Home")?;

    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Home)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking icon-text tab with icon on bottom should produce message"
    );

    Ok(())
}

#[test]
fn tab_bar_icon_text_position_left_click() -> Result<(), Error> {
    use iced_aw::tab_bar::Position;

    let (mut app, _) = App::new(|| {
        TabBar::new(Message::TabSelected)
            .push(TabId::Home, TabLabel::IconText('🏠', "Home".to_string()))
            .set_position(Position::Left)
            .into()
    });

    let mut ui = simulator(&app);
    ui.click("Home")?;

    let mut got_tab_selected = false;
    for message in ui.into_messages() {
        if matches!(message, Message::TabSelected(TabId::Home)) {
            got_tab_selected = true;
        }
        app.update(message);
    }

    assert!(
        got_tab_selected,
        "Clicking icon-text tab with icon on left should produce message"
    );

    Ok(())
}
