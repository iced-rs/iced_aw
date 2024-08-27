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

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            text_color: Color::BLACK,
            background: Background::Color([0.87, 0.87, 0.87].into()),
            border_width: 1.0,
            border_color: [0.7, 0.7, 0.7].into(),
        }
    }
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
pub fn primary(_theme: &Theme, status: Status) -> Style {
    let base = Style::default();

    match status {
        Status::Hovered => Style {
            text_color: Color::WHITE,
            background: Background::Color([0.0, 0.5, 1.0].into()),
            ..base
        },
        Status::Selected => Style {
            text_color: Color::WHITE,
            background: Background::Color([0.2, 0.5, 0.8].into()),
            ..base
        },
        _ => base,
    }
}
