// This example demonstrates DatePicker styling.
//
// It shows two date pickers side by side:
// - Left: default theme styling
// - Right: custom fuchsia accent styling
//
// Both pickers can be open simultaneously.
//
// This is used to demonstrate issue #449: the custom-styled picker can style
// the calendar grid (background, border, day colors) but NOT the internal
// buttons (OK, Cancel, navigation arrows).

use iced::{
    Alignment, Color, Element, Length,
    widget::{Button, Column, Container, Row, Text},
};
use iced_aw::ICED_AW_FONT_BYTES;
use iced_aw::{date_picker::Date, helpers::date_picker};

fn main() -> iced::Result {
    iced::application(
        DatePickerExample::default,
        DatePickerExample::update,
        DatePickerExample::view,
    )
    .font(ICED_AW_FONT_BYTES)
    .window_size((1200.0, 600.0))
    .run()
}

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
enum Message {
    // Default picker messages
    ChooseDate1,
    SubmitDate1(Date),
    CancelDate1,
    // Styled picker messages
    ChooseDate2,
    SubmitDate2(Date),
    CancelDate2,
}

#[derive(Default)]
struct DatePickerExample {
    date1: Date,
    date2: Date,
    show_picker1: bool,
    show_picker2: bool,
}

/// A fuchsia accent color used for custom styling
const FUCHSIA: Color = Color {
    r: 0.85,
    g: 0.2,
    b: 0.6,
    a: 1.0,
};

/// Custom date picker style with a fuchsia accent theme
fn fuchsia_style(
    _theme: &iced::Theme,
    status: iced_aw::style::Status,
) -> iced_aw::date_picker::Style {
    let fuchsia_dim = Color {
        r: FUCHSIA.r * 0.5,
        g: FUCHSIA.g * 0.5,
        b: FUCHSIA.b * 0.5,
        a: 0.8,
    };

    let base = iced_aw::date_picker::Style {
        background: Color::from_rgb(0.15, 0.15, 0.18).into(),
        border_radius: 12.0,
        border_width: 3.0,
        border_color: FUCHSIA,
        text_color: Color::WHITE,
        text_attenuated_color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.35,
        },
        day_background: Color::from_rgb(0.2, 0.2, 0.24).into(),
        button_background: fuchsia_dim.into(),
        button_text_color: Color::WHITE,
        button_border_radius: 12.0,
        button_border_width: 1.0,
        button_border_color: FUCHSIA,
        button_font_size: None,
    };

    match status {
        iced_aw::style::Status::Selected => iced_aw::date_picker::Style {
            day_background: FUCHSIA.into(),
            text_color: Color::WHITE,
            ..base
        },
        iced_aw::style::Status::Hovered => iced_aw::date_picker::Style {
            day_background: Color {
                r: FUCHSIA.r * 0.7,
                g: FUCHSIA.g * 0.7,
                b: FUCHSIA.b * 0.7,
                a: 0.6,
            }
            .into(),
            text_color: FUCHSIA,
            button_background: FUCHSIA.into(),
            button_text_color: Color::WHITE,
            ..base
        },
        iced_aw::style::Status::Focused => iced_aw::date_picker::Style {
            border_color: Color {
                r: FUCHSIA.r,
                g: FUCHSIA.g,
                b: FUCHSIA.b,
                a: 0.8,
            },
            ..base
        },
        _ => base,
    }
}

impl DatePickerExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChooseDate1 => {
                self.show_picker1 = true;
            }
            Message::SubmitDate1(date) => {
                self.date1 = date;
                self.show_picker1 = false;
            }
            Message::CancelDate1 => self.show_picker1 = false,
            Message::ChooseDate2 => {
                self.show_picker2 = true;
            }
            Message::SubmitDate2(date) => {
                self.date2 = date;
                self.show_picker2 = false;
            }
            Message::CancelDate2 => self.show_picker2 = false,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Left: default style
        let picker1 = date_picker(
            self.show_picker1,
            self.date1,
            Button::new(Text::new("Default Style")).on_press(Message::ChooseDate1),
            Message::CancelDate1,
            Message::SubmitDate1,
        );

        let col1 = Column::new()
            .align_x(Alignment::Center)
            .spacing(10)
            .push(Text::new("Default Theme"))
            .push(picker1)
            .push(Text::new(format!("Date: {}", self.date1)));

        // Right: fuchsia accent style
        let picker2 = date_picker(
            self.show_picker2,
            self.date2,
            Button::new(Text::new("Fuchsia Style")).on_press(Message::ChooseDate2),
            Message::CancelDate2,
            Message::SubmitDate2,
        )
        .style(fuchsia_style);

        let col2 = Column::new()
            .align_x(Alignment::Center)
            .spacing(10)
            .push(Text::new("Custom Fuchsia Theme"))
            .push(picker2)
            .push(Text::new(format!("Date: {}", self.date2)));

        let left = Container::new(col1)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let right = Container::new(col2)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        Row::new()
            .push(left)
            .push(right)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
