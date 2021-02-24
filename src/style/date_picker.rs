//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
#[cfg(target_arch = "wasm32")]
use iced_web::{Background, Color};

/// The appearance of a [`DatePicker`](crate::native::DatePicker).
#[derive(Clone, Copy, Debug)]
pub struct Style {
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
    /// The normal appearance of a [`DatePicker`](crate::native::DatePicker).
    fn active(&self) -> Style;

    /// The appearance when something is selected of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn selected(&self) -> Style;

    /// The appearance when something is hovered of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn hovered(&self) -> Style;

    /// The appearance when something is focused of the
    /// [`DatePicker`](crate::native::DatePicker).
    fn focused(&self) -> Style;
}

/// The default appearance of the [`DatePicker`](crate::native::DatePicker).
#[derive(Clone, Copy, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            background: Color::WHITE.into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            text_attenuated_color: [0.87, 0.87, 0.87].into(),
            day_background: Color::WHITE.into(),
        }
    }

    fn selected(&self) -> Style {
        Style {
            day_background: Background::Color([0.87, 0.87, 0.87].into()),
            ..self.active()
        }
    }

    fn hovered(&self) -> Style {
        Style {
            day_background: Background::Color([0.87, 0.87, 0.87].into()),
            ..self.active()
        }
    }

    fn focused(&self) -> Style {
        Style {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active()
        }
    }
}

#[allow(clippy::use_self)]
impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

#[allow(clippy::use_self)]
impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
