//! Use a spinner to suggest to the user something is loading.
//!
//! *This API requires the following crate features to be activated: spinner*

use iced_widget::style::Theme;

/// The style of a [`Spinner`](crate::native::spinner::Spinner).
#[derive(Default)]
#[allow(missing_debug_implementations)]
pub enum SpinnerStyle {
    /// The default style
    #[default]
    Default,
    /// Custom style
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

/// The appearance of a [`Spinner`](crate::native::spinner::Spinner).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {}

/// A set of rules that dictate the style of a [`Spinner`](crate::native::spinner::Spinner).
pub trait StyleSheet {
    /// Style for the trait to use.
    type Style: Default;
    /// The normal appearance of a [`Spinner`](crate::native::spinner::Spinner).
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

impl StyleSheet for Theme {
    type Style = SpinnerStyle;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {}
    }
}
