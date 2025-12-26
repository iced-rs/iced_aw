//! Integration tests for the Badge widget
//!
//! These tests verify the Badge widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::Settings;
use iced::{Alignment, Color, Element, Length, Theme};
use iced_aw::Badge;
use iced_test::{Error, Simulator};
use iced_widget::text::Text;

#[derive(Clone)]
#[allow(dead_code)]
enum Message {
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
fn badge_can_be_created_with_text() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| Badge::new(Text::new("Test Badge")).into());
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Create simulator again and find text inside Badge
    let mut ui = simulator(&app);

    // Now that Badge implements operate(), the built-in string selector works!
    assert!(
        ui.find("Test Badge").is_ok(),
        "Text inside Badge should be found!"
    );

    Ok(())
}

#[test]
fn badge_can_be_created_with_different_text_content() -> Result<(), Error> {
    let texts = vec!["New", "1", "999+", "Alert", "!"];

    for text in texts {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || Badge::new(Text::new(text_copy.clone())).into());
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with text '{}' should be found",
            text
        );
    }

    Ok(())
}

#[test]
fn badge_can_set_padding() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Badge::new(Text::new("Padded"))
            .padding(10)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Padded").is_ok(),
        "Badge with padding should render and text should be found"
    );

    Ok(())
}

#[test]
fn badge_can_set_different_padding_values() -> Result<(), Error> {
    let padding_values = vec![0, 5, 10, 15, 20, 50, 100];

    for padding in padding_values {
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new("Test"))
                .padding(padding)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find("Test").is_ok(),
            "Badge with padding {} should render",
            padding
        );
    }

    Ok(())
}

#[test]
fn badge_can_set_width() -> Result<(), Error> {
    let configs = vec![
        ("Fixed", Length::Fixed(100.0)),
        ("Fill", Length::Fill),
        ("Shrink", Length::Shrink),
    ];

    for (text, width) in configs {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .width(width)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with width configuration should render"
        );
    }

    Ok(())
}

#[test]
fn badge_can_set_height() -> Result<(), Error> {
    let configs = vec![
        ("Fixed", Length::Fixed(50.0)),
        ("Fill", Length::Fill),
        ("Shrink", Length::Shrink),
    ];

    for (text, height) in configs {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .height(height)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with height configuration should render"
        );
    }

    Ok(())
}

#[test]
fn badge_can_set_horizontal_alignment() -> Result<(), Error> {
    let alignments = vec![
        ("Start", Alignment::Start),
        ("Center", Alignment::Center),
        ("End", Alignment::End),
    ];

    for (text, align) in alignments {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .align_x(align)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with horizontal alignment should render"
        );
    }

    Ok(())
}

#[test]
fn badge_can_set_vertical_alignment() -> Result<(), Error> {
    let alignments = vec![
        ("Start", Alignment::Start),
        ("Center", Alignment::Center),
        ("End", Alignment::End),
    ];

    for (text, align) in alignments {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .align_y(align)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with vertical alignment should render"
        );
    }

    Ok(())
}

#[test]
fn badge_can_set_both_alignments() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Badge::new(Text::new("Aligned"))
            .align_x(Alignment::Start)
            .align_y(Alignment::End)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Aligned").is_ok(),
        "Badge with both alignments should render"
    );

    Ok(())
}

#[test]
fn badge_can_be_styled() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let (mut app, _command) = App::new(|| {
        Badge::new(Text::new("Styled"))
            .style(|_theme: &Theme, _status: Status| style::badge::Style {
                background: Background::Color(Color::from_rgb(0.8, 0.2, 0.2)),
                border_radius: Some(10.0),
                border_width: 2.0,
                border_color: Some(Color::from_rgb(0.0, 0.0, 0.0)),
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
            })
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Styled").is_ok(),
        "Styled badge should render"
    );

    Ok(())
}

#[test]
fn badge_can_use_custom_class() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Badge::new(Text::new("Classed"))
            .class(<Theme as iced_aw::style::badge::Catalog>::default())
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Classed").is_ok(),
        "Badge with custom class should render"
    );

    Ok(())
}

