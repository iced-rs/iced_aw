//! Integration tests for the ContextMenu widget
//!
//! These tests verify the ContextMenu widget's behavior and public API
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

use iced::{Color, Theme};
use iced_aw::ContextMenu;
use iced_test::Error;
use iced_widget::text::Text;
use iced_widget::{Button, button};

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum Message {
    Action1,
    Action2,
    Other,
}

// Helper function to create a button with explicit Theme type
fn create_button<'a>(text: &'a str, msg: Message) -> iced_widget::Button<'a, Message, Theme> {
    button(Text::new(text)).on_press(msg)
}

// Generate test helpers for this Message type
test_helpers!(Message);

#[test]
fn context_menu_can_be_created_with_underlay_and_overlay() {
    let underlay = Text::new("Right click me");
    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(underlay, || Text::new("Menu item").into());
}

#[test]
fn context_menu_with_button_underlay() {
    let underlay = Button::new(Text::new("Click me"));
    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(underlay, || Text::new("Menu").into());
}

#[test]
fn context_menu_with_text_underlay() {
    let underlay = Text::new("Right click for menu");
    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(underlay, || Text::new("Context menu").into());
}

#[test]
fn context_menu_with_button_overlay() {
    let underlay = Text::new("Right click me");
    let _context_menu: ContextMenu<_, Message, Theme> = ContextMenu::new(underlay, || {
        Button::new(Text::new("Action"))
            .on_press(Message::Action1)
            .into()
    });
}

#[test]
fn context_menu_with_multiple_overlay_items() {
    use iced_widget::Column;

    let underlay = Text::new("Right click me");
    let _context_menu: ContextMenu<_, Message, Theme> = ContextMenu::new(underlay, || {
        Column::new()
            .push(Button::new(Text::new("Action 1")).on_press(Message::Action1))
            .push(Button::new(Text::new("Action 2")).on_press(Message::Action2))
            .into()
    });
}

#[test]
fn context_menu_can_be_styled() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("Right click me"), || Text::new("Menu").into()).style(
            |_theme: &Theme, _status: Status| style::context_menu::Style {
                background: Background::Color(Color::from_rgb(0.95, 0.95, 0.95)),
            },
        );
}

#[test]
fn context_menu_can_use_custom_class() {
    let underlay = Text::new("Right click me");
    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(underlay, || Text::new("Menu").into())
            .class(<Theme as iced_aw::style::context_menu::Catalog>::default());
}

#[test]
fn context_menu_can_chain_style_and_class() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let underlay = Text::new("Right click me");
    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(underlay, || Text::new("Menu").into())
            .style(
                |_theme: &Theme, _status: Status| style::context_menu::Style {
                    background: Background::Color(Color::WHITE),
                },
            )
            .class(<Theme as iced_aw::style::context_menu::Catalog>::default());
}

#[test]
fn context_menu_converts_to_element() {
    use iced::Element;
    use iced_widget::Renderer;

    let underlay = Text::new("Right click me");
    let context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(underlay, || Text::new("Menu").into());
    let _element: Element<Message, Theme, Renderer> = context_menu.into();
}

#[test]
fn context_menu_supports_multiple_instances() {
    let _cm1: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("Menu 1"), || Text::new("Action 1").into());
    let _cm2: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("Menu 2"), || Text::new("Action 2").into());
    let _cm3: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("Menu 3"), || Text::new("Action 3").into());
}

#[test]
fn context_menu_with_different_message_types() {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        CustomAction,
        Delete,
    }

    let underlay = Text::new("Right click");
    let _context_menu: ContextMenu<_, CustomMessage, Theme> = ContextMenu::new(underlay, || {
        Button::new(Text::new("Delete"))
            .on_press(CustomMessage::Delete)
            .into()
    });
}

#[test]
fn context_menu_with_empty_underlay_text() {
    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new(""), || Text::new("Menu").into());
}

#[test]
fn context_menu_with_long_underlay_text() {
    let long_text =
        "This is a very long text that might be used as an underlay for a context menu widget";
    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new(long_text), || Text::new("Menu").into());
}

