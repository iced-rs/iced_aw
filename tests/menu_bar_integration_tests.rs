//! Integration tests for the MenuBar widget
//!
//! These tests verify the MenuBar widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

// Simulator API https://raw.githubusercontent.com/iced-rs/iced/master/test/src/simulator.rs

#[macro_use]
mod common;

use iced_aw::menu::Menu;
use iced_aw::{menu_bar, menu_items};
use iced_test::Error;
use iced_widget::{button, text::Text};

#[derive(Clone, Debug, PartialEq)]
enum Message {
    FileNew,
    FileOpen,
    FileSave,
    EditCut,
    EditCopy,
}

// Generate test helpers for this Message type
test_helpers!(Message);

// ============================================================================
// Basic MenuBar Tests
// ============================================================================

#[test]
fn menu_bar_can_find_items() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        menu_bar!(
            (Text::new("File")),
            (Text::new("Edit")),
            (Text::new("View"))
        )
        .into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("File").is_ok(), "Should find File menu item");
    assert!(ui.find("Edit").is_ok(), "Should find Edit menu item");
    assert!(ui.find("View").is_ok(), "Should find View menu item");

    Ok(())
}

#[test]
fn menu_bar_with_single_item() {
    run_test_and_find(|| menu_bar!((Text::new("File"))).into(), "File");
}

#[test]
fn menu_bar_with_multiple_items() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        menu_bar!(
            (Text::new("File")),
            (Text::new("Edit")),
            (Text::new("View")),
            (Text::new("Help"))
        )
        .into()
    });

    let mut ui = simulator(&app);
    assert!(ui.find("File").is_ok(), "Should find File menu item");
    assert!(ui.find("Edit").is_ok(), "Should find Edit menu item");
    assert!(ui.find("View").is_ok(), "Should find View menu item");
    assert!(ui.find("Help").is_ok(), "Should find Help menu item");

    Ok(())
}

// ============================================================================
// Nested Menu Tests
// ============================================================================

#[test]
fn menu_bar_with_submenu() {
    run_test_and_find(
        || {
            let file_menu = Menu::new(menu_items!(
                (button(Text::new("New")).on_press(Message::FileNew)),
                (button(Text::new("Open")).on_press(Message::FileOpen)),
                (button(Text::new("Save")).on_press(Message::FileSave))
            ));

            menu_bar!((Text::new("File"), file_menu)).into()
        },
        "File",
    );
}

#[test]
fn menu_bar_clicking_item_shows_submenu() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        let file_menu = Menu::new(menu_items!(
            (button(Text::new("New")).on_press(Message::FileNew)),
            (button(Text::new("Open")).on_press(Message::FileOpen)),
            (button(Text::new("Save")).on_press(Message::FileSave))
        ));

        menu_bar!((Text::new("File"), file_menu)).into()
    });

    let mut ui = simulator(&app);

    // Click on File to open the menu
    ui.click("File")?;

    // After clicking, the submenu items should be visible
    assert!(
        ui.find("New").is_ok(),
        "Should find New menu item after clicking File"
    );
    assert!(
        ui.find("Open").is_ok(),
        "Should find Open menu item after clicking File"
    );
    assert!(
        ui.find("Save").is_ok(),
        "Should find Save menu item after clicking File"
    );

    Ok(())
}

// ============================================================================
// Multiple Menus Tests
// ============================================================================

#[test]
fn menu_bar_with_multiple_menus() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        let file_menu = Menu::new(menu_items!(
            (button(Text::new("New")).on_press(Message::FileNew)),
            (button(Text::new("Open")).on_press(Message::FileOpen))
        ));

        let edit_menu = Menu::new(menu_items!(
            (button(Text::new("Cut")).on_press(Message::EditCut)),
            (button(Text::new("Copy")).on_press(Message::EditCopy))
        ));

        menu_bar!(
            (Text::new("File"), file_menu),
            (Text::new("Edit"), edit_menu)
        )
        .into()
    });

    let ui = simulator(&app);
    process_messages(ui, &mut app);

    let mut ui = simulator(&app);
    assert!(ui.find("File").is_ok(), "Should find File menu");
    assert!(ui.find("Edit").is_ok(), "Should find Edit menu");

    Ok(())
}

