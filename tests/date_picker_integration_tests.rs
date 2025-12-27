//! Integration tests for the DatePicker widget
//!
//! These tests verify the DatePicker widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::{Element, Pixels, Settings, Theme};
use iced_aw::{date_picker::Date, DatePicker};
use iced_test::{Error, Simulator};
use iced_widget::{button, text::Text};

#[derive(Clone, Debug)]
enum Message {
    Open,
    Cancel,
    Submit(Date),
    Other,
}

// Helper function to create a button with explicit Theme type
fn create_button<'a>(text: &'a str) -> iced_widget::Button<'a, Message, Theme> {
    button(Text::new(text)).on_press(Message::Open)
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
            Message::Open | Message::Cancel | Message::Submit(_) | Message::Other => {
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

#[test]
fn date_picker_can_be_created_with_basic_configuration() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 1);

    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_can_be_created_with_picker_shown() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 1);

    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_with_default_date() -> Result<(), Error> {
    let date = Date::default();

    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_works_with_various_dates() -> Result<(), Error> {
    let dates = vec![
        Date::from_ymd(2024, 1, 1),
        Date::from_ymd(2024, 12, 31),
        Date::from_ymd(2000, 6, 15),
        Date::from_ymd(1999, 12, 31),
        Date::from_ymd(2025, 7, 4),
    ];

    for date in dates {
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_different_underlay_types() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 1);

    // Button underlay
    let (mut app, _command) = App::new(move || {
        let button = create_button("Open DatePicker");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }

    // Text underlay
    let (mut app, _command) = App::new(move || {
        DatePicker::new(
            false,
            date,
            Text::new("Click to pick date"),
            Message::Cancel,
            Message::Submit,
        )
        .into()
    });
    let ui = simulator(&app);
    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_can_be_created_with_different_messages() -> Result<(), Error> {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        OpenPicker,
        ClosePicker,
        DateSelected(Date),
    }

    let date = Date::from_ymd(2024, 1, 1);

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
                CustomMessage::OpenPicker
                | CustomMessage::ClosePicker
                | CustomMessage::DateSelected(_) => {}
            }
        }

        fn view(&self) -> Element<'_, CustomMessage> {
            (self.view_fn)()
        }
    }

    let (mut app, _command) = CustomApp::new(move || {
        let button: iced_widget::Button<'_, CustomMessage, Theme> =
            button(Text::new("Pick date")).on_press(CustomMessage::OpenPicker);
        DatePicker::new(
            false,
            date,
            button,
            CustomMessage::ClosePicker,
            CustomMessage::DateSelected,
        )
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

    Ok(())
}

#[test]
fn date_picker_can_set_font_size() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 1);
    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
            .font_size(16.0)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_works_with_different_font_sizes() -> Result<(), Error> {
    let font_sizes = vec![12.0, 14.0, 16.0, 18.0, 20.0, 24.0];

    for size in font_sizes {
        let date = Date::from_ymd(2024, 1, 1);
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
                .font_size(size)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_font_size_with_pixels() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 1);
    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
            .font_size(Pixels(18.0))
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_can_be_styled() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 1, 1);
    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
            .style(|_theme: &Theme, _status: Status| style::date_picker::Style {
                background: Background::Color(iced::Color::WHITE),
                border_radius: 8.0,
                border_width: 1.0,
                border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                text_color: iced::Color::BLACK,
                text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                day_background: Background::Color(iced::Color::TRANSPARENT),
            })
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_can_use_custom_class() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 1, 1);
    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
            .class(<Theme as iced_aw::style::date_picker::Catalog>::default())
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_chain_style_and_font_size() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 1, 1);
    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
            .style(|_theme: &Theme, _status: Status| style::date_picker::Style {
                background: Background::Color(iced::Color::WHITE),
                border_radius: 8.0,
                border_width: 1.0,
                border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                text_color: iced::Color::BLACK,
                text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                day_background: Background::Color(iced::Color::TRANSPARENT),
            })
            .font_size(18.0)
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_converts_to_element() -> Result<(), Error> {
    use iced_widget::Renderer;

    let date = Date::from_ymd(2024, 1, 1);

    let (mut app, _command) = App::new(move || {
        let button1 = create_button("Pick date");
        let picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, button1, Message::Cancel, Message::Submit);
        let _element: Element<Message, Theme, Renderer> = picker.into();
        let button2 = create_button("Pick date");
        DatePicker::new(false, date, button2, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_supports_multiple_instances() -> Result<(), Error> {
    let instances = vec![
        (Date::from_ymd(2024, 1, 1), false),
        (Date::from_ymd(2024, 6, 15), true),
        (Date::from_ymd(2024, 12, 31), false),
    ];

    for (date, show_picker) in instances {
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(show_picker, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_leap_year_date() -> Result<(), Error> {
    let date = Date::from_ymd(2024, 2, 29);

    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_works_with_all_months() -> Result<(), Error> {
    for month in 1..=12 {
        let date = Date::from_ymd(2024, month, 1);
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_month_boundaries() -> Result<(), Error> {
    let dates = vec![
        Date::from_ymd(2024, 1, 1),   // First day of January
        Date::from_ymd(2024, 1, 31),  // Last day of January
        Date::from_ymd(2024, 2, 1),   // First day of February
        Date::from_ymd(2024, 2, 29),  // Last day of February (leap year)
        Date::from_ymd(2024, 12, 1),  // First day of December
        Date::from_ymd(2024, 12, 31), // Last day of December
    ];

    for date in dates {
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_year_boundaries() -> Result<(), Error> {
    let dates = vec![
        Date::from_ymd(1900, 1, 1),
        Date::from_ymd(2000, 1, 1),
        Date::from_ymd(2024, 1, 1),
        Date::from_ymd(2099, 12, 31),
        Date::from_ymd(2100, 1, 1),
    ];

    for date in dates {
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_works_with_various_styles() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let styles = vec![
        style::date_picker::Style {
            background: Background::Color(iced::Color::WHITE),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: iced::Color::from_rgb(0.9, 0.9, 0.9),
            text_color: iced::Color::BLACK,
            text_attenuated_color: iced::Color::from_rgb(0.6, 0.6, 0.6),
            day_background: Background::Color(iced::Color::TRANSPARENT),
        },
        style::date_picker::Style {
            background: Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.1)),
            border_radius: 10.0,
            border_width: 2.0,
            border_color: iced::Color::from_rgb(0.3, 0.3, 0.3),
            text_color: iced::Color::from_rgb(0.9, 0.9, 0.9),
            text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
            day_background: Background::Color(iced::Color::TRANSPARENT),
        },
    ];

    for (i, date_style) in styles.into_iter().enumerate() {
        let date = Date::from_ymd(2024, 1, 1);
        let (mut app, _command) = App::new(move || {
            let text = format!("Picker {}", i);
            let button: iced_widget::Button<'_, Message, Theme> =
                button(Text::new(text)).on_press(Message::Open);
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
                .style(move |_theme: &Theme, _status: Status| date_style)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_transparent_background() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 1, 1);
    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
            .style(|_theme: &Theme, _status: Status| style::date_picker::Style {
                background: Background::Color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
                border_radius: 8.0,
                border_width: 1.0,
                border_color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                text_color: iced::Color::BLACK,
                text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                day_background: Background::Color(iced::Color::TRANSPARENT),
            })
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_create_multiple_with_different_configs() -> Result<(), Error> {
    for i in 0..5 {
        let date = Date::from_ymd(2024, (i % 12) + 1, 1);
        let show_picker = i % 2 == 0;
        let font_size = 12.0 + (i as f32) * 2.0;

        let (mut app, _command) = App::new(move || {
            let text = format!("Picker {}", i);
            let button: iced_widget::Button<'_, Message, Theme> =
                button(Text::new(text)).on_press(Message::Open);
            DatePicker::new(show_picker, date, button, Message::Cancel, Message::Submit)
                .font_size(font_size)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_extreme_font_sizes() -> Result<(), Error> {
    let sizes = vec![10.0, 32.0];

    for size in sizes {
        let date = Date::from_ymd(2024, 1, 1);
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
                .font_size(size)
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_all_methods_chained() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 6, 15);
    let (mut app, _command) = App::new(move || {
        let button = create_button("Pick date");
        DatePicker::new(true, date, button, Message::Cancel, Message::Submit)
            .font_size(16.0)
            .style(|_theme: &Theme, _status: Status| style::date_picker::Style {
                background: Background::Color(iced::Color::WHITE),
                border_radius: 8.0,
                border_width: 1.0,
                border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                text_color: iced::Color::BLACK,
                text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                day_background: Background::Color(iced::Color::TRANSPARENT),
            })
            .class(<Theme as iced_aw::style::date_picker::Catalog>::default())
            .into()
    });
    let ui = simulator(&app);

    for message in ui.into_messages() {
        app.update(message);
    }

    Ok(())
}

#[test]
fn date_picker_state_now() {
    use iced_aw::date_picker::State;
    let _state = State::now();
}

#[test]
fn date_picker_state_new() {
    use iced_aw::date_picker::State;
    let date = Date::from_ymd(2024, 6, 15);
    let _state = State::new(date);
}

#[test]
fn date_picker_state_reset() {
    use iced_aw::date_picker::State;
    let mut state = State::new(Date::from_ymd(2000, 1, 1));
    state.reset();
    // After reset, the state should have today's date
}

#[test]
fn date_picker_with_february_dates() -> Result<(), Error> {
    let dates = vec![
        Date::from_ymd(2023, 2, 28), // Non-leap year
        Date::from_ymd(2024, 2, 29), // Leap year
    ];

    for date in dates {
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_different_month_lengths() -> Result<(), Error> {
    let dates = vec![
        Date::from_ymd(2024, 4, 30),  // April (30 days)
        Date::from_ymd(2024, 6, 30),  // June (30 days)
        Date::from_ymd(2024, 9, 30),  // September (30 days)
        Date::from_ymd(2024, 11, 30), // November (30 days)
        Date::from_ymd(2024, 1, 31),  // January (31 days)
        Date::from_ymd(2024, 3, 31),  // March (31 days)
        Date::from_ymd(2024, 5, 31),  // May (31 days)
        Date::from_ymd(2024, 7, 31),  // July (31 days)
        Date::from_ymd(2024, 8, 31),  // August (31 days)
        Date::from_ymd(2024, 10, 31), // October (31 days)
        Date::from_ymd(2024, 12, 31), // December (31 days)
    ];

    for date in dates {
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_extreme_dates() -> Result<(), Error> {
    let dates = vec![
        Date::from_ymd(1776, 7, 4),   // Historical
        Date::from_ymd(1492, 10, 12), // Historical
        Date::from_ymd(1969, 7, 20),  // Historical
        Date::from_ymd(2030, 1, 1),   // Future
        Date::from_ymd(2050, 6, 15),  // Future
        Date::from_ymd(2100, 12, 31), // Future
    ];

    for date in dates {
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit).into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_various_border_styles() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let radii = vec![0.0, 4.0, 8.0, 12.0, 16.0, 20.0];

    for radius in radii {
        let date = Date::from_ymd(2024, 1, 1);
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
                .style(move |_theme: &Theme, _status: Status| style::date_picker::Style {
                    background: Background::Color(iced::Color::WHITE),
                    border_radius: radius,
                    border_width: 1.0,
                    border_color: iced::Color::BLACK,
                    text_color: iced::Color::BLACK,
                    text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                    day_background: Background::Color(iced::Color::TRANSPARENT),
                })
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}

#[test]
fn date_picker_with_various_border_widths() -> Result<(), Error> {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let widths = vec![0.0, 1.0, 2.0, 3.0, 5.0];

    for width in widths {
        let date = Date::from_ymd(2024, 1, 1);
        let (mut app, _command) = App::new(move || {
            let button = create_button("Pick date");
            DatePicker::new(false, date, button, Message::Cancel, Message::Submit)
                .style(move |_theme: &Theme, _status: Status| style::date_picker::Style {
                    background: Background::Color(iced::Color::WHITE),
                    border_radius: 8.0,
                    border_width: width,
                    border_color: iced::Color::BLACK,
                    text_color: iced::Color::BLACK,
                    text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                    day_background: Background::Color(iced::Color::TRANSPARENT),
                })
                .into()
        });
        let ui = simulator(&app);

        for message in ui.into_messages() {
            app.update(message);
        }
    }

    Ok(())
}
