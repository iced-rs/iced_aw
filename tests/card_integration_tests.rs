//! Integration tests for the Card widget
//!
//! These tests verify the Card widget's behavior by actually exercising
//! the widget through the iced test framework.

use iced::{Color, Element, Length, Padding, Settings, Theme};
use iced_aw::Card;
use iced_test::Simulator;
use iced_widget::text::Text;

#[derive(Clone)]
#[allow(dead_code)]
enum Message {
    CloseCard,
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

    fn view(&self) -> Element<'_, Message> {
        (self.view_fn)()
    }
}

/// Helper to run a test with a given view
fn run_test<F>(view_fn: F)
where
    F: Fn() -> Element<'static, Message> + 'static,
{
    let (app, _) = App::new(view_fn);
    let _ui = Simulator::with_settings(Settings::default(), app.view());
}

/// Helper to verify text can be found (tests operate() implementation)
fn run_test_and_find<F>(view_fn: F, text: &str)
where
    F: Fn() -> Element<'static, Message> + 'static,
{
    let (app, _) = App::new(view_fn);
    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(
        ui.find(text).is_ok(),
        "Failed to find text '{}' in card",
        text
    );
}

#[test]
fn card_basic_renders_and_finds_text() {
    run_test_and_find(
        || Card::new(Text::new("Header"), Text::new("Body")).into(),
        "Header",
    );
}

#[test]
fn card_with_footer_renders() {
    let (app, _) = App::new(|| {
        Card::new(Text::new("Header"), Text::new("Body"))
            .foot(Text::new("Footer"))
            .into()
    });

    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(ui.find("Header").is_ok() && ui.find("Footer").is_ok());
}

#[test]
fn card_with_width_configurations_renders() {
    run_test_and_find(
        || {
            Card::new(Text::new("Fixed"), Text::new("Body"))
                .width(400)
                .into()
        },
        "Fixed",
    );

    run_test_and_find(
        || {
            Card::new(Text::new("Fill"), Text::new("Body"))
                .width(Length::Fill)
                .into()
        },
        "Fill",
    );

    run_test_and_find(
        || {
            Card::new(Text::new("Shrink"), Text::new("Body"))
                .width(Length::Shrink)
                .into()
        },
        "Shrink",
    );

    run_test_and_find(
        || {
            Card::new(Text::new("Portion"), Text::new("Body"))
                .width(Length::FillPortion(2))
                .into()
        },
        "Portion",
    );
}

#[test]
fn card_with_height_configurations_renders() {
    run_test_and_find(
        || {
            Card::new(Text::new("Fixed"), Text::new("Body"))
                .height(300)
                .into()
        },
        "Fixed",
    );

    run_test_and_find(
        || {
            Card::new(Text::new("Fill"), Text::new("Body"))
                .height(Length::Fill)
                .into()
        },
        "Fill",
    );

    run_test_and_find(
        || {
            Card::new(Text::new("Shrink"), Text::new("Body"))
                .height(Length::Shrink)
                .into()
        },
        "Shrink",
    );

    run_test_and_find(
        || {
            Card::new(Text::new("Portion"), Text::new("Body"))
                .height(Length::FillPortion(2))
                .into()
        },
        "Portion",
    );
}

#[test]
fn card_with_max_dimensions_renders() {
    run_test_and_find(
        || {
            Card::new(Text::new("MaxDims"), Text::new("Body"))
                .max_width(500.0)
                .max_height(400.0)
                .into()
        },
        "MaxDims",
    );
}

#[test]
fn card_with_padding_renders() {
    run_test_and_find(
        || {
            Card::new(Text::new("Padded"), Text::new("Body"))
                .padding(Padding::new(10.0))
                .into()
        },
        "Padded",
    );
}

#[test]
fn card_with_section_specific_padding_renders() {
    run_test_and_find(
        || {
            Card::new(Text::new("SectionPadding"), Text::new("Body"))
                .foot(Text::new("Footer"))
                .padding_head(Padding::new(5.0))
                .padding_body(Padding::new(10.0))
                .padding_foot(Padding::new(5.0))
                .into()
        },
        "SectionPadding",
    );
}

#[test]
fn card_with_custom_style_renders() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    run_test_and_find(
        || {
            Card::new(Text::new("Styled"), Text::new("Body"))
                .style(|_theme: &Theme, _status: Status| style::card::Style {
                    background: Background::Color(Color::WHITE),
                    border_radius: 10.0,
                    border_width: 2.0,
                    border_color: Color::from_rgb(0.7, 0.7, 0.7),
                    head_background: Background::Color(Color::from_rgb(0.2, 0.4, 0.8)),
                    head_text_color: Color::WHITE,
                    body_background: Background::Color(Color::WHITE),
                    body_text_color: Color::BLACK,
                    foot_background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
                    foot_text_color: Color::from_rgb(0.3, 0.3, 0.3),
                    close_color: Color::WHITE,
                })
                .into()
        },
        "Styled",
    );
}

