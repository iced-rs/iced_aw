//! Integration tests for the DatePicker widget
//!
//! These tests verify the DatePicker widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::{Pixels, Theme};
use iced_aw::{DatePicker, date_picker::Date};
use iced_widget::Button;
use iced_widget::text::Text;

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Message {
    Open,
    Cancel,
    Submit(Date),
    Other,
}

#[test]
fn date_picker_can_be_created_with_hidden_picker() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
}

#[test]
fn date_picker_can_be_created_with_visible_picker() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(true, date, underlay, Message::Cancel, Message::Submit);
}

#[test]
fn date_picker_with_default_date() {
    let date = Date::default();
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
}

#[test]
fn date_picker_with_various_dates() {
    let dates = vec![
        Date::from_ymd(2024, 1, 1),
        Date::from_ymd(2024, 12, 31),
        Date::from_ymd(2000, 6, 15),
        Date::from_ymd(1999, 12, 31),
        Date::from_ymd(2025, 7, 4),
    ];

    for date in dates {
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    }
}

#[test]
fn date_picker_with_button_underlay() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Open DatePicker"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
}

#[test]
fn date_picker_with_text_underlay() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Text::new("Click to pick date");
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
}

#[test]
fn date_picker_with_different_cancel_message() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Other, Message::Submit);
}

#[test]
fn date_picker_can_set_font_size() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit).font_size(16.0);
}

#[test]
fn date_picker_with_different_font_sizes() {
    let font_sizes = vec![12.0, 14.0, 16.0, 18.0, 20.0, 24.0];

    for size in font_sizes {
        let date = Date::from_ymd(2024, 1, 1);
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit)
                .font_size(size);
    }
}

#[test]
fn date_picker_font_size_with_pixels() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit)
            .font_size(Pixels(18.0));
}

#[test]
fn date_picker_can_be_styled() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit).style(
            |_theme: &Theme, _status: Status| style::date_picker::Style {
                background: Background::Color(iced::Color::WHITE),
                border_radius: 8.0,
                border_width: 1.0,
                border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                text_color: iced::Color::BLACK,
                text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                day_background: Background::Color(iced::Color::TRANSPARENT),
            },
        );
}

#[test]
fn date_picker_can_use_custom_class() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit)
            .class(<Theme as iced_aw::style::date_picker::Catalog>::default());
}

#[test]
fn date_picker_can_chain_methods() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit).font_size(16.0);
}

#[test]
fn date_picker_can_chain_style_and_font_size() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit)
            .style(
                |_theme: &Theme, _status: Status| style::date_picker::Style {
                    background: Background::Color(iced::Color::WHITE),
                    border_radius: 8.0,
                    border_width: 1.0,
                    border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                    text_color: iced::Color::BLACK,
                    text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                    day_background: Background::Color(iced::Color::TRANSPARENT),
                },
            )
            .font_size(18.0);
}

#[test]
fn date_picker_converts_to_element() {
    use iced::Element;
    use iced_widget::Renderer;

    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    let _element: Element<Message, Theme, Renderer> = picker.into();
}

#[test]
fn date_picker_supports_multiple_instances() {
    let date1 = Date::from_ymd(2024, 1, 1);
    let _picker1: DatePicker<Message, Theme> = DatePicker::new(
        false,
        date1,
        Button::new(Text::new("Picker 1")),
        Message::Cancel,
        Message::Submit,
    );

    let date2 = Date::from_ymd(2024, 6, 15);
    let _picker2: DatePicker<Message, Theme> = DatePicker::new(
        true,
        date2,
        Button::new(Text::new("Picker 2")),
        Message::Cancel,
        Message::Submit,
    );

    let date3 = Date::from_ymd(2024, 12, 31);
    let _picker3: DatePicker<Message, Theme> = DatePicker::new(
        false,
        date3,
        Button::new(Text::new("Picker 3")),
        Message::Cancel,
        Message::Submit,
    );
}