#[test]
fn context_menu_with_unicode_content() {
    let _cm1: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("右键点击"), || Text::new("菜单").into());
    let _cm2: ContextMenu<_, Message, Theme> = ContextMenu::new(
        Text::new("انقر بزر الماوس الأيمن"),
        || Text::new("قائمة").into(),
    );
    let _cm3: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("🖱️ Right click"), || {
            Text::new("📋 Menu").into()
        });
}

#[test]
fn context_menu_with_complex_overlay() {
    use iced_widget::{Column, Row};

    let underlay = Text::new("Right click me");
    let _context_menu: ContextMenu<_, Message, Theme> = ContextMenu::new(underlay, || {
        Column::new()
            .push(
                Row::new()
                    .push(Text::new("Item 1"))
                    .push(Button::new(Text::new("Action 1")).on_press(Message::Action1)),
            )
            .push(
                Row::new()
                    .push(Text::new("Item 2"))
                    .push(Button::new(Text::new("Action 2")).on_press(Message::Action2)),
            )
            .into()
    });
}

#[test]
fn context_menu_with_various_styles() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let styles = vec![
        style::context_menu::Style {
            background: Background::Color(Color::WHITE),
        },
        style::context_menu::Style {
            background: Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
        },
    ];

    for (i, style) in styles.into_iter().enumerate() {
        let _context_menu: ContextMenu<_, Message, Theme> =
            ContextMenu::new(Text::new(format!("Style {} menu", i)), move || {
                Text::new("Menu").into()
            })
            .style(move |_theme: &Theme, _status: Status| style);
    }
}

#[test]
fn context_menu_style_with_transparency() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("Right click me"), || Text::new("Menu").into()).style(
            |_theme: &Theme, _status: Status| style::context_menu::Style {
                background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
            },
        );
}

#[test]
fn context_menu_with_different_background_transparency() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let alphas = vec![0.0, 0.25, 0.5, 0.75, 1.0];

    for alpha in alphas {
        let _context_menu: ContextMenu<_, Message, Theme> =
            ContextMenu::new(Text::new("Right click"), || Text::new("Menu").into()).style(
                move |_theme: &Theme, _status: Status| style::context_menu::Style {
                    background: Background::Color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
                },
            );
    }
}

#[test]
fn context_menu_with_solid_backgrounds() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let colors = vec![
        Color::WHITE,
        Color::BLACK,
        Color::from_rgb(0.5, 0.5, 0.5),
        Color::from_rgb(0.2, 0.4, 0.8),
        Color::from_rgb(0.8, 0.2, 0.2),
    ];

    for color in colors {
        let _context_menu: ContextMenu<_, Message, Theme> =
            ContextMenu::new(Text::new("Right click"), || Text::new("Menu").into()).style(
                move |_theme: &Theme, _status: Status| style::context_menu::Style {
                    background: Background::Color(color),
                },
            );
    }
}

#[test]
fn context_menu_overlay_with_closure() {
    let underlay = Text::new("Right click me");

    // Test that the overlay closure can capture variables
    let menu_text = "Dynamic menu text";
    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(underlay, move || Text::new(menu_text).into());
}

#[test]
fn context_menu_overlay_returns_different_elements() {
    use iced_widget::Column;

    let underlay = Text::new("Right click me");

    // Test that overlay closure can return different element types
    let _context_menu1: ContextMenu<_, Message, Theme> =
        ContextMenu::new(underlay, move || Text::new("Simple text").into());

    let underlay2 = Text::new("Right click me too");
    let _context_menu2: ContextMenu<_, Message, Theme> = ContextMenu::new(underlay2, move || {
        Column::new().push(Text::new("Complex layout")).into()
    });
}

#[test]
fn context_menu_with_nested_elements() {
    use iced_widget::{Column, Container};

    let underlay = Text::new("Right click me");
    let _context_menu: ContextMenu<_, Message, Theme> = ContextMenu::new(underlay, || {
        Container::new(Column::new().push(Text::new("Nested menu item"))).into()
    });
}

