//! Integration tests for the MenuBar widget
//!
//! These tests verify the MenuBar widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

// Simulator API https://raw.githubusercontent.com/iced-rs/iced/master/test/src/simulator.rs

use iced::{Element, Settings};
use iced_aw::menu::Menu;
use iced_aw::{menu_bar, menu_items};
use iced_test::{Error, Simulator};
use iced_widget::{button, text::Text};

#[derive(Clone, Debug, PartialEq)]
enum Message {
    FileNew,
    FileOpen,
    FileSave,
    EditCut,
    EditCopy,
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

    fn update(&mut self, _message: Message) {
        // No state changes needed for these tests
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
// Basic MenuBar Tests
// ============================================================================

#[test]
fn can_find_menu_bar_items() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        menu_bar!(
            (Text::new("File")),
            (Text::new("Edit")),
            (Text::new("View"))
        )
        .into()
    });

    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("File").is_ok(), "Should find File menu item");
    assert!(ui.find("Edit").is_ok(), "Should find Edit menu item");
    assert!(ui.find("View").is_ok(), "Should find View menu item");

    Ok(())
}

#[test]
fn menu_bar_with_single_item() -> Result<(), Error> {
    let (mut app, _) = App::new(|| menu_bar!((Text::new("File"))).into());

    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("File").is_ok(), "Should find File menu item");

    Ok(())
}

#[test]
fn menu_bar_with_multiple_items() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        menu_bar!(
            (Text::new("File")),
            (Text::new("Edit")),
            (Text::new("View")),
            (Text::new("Help"))
        )
        .into()
    });

    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }

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
fn menu_bar_with_submenu() -> Result<(), Error> {
    let (mut app, _) = App::new(|| {
        let file_menu = Menu::new(menu_items!(
            (button(Text::new("New")).on_press(Message::FileNew)),
            (button(Text::new("Open")).on_press(Message::FileOpen)),
            (button(Text::new("Save")).on_press(Message::FileSave))
        ));

        menu_bar!((Text::new("File"), file_menu)).into()
    });

    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("File").is_ok(), "Should find File menu item");

    Ok(())
}

#[test]
fn clicking_menu_bar_item_shows_submenu() -> Result<(), Error> {
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
    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(ui.find("File").is_ok(), "Should find File menu");
    assert!(ui.find("Edit").is_ok(), "Should find Edit menu");

    Ok(())
}

// ============================================================================
// Keyboard Navigation Tests
// ============================================================================

#[test]
fn keyboard_navigation_in_menu() -> Result<(), Error> {
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
fn clicking_outside_closes_menu() -> Result<(), Error> {
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
fn menu_with_buttons_and_text() -> Result<(), Error> {
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
    for message in ui.into_messages() {
        app.update(message);
    }

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
    for message in ui.into_messages() {
        app.update(message);
    }

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
    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("File").is_ok(),
        "Should find File with custom padding"
    );

    Ok(())
}
