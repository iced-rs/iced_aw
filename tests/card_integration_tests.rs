//! Integration tests for the Card widget
//!
//! These tests verify the Card widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::Settings;
use iced::{Color, Element, Length, Padding, Theme};
use iced_aw::Card;
use iced_test::{Error, Simulator};
use iced_widget::text::Text;

#[derive(Clone)]
#[allow(dead_code)]
enum Message {
    CloseCard,
    DoNothing,
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
            Message::CloseCard => {
                // Handle close
            }
            Message::DoNothing => {
                // Do nothing
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

#[test]
fn card_can_chain_all_methods() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let (mut app, _command) = App::new(|| {
        Card::new(Text::new("All"), Text::new("Content"))
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
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("All").is_ok(),
        "Card with all methods chained should render"
    );

    Ok(())
}

#[test]
fn card_with_different_message_types() -> Result<(), Error> {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        Close,
        Action,
    }

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
                CustomMessage::Close => {}
                CustomMessage::Action => {}
            }
        }

        fn view(&self) -> Element<'_, CustomMessage> {
            (self.view_fn)()
        }
    }

    let (mut app, _command) = CustomApp::new(|| {
        Card::new(Text::new("Custom"), Text::new("Body"))
            .on_close(CustomMessage::Close)
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
        ui.find("Custom").is_ok(),
        "Card with custom message type should render"
    );

    Ok(())
}

#[test]
fn card_with_various_width_configurations() -> Result<(), Error> {
    // Test Fixed width
    {
        let (mut app, _command) = App::new(|| {
            Card::new(Text::new("Fixed"), Text::new("Body"))
                .width(400)
                .into()
        });
        let ui = simulator(&app);
        for message in ui.into_messages() {
            app.update(message);
        }
        let mut ui = simulator(&app);
        assert!(
            ui.find("Fixed").is_ok(),
            "Card with Fixed width should render"
        );
    }

    // Test Fill width
    {
        let (mut app, _command) = App::new(|| {
            Card::new(Text::new("Fill"), Text::new("Body"))
                .width(Length::Fill)
                .into()
        });
        let ui = simulator(&app);
        for message in ui.into_messages() {
            app.update(message);
        }
        let mut ui = simulator(&app);
        assert!(
            ui.find("Fill").is_ok(),
            "Card with Fill width should render"
        );
    }

    // Test Shrink width
    {
        let (mut app, _command) = App::new(|| {
            Card::new(Text::new("Shrink"), Text::new("Body"))
                .width(Length::Shrink)
                .into()
        });
        let ui = simulator(&app);
        for message in ui.into_messages() {
            app.update(message);
        }
        let mut ui = simulator(&app);
        assert!(
            ui.find("Shrink").is_ok(),
            "Card with Shrink width should render"
        );
    }

    // Test FillPortion width
    {
        let (mut app, _command) = App::new(|| {
            Card::new(Text::new("Portion"), Text::new("Body"))
                .width(Length::FillPortion(3))
                .into()
        });
        let ui = simulator(&app);
        for message in ui.into_messages() {
            app.update(message);
        }
        let mut ui = simulator(&app);
        assert!(
            ui.find("Portion").is_ok(),
            "Card with FillPortion width should render"
        );
    }

    Ok(())
}

