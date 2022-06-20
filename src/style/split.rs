//! Use a split to split the available space in two parts to display two different elements.
//!
//! *This API requires the following crate features to be activated: split*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};

/// The appearance of a [`Split`](crate::native::split::Split).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The optional background of the [`Split`](crate::native::split::Split).
    pub background: Option<Background>,
    /// The optional background of the first element of the [`Split`](crate::native::split::Split).
    pub first_background: Option<Background>,
    /// The optional background of the second element of the [`Split`](crate::native::split::Split).
    pub second_background: Option<Background>,
    /// The border width of the [`Split`](crate::native::split::Split).
    pub border_width: f32,
    /// The border color of the [`Split`](crate::native::split::Split).
    pub border_color: Color,
    /// The background of the divider of the [`Split`](crate::native::split::Split).
    pub divider_background: Background,
    /// The border width of the divider of the [`Split`](crate::native::split::Split).
    pub divider_border_width: f32,
    /// The border color of the divider of the [`Split`](crate::native::split::Split).
    pub divider_border_color: Color,
}

/// The appearance of a [`Split`](crate::native::split::Split).
pub trait StyleSheet {
    /// The normal appearance of a [`Split`](crate::native::split::Split).
    fn active(&self) -> Style;

    /// The appearance when the [`Split`](crate::native::split::Split) is hovered.
    fn hovered(&self) -> Style;

    /// The appearance when the divider of the [`Split`](crate::native::split::Split) is dragged
    fn dragged(&self) -> Style;
}

/// The default appearance of the [`Split`](crate::native::split::Split).
#[derive(Clone, Copy, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            background: None,
            first_background: None,
            second_background: None,
            border_width: 1.0,
            border_color: Color::from_rgb(0.6, 0.6, 0.6),
            divider_background: Color::WHITE.into(),
            divider_border_width: 1.0,
            divider_border_color: Color::from_rgb(0.8, 0.8, 0.8),
        }
    }

    fn hovered(&self) -> Style {
        Style {
            divider_background: Color::from_rgb(0.8, 0.8, 0.8).into(),
            ..self.active()
        }
    }

    fn dragged(&self) -> Style {
        Style {
            divider_background: Color::from_rgb(0.7, 0.7, 0.7).into(),
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
