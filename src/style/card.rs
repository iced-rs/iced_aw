//! Displays a [`Card`](crate::widgets::Card).
//!
//! *This API requires the following crate features to be activated: card*

use super::{colors, Status, StyleFn};
use iced::{Background, Color, Theme};

/// The appearance of a [`Card`](crate::widgets::card::Card).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`Card`](crate::widgets::card::Card).
    pub background: Background,

    /// The border radius of the [`Card`](crate::widgets::card::Card).
    pub border_radius: f32,

    /// The border width of the [`Card`](crate::widgets::card::Card).
    pub border_width: f32,

    /// The border color of the [`Card`](crate::widgets::card::Card).
    pub border_color: Color,

    /// The background of the head of the [`Card`](crate::widgets::card::Card).
    pub head_background: Background,

    /// The text color of the head of the [`Card`](crate::widgets::card::Card).
    pub head_text_color: Color,

    /// The background of the body of the [`Card`](crate::widgets::card::Card).
    pub body_background: Background,

    /// The text color of the body of the [`Card`](crate::widgets::card::Card).
    pub body_text_color: Color,

    /// The background of the foot of the [`Card`](crate::widgets::card::Card).
    pub foot_background: Background,

    /// The text color of the foot of the [`Card`](crate::widgets::card::Card).
    pub foot_text_color: Color,

    /// The color of the close icon of the [`Card`](crate::widgets::card::Card).
    pub close_color: Color,
}

/// The appearance of a [`Card`](crate::widgets::card::Card).
pub trait Catalog {
    ///Style for the trait to use.
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Color::WHITE.into(),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: [0.87, 0.87, 0.87].into(),
            head_background: Background::Color([0.87, 0.87, 0.87].into()),
            head_text_color: Color::BLACK,
            body_background: Color::TRANSPARENT.into(),
            body_text_color: Color::BLACK,
            foot_background: Color::TRANSPARENT.into(),
            foot_text_color: Color::BLACK,
            close_color: Color::BLACK,
        }
    }
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

/// The primary theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn primary(theme: &Theme, _status: Status) -> Style {
    backing_with_text(theme, colors::PRIMARY, colors::WHITE)
}

/// The secondary theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn secondary(theme: &Theme, _status: Status) -> Style {
    backing_with_text(theme, colors::SECONDARY, colors::WHITE)
}

/// The success theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn success(theme: &Theme, _status: Status) -> Style {
    backing_with_text(theme, colors::SUCCESS, colors::WHITE)
}

/// The danger theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn danger(theme: &Theme, _status: Status) -> Style {
    backing_with_text(theme, colors::DANGER, colors::WHITE)
}

/// The warning theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn warning(theme: &Theme, _status: Status) -> Style {
    backing_only(theme, colors::WARNING)
}

/// The info theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn info(theme: &Theme, _status: Status) -> Style {
    backing_only(theme, colors::INFO)
}

/// The light theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn light(theme: &Theme, _status: Status) -> Style {
    backing_only(theme, colors::LIGHT)
}

/// The dark theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn dark(theme: &Theme, _status: Status) -> Style {
    backing_with_text(theme, colors::DARK, colors::WHITE)
}

/// The white theme of a [`Card`](crate::widgets::card::Card).
#[must_use]
pub fn white(theme: &Theme, _status: Status) -> Style {
    backing_only(theme, colors::WHITE)
}

fn backing_with_text(theme: &Theme, color: Color, text_color: Color) -> Style {
    let palette = theme.extended_palette();
    let foreground = theme.palette();

    Style {
        border_color: color,
        head_background: color.into(),
        head_text_color: text_color,
        close_color: text_color,
        background: palette.background.base.color.into(),
        body_text_color: foreground.text,
        foot_text_color: foreground.text,
        ..Style::default()
    }
}

fn backing_only(theme: &Theme, color: Color) -> Style {
    let palette = theme.extended_palette();
    let foreground = theme.palette();

    Style {
        border_color: color,
        head_background: color.into(),
        background: palette.background.base.color.into(),
        body_text_color: foreground.text,
        foot_text_color: foreground.text,
        ..Style::default()
    }
}
