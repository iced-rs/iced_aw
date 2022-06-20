//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};

/// The appearance of a [`TimePicker`](crate::native::TimePicker).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`TimePicker`](crate::native::TimePicker).
    pub background: Background,

    /// The border radius of the [`TimePicker`](crate::native::TimePicker).
    pub border_radius: f32,

    /// The border width of the [`TimePicker`](crate::native::TimePicker).
    pub border_width: f32,

    /// The border color of the [`TimePicker`](crate::native::TimePicker).
    pub border_color: Color,

    /// The text color of the [`TimePicker`](crate::native::TimePicker).
    pub text_color: Color,

    /// The color of the clock numbers of the
    /// [`TimePicker`](crate::native::TimePicker).
    pub clock_number_color: Color,

    /// The background of the clock numbers of the
    /// [`TimePicker`](crate::native::TimePicker).
    pub clock_number_background: Color,

    /// The color of the dots on the clock of the
    /// [`TimePicker`](crate::native::TimePicker).
    pub clock_dots_color: Color,

    /// The color of the hands of the clock of the
    /// [`TimePicker`](crate::native::TimePicker).
    pub clock_hand_color: Color,

    /// The with of the hands of the clock of the
    /// [`TimePicker](crate::native::TimePicker).
    pub clock_hand_width: f32,
}

/// The appearance of a [`TimePicker`](crate::native::TimePicker).
pub trait StyleSheet {
    /// The normal appearance of a [`TimePicker`](crate::native::TimePicker).
    fn active(&self) -> Style;

    /// The appearance when something is selected of the
    /// [`TimePicker`](crate::native::TimePicker)
    fn selected(&self) -> Style;

    /// The appearance when something is hovered of the
    /// [`TimePicker`](crate::native::TimePicker).
    fn hovered(&self) -> Style;

    /// The appearance when something is focused of the
    /// [`TimePicker`](crate::native::TimePicker).
    fn focused(&self) -> Style;
}

/// The default appearance of the [`TimePicker`](crate::native::TimePicker)
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
            clock_number_color: Color::BLACK,
            clock_number_background: Color::WHITE,
            clock_dots_color: [0.87, 0.87, 0.87].into(),
            clock_hand_color: [0.87, 0.87, 0.87].into(),
            clock_hand_width: 1.0,
        }
    }

    fn selected(&self) -> Style {
        Style {
            clock_number_background: [0.87, 0.87, 0.87].into(),
            ..self.active()
        }
    }

    fn hovered(&self) -> Style {
        Style {
            clock_number_background: [0.87, 0.87, 0.87].into(),
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