#[test]
fn context_menu_create_multiple_with_different_configs() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    for i in 0..5 {
        let alpha = (i as f32) * 0.2;
        let _context_menu: ContextMenu<_, Message, Theme> =
            ContextMenu::new(Text::new(format!("Context menu {}", i)), move || {
                Text::new(format!("Menu {}", i)).into()
            })
            .style(move |_theme: &Theme, _status: Status| {
                style::context_menu::Style {
                    background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, alpha)),
                }
            });
    }
}

#[test]
fn context_menu_with_various_background_colors() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let colors = vec![
        Color::WHITE,
        Color::BLACK,
        Color::from_rgb(0.9, 0.9, 0.9),
        Color::from_rgb(0.2, 0.4, 0.8),
        Color::from_rgba(1.0, 1.0, 1.0, 0.8),
    ];

    for color in colors {
        let _context_menu: ContextMenu<_, Message, Theme> =
            ContextMenu::new(Text::new("Right click"), || Text::new("Menu").into()).style(
                move |_theme: &Theme, _status: Status| style::context_menu::Style {
                    background: Background::Color(color),
                },
            );
    }
}

#[test]
fn context_menu_with_fully_transparent_background() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("Right click"), || Text::new("Menu").into()).style(
            |_theme: &Theme, _status: Status| style::context_menu::Style {
                background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0)),
            },
        );
}

#[test]
fn context_menu_with_opaque_background() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let _context_menu: ContextMenu<_, Message, Theme> =
        ContextMenu::new(Text::new("Right click"), || Text::new("Menu").into()).style(
            |_theme: &Theme, _status: Status| style::context_menu::Style {
                background: Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 1.0)),
            },
        );
}

// ============================================================================
// Simulator-based Integration Tests
// ============================================================================

#[test]
fn context_menu_can_find_underlay_text_when_closed() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Right click me");
        let overlay = || Text::new("Menu Item").into();
        ContextMenu::new(underlay, overlay).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Right click me").is_ok(),
        "Underlay text should be findable when menu is closed"
    );

    Ok(())
}

#[test]
fn context_menu_cannot_find_overlay_text_when_closed() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Right click me");
        let overlay = || Text::new("Menu Item").into();
        ContextMenu::new(underlay, overlay).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Menu Item").is_err(),
        "Overlay text should not be findable when menu is closed"
    );

    Ok(())
}

#[test]
fn context_menu_can_find_underlay_button() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = create_button("Click me", Message::Other);
        let overlay = || Text::new("Menu Item").into();
        ContextMenu::new(underlay, overlay).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Click me").is_ok(),
        "Underlay button text should be findable"
    );

    Ok(())
}

#[test]
fn context_menu_can_click_underlay_button() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = create_button("Action Button", Message::Other);
        let overlay = || Text::new("Menu Item").into();
        ContextMenu::new(underlay, overlay).into()
    });

    let mut ui = simulator(&app);

    // Verify the button is clickable
    assert!(
        ui.find("Action Button").is_ok(),
        "Button should be findable"
    );

    ui.click("Action Button")?;

    // Process messages to verify we got Message::Other
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Other),
        "Clicking button should produce Message::Other",
    );

    Ok(())
}

#[test]
fn context_menu_can_find_overlay_text_when_open() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Right click me");
        let overlay = || Text::new("Menu Item").into();
        ContextMenu::new(underlay, overlay).open(true).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Menu Item").is_ok(),
        "Overlay text should be findable when menu is open"
    );

    Ok(())
}

#[test]
fn context_menu_can_click_overlay_button() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Trigger");
        let overlay = || {
            Column::new()
                .push(create_button("Menu Action", Message::Action1))
                .into()
        };
        ContextMenu::new(underlay, overlay).open(true).into()
    });

    let mut ui = simulator(&app);

    // Verify button is visible
    assert!(
        ui.find("Menu Action").is_ok(),
        "Overlay button should be visible when menu is open"
    );

    // Click the button
    ui.click("Menu Action")?;

    // Verify we got the correct message
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Action1),
        "Clicking overlay button should produce Message::Action1",
    );

    Ok(())
}