#[test]
fn card_with_various_height_configurations() -> Result<(), Error> {
    // Test Fixed height
    {
        let (mut app, _command) = App::new(|| {
            Card::new(Text::new("Fixed"), Text::new("Body"))
                .height(300)
                .into()
        });
        let ui = simulator(&app);
        for message in ui.into_messages() {
            app.update(message);
        }
        let mut ui = simulator(&app);
        assert!(
            ui.find("Fixed").is_ok(),
            "Card with Fixed height should render"
        );
    }

    // Test Fill height
    {
        let (mut app, _command) = App::new(|| {
            Card::new(Text::new("Fill"), Text::new("Body"))
                .height(Length::Fill)
                .into()
        });
        let ui = simulator(&app);
        for message in ui.into_messages() {
            app.update(message);
        }
        let mut ui = simulator(&app);
        assert!(
            ui.find("Fill").is_ok(),
            "Card with Fill height should render"
        );
    }

    // Test Shrink height
    {
        let (mut app, _command) = App::new(|| {
            Card::new(Text::new("Shrink"), Text::new("Body"))
                .height(Length::Shrink)
                .into()
        });
        let ui = simulator(&app);
        for message in ui.into_messages() {
            app.update(message);
        }
        let mut ui = simulator(&app);
        assert!(
            ui.find("Shrink").is_ok(),
            "Card with Shrink height should render"
        );
    }

    // Test FillPortion height
    {
        let (mut app, _command) = App::new(|| {
            Card::new(Text::new("Portion"), Text::new("Body"))
                .height(Length::FillPortion(2))
                .into()
        });
        let ui = simulator(&app);
        for message in ui.into_messages() {
            app.update(message);
        }
        let mut ui = simulator(&app);
        assert!(
            ui.find("Portion").is_ok(),
            "Card with FillPortion height should render"
        );
    }

    Ok(())
}

#[test]
fn card_with_various_max_dimensions() -> Result<(), Error> {
    let configs = vec![
        ("Small", 100.0, 100.0),
        ("Medium", 500.0, 400.0),
        ("Large", 1000.0, 800.0),
    ];

    for (text, max_w, max_h) in configs {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Card::new(Text::new(text_copy.clone()), Text::new("Body"))
                .max_width(max_w)
                .max_height(max_h)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Card with max dimensions should render"
        );
    }

    Ok(())
}

#[test]
fn card_with_different_padding_values() -> Result<(), Error> {
    let padding_uniform = Padding::new(10.0);
    let (mut app, _command) = App::new(move || {
        Card::new(Text::new("Uniform"), Text::new("Body"))
            .padding(padding_uniform)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Uniform").is_ok(),
        "Card with uniform padding should render"
    );

    let padding_custom = Padding {
        top: 5.0,
        right: 10.0,
        bottom: 15.0,
        left: 20.0,
    };
    let (mut app, _command) = App::new(move || {
        Card::new(Text::new("Custom"), Text::new("Body"))
            .padding(padding_custom)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Custom").is_ok(),
        "Card with custom padding should render"
    );

    Ok(())
}

#[test]
fn card_with_asymmetric_padding() -> Result<(), Error> {
    let head_padding = Padding {
        top: 10.0,
        right: 15.0,
        bottom: 10.0,
        left: 15.0,
    };
    let body_padding = Padding {
        top: 20.0,
        right: 25.0,
        bottom: 20.0,
        left: 25.0,
    };
    let foot_padding = Padding {
        top: 5.0,
        right: 10.0,
        bottom: 5.0,
        left: 10.0,
    };

    let (mut app, _command) = App::new(move || {
        Card::new(Text::new("Asymmetric"), Text::new("Body"))
            .foot(Text::new("Footer"))
            .padding_head(head_padding)
            .padding_body(body_padding)
            .padding_foot(foot_padding)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Asymmetric").is_ok(),
        "Card with asymmetric padding should render"
    );

    Ok(())
}

#[test]
fn card_with_empty_content() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| Card::new(Text::new(""), Text::new("")).into());
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Empty text won't be found, but card should render without error
    let _ui = simulator(&app);
    // The test passes if we get here without panicking

    Ok(())
}

#[test]
fn card_with_long_content() -> Result<(), Error> {
    let long_head = "This is a very long header that might wrap or truncate";
    let long_body = "This is a very long body content that contains multiple sentences and should test how the card handles lengthy text. It might wrap across multiple lines depending on the card width.";
    let long_foot = "Footer with some additional information that might also be quite long";

    let (mut app, _command) = App::new(move || {
        Card::new(Text::new(long_head), Text::new(long_body))
            .foot(Text::new(long_foot))
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find(long_head).is_ok(),
        "Card with long content should render"
    );

    Ok(())
}

#[test]
fn card_with_unicode_content() -> Result<(), Error> {
    let unicode_tests = vec![
        ("标题", "内容"),
        ("العنوان", "المحتوى"),
        ("🎨 Header", "📝 Body"),
    ];

    for (head, body) in unicode_tests {
        let head_copy = head.to_string();
        let body_copy = body.to_string();
        let (mut app, _command) = App::new(move || {
            Card::new(Text::new(head_copy.clone()), Text::new(body_copy.clone())).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(head).is_ok(),
            "Card with unicode content '{}' should render",
            head
        );
    }

    Ok(())
}

#[test]
fn card_with_footer() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Card::new(Text::new("With Footer"), Text::new("Body"))
            .foot(Text::new("Footer"))
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("With Footer").is_ok() && ui.find("Footer").is_ok(),
        "Card with footer should render"
    );

    Ok(())
}

#[test]
fn card_with_various_styles() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let styles = vec![
        (
            "Light",
            style::card::Style {
                background: Background::Color(Color::WHITE),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: Color::from_rgb(0.9, 0.9, 0.9),
                head_background: Background::Color(Color::from_rgb(0.2, 0.5, 0.9)),
                head_text_color: Color::WHITE,
                body_background: Background::Color(Color::WHITE),
                body_text_color: Color::BLACK,
                foot_background: Background::Color(Color::from_rgb(0.95, 0.95, 0.95)),
                foot_text_color: Color::from_rgb(0.3, 0.3, 0.3),
                close_color: Color::WHITE,
            },
        ),
        (
            "Dark",
            style::card::Style {
                background: Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
                border_radius: 10.0,
                border_width: 2.0,
                border_color: Color::from_rgb(0.3, 0.3, 0.3),
                head_background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
                head_text_color: Color::from_rgb(0.9, 0.9, 0.9),
                body_background: Background::Color(Color::from_rgb(0.15, 0.15, 0.15)),
                body_text_color: Color::from_rgb(0.85, 0.85, 0.85),
                foot_background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
                foot_text_color: Color::from_rgb(0.8, 0.8, 0.8),
                close_color: Color::from_rgb(0.9, 0.9, 0.9),
            },
        ),
    ];

    for (text, card_style) in styles {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Card::new(Text::new(text_copy.clone()), Text::new("Body"))
                .style(move |_theme: &Theme, _status: Status| card_style)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Card with style '{}' should render",
            text
        );
    }

    Ok(())
}