#[test]
fn card_with_all_methods_chained_renders() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    run_test_and_find(
        || {
            Card::new(Text::new("Complete"), Text::new("Content"))
                .foot(Text::new("Actions"))
                .width(500)
                .height(400)
                .max_width(800.0)
                .max_height(600.0)
                .padding_head(Padding::new(10.0))
                .padding_body(Padding::new(20.0))
                .padding_foot(Padding::new(10.0))
                .on_close(Message::CloseCard)
                .close_size(18.0)
                .style(|_theme: &Theme, _status: Status| style::card::Style {
                    background: Background::Color(Color::WHITE),
                    border_radius: 12.0,
                    border_width: 2.0,
                    border_color: Color::from_rgb(0.7, 0.7, 0.7),
                    head_background: Background::Color(Color::from_rgb(0.1, 0.3, 0.7)),
                    head_text_color: Color::WHITE,
                    body_background: Background::Color(Color::from_rgb(0.98, 0.98, 0.98)),
                    body_text_color: Color::BLACK,
                    foot_background: Background::Color(Color::from_rgb(0.85, 0.85, 0.85)),
                    foot_text_color: Color::from_rgb(0.3, 0.3, 0.3),
                    close_color: Color::from_rgb(0.9, 0.9, 0.9),
                })
                .class(<Theme as iced_aw::style::card::Catalog>::default())
                .into()
        },
        "Complete",
    );
}

#[test]
fn card_without_on_close_renders() {
    run_test_and_find(
        || Card::new(Text::new("No Close"), Text::new("Body")).into(),
        "No Close",
    );
}

#[test]
fn card_with_on_close_renders() {
    run_test_and_find(
        || {
            Card::new(Text::new("With Close"), Text::new("Body"))
                .on_close(Message::CloseCard)
                .into()
        },
        "With Close",
    );
}

#[test]
fn card_with_empty_content_renders() {
    run_test(|| Card::new(Text::new(""), Text::new("")).into());
}

// ============================================================================
// Close button interaction tests - these verify actual behavioral functionality
// ============================================================================

#[test]
fn card_close_button_can_be_clicked() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct StatefulApp {
        show_card: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new() -> (Self, iced::Task<Message>) {
            (
                StatefulApp {
                    show_card: Rc::new(RefCell::new(true)),
                },
                iced::Task::none(),
            )
        }

        fn update(&mut self, message: Message) {
            match message {
                Message::CloseCard => {
                    *self.show_card.borrow_mut() = false;
                }
            }
        }

        fn view(&self) -> Element<'_, Message> {
            if *self.show_card.borrow() {
                Card::new(Text::new("Closeable"), Text::new("Click to close"))
                    .on_close(Message::CloseCard)
                    .into()
            } else {
                Text::new("Closed").into()
            }
        }
    }

    let (mut app, _) = StatefulApp::new();

    // Verify card is visible
    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(ui.find("Closeable").is_ok());

    // Click close button
    let _ = ui.click("\u{e800}");

    // Process messages
    for message in ui.into_messages() {
        app.update(message);
    }

    // Verify card is closed
    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(ui.find("Closed").is_ok());
    assert!(ui.find("Closeable").is_err());
}

#[test]
fn card_close_button_can_be_clicked_multiple_times() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct CountingApp {
        count: Rc<RefCell<usize>>,
    }

    impl CountingApp {
        fn new() -> (Self, iced::Task<Message>) {
            (
                CountingApp {
                    count: Rc::new(RefCell::new(0)),
                },
                iced::Task::none(),
            )
        }

        fn update(&mut self, message: Message) {
            match message {
                Message::CloseCard => {
                    *self.count.borrow_mut() += 1;
                }
            }
        }

        fn view(&self) -> Element<'_, Message> {
            let count = *self.count.borrow();
            Card::new(
                Text::new("Counter"),
                Text::new(format!("Clicked {} times", count)),
            )
            .on_close(Message::CloseCard)
            .into()
        }
    }

    let (mut app, _) = CountingApp::new();

    // Click 3 times
    for _ in 0..3 {
        let mut ui = Simulator::with_settings(Settings::default(), app.view());
        let _ = ui.click("\u{e800}");

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    // Verify count
    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(ui.find("Clicked 3 times").is_ok());
}

#[test]
fn card_with_custom_close_size_can_be_clicked() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct StatefulApp {
        show_card: Rc<RefCell<bool>>,
    }

    impl StatefulApp {
        fn new() -> (Self, iced::Task<Message>) {
            (
                StatefulApp {
                    show_card: Rc::new(RefCell::new(true)),
                },
                iced::Task::none(),
            )
        }

        fn update(&mut self, message: Message) {
            match message {
                Message::CloseCard => {
                    *self.show_card.borrow_mut() = false;
                }
            }
        }

        fn view(&self) -> Element<'_, Message> {
            if *self.show_card.borrow() {
                Card::new(Text::new("Custom Size"), Text::new("Body"))
                    .on_close(Message::CloseCard)
                    .close_size(32.0)
                    .into()
            } else {
                Text::new("Closed").into()
            }
        }
    }

    let (mut app, _) = StatefulApp::new();

    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(ui.find("Custom Size").is_ok());

    let _ = ui.click("\u{e800}");

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(ui.find("Closed").is_ok());
}
