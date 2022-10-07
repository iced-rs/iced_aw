//! Use a split to split the available space in two parts to display two different elements.
//!
//! *This API requires the following crate features to be activated: split*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
use iced_style::Theme;

/// The appearance of a [`Split`](crate::native::split::Split).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
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
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub trait StyleSheet {
    type Style: Default + Copy;
    /// The normal appearance of a [`Split`](crate::native::split::Split).
    fn active(&self, style: Self::Style) -> Appearance;

    /// The appearance when the [`Split`](crate::native::split::Split) is hovered.
    fn hovered(&self, style: Self::Style) -> Appearance;

    /// The appearance when the divider of the [`Split`](crate::native::split::Split) is dragged
    fn dragged(&self, style: Self::Style) -> Appearance;
}

/// The default appearance of the [`Split`](crate::native::split::Split).
#[derive(Clone, Copy, Debug, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``Split`` Styles
pub enum SplitStyles {
    #[default]
    Default,
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
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
}

impl StyleSheet for Theme {
    type Style = SplitStyles;
    fn active(&self, _style: Self::Style) -> Appearance {
        Appearance::default()
    }

    fn hovered(&self, _style: Self::Style) -> Appearance {
        Appearance {
            divider_background: Color::from_rgb(0.8, 0.8, 0.8).into(),
            ..Appearance::default()
        }
    }

    fn dragged(&self, _style: Self::Style) -> Appearance {
        Appearance {
            divider_background: Color::from_rgb(0.7, 0.7, 0.7).into(),
            ..Appearance::default()
        }
    }
}
