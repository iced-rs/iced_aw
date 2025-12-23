//! Integration tests for the ColorPicker widget
//!
//! These tests verify the ColorPicker widget's behavior and public API
//! from an external perspective, testing the widget as a user of the
//! library would interact with it.

use iced::{Color, Theme};
use iced_aw::ColorPicker;
use iced_widget::{button, text::Text};

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Open,
    Cancel,
    Submit(Color),
}

// Helper function to create a button with explicit Theme type
fn create_button<'a>(text: &'a str) -> iced_widget::Button<'a, Message, Theme> {
    button(Text::new(text)).on_press(Message::Open)
}

#[test]
fn color_picker_can_be_created_with_basic_configuration() {
    let color = Color::from_rgb(0.5, 0.5, 0.5);
    let button = create_button("Pick color");

    let _picker = ColorPicker::new(false, color, button, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_can_be_created_with_picker_shown() {
    let color = Color::from_rgb(0.3, 0.6, 0.9);
    let button = create_button("Pick color");

    let _picker = ColorPicker::new(true, color, button, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_works_with_black_color() {
    let black = Color::from_rgb(0.0, 0.0, 0.0);
    let button = create_button("Pick");

    let _picker = ColorPicker::new(false, black, button, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_works_with_white_color() {
    let white = Color::from_rgb(1.0, 1.0, 1.0);
    let button = create_button("Pick");

    let _picker = ColorPicker::new(false, white, button, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_works_with_red_color() {
    let red = Color::from_rgb(1.0, 0.0, 0.0);
    let button = create_button("Pick");

    let _picker = ColorPicker::new(false, red, button, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_works_with_green_color() {
    let green = Color::from_rgb(0.0, 1.0, 0.0);
    let button = create_button("Pick");

    let _picker = ColorPicker::new(false, green, button, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_works_with_blue_color() {
    let blue = Color::from_rgb(0.0, 0.0, 1.0);
    let button = create_button("Pick");

    let _picker = ColorPicker::new(false, blue, button, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_works_with_custom_colors() {
    let colors = vec![
        Color::from_rgb(0.25, 0.50, 0.75),
        Color::from_rgb(0.15, 0.85, 0.45),
        Color::from_rgb(0.99, 0.01, 0.50),
        Color::from_rgb(0.33, 0.33, 0.33),
        Color::from_rgb(0.66, 0.66, 0.66),
    ];

    for color in colors {
        let button = create_button("Pick");
        let _picker = ColorPicker::new(false, color, button, Message::Cancel, Message::Submit);
    }
}

#[test]
fn color_picker_can_be_created_with_different_messages() {
    #[derive(Clone)]
    #[allow(dead_code)]
    enum CustomMessage {
        ShowPicker,
        HidePicker,
        ColorSelected(Color),
    }

    let color = Color::from_rgb(0.5, 0.5, 0.5);
    let button: iced_widget::Button<'_, CustomMessage, Theme> =
        button(Text::new("Pick color")).on_press(CustomMessage::ShowPicker);

    let _picker = ColorPicker::new(
        false,
        color,
        button,
        CustomMessage::HidePicker,
        CustomMessage::ColorSelected,
    );
}

#[test]
fn color_picker_can_be_styled() {
    use iced_aw::style::{self, Status};

    let color = Color::from_rgb(0.5, 0.5, 0.5);
    let button = create_button("Pick color");

    let _picker = ColorPicker::new(false, color, button, Message::Cancel, Message::Submit)
        .style(|_theme: &Theme, _status: Status| style::color_picker::Style {
            background: iced::Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.0, 0.0, 0.0),
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: Color::from_rgb(0.0, 0.0, 0.0),
        });
}

#[test]
fn color_picker_can_use_custom_class() {
    let color = Color::from_rgb(0.5, 0.5, 0.5);
    let button = create_button("Pick color");

    let _picker: ColorPicker<Message, Theme> = ColorPicker::new(
        false,
        color,
        button,
        Message::Cancel,
        Message::Submit,
    )
    .class(<Theme as iced_aw::style::color_picker::Catalog>::default());
}

#[test]
fn color_picker_supports_multiple_instances() {
    let color1 = Color::from_rgb(0.1, 0.2, 0.3);
    let color2 = Color::from_rgb(0.4, 0.5, 0.6);
    let color3 = Color::from_rgb(0.7, 0.8, 0.9);

    let button1 = create_button("Picker 1");
    let button2 = create_button("Picker 2");
    let button3 = create_button("Picker 3");

    let _picker1 = ColorPicker::new(false, color1, button1, Message::Cancel, Message::Submit);
    let _picker2 = ColorPicker::new(true, color2, button2, Message::Cancel, Message::Submit);
    let _picker3 = ColorPicker::new(false, color3, button3, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_converts_to_element() {
    use iced::Element;
    use iced_widget::Renderer;

    let color = Color::from_rgb(0.5, 0.5, 0.5);
    let button = create_button("Pick color");

    let picker: ColorPicker<Message, Theme> =
        ColorPicker::new(false, color, button, Message::Cancel, Message::Submit);

    let _element: Element<Message, Theme, Renderer> = picker.into();
}

#[test]
fn color_picker_with_rgba_colors() {
    let semi_transparent_red = Color::from_rgba(1.0, 0.0, 0.0, 0.5);
    let semi_transparent_blue = Color::from_rgba(0.0, 0.0, 1.0, 0.75);
    let almost_transparent = Color::from_rgba(0.5, 0.5, 0.5, 0.1);

    for color in [semi_transparent_red, semi_transparent_blue, almost_transparent] {
        let button = create_button("Pick");
        let _picker = ColorPicker::new(false, color, button, Message::Cancel, Message::Submit);
    }
}

#[test]
fn color_picker_with_different_button_types() {
    let color = Color::from_rgb(0.5, 0.5, 0.5);

    // Simple text button
    let button1 = create_button("Pick color");
    let _picker1 = ColorPicker::new(false, color, button1, Message::Cancel, Message::Submit);

    // Button with different text
    let button2 = create_button("Choose");
    let _picker2 = ColorPicker::new(false, color, button2, Message::Cancel, Message::Submit);

    // Button with emoji (if supported)
    let button3 = create_button("🎨");
    let _picker3 = ColorPicker::new(false, color, button3, Message::Cancel, Message::Submit);
}

#[test]
fn color_picker_chain_style_and_class() {
    use iced_aw::style::{self, Status};

    let color = Color::from_rgb(0.5, 0.5, 0.5);
    let button = create_button("Pick color");

    let _picker: ColorPicker<Message, Theme> = ColorPicker::new(
        false,
        color,
        button,
        Message::Cancel,
        Message::Submit,
    )
    .style(|_theme: &Theme, _status: Status| style::color_picker::Style {
        background: iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.95)),
        border_radius: 20.0,
        border_width: 2.0,
        border_color: Color::from_rgb(0.1, 0.1, 0.1),
        bar_border_radius: 8.0,
        bar_border_width: 1.5,
        bar_border_color: Color::from_rgb(0.2, 0.2, 0.2),
    })
    .class(<Theme as iced_aw::style::color_picker::Catalog>::default());
}

#[test]
fn color_picker_extreme_color_values() {
    // Test edge cases for color values
    let colors = vec![
        Color::from_rgb(0.0, 0.0, 0.0),     // Minimum
        Color::from_rgb(1.0, 1.0, 1.0),     // Maximum
        Color::from_rgb(0.5, 0.0, 1.0),     // Mixed
        Color::from_rgb(1.0, 0.5, 0.0),     // Mixed
        Color::from_rgb(0.0, 1.0, 0.5),     // Mixed
        Color::from_rgba(0.0, 0.0, 0.0, 0.0),   // Fully transparent
        Color::from_rgba(1.0, 1.0, 1.0, 1.0),   // Fully opaque
    ];

    for color in colors {
        let button = create_button("Pick");
        let _picker = ColorPicker::new(false, color, button, Message::Cancel, Message::Submit);
    }
}

#[test]
fn color_picker_precision_test() {
    // Test with high precision color values
    let precise_colors = vec![
        Color::from_rgb(0.123456, 0.789012, 0.345678),
        Color::from_rgb(0.999999, 0.000001, 0.500000),
        Color::from_rgb(0.333333, 0.666666, 0.111111),
    ];

    for color in precise_colors {
        let button = create_button("Pick");
        let _picker = ColorPicker::new(false, color, button, Message::Cancel, Message::Submit);
    }
}

#[test]
fn color_picker_create_and_style_multiple_times() {
    use iced_aw::style::{self, Status};

    let color = Color::from_rgb(0.5, 0.5, 0.5);

    for i in 0..5 {
        let text = format!("Picker {}", i);
        let button: iced_widget::Button<'_, Message, Theme> =
            button(Text::new(text)).on_press(Message::Open);
        let border_radius = 10.0 + (i as f32) * 2.0;

        let _picker = ColorPicker::new(false, color, button, Message::Cancel, Message::Submit)
            .style(move |_theme: &Theme, _status: Status| style::color_picker::Style {
                background: iced::Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
                border_radius,
                border_width: 1.0,
                border_color: Color::from_rgb(0.0, 0.0, 0.0),
                bar_border_radius: 5.0,
                bar_border_width: 1.0,
                bar_border_color: Color::from_rgb(0.0, 0.0, 0.0),
            });
    }
}
