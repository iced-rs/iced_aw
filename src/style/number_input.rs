//! Display fields that can only be filled with numeric type.
//!
//! *This API requires the following crate features to be activated: `number_input`*

use super::{Status, StyleFn};
use iced::{widget, Background, Color, Theme};

/// The appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`NumberInput`](crate::native::number_input::NumberInput).
    pub button_background: Option<Background>,
    /// The Color of the arrows of [`NumberInput`](crate::native::number_input::NumberInput).
    pub icon_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            button_background: None,
            icon_color: Color::BLACK,
        }
    }
}

/// The Catalog of a [`NumberInput`](crate::native::number_input::NumberInput).
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

/// The Extended Catalog of a [`NumberInput`](crate::native::number_input::NumberInput).
pub trait ExtendedCatalog:
    widget::text_input::Catalog + widget::container::Catalog + widget::text::Catalog + self::Catalog
{
    /// The default class produced by the [`Catalog`].
    #[must_use]
    fn default_input<'a>() -> <Self as widget::text_input::Catalog>::Class<'a> {
        <Self as widget::text_input::Catalog>::default()
    }

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &<Self as self::Catalog>::Class<'_>, status: Status) -> Style;
}

impl ExtendedCatalog for Theme {
    fn style(&self, class: &<Self as self::Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The primary theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        button_background: Some(palette.primary.strong.color.into()),
        icon_color: palette.primary.strong.text,
    };

    match status {
        Status::Active | Status::Pressed | Status::Focused | Status::Hovered => base,
        Status::Disabled => Style {
            button_background: base.button_background.map(|bg| match bg {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
                Background::Gradient(grad) => Background::Gradient(grad),
            }),
            icon_color: Color {
                a: base.icon_color.a * 0.5,
                ..base.icon_color
            },
        },
    }
}