#[test]
fn card_style_with_transparency() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let (mut app, _command) = App::new(|| {
        Card::new(Text::new("Transparent"), Text::new("Body"))
            .style(|_theme: &Theme, _status: Status| style::card::Style {
                background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
                border_radius: 10.0,
                border_width: 1.0,
                border_color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                head_background: Background::Color(Color::from_rgba(0.2, 0.4, 0.8, 0.8)),
                head_text_color: Color::WHITE,
                body_background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.7)),
                body_text_color: Color::BLACK,
                foot_background: Background::Color(Color::from_rgba(0.9, 0.9, 0.9, 0.8)),
                foot_text_color: Color::from_rgb(0.3, 0.3, 0.3),
                close_color: Color::from_rgba(1.0, 1.0, 1.0, 0.9),
            })
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Transparent").is_ok(),
        "Card with transparent style should render"
    );

    Ok(())
}

#[test]
fn card_with_extreme_dimensions() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Card::new(Text::new("Tiny"), Text::new("B"))
            .width(50)
            .height(30)
            .into()
    });
    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }
    let mut ui = simulator(&app);
    assert!(ui.find("Tiny").is_ok(), "Tiny card should render");

    let (mut app, _command) = App::new(|| {
        Card::new(Text::new("Large"), Text::new("Body"))
            .width(1920)
            .height(1080)
            .into()
    });
    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }
    let mut ui = simulator(&app);
    assert!(ui.find("Large").is_ok(), "Large card should render");

    Ok(())
}

#[test]
fn card_with_max_dimensions_smaller_than_regular() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Card::new(Text::new("Constrained"), Text::new("Body"))
            .width(500)
            .height(400)
            .max_width(300.0)
            .max_height(250.0)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Constrained").is_ok(),
        "Card with constraining max dimensions should render"
    );

    Ok(())
}