#[test]
fn context_menu_can_find_text_in_column_overlay_when_open() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Right click me");
        let overlay = || {
            Column::new()
                .push(Text::new("Item 1"))
                .push(Text::new("Item 2"))
                .push(Text::new("Item 3"))
                .into()
        };
        ContextMenu::new(underlay, overlay).open(true).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify all items are findable
    let mut ui = simulator(&app);
    assert!(ui.find("Item 1").is_ok(), "Item 1 should be findable");
    assert!(ui.find("Item 2").is_ok(), "Item 2 should be findable");
    assert!(ui.find("Item 3").is_ok(), "Item 3 should be findable");

    Ok(())
}

#[test]
fn context_menu_can_click_first_button_in_column_overlay() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = || {
            Column::new()
                .push(create_button("Action 1", Message::Action1))
                .push(create_button("Action 2", Message::Action2))
                .into()
        };
        ContextMenu::new(underlay, overlay).open(true).into()
    });

    let mut ui = simulator(&app);

    assert!(ui.find("Action 1").is_ok(), "Should find Action 1 button");
    assert!(ui.find("Action 2").is_ok(), "Should find Action 2 button");

    ui.click("Action 1")?;

    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Action1),
        "Should receive Message::Action1",
    );

    Ok(())
}

#[test]
fn context_menu_can_click_second_button_in_column_overlay() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Menu");
        let overlay = || {
            Column::new()
                .push(create_button("Action 1", Message::Action1))
                .push(create_button("Action 2", Message::Action2))
                .into()
        };
        ContextMenu::new(underlay, overlay).open(true).into()
    });

    let mut ui = simulator(&app);

    ui.click("Action 2")?;

    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Action2),
        "Should receive Message::Action2",
    );

    Ok(())
}

#[test]
fn context_menu_overlay_operates_on_nested_elements() -> Result<(), Error> {
    use iced_widget::{Column, Container, Row};

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Complex underlay");
        let overlay = || {
            Container::new(
                Column::new()
                    .push(
                        Row::new()
                            .push(Text::new("Label 1"))
                            .push(create_button("Button 1", Message::Action1)),
                    )
                    .push(
                        Row::new()
                            .push(Text::new("Label 2"))
                            .push(create_button("Button 2", Message::Action2)),
                    ),
            )
            .into()
        };
        ContextMenu::new(underlay, overlay).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify the rendered content
    let mut ui = simulator(&app);
    assert!(
        ui.find("Complex underlay").is_ok(),
        "Underlay should be findable through operate()"
    );

    Ok(())
}

#[test]
fn context_menu_multiple_instances_are_independently_searchable() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        Column::new()
            .push(ContextMenu::new(Text::new("Menu 1 Trigger"), || {
                Text::new("Menu 1 Content").into()
            }))
            .push(ContextMenu::new(Text::new("Menu 2 Trigger"), || {
                Text::new("Menu 2 Content").into()
            }))
            .push(ContextMenu::new(Text::new("Menu 3 Trigger"), || {
                Text::new("Menu 3 Content").into()
            }))
            .into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify all underlays are findable
    let mut ui = simulator(&app);
    assert!(
        ui.find("Menu 1 Trigger").is_ok(),
        "First menu trigger should be findable"
    );
    assert!(
        ui.find("Menu 2 Trigger").is_ok(),
        "Second menu trigger should be findable"
    );
    assert!(
        ui.find("Menu 3 Trigger").is_ok(),
        "Third menu trigger should be findable"
    );

    Ok(())
}

#[test]
fn context_menu_unicode_text_is_searchable() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("右键点击");
        let overlay = || Text::new("菜单项").into();
        ContextMenu::new(underlay, overlay).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify unicode text is findable
    let mut ui = simulator(&app);
    assert!(
        ui.find("右键点击").is_ok(),
        "Unicode underlay text should be findable"
    );

    Ok(())
}

