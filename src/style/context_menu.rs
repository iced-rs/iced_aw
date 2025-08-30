//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme};

/// The style of a [`ContextMenu`](crate::widget::ContextMenu).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`ContextMenu`](crate::widget::ContextMenu).
    ///
    /// This is used to color the backdrop of the modal.
    pub background: Background,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Background::Color([0.87, 0.87, 0.87, 0.30].into()),
        }
    }
}

/// The Catalog of a [`ContextMenu`](crate::widget::ContextMenu).
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

/// The primary theme of a [`ContextMenu`](crate::widget::ContextMenu).
#[must_use]
pub fn primary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Background::Color(Color {
            a: 0f32,
            ..palette.background.base.color
        }),
    }
}