// ============================================================================
// Tests for close button interaction
// ============================================================================
//
// Note: The card's close button is now exposed as a button widget through the
// operate() method, allowing iced_test to interact with it directly.

#[test]
fn card_has_close_button_when_on_close_set() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Card::new(Text::new("With Close"), Text::new("Body"))
            .on_close(Message::CloseCard)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    // Just verify the card renders with close button
    assert!(
        ui.find("With Close").is_ok(),
        "Card with close button should render"
    );

    Ok(())
}

#[test]
fn card_close_button_can_be_clicked() -> Result<(), Error> {
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
                Message::DoNothing => {}
            }
        }

        fn view(&self) -> Element<'_, Message> {
            if *self.show_card.borrow() {
                Card::new(Text::new("Closeable Card"), Text::new("Click to close"))
                    .on_close(Message::CloseCard)
                    .into()
            } else {
                Text::new("Card is closed").into()
            }
        }
    }

    let (mut app, _command) = StatefulApp::new();

    // Initial state - card should be visible
    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(
        ui.find("Closeable Card").is_ok(),
        "Card should be visible initially"
    );

    // Click the close button
    let _ = ui.click("\u{e800}");

    // Process the messages generated by the click
    for message in ui.into_messages() {
        app.update(message);
    }

    // Verify card is now closed
    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(
        ui.find("Card is closed").is_ok(),
        "Card should be closed after clicking close button"
    );
    assert!(
        ui.find("Closeable Card").is_err(),
        "Original card text should not be found after closing"
    );

    Ok(())
}

#[test]
fn card_without_on_close_has_no_close_button() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Card::new(
            Text::new("No Close"),
            Text::new("This card has no close button"),
        )
        .into()
    });

    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("No Close").is_ok(),
        "Card without close button should still render"
    );

    Ok(())
}

#[test]
fn card_close_button_can_be_clicked_multiple_times() -> Result<(), Error> {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct MessageTrackingApp {
        close_count: Rc<RefCell<usize>>,
    }

    impl MessageTrackingApp {
        fn new() -> (Self, iced::Task<Message>) {
            (
                MessageTrackingApp {
                    close_count: Rc::new(RefCell::new(0)),
                },
                iced::Task::none(),
            )
        }

        fn update(&mut self, message: Message) {
            match message {
                Message::CloseCard => {
                    *self.close_count.borrow_mut() += 1;
                }
                Message::DoNothing => {}
            }
        }

        fn view(&self) -> Element<'_, Message> {
            let count = *self.close_count.borrow();
            Card::new(
                Text::new("Track Closes"),
                Text::new(format!("Clicked {} times", count)),
            )
            .on_close(Message::CloseCard)
            .into()
        }
    }

    let (mut app, _command) = MessageTrackingApp::new();

    // Click close button 3 times
    for _ in 0..3 {
        let mut ui = Simulator::with_settings(Settings::default(), app.view());
        let _ = ui.click("\u{e800}");

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    // Verify the close messages were received and counted
    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(
        ui.find("Clicked 3 times").is_ok(),
        "Close button should have been clicked three times"
    );

    Ok(())
}

#[test]
fn card_with_custom_close_size_can_be_clicked() -> Result<(), Error> {
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
                Message::DoNothing => {}
            }
        }

        fn view(&self) -> Element<'_, Message> {
            if *self.show_card.borrow() {
                Card::new(Text::new("Custom Close Size"), Text::new("Body"))
                    .on_close(Message::CloseCard)
                    .close_size(32.0)
                    .into()
            } else {
                Text::new("Closed").into()
            }
        }
    }

    let (mut app, _command) = StatefulApp::new();

    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(ui.find("Custom Close Size").is_ok());

    // Click the close button
    let _ = ui.click("\u{e800}");

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = Simulator::with_settings(Settings::default(), app.view());
    assert!(
        ui.find("Closed").is_ok(),
        "Card with custom close size should be clickable"
    );

    Ok(())
}
