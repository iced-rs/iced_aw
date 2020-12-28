//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: date_picker*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
#[cfg(target_arch = "wasm32")]
use iced_web::{Background, Color};

/// The appearance of a [`DatePicker`](crate::native::DatePicker).
#[allow(missing_debug_implementations)]
pub struct Style {
    /// The background of the [`DatePicker`](crate::native::DatePicker).
    pub background: Background,

    /// The border radius of the [`DatePicker`](crate::native::DatePicker).
    pub border_radius: f32,

    /// The border width of the [`DatePicker`](crate::native::DatePicker).
    pub border_width: f32,

    /// The border color of the [`DatePicker`](crate::native::DatePicker).
    pub border_color: Color,

    /// The text_color of the [`DatePicker`](crate::native::DatePicker).
    pub text_color: Color,

    /// The attenuated color of the days which are not in the selected month
    /// of the [`DatePicker`](crate::native::DatePicker).
    pub text_attenuated_color: Color,

    /// TODO: Button support
    //pub button: Box<dyn iced_style::button::StyleSheet>,

    /// The background of the selected day of the
    /// [`DatePicker`](crate::native::DatePicker).
    pub day_selected_background: Background,

    /// The text color of the selected day of the
    /// [`DatePicker`](crate::native::DatePicker).
    pub day_selected_color: Color,

    /// The background of the hovered day of the
    /// [`DatePicker`](crate::native::DatePicker).
    pub day_hover_background: Background,

    /// The text color of the hovered day of the
    /// [`DatePicker`](crate::native::DatePicker).
    pub day_hover_color: Color,
}

/// The appearance of a [`DatePicker`](crate::native::DatePicker).
pub trait StyleSheet {
    /// The normal appearance of a [`DatePicker`](crate::native::DatePicker).
    fn active(&self) -> Style;
}

/// The default appearance of the [`DatePicker`](crate::native::DatePicker).
#[derive(Clone, Debug)]
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
            day_selected_background: Background::Color([0.87, 0.87, 0.87].into()),
            day_selected_color: Color::BLACK,
            day_hover_background: Background::Color([0.87, 0.87, 0.87].into()),
            day_hover_color: Color::BLACK,
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}