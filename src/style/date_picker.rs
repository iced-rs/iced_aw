//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*
use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme};

/// The appearance of a [`DatePicker`](crate::widget::DatePicker).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`DatePicker`](crate::widget::DatePicker).
    pub background: Background,

    /// The border radius of the [`DatePicker`](crate::widget::DatePicker).
    pub border_radius: f32,

    /// The border with of the [`DatePicker`](crate::widget::DatePicker).
    pub border_width: f32,

    /// The border color of the [`DatePicker`](crate::widget::DatePicker).
    pub border_color: Color,

    /// The text color of the [`DatePicker`](crate::widget::DatePicker).
    pub text_color: Color,

    /// The attenuated color of the days which are not in the selected month
    /// of the [`DatePicker`](crate::widget::DatePicker).
    pub text_attenuated_color: Color,

    /// The background of the days in the calender of the
    /// [`DatePicker`](crate::widget::DatePicker).
    pub day_background: Background,
}

/// The Catalog of a [`DatePicker`](crate::widget::DatePicker).
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
        text_color: foreground.text,
        text_attenuated_color: Color {
            a: foreground.text.a * 0.5,
            ..foreground.text
        },
        day_background: palette.background.base.color.into(),
    };

    match status {
        Status::Selected => Style {
            day_background: palette.primary.strong.color.into(),
            text_color: palette.primary.strong.text,
            ..base
        },
        Status::Hovered => Style {
            day_background: palette.primary.weak.color.into(),
            text_color: palette.primary.weak.text,
            ..base
        },
        Status::Focused => Style {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
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
        assert!(matches!(style.day_background, Background::Color(_)));
    }

    #[test]
    fn primary_theme_selected() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Selected);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(matches!(style.day_background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(matches!(style.day_background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn primary_theme_focused() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Focused);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_color, Color::from_rgb(0.5, 0.5, 0.5));
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
    fn selected_changes_day_background_and_text() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let selected_style = primary(&theme, Status::Selected);

        // Day background and text color should be different when selected
        assert_ne!(
            format!("{:?}", base_style.day_background),
            format!("{:?}", selected_style.day_background)
        );
        assert_ne!(base_style.text_color, selected_style.text_color);

        // Other properties should remain the same
        assert_eq!(base_style.background, selected_style.background);
        assert_eq!(base_style.border_radius, selected_style.border_radius);
        assert_eq!(base_style.border_width, selected_style.border_width);
        assert_eq!(base_style.border_color, selected_style.border_color);
    }

    #[test]
    fn hovered_changes_day_background_and_text() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let hovered_style = primary(&theme, Status::Hovered);

        // Day background and text color should be different when hovered
        assert_ne!(
            format!("{:?}", base_style.day_background),
            format!("{:?}", hovered_style.day_background)
        );
        assert_ne!(base_style.text_color, hovered_style.text_color);

        // Other properties should remain the same
        assert_eq!(base_style.background, hovered_style.background);
        assert_eq!(base_style.border_radius, hovered_style.border_radius);
        assert_eq!(base_style.border_width, hovered_style.border_width);
        assert_eq!(base_style.border_color, hovered_style.border_color);
    }

    #[test]
    fn focused_changes_border_color() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let focused_style = primary(&theme, Status::Focused);

        // Border color should be different when focused
        assert_ne!(base_style.border_color, focused_style.border_color);
        assert_eq!(focused_style.border_color, Color::from_rgb(0.5, 0.5, 0.5));

        // Other properties should remain the same
        assert_eq!(base_style.background, focused_style.background);
        assert_eq!(base_style.border_radius, focused_style.border_radius);
        assert_eq!(base_style.border_width, focused_style.border_width);
        assert_eq!(base_style.text_color, focused_style.text_color);
        assert_eq!(
            format!("{:?}", base_style.day_background),
            format!("{:?}", focused_style.day_background)
        );
    }

    #[test]
    fn text_attenuated_color_has_reduced_alpha() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        // Attenuated color should have half the alpha
        assert!(
            style.text_attenuated_color.a <= style.text_color.a,
            "Attenuated color should have reduced alpha"
        );
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
    }

    #[test]
    fn catalog_style_with_different_statuses() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();

        let active_style = theme.style(&class, Status::Active);
        let selected_style = theme.style(&class, Status::Selected);
        let hovered_style = theme.style(&class, Status::Hovered);
        let focused_style = theme.style(&class, Status::Focused);

        assert!(matches!(active_style.background, Background::Color(_)));
        assert!(matches!(selected_style.background, Background::Color(_)));
        assert!(matches!(hovered_style.background, Background::Color(_)));
        assert!(matches!(focused_style.background, Background::Color(_)));
    }

    #[test]
    fn style_fields_are_valid() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        // Verify all fields have valid values
        assert!(style.border_radius > 0.0);
        assert!(style.border_width > 0.0);
        assert!(style.text_attenuated_color.a >= 0.0);
        assert!(style.text_attenuated_color.a <= 1.0);
    }
}
