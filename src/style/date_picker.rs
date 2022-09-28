//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};

/// The appearance of a [`DatePicker`](crate::native::DatePicker).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`DatePicker`](crate::native::DatePicker).
    pub background: Background,

    /// The border radius of the [`DatePicker`](crate::native::DatePicker).
    pub border_radius: f32,

    /// The border with of the [`DatePicker`](crate::native::DatePicker).
    pub border_width: f32,

    /// The border color of the [`DatePicker`](crate::native::DatePicker).
    pub border_color: Color,

    /// The text color of the [`DatePicker`](crate::native::DatePicker).
    pub text_color: Color,

    /// The attenuated color of the days which are not in the selected month
    /// of the [`DatePicker`](crate::native::DatePicker).
    pub text_attenuated_color: Color,

    /// The background of the days in the calender of the
    /// [`DatePicker`](crate::native::DatePicker).
    pub day_background: Background,
}

/// The appearance of a [`DatePicker`](crate::native::DatePicker).
pub trait StyleSheet {
    type Appearance: std::default::Default + Copy;
    /// The normal appearance of a [`DatePicker`](crate::native::DatePicker).
    fn active(&self) -> Appearance;

    /// The appearance when something is selected of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn selected(&self) -> Appearance;

    /// The appearance when something is hovered of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn hovered(&self) -> Appearance;

    /// The appearance when something is focused of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn focused(&self) -> Appearance;
}

/// The default appearance of the [`DatePicker`](crate::native::DatePicker).
#[derive(Clone, Copy, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Appearance {
        Appearance {
            background: Color::WHITE.into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            text_attenuated_color: [0.87, 0.87, 0.87].into(),
            day_background: Color::WHITE.into(),
        }
    }

    fn selected(&self) -> Appearance {
        Appearance {
            day_background: Background::Color([0.87, 0.87, 0.87].into()),
            ..self.active()
        }
    }

    fn hovered(&self) -> Appearance {
        Appearance {
            day_background: Background::Color([0.87, 0.87, 0.87].into()),
            ..self.active()
        }
    }

    fn focused(&self) -> Appearance {
        Appearance {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active()
        }
    }
}
