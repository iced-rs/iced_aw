//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use super::colors;
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
use iced_style::Theme;

/// The appearance of a [`Badge`](crate::native::badge::Badge).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
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

/// The appearance of a [`Badge`](crate::native::badge::Badge).
pub trait StyleSheet {
    ///Style for the trait to use.
    type Style: Default + Copy;
    /// The normal appearance of a [`Badge`](crate::native::badge::Badge).
    fn active(&self, style: Self::Style) -> Appearance;

    /// The appearance when the [`Badge`](crate::native::badge::Badge) is hovered.
    fn hovered(&self, style: Self::Style) -> Appearance {
        self.active(style)
    }
}

impl std::default::Default for Appearance {
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

#[derive(Debug, Clone, Copy, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``Badge`` Styles
pub enum BadgeStyles {
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
    type Style = BadgeStyles;

    fn active(&self, style: Self::Style) -> Appearance {
        let from_colors = |color: Color, text_color: Color| Appearance {
            background: Background::Color(color),
            border_color: Some(color),
            text_color,
            ..Appearance::default()
        };

        match style {
            BadgeStyles::Primary => from_colors(colors::PRIMARY, colors::WHITE),
            BadgeStyles::Secondary => from_colors(colors::SECONDARY, colors::WHITE),
            BadgeStyles::Success => from_colors(colors::SUCCESS, colors::WHITE),
            BadgeStyles::Danger => from_colors(colors::DANGER, colors::WHITE),
            BadgeStyles::Warning => from_colors(colors::WARNING, colors::BLACK),
            BadgeStyles::Info => from_colors(colors::INFO, colors::BLACK),
            BadgeStyles::Light => from_colors(colors::LIGHT, colors::BLACK),
            BadgeStyles::Dark => from_colors(colors::DARK, colors::WHITE),
            BadgeStyles::White => from_colors(colors::WHITE, colors::BLACK),
            BadgeStyles::Default => Appearance::default(),
        }
    }

    fn hovered(&self, style: Self::Style) -> Appearance {
        self.active(style)
    }
}
