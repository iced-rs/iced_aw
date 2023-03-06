//! Use a spinner to suggest to the user something is loading.
//!
//! *This API requires the following crate features to be activated: spinner*

use iced_native::Color;
use iced_style::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(missing_docs)]
pub struct SpinnerStyle;

/// The appearance of a [`Spinner`](crate::native::spinner::Spinner).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The radius of the spinning circle.
    pub circle_radius: f32,
    /// The color of the spinning circle.
    pub circle_color: Color,
    /// The padding of the spinning circle.
    pub padding: f32,
}

/// A set of rules that dictate the style of a [`Spinner`](crate::native::spinner::Spinner).
pub trait StyleSheet {
    /// Style for the trait to use.
    type Style: Default + Copy;
    /// The normal appearance of a [`Spinner`](crate::native::spinner::Spinner).
    fn active(&self, style: Self::Style) -> Appearance;
}

impl StyleSheet for Theme {
    type Style = SpinnerStyle;

    fn active(&self, _style: Self::Style) -> Appearance {
        Appearance {
            circle_color: self.palette().text,
            circle_radius: 2.0,
            padding: 0.0,
        }
    }
}
