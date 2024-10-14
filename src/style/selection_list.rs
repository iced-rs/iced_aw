//! Selection List
//!
//! *This API requires the following crate features to be activated: `selection_list`*

use super::{Status, StyleFn};
use iced::{Background, Color, Theme};

/// The appearance of a menu.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The List Label Text Color
    pub text_color: Color,
    /// The background
    pub background: Background,
    /// The container Border width
    pub border_width: f32,
    /// The container Border color
    pub border_color: Color,
}

/// The Catalog of a [`Badge`](crate::widget::selection_list::SelectionList).
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

/// The primary theme of a [`Badge`](crate::widget::selection_list::SelectionList).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.background.base.text,
        background: palette.background.base.color.into(),
        border_width: 1.0,
        border_color: palette.background.weak.color,
    };

    match status {
        Status::Hovered => Style {
            text_color: palette.primary.base.text,
            background: palette.primary.base.color.into(),
            ..base
        },
        Status::Selected => Style {
            text_color: palette.primary.strong.text,
            background: palette.primary.strong.color.into(),
            ..base
        },
        _ => base,
    }
}