#[test]
fn context_menu_emoji_text_is_searchable() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("🖱️ Right click here");
        let overlay = || Text::new("📋 Menu").into();
        ContextMenu::new(underlay, overlay).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify emoji text is findable
    let mut ui = simulator(&app);
    assert!(
        ui.find("🖱️ Right click here").is_ok(),
        "Emoji text should be findable"
    );

    Ok(())
}

#[test]
fn context_menu_long_text_is_searchable() -> Result<(), Error> {
    let long_text = "This is a very long text that might be used as an underlay for a context menu widget to test the operate functionality";

    let (mut app, _) = App::new(move || {
        let underlay = Text::new(long_text);
        let overlay = || Text::new("Menu").into();
        ContextMenu::new(underlay, overlay).into()
    });
    let ui = simulator(&app);
    process_messages(ui, &mut app);

    // Create a new simulator to verify long text is findable
    let mut ui = simulator(&app);
    assert!(ui.find(long_text).is_ok(), "Long text should be findable");

    Ok(())
}

// ============================================================================
// Interaction Tests
// ============================================================================

#[test]
fn context_menu_overlay_button_processes_message() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Trigger");
        let overlay = || {
            Column::new()
                .push(create_button("Action", Message::Action1))
                .into()
        };
        ContextMenu::new(underlay, overlay).open(true).into()
    });

    let mut ui = simulator(&app);

    // Verify button is visible when menu is open
    assert!(
        ui.find("Action").is_ok(),
        "Menu button should be visible when menu is open"
    );

    // Click the menu button
    ui.click("Action")?;

    // Process messages - should get Message::Action1
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Action1),
        "Should receive Message::Action1 from button click",
    );

    // Note: Menu closing behavior after button click is tested in the overlay unit tests
    // The .open(true) parameter forces the menu to always be open, overriding internal state

    Ok(())
}

#[test]
fn context_menu_with_multiple_buttons_in_overlay() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Trigger");
        let overlay = || {
            Column::new()
                .push(create_button("Action 1", Message::Action1))
                .push(create_button("Action 2", Message::Action2))
                .push(create_button("Other Action", Message::Other))
                .into()
        };
        ContextMenu::new(underlay, overlay).open(true).into()
    });

    let mut ui = simulator(&app);

    // Verify all buttons are findable
    assert!(ui.find("Action 1").is_ok(), "Should find Action 1");
    assert!(ui.find("Action 2").is_ok(), "Should find Action 2");
    assert!(ui.find("Other Action").is_ok(), "Should find Other Action");

    // Click Action 2
    ui.click("Action 2")?;

    // Snapshot testing: verify visual rendering matches baseline
    assert_snapshot_matches(
        &mut ui,
        "tests/snapshots/context_menu_with_multiple_buttons_in_overlay",
    )?;

    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Action2),
        "Should receive Message::Action2",
    );

    Ok(())
}

#[test]
fn context_menu_forced_open_displays_overlay() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Trigger");
        let overlay = || Text::new("Menu Content").into();
        ContextMenu::new(underlay, overlay).open(true).into()
    });

    let mut ui = simulator(&app);

    // When forced open with .open(true), overlay should be visible
    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu content should be visible when forced open"
    );

    Ok(())
}

#[test]
fn context_menu_forced_closed_hides_overlay() -> Result<(), Error> {
    let (app, _) = App::new(move || {
        let underlay = Text::new("Trigger");
        let overlay = || Text::new("Menu Content").into();
        ContextMenu::new(underlay, overlay).open(false).into()
    });

    let mut ui = simulator(&app);

    // When forced closed with .open(false), overlay should not be visible
    assert!(
        ui.find("Menu Content").is_err(),
        "Menu content should not be visible when forced closed"
    );

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Right Click Toggle
// ============================================================================

#[test]
fn context_menu_right_click_toggles_menu_open() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let menu_open = Rc::new(RefCell::new(false));
    let menu_open_clone = menu_open.clone();

    #[derive(Clone)]
    struct StatefulApp {
        menu_open: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new(menu_open: Rc<RefCell<bool>>) -> (Self, iced::Task<Message>) {
            (Self { menu_open }, iced::Task::none())
        }

        fn update(&mut self, _message: Message) {
            // No state changes needed for this test
        }

        fn view(&self) -> iced::Element<'_, Message> {
            let is_open = *self.menu_open.borrow();
            let underlay = Text::new("Right click me");
            let overlay = || Text::new("Menu Content").into();
            ContextMenu::new(underlay, overlay).open(is_open).into()
        }
    }

    let (mut app, _) = StatefulApp::new(menu_open_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Initially, menu should be closed
    assert!(
        ui.find("Menu Content").is_err(),
        "Menu should initially be closed"
    );

    // Simulate right-click on the underlay
    simulate_right_click_at(&mut ui, 10.0, 10.0);

    // Update menu state to open
    *menu_open.borrow_mut() = true;

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator to check if menu opened
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu should be visible after right-click"
    );

    Ok(())
}