#[test]
fn badge_can_chain_multiple_methods() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        Badge::new(Text::new("Chained"))
            .padding(15)
            .width(200)
            .height(60)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Chained").is_ok(),
        "Badge with chained methods should render"
    );

    Ok(())
}

#[test]
fn badge_can_chain_all_methods() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let (mut app, _command) = App::new(|| {
        Badge::new(Text::new("All"))
            .padding(10)
            .width(150)
            .height(40)
            .align_x(Alignment::Start)
            .align_y(Alignment::End)
            .style(|_theme: &Theme, _status: Status| style::badge::Style {
                background: Background::Color(Color::from_rgb(0.3, 0.7, 0.3)),
                border_radius: Some(8.0),
                border_width: 1.0,
                border_color: Some(Color::from_rgb(0.1, 0.1, 0.1)),
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
            })
            .class(<Theme as iced_aw::style::badge::Catalog>::default())
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("All").is_ok(),
        "Badge with all methods chained should render"
    );

    Ok(())
}

#[test]
fn badge_converts_to_element() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| {
        let badge: Badge<Message, Theme> = Badge::new(Text::new("Element"));
        let _element: Element<Message, Theme> = badge.into();
        Badge::new(Text::new("Element")).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find("Element").is_ok(),
        "Badge should convert to Element"
    );

    Ok(())
}

#[test]
fn badge_supports_multiple_instances() -> Result<(), Error> {
    // Test that we can create multiple badge instances
    let instances = vec![
        ("First", 5),
        ("Second", 10),
        ("Third", 15),
    ];

    for (text, padding) in instances {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .padding(padding)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge instance with text '{}' should render",
            text
        );
    }

    Ok(())
}

#[test]
fn badge_with_different_message_types() -> Result<(), Error> {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        BadgeClicked,
        Other,
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
                CustomMessage::BadgeClicked => {}
                CustomMessage::Other => {}
            }
        }

        fn view(&self) -> Element<'_, CustomMessage> {
            (self.view_fn)()
        }
    }

    let (mut app, _command) = CustomApp::new(|| Badge::new(Text::new("Custom")).into());
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
        "Badge with custom message type should render"
    );

    Ok(())
}

#[test]
fn badge_with_various_width_configurations() -> Result<(), Error> {
    let configs = vec![
        ("Fixed", Length::Fixed(100.0)),
        ("Fill", Length::Fill),
        ("Shrink", Length::Shrink),
        ("Portion", Length::FillPortion(2)),
    ];

    for (text, width) in configs {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .width(width)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with width '{}' should render",
            text
        );
    }

    Ok(())
}

#[test]
fn badge_with_various_height_configurations() -> Result<(), Error> {
    let configs = vec![
        ("Fixed", Length::Fixed(30.0)),
        ("Fill", Length::Fill),
        ("Shrink", Length::Shrink),
        ("Portion", Length::FillPortion(3)),
    ];

    for (text, height) in configs {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .height(height)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with height '{}' should render",
            text
        );
    }

    Ok(())
}

#[test]
fn badge_with_extreme_padding_values() -> Result<(), Error> {
    let configs = vec![("Zero", 0), ("Large", 255)];

    for (text, padding) in configs {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .padding(padding)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with padding {} should render",
            padding
        );
    }

    Ok(())
}

#[test]
fn badge_with_all_alignment_combinations() -> Result<(), Error> {
    let h_alignments = vec![Alignment::Start, Alignment::Center, Alignment::End];
    let v_alignments = vec![Alignment::Start, Alignment::Center, Alignment::End];

    for &h_align in &h_alignments {
        for &v_align in &v_alignments {
            let (mut app, _command) = App::new(move || {
                Badge::new(Text::new("Aligned"))
                    .align_x(h_align)
                    .align_y(v_align)
                    .into()
            });
            let ui = simulator(&app);

            for message in ui.into_messages() {
                app.update(message);
            }

            let mut ui = simulator(&app);
            assert!(
                ui.find("Aligned").is_ok(),
                "Badge with alignment combination should render"
            );
        }
    }

    Ok(())
}

