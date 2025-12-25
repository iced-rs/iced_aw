//! Integration tests for the ContextMenu widget
//!
//! These tests verify the ContextMenu widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::{Color, Theme};
use iced_aw::ContextMenu;
use iced_widget::Button;
use iced_widget::text::Text;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum Message {
    Action1,
    Action2,
    Other,
}

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
