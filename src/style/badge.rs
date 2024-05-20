//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use super::{
    colors,
    status::{Status, StyleFn},
};
use iced::{theme::palette, Background, Color, Theme};

/// The style of a [`Badge`](crate::native::badge::Badge).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`Badge`](crate::native::badge::Badge).
    pub background: Background,

    /// The border radius of the [`Badge`](crate::native::badge::Badge).
    /// If no radius is specified the default one will be used.
    pub border_radius: Option<f32>,

    /// The border with of the [`Badge`](crate::native::badge::Badge).
    pub border_width: f32,

    /// The border color of the [`Badge`](crate::native::badge::Badge).
    pub border_color: Option<Color>,

    /// The default text color of the [`Badge`](crate::native::badge::Badge).
    pub text_color: Color,
}

/// The Catalog of a [`Badge`](crate::native::badge::Badge).
pub trait Catalog {
    ///Style for the trait to use.
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            background: Background::Color([0.87, 0.87, 0.87].into()),
            border_radius: None,
            border_width: 1.0,
            border_color: Some([0.8, 0.8, 0.8].into()),
            text_color: Color::BLACK,
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

/// The primary theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.primary.strong);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// The secondary theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.secondary.strong);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// The success theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn success(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.success.strong);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// The danger theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.danger.strong);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// The warning theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn warning(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::WARNING, colors::BLACK);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: base.background,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// The info theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn info(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::INFO, colors::BLACK);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: base.background,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// The light theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn light(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::LIGHT, colors::BLACK);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: base.background,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// The dark theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn dark(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::DARK, colors::WHITE);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: base.background,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// The white theme of a [`Badge`](crate::native::badge::Badge).
#[must_use]
pub fn white(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::WHITE, colors::BLACK);

    match status {
        Status::Active | Status::Pressed | Status::Focused => base,
        Status::Hovered => Style {
            background: base.background,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn from_color(color: Color, text_color: Color) -> Style {
    Style {
        background: Background::Color(color),
        border_color: Some(color),
        text_color,
        ..Style::default()
    }
}

fn styled(pair: palette::Pair) -> Style {
    Style {
        background: Background::Color(pair.color),
        border_color: Some(pair.color),
        text_color: pair.text,
        ..Style::default()
    }
}

fn disabled(style: Style) -> Style {
    Style {
        background: style.background.scale_alpha(0.5),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}