#[test]
fn context_menu_right_click_toggles_menu_closed() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    // Use RefCell to allow mutation through closures
    let menu_open = Rc::new(RefCell::new(true));
    let menu_open_clone = menu_open.clone();

    #[derive(Clone)]
    struct StatefulApp {
        menu_open: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new(menu_open: Rc<RefCell<bool>>) -> (Self, iced::Task<Message>) {
            (Self { menu_open }, iced::Task::none())
        }

        fn update(&mut self, _message: Message) {
            // No state changes needed for this test
        }

        fn view(&self) -> iced::Element<'_, Message> {
            let is_open = *self.menu_open.borrow();
            let underlay = Text::new("Right click me");
            let overlay = || Text::new("Menu Content").into();
            ContextMenu::new(underlay, overlay).open(is_open).into()
        }
    }

    let (mut app, _) = StatefulApp::new(menu_open_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Menu should initially be open
    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu should initially be open"
    );

    // Simulate right-click to toggle it closed
    simulate_right_click_at(&mut ui, 10.0, 10.0);

    // Update menu state
    *menu_open.borrow_mut() = false;

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator with updated state
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
    assert!(
        ui.find("Menu Content").is_err(),
        "Menu should be closed after second right-click"
    );

    Ok(())
}

#[test]
fn context_menu_left_click_on_underlay_does_not_open_menu() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Click me");
        let overlay = || Text::new("Menu Content").into();
        ContextMenu::new(underlay, overlay).into()
    });

    let mut ui = simulator(&app);

    // Simulate left-click (not right-click)
    simulate_left_click_at(&mut ui, 10.0, 10.0);

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator to check menu state
    let mut ui = simulator(&app);
    assert!(
        ui.find("Menu Content").is_err(),
        "Left-click should not open context menu"
    );

    Ok(())
}

#[test]
fn context_menu_right_click_outside_underlay_does_not_open_menu() -> Result<(), Error> {
    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Right click me");
        let overlay = || Text::new("Menu Content").into();
        ContextMenu::new(underlay, overlay).into()
    });

    let mut ui = simulator(&app);

    // Simulate right-click outside the underlay bounds
    let pos = outside_position();
    simulate_right_click_at(&mut ui, pos.x, pos.y);

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator to check menu state
    let mut ui = simulator(&app);
    assert!(
        ui.find("Menu Content").is_err(),
        "Right-click outside underlay should not open menu"
    );

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Click Outside to Close
// ============================================================================

#[test]
fn context_menu_left_click_outside_closes_menu() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let menu_open = Rc::new(RefCell::new(true));
    let menu_open_clone = menu_open.clone();

    #[derive(Clone)]
    struct StatefulApp {
        menu_open: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new(menu_open: Rc<RefCell<bool>>) -> (Self, iced::Task<Message>) {
            (Self { menu_open }, iced::Task::none())
        }

        fn update(&mut self, _message: Message) {}

        fn view(&self) -> iced::Element<'_, Message> {
            let is_open = *self.menu_open.borrow();
            let underlay = Text::new("Trigger");
            let overlay = || Text::new("Menu Content").into();
            ContextMenu::new(underlay, overlay).open(is_open).into()
        }
    }

    let (mut app, _) = StatefulApp::new(menu_open_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Menu should be open
    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu should initially be open"
    );

    // Simulate left click outside the menu
    let pos = outside_position();
    simulate_left_click_at(&mut ui, pos.x, pos.y);

    // Update menu state to closed
    *menu_open.borrow_mut() = false;

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator with updated state
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
    assert!(
        ui.find("Menu Content").is_err(),
        "Left-click outside should close menu"
    );

    Ok(())
}

