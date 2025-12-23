//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme};

/// The appearance of a [`ColorPicker`](crate::widget::ColorPicker).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`ColorPicker`](crate::widget::ColorPicker).
    pub background: Background,

    /// The border radius of the [`ColorPicker`](crate::widget::ColorPicker).
    pub border_radius: f32,

    /// The border with of the [`ColorPicker`](crate::widget::ColorPicker).
    pub border_width: f32,

    /// The border color of the [`ColorPicker`](crate::widget::ColorPicker).
    pub border_color: Color,

    /// The border radius of the bars of the [`ColorPicker`](crate::widget::ColorPicker).
    pub bar_border_radius: f32,

    /// The border width of the bars of the [`ColorPicker`](crate::widget::ColorPicker).
    pub bar_border_width: f32,

    /// The border color of the bars of the [`ColorPicker`](crate::widget::ColorPicker).
    pub bar_border_color: Color,
}

/// The Catalog of a [`ColorPicker`](crate::widget::ColorPicker).
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

/// The primary theme of a [`Badge`](crate::widget::badge::Badge).
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

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::{Background, Theme};

    #[test]
    fn primary_theme_active() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
        assert_eq!(style.border_width, 1.0);
        assert_eq!(style.bar_border_radius, 5.0);
        assert_eq!(style.bar_border_width, 1.0);
    }

    #[test]
    fn primary_theme_focused() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Focused);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
        assert_eq!(style.border_width, 1.0);
        assert_eq!(style.bar_border_radius, 5.0);
        assert_eq!(style.bar_border_width, 1.0);
    }

    #[test]
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn primary_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn focused_changes_border_colors() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let focused_style = primary(&theme, Status::Focused);

        // Border colors should be different when focused
        assert_ne!(base_style.border_color, focused_style.border_color);
        assert_ne!(base_style.bar_border_color, focused_style.bar_border_color);

        // Other properties should remain the same
        assert_eq!(base_style.background, focused_style.background);
        assert_eq!(base_style.border_radius, focused_style.border_radius);
        assert_eq!(base_style.border_width, focused_style.border_width);
        assert_eq!(
            base_style.bar_border_radius,
            focused_style.bar_border_radius
        );
        assert_eq!(base_style.bar_border_width, focused_style.bar_border_width);
    }

    #[test]
    fn catalog_default_class() {
        let _class = <Theme as Catalog>::default();
    }

    #[test]
    fn catalog_style() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();
        let style = theme.style(&class, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
        assert_eq!(style.border_width, 1.0);
        assert_eq!(style.bar_border_radius, 5.0);
        assert_eq!(style.bar_border_width, 1.0);
    }

    #[test]
    fn catalog_style_focused() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();
        let style = theme.style(&class, Status::Focused);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
    }

    #[test]
    fn style_fields_are_valid() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        // Verify all fields have valid values
        assert!(style.border_radius > 0.0);
        assert!(style.border_width > 0.0);
        assert!(style.bar_border_radius > 0.0);
        assert!(style.bar_border_width > 0.0);
    }
}
