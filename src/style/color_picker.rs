//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use super::{Status, StyleFn};
use iced::{Background, Color, Theme};

/// The appearance of a [`ColorPicker`](crate::native::ColorPicker).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`ColorPicker`](crate::native::ColorPicker).
    pub background: Background,

    /// The border radius of the [`ColorPicker`](crate::native::ColorPicker).
    pub border_radius: f32,

    /// The border with of the [`ColorPicker`](crate::native::ColorPicker).
    pub border_width: f32,

    /// The border color of the [`ColorPicker`](crate::native::ColorPicker).
    pub border_color: Color,

    /// The border radius of the bars of the [`ColorPicker`](crate::native::ColorPicker).
    pub bar_border_radius: f32,

    /// The border width of the bars of the [`ColorPicker`](crate::native::ColorPicker).
    pub bar_border_width: f32,

    /// The border color of the bars of the [`ColorPicker`](crate::native::ColorPicker).
    pub bar_border_color: Color,
}

/// The Catalog of a [`ColorPicker`](crate::native::ColorPicker).
pub trait Catalog {
    ///Style for the trait to use.
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The primary theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let foreground = theme.palette();

    let base = Style {
        background: palette.background.base.color.into(),
        border_radius: 15.0,
        border_width: 1.0,
        border_color: foreground.text,
        bar_border_radius: 5.0,
        bar_border_width: 1.0,
        bar_border_color: foreground.text,
    };

    match status {
        Status::Focused => Style {
            border_color: palette.background.strong.color,
            bar_border_color: palette.background.strong.color,
            ..base
        },
        _ => base,
    }
}