// ============================================================================
// Keyboard Navigation Tests
// ============================================================================

#[test]
fn menu_bar_keyboard_navigation_in_menu() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        let file_menu = Menu::new(menu_items!(
            (button(Text::new("New")).on_press(Message::FileNew)),
            (button(Text::new("Open")).on_press(Message::FileOpen)),
            (button(Text::new("Save")).on_press(Message::FileSave))
        ));

        menu_bar!((Text::new("File"), file_menu)).into()
    });

    let mut ui = simulator(&app);

    // Click File to open menu
    ui.click("File")?;

    // Try keyboard navigation with arrow keys
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::ArrowDown,
    ));

    // The menu should still be visible
    assert!(
        ui.find("New").is_ok(),
        "Menu should still be visible after arrow key"
    );

    Ok(())
}

// ============================================================================
// Click Behavior Tests
// ============================================================================

#[test]
fn menu_bar_clicking_outside_closes_menu() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        let file_menu = Menu::new(menu_items!(
            (button(Text::new("New")).on_press(Message::FileNew))
        ));

        menu_bar!((Text::new("File"), file_menu)).into()
    });

    let mut ui = simulator(&app);

    // Click File to open menu
    ui.click("File")?;
    assert!(ui.find("New").is_ok(), "Menu should be open");

    // Note: Clicking outside would require the simulator to support clicking at specific coordinates
    // This test is a placeholder for that functionality

    Ok(())
}

// ============================================================================
// Menu Item Interaction Tests
// ============================================================================

#[test]
fn menu_bar_with_buttons_and_text() -> Result<(), Error> {
    let (app, _) = App::new(|| {
        let file_menu = Menu::new(menu_items!(
            (button(Text::new("New")).on_press(Message::FileNew)),
            (Text::new("Separator")),
            (button(Text::new("Open")).on_press(Message::FileOpen))
        ));

        menu_bar!((Text::new("File"), file_menu)).into()
    });

    let mut ui = simulator(&app);

    // Click File to open menu
    ui.click("File")?;

    assert!(ui.find("New").is_ok(), "Should find New button");
    assert!(ui.find("Separator").is_ok(), "Should find Separator text");
    assert!(ui.find("Open").is_ok(), "Should find Open button");

    Ok(())
}

// ============================================================================
// Empty and Edge Case Tests
// ============================================================================

#[test]
fn menu_bar_with_empty_text() -> Result<(), Error> {
    let (mut app, _) = App::new(|| menu_bar!((Text::new(""))).into());

    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }

    // Empty text should still create a menu bar item, just with no visible text
    // The test verifies the menu bar doesn't crash with empty text
    Ok(())
}

#[test]
fn menu_bar_with_unicode_text() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        menu_bar!(
            (Text::new("文件")),
            (Text::new("编辑")),
            (Text::new("🎨 View"))
        )
        .into()
    });

    let ui = simulator(&app);
    process_messages(ui, &mut app);

    let mut ui = simulator(&app);
    assert!(ui.find("文件").is_ok(), "Should find Chinese File menu");
    assert!(ui.find("编辑").is_ok(), "Should find Chinese Edit menu");
    assert!(
        ui.find("🎨 View").is_ok(),
        "Should find View menu with emoji"
    );

    Ok(())
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn menu_bar_with_custom_spacing() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        menu_bar!((Text::new("File")), (Text::new("Edit")))
            .spacing(20.0)
            .into()
    });

    let ui = simulator(&app);
    process_messages(ui, &mut app);

    let mut ui = simulator(&app);
    assert!(
        ui.find("File").is_ok(),
        "Should find File with custom spacing"
    );
    assert!(
        ui.find("Edit").is_ok(),
        "Should find Edit with custom spacing"
    );

    Ok(())
}

#[test]
fn menu_bar_with_custom_padding() -> Result<(), Error> {
    let (mut app, _) = App::new(|| menu_bar!((Text::new("File"))).padding(10.0).into());

    let ui = simulator(&app);
    process_messages(ui, &mut app);

    let mut ui = simulator(&app);
    assert!(
        ui.find("File").is_ok(),
        "Should find File with custom padding"
    );

    Ok(())
}