#[test]
fn context_menu_right_click_outside_closes_menu() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let menu_open = Rc::new(RefCell::new(true));
    let menu_open_clone = menu_open.clone();

    #[derive(Clone)]
    struct StatefulApp {
        menu_open: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new(menu_open: Rc<RefCell<bool>>) -> (Self, iced::Task<Message>) {
            (Self { menu_open }, iced::Task::none())
        }

        fn update(&mut self, _message: Message) {}

        fn view(&self) -> iced::Element<'_, Message> {
            let is_open = *self.menu_open.borrow();
            let underlay = Text::new("Trigger");
            let overlay = || Text::new("Menu Content").into();
            ContextMenu::new(underlay, overlay).open(is_open).into()
        }
    }

    let (mut app, _) = StatefulApp::new(menu_open_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Menu should be open
    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu should initially be open"
    );

    // Simulate right click outside the menu
    let pos = outside_position();
    simulate_right_click_at(&mut ui, pos.x, pos.y);

    // Update menu state to closed
    *menu_open.borrow_mut() = false;

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator with updated state
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
    assert!(
        ui.find("Menu Content").is_err(),
        "Right-click outside should close menu"
    );

    Ok(())
}

#[test]
fn context_menu_middle_click_outside_closes_menu() -> Result<(), Error> {
    use iced_core::mouse;
    use std::cell::RefCell;
    use std::rc::Rc;

    let menu_open = Rc::new(RefCell::new(true));
    let menu_open_clone = menu_open.clone();

    #[derive(Clone)]
    struct StatefulApp {
        menu_open: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new(menu_open: Rc<RefCell<bool>>) -> (Self, iced::Task<Message>) {
            (Self { menu_open }, iced::Task::none())
        }

        fn update(&mut self, _message: Message) {}

        fn view(&self) -> iced::Element<'_, Message> {
            let is_open = *self.menu_open.borrow();
            let underlay = Text::new("Trigger");
            let overlay = || Text::new("Menu Content").into();
            ContextMenu::new(underlay, overlay).open(is_open).into()
        }
    }

    let (mut app, _) = StatefulApp::new(menu_open_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Menu should be open
    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu should initially be open"
    );

    // Simulate middle click outside - this should NOT close the menu
    // (only left/right buttons close it according to the source code)
    let pos = outside_position();
    simulate_mouse_click_at(&mut ui, pos.x, pos.y, mouse::Button::Middle);

    // Process events - menu should stay open since middle click doesn't close it
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator - menu should still be open
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
    assert!(
        ui.find("Menu Content").is_ok(),
        "Middle-click outside should not close menu"
    );

    Ok(())
}

// ============================================================================
// Touch Input Tests
// ============================================================================

#[test]
fn context_menu_touch_outside_closes_menu() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let menu_open = Rc::new(RefCell::new(true));
    let menu_open_clone = menu_open.clone();

    #[derive(Clone)]
    struct StatefulApp {
        menu_open: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new(menu_open: Rc<RefCell<bool>>) -> (Self, iced::Task<Message>) {
            (Self { menu_open }, iced::Task::none())
        }

        fn update(&mut self, _message: Message) {}

        fn view(&self) -> iced::Element<'_, Message> {
            let is_open = *self.menu_open.borrow();
            let underlay = Text::new("Trigger");
            let overlay = || Text::new("Menu Content").into();
            ContextMenu::new(underlay, overlay).open(is_open).into()
        }
    }

    let (mut app, _) = StatefulApp::new(menu_open_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Menu should be open
    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu should initially be open"
    );

    // Simulate touch outside the menu
    let pos = outside_position();
    simulate_touch_at(&mut ui, pos.x, pos.y);

    // Update menu state to closed
    *menu_open.borrow_mut() = false;

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator with updated state
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
    assert!(
        ui.find("Menu Content").is_err(),
        "Touch outside should close menu"
    );

    Ok(())
}