#[test]
fn badge_with_various_styles() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let styles = vec![
        (
            "Red",
            style::badge::Style {
                background: Background::Color(Color::from_rgb(1.0, 0.0, 0.0)),
                border_radius: Some(5.0),
                border_width: 1.0,
                border_color: Some(Color::BLACK),
                text_color: Color::WHITE,
            },
        ),
        (
            "Green",
            style::badge::Style {
                background: Background::Color(Color::from_rgb(0.0, 1.0, 0.0)),
                border_radius: Some(10.0),
                border_width: 2.0,
                border_color: Some(Color::from_rgb(0.2, 0.2, 0.2)),
                text_color: Color::from_rgb(0.1, 0.1, 0.1),
            },
        ),
        (
            "Blue",
            style::badge::Style {
                background: Background::Color(Color::from_rgb(0.0, 0.0, 1.0)),
                border_radius: None,
                border_width: 0.0,
                border_color: None,
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
            },
        ),
    ];

    for (text, badge_style) in styles {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .style(move |_theme: &Theme, _status: Status| badge_style)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with style '{}' should render",
            text
        );
    }

    Ok(())
}

#[test]
fn badge_with_empty_text() -> Result<(), Error> {
    let (mut app, _command) = App::new(|| Badge::new(Text::new("")).into());
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    // Empty text won't be found, but badge should render without error
    let _ui = simulator(&app);
    // The test passes if we get here without panicking

    Ok(())
}

#[test]
fn badge_with_long_text() -> Result<(), Error> {
    let long_text = "This is a very long badge text that might wrap or overflow";
    let (mut app, _command) = App::new(move || Badge::new(Text::new(long_text)).into());
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    let mut ui = simulator(&app);
    assert!(
        ui.find(long_text).is_ok(),
        "Badge with long text should render"
    );

    Ok(())
}

#[test]
fn badge_with_unicode_text() -> Result<(), Error> {
    let unicode_texts = vec!["你好", "🎉", "مرحبا"];

    for text in unicode_texts {
        let text_copy = text.to_string();
        let (mut app, _command) = App::new(move || Badge::new(Text::new(text_copy.clone())).into());
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text).is_ok(),
            "Badge with unicode text '{}' should render",
            text
        );
    }

    Ok(())
}

#[test]
fn badge_style_with_transparency() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let (mut app, _command) = App::new(|| {
        Badge::new(Text::new("Transparent"))
            .style(|_theme: &Theme, _status: Status| style::badge::Style {
                background: Background::Color(Color::from_rgba(1.0, 0.0, 0.0, 0.5)),
                border_radius: Some(8.0),
                border_width: 1.0,
                border_color: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.8)),
                text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.9),
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
        "Badge with transparent style should render"
    );

    Ok(())
}

#[test]
fn badge_create_multiple_with_different_configs() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    for i in 0..10 {
        let text = format!("Badge {}", i);
        let text_copy = text.clone();
        let padding = (i as u16) * 2;
        let width = 50 + (i * 10);

        let (mut app, _command) = App::new(move || {
            Badge::new(Text::new(text_copy.clone()))
                .padding(padding)
                .width(width)
                .align_x(Alignment::Center)
                .style(move |_theme: &Theme, _status: Status| style::badge::Style {
                    background: Background::Color(Color::from_rgb(
                        (i as f32) * 0.1,
                        0.5,
                        1.0 - (i as f32) * 0.1,
                    )),
                    border_radius: Some((i as f32) * 2.0),
                    border_width: 1.0,
                    border_color: Some(Color::BLACK),
                    text_color: Color::WHITE,
                })
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }

        let mut ui = simulator(&app);
        assert!(
            ui.find(text.as_str()).is_ok(),
            "Badge '{}' with custom config should render",
            text
        );
    }

    Ok(())
}