#[test]
fn date_picker_with_different_message_types() {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        OpenPicker,
        ClosePicker,
        DateSelected(Date),
    }

    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<CustomMessage, Theme> = DatePicker::new(
        false,
        date,
        underlay,
        CustomMessage::ClosePicker,
        CustomMessage::DateSelected,
    );
}

#[test]
fn date_picker_with_leap_year_date() {
    let date = Date::from_ymd(2024, 2, 29);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
}

#[test]
fn date_picker_with_various_months() {
    for month in 1..=12 {
        let date = Date::from_ymd(2024, month, 1);
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    }
}

#[test]
fn date_picker_with_month_boundaries() {
    let dates = vec![
        Date::from_ymd(2024, 1, 1),   // First day of January
        Date::from_ymd(2024, 1, 31),  // Last day of January
        Date::from_ymd(2024, 2, 1),   // First day of February
        Date::from_ymd(2024, 2, 29),  // Last day of February (leap year)
        Date::from_ymd(2024, 12, 1),  // First day of December
        Date::from_ymd(2024, 12, 31), // Last day of December
    ];

    for date in dates {
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    }
}

#[test]
fn date_picker_with_year_boundaries() {
    let dates = vec![
        Date::from_ymd(1900, 1, 1),
        Date::from_ymd(2000, 1, 1),
        Date::from_ymd(2024, 1, 1),
        Date::from_ymd(2099, 12, 31),
        Date::from_ymd(2100, 1, 1),
    ];

    for date in dates {
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    }
}

#[test]
fn date_picker_with_various_styles() {
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

    for (i, style) in styles.into_iter().enumerate() {
        let date = Date::from_ymd(2024, 1, 1);
        let underlay = Button::new(Text::new(format!("Picker {}", i)));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit)
                .style(move |_theme: &Theme, _status: Status| style);
    }
}

#[test]
fn date_picker_with_transparent_background() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit).style(
            |_theme: &Theme, _status: Status| style::date_picker::Style {
                background: Background::Color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
                border_radius: 8.0,
                border_width: 1.0,
                border_color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                text_color: iced::Color::BLACK,
                text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                day_background: Background::Color(iced::Color::TRANSPARENT),
            },
        );
}

#[test]
fn date_picker_create_multiple_with_different_configs() {
    for i in 0..5 {
        let date = Date::from_ymd(2024, (i % 12) + 1, 1);
        let show_picker = i % 2 == 0;
        let font_size = 12.0 + (i as f32) * 2.0;

        let underlay = Button::new(Text::new(format!("Picker {}", i)));
        let _picker: DatePicker<Message, Theme> = DatePicker::new(
            show_picker,
            date,
            underlay,
            Message::Cancel,
            Message::Submit,
        )
        .font_size(font_size);
    }
}

#[test]
fn date_picker_with_small_font_size() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit).font_size(10.0);
}

#[test]
fn date_picker_with_large_font_size() {
    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit).font_size(32.0);
}

#[test]
fn date_picker_with_all_methods_chained() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 6, 15);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(true, date, underlay, Message::Cancel, Message::Submit)
            .font_size(16.0)
            .style(
                |_theme: &Theme, _status: Status| style::date_picker::Style {
                    background: Background::Color(iced::Color::WHITE),
                    border_radius: 8.0,
                    border_width: 1.0,
                    border_color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                    text_color: iced::Color::BLACK,
                    text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                    day_background: Background::Color(iced::Color::TRANSPARENT),
                },
            )
            .class(<Theme as iced_aw::style::date_picker::Catalog>::default());
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
fn date_picker_with_february_non_leap_year() {
    let date = Date::from_ymd(2023, 2, 28);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
}

#[test]
fn date_picker_with_30_day_months() {
    let dates = vec![
        Date::from_ymd(2024, 4, 30),  // April
        Date::from_ymd(2024, 6, 30),  // June
        Date::from_ymd(2024, 9, 30),  // September
        Date::from_ymd(2024, 11, 30), // November
    ];

    for date in dates {
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    }
}

