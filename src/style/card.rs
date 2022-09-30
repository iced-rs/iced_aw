//! Displays a [`Card`](crate::native::Card).
//!
//! *This API requires the following crate features to be activated: card*
#[cfg(not(target_arch = "wasm32"))]
use super::colors;
use iced_native::{Background, Color};
use iced_style::Theme;

/// The appearance of a [`Card`](crate::native::card::Card).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`Card`](crate::native::card::Card).
    pub background: Background,

    /// The border radius of the [`Card`](crate::native::card::Card).
    pub border_radius: f32,

    /// The border width of the [`Card`](crate::native::card::Card).
    pub border_width: f32,

    /// The border color of the [`Card`](crate::native::card::Card).
    pub border_color: Color,

    /// The background of the head of the [`Card`](crate::native::card::Card).
    pub head_background: Background,

    /// The text color of the head of the [`Card`](crate::native::card::Card).
    pub head_text_color: Color,

    /// The background of the body of the [`Card`](crate::native::card::Card).
    pub body_background: Background,

    /// The text color of the body of the [`Card`](crate::native::card::Card).
    pub body_text_color: Color,

    /// The background of the foot of the [`Card`](crate::native::card::Card).
    pub foot_background: Background,

    /// The text color of the foot of the [`Card`](crate::native::card::Card).
    pub foot_text_color: Color,

    /// The color of the close icon of the [`Card`](crate::native::card::Card).
    pub close_color: Color,
}

/// The appearance of a [`Card`](crate::native::card::Card).
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub trait StyleSheet {
    type Style: std::default::Default + Copy;
    /// The normal appearance of a [`Card`](crate::native::card::Card).
    fn active(&self, style: Self::Style) -> Appearance;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``Card`` Styles
pub enum CardStyles {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    White,
    #[default]
    Default,
}

impl StyleSheet for Theme {
    type Style = CardStyles;

    fn active(&self, style: Self::Style) -> Appearance {
        let backing_with_text = |color: Color, text_color: Color| Appearance {
            border_color: color,
            head_background: color.into(),
            head_text_color: text_color,
            close_color: text_color,
            ..Appearance::default()
        };

        let backing_only = |color: Color| Appearance {
            border_color: color,
            head_background: color.into(),
            ..Appearance::default()
        };

        match style {
            CardStyles::Primary => backing_with_text(colors::PRIMARY, colors::WHITE),
            CardStyles::Secondary => backing_with_text(colors::SECONDARY, colors::WHITE),
            CardStyles::Success => backing_with_text(colors::SUCCESS, colors::WHITE),
            CardStyles::Danger => backing_with_text(colors::DANGER, colors::WHITE),
            CardStyles::Warning => backing_only(colors::WARNING),
            CardStyles::Info => backing_only(colors::INFO),
            CardStyles::Light => backing_only(colors::LIGHT),
            CardStyles::Dark => backing_with_text(colors::DARK, colors::WHITE),
            CardStyles::White => backing_only(colors::WHITE),
            CardStyles::Default => Appearance::default(),
        }
    }
}

impl std::default::Default for Appearance {
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
