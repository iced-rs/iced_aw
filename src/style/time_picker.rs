//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
#![allow(clippy::doc_markdown)]
use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme};

/// The style of a [`TimePicker`](crate::widget::TimePicker).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`TimePicker`](crate::widget::TimePicker).
    pub background: Background,

    /// The border radius of the [`TimePicker`](crate::widget::TimePicker).
    pub border_radius: f32,

    /// The border width of the [`TimePicker`](crate::widget::TimePicker).
    pub border_width: f32,

    /// The border color of the [`TimePicker`](crate::widget::TimePicker).
    pub border_color: Color,

    /// The text color of the [`TimePicker`](crate::widget::TimePicker).
    pub text_color: Color,

    /// The color of the clock numbers of the
    /// [`TimePicker`](crate::widget::TimePicker).
    pub clock_number_color: Color,

    /// The background of the clock numbers of the
    /// [`TimePicker`](crate::widget::TimePicker).
    pub clock_number_background: Color,

    /// The color of the dots on the clock of the
    /// [`TimePicker`](crate::widget::TimePicker).
    pub clock_dots_color: Color,

    /// The color of the hands of the clock of the
    /// [`TimePicker`](crate::widget::TimePicker).
    pub clock_hand_color: Color,

    /// The with of the hands of the clock of the
    /// [`TimePicker](crate::widget::TimePicker).
    pub clock_hand_width: f32,
}

/// The Catalog of a [`TimePicker`](crate::widget::TimePicker).
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

/// The primary theme of a [`TimePicker`](crate::widget::TimePicker).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let foreground = theme.palette();

    let base = Style {
        background: palette.background.base.color.into(),
        border_radius: 15.0,
        border_width: 1.0,
        border_color: foreground.text,
        text_color: foreground.text,
        clock_number_color: foreground.text,
        clock_number_background: palette.background.base.color,
        clock_dots_color: [0.87, 0.87, 0.87].into(),
        clock_hand_color: [0.87, 0.87, 0.87].into(),
        clock_hand_width: 3.0,
    };

    match status {
        Status::Focused => Style {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..base
        },
        Status::Hovered => Style {
            clock_number_color: palette.primary.weak.text,
            clock_number_background: palette.primary.weak.color,
            ..base
        },
        Status::Selected => Style {
            clock_number_color: palette.primary.strong.text,
            clock_number_background: palette.primary.strong.color,
            ..base
        },
        _ => base,
    }
}