#[test]
fn date_picker_with_31_day_months() {
    let dates = vec![
        Date::from_ymd(2024, 1, 31),  // January
        Date::from_ymd(2024, 3, 31),  // March
        Date::from_ymd(2024, 5, 31),  // May
        Date::from_ymd(2024, 7, 31),  // July
        Date::from_ymd(2024, 8, 31),  // August
        Date::from_ymd(2024, 10, 31), // October
        Date::from_ymd(2024, 12, 31), // December
    ];

    for date in dates {
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    }
}

#[test]
fn date_picker_style_then_class() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let date = Date::from_ymd(2024, 1, 1);
    let underlay = Button::new(Text::new("Pick date"));
    let _picker: DatePicker<Message, Theme> =
        DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit)
            .style(
                |_theme: &Theme, _status: Status| style::date_picker::Style {
                    background: Background::Color(iced::Color::WHITE),
                    border_radius: 8.0,
                    border_width: 1.0,
                    border_color: iced::Color::BLACK,
                    text_color: iced::Color::BLACK,
                    text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                    day_background: Background::Color(iced::Color::TRANSPARENT),
                },
            )
            .class(<Theme as iced_aw::style::date_picker::Catalog>::default());
}

#[test]
fn date_picker_with_historical_dates() {
    let dates = vec![
        Date::from_ymd(1776, 7, 4),
        Date::from_ymd(1492, 10, 12),
        Date::from_ymd(1969, 7, 20),
    ];

    for date in dates {
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    }
}

#[test]
fn date_picker_with_future_dates() {
    let dates = vec![
        Date::from_ymd(2030, 1, 1),
        Date::from_ymd(2050, 6, 15),
        Date::from_ymd(2100, 12, 31),
    ];

    for date in dates {
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit);
    }
}

#[test]
fn date_picker_both_show_states() {
    let date = Date::from_ymd(2024, 1, 1);

    let _picker_hidden: DatePicker<Message, Theme> = DatePicker::new(
        false,
        date,
        Button::new(Text::new("Hidden")),
        Message::Cancel,
        Message::Submit,
    );

    let _picker_visible: DatePicker<Message, Theme> = DatePicker::new(
        true,
        date,
        Button::new(Text::new("Visible")),
        Message::Cancel,
        Message::Submit,
    );
}

#[test]
fn date_picker_with_various_border_radii() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let radii = vec![0.0, 4.0, 8.0, 12.0, 16.0, 20.0];

    for radius in radii {
        let date = Date::from_ymd(2024, 1, 1);
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit).style(
                move |_theme: &Theme, _status: Status| style::date_picker::Style {
                    background: Background::Color(iced::Color::WHITE),
                    border_radius: radius,
                    border_width: 1.0,
                    border_color: iced::Color::BLACK,
                    text_color: iced::Color::BLACK,
                    text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                    day_background: Background::Color(iced::Color::TRANSPARENT),
                },
            );
    }
}

#[test]
fn date_picker_with_various_border_widths() {
    use iced::Background;
    use iced_aw::style::{self, Status};

    let widths = vec![0.0, 1.0, 2.0, 3.0, 5.0];

    for width in widths {
        let date = Date::from_ymd(2024, 1, 1);
        let underlay = Button::new(Text::new("Pick date"));
        let _picker: DatePicker<Message, Theme> =
            DatePicker::new(false, date, underlay, Message::Cancel, Message::Submit).style(
                move |_theme: &Theme, _status: Status| style::date_picker::Style {
                    background: Background::Color(iced::Color::WHITE),
                    border_radius: 8.0,
                    border_width: width,
                    border_color: iced::Color::BLACK,
                    text_color: iced::Color::BLACK,
                    text_attenuated_color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                    day_background: Background::Color(iced::Color::TRANSPARENT),
                },
            );
    }
}