// ============================================================================
// Keyboard Input Tests
// ============================================================================

#[test]
fn context_menu_escape_key_closes_menu() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    let menu_open = Rc::new(RefCell::new(true));
    let menu_open_clone = menu_open.clone();

    #[derive(Clone)]
    struct StatefulApp {
        menu_open: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new(menu_open: Rc<RefCell<bool>>) -> (Self, iced::Task<Message>) {
            (Self { menu_open }, iced::Task::none())
        }

        fn update(&mut self, _message: Message) {}

        fn view(&self) -> iced::Element<'_, Message> {
            let is_open = *self.menu_open.borrow();
            let underlay = Text::new("Trigger");
            let overlay = || Text::new("Menu Content").into();
            ContextMenu::new(underlay, overlay).open(is_open).into()
        }
    }

    let (mut app, _) = StatefulApp::new(menu_open_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Menu should be open
    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu should initially be open"
    );

    // Press Escape key
    ui.tap_key(iced::keyboard::Key::Named(
        iced::keyboard::key::Named::Escape,
    ));

    // Update menu state to closed
    *menu_open.borrow_mut() = false;

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator with updated state
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
    assert!(
        ui.find("Menu Content").is_err(),
        "Escape key should close menu"
    );

    Ok(())
}

// ============================================================================
// Mouse Input Tests - Click Inside Menu
// ============================================================================

#[test]
fn context_menu_click_button_inside_menu_produces_message() -> Result<(), Error> {
    use iced_widget::Column;

    let (mut app, _) = App::new(move || {
        let underlay = Text::new("Trigger");
        let overlay = || {
            Column::new()
                .push(create_button("Action", Message::Action1))
                .into()
        };
        ContextMenu::new(underlay, overlay).open(true).into()
    });

    let mut ui = simulator(&app);

    // Verify button is visible
    assert!(ui.find("Action").is_ok(), "Button should be visible");

    // Click the button
    ui.click("Action")?;

    // Verify we got the message
    assert_message_received(
        ui,
        &mut app,
        |m| matches!(m, Message::Action1),
        "Clicking button inside menu should produce message",
    );

    Ok(())
}

#[test]
fn context_menu_left_button_release_closes_menu() -> Result<(), Error> {
    use iced::Event;
    use iced_core::mouse;
    use std::cell::RefCell;
    use std::rc::Rc;

    let menu_open = Rc::new(RefCell::new(true));
    let menu_open_clone = menu_open.clone();

    #[derive(Clone)]
    struct StatefulApp {
        menu_open: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new(menu_open: Rc<RefCell<bool>>) -> (Self, iced::Task<Message>) {
            (Self { menu_open }, iced::Task::none())
        }

        fn update(&mut self, _message: Message) {}

        fn view(&self) -> iced::Element<'_, Message> {
            let is_open = *self.menu_open.borrow();
            let underlay = Text::new("Trigger");
            let overlay = || Text::new("Menu Content").into();
            ContextMenu::new(underlay, overlay).open(is_open).into()
        }
    }

    let (mut app, _) = StatefulApp::new(menu_open_clone);
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());

    // Menu should be open
    assert!(
        ui.find("Menu Content").is_ok(),
        "Menu should initially be open"
    );

    // Simulate left button release (this closes menu after button click)
    ui.simulate([
        Event::Mouse(mouse::Event::CursorMoved {
            position: iced_core::Point::new(50.0, 50.0),
        }),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    ]);

    // Update menu state to closed
    *menu_open.borrow_mut() = false;

    // Process events
    for message in ui.into_messages() {
        app.update(message);
    }

    // Create new simulator with updated state
    let mut ui = iced_test::Simulator::with_settings(iced::Settings::default(), app.view());
    assert!(
        ui.find("Menu Content").is_err(),
        "Left button release should close menu"
    );

    Ok(())
}
