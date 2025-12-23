//! Selection List
//!
//! *This API requires the following crate features to be activated: `selection_list`*

use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme};

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

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::{Background, Theme};

    #[test]
    fn primary_theme_active() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn primary_theme_selected() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Selected);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn primary_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn primary_theme_focused() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Focused);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn hovered_changes_text_and_background() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let hovered_style = primary(&theme, Status::Hovered);

        // Text color and background should be different when hovered
        assert_ne!(base_style.text_color, hovered_style.text_color);
        assert_ne!(
            format!("{:?}", base_style.background),
            format!("{:?}", hovered_style.background)
        );

        // Border properties should remain the same
        assert_eq!(base_style.border_width, hovered_style.border_width);
        assert_eq!(base_style.border_color, hovered_style.border_color);
    }

    #[test]
    fn selected_changes_text_and_background() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let selected_style = primary(&theme, Status::Selected);

        // Text color and background should be different when selected
        assert_ne!(base_style.text_color, selected_style.text_color);
        assert_ne!(
            format!("{:?}", base_style.background),
            format!("{:?}", selected_style.background)
        );

        // Border properties should remain the same
        assert_eq!(base_style.border_width, selected_style.border_width);
        assert_eq!(base_style.border_color, selected_style.border_color);
    }

    #[test]
    fn selected_and_hovered_have_different_styles() {
        let theme = Theme::TokyoNight;
        let hovered_style = primary(&theme, Status::Hovered);
        let selected_style = primary(&theme, Status::Selected);

        // Selected and hovered should use different colors
        assert_ne!(hovered_style.text_color, selected_style.text_color);
        assert_ne!(
            format!("{:?}", hovered_style.background),
            format!("{:?}", selected_style.background)
        );
    }

    #[test]
    fn non_interactive_statuses_use_base_style() {
        let theme = Theme::TokyoNight;
        let active_style = primary(&theme, Status::Active);
        let disabled_style = primary(&theme, Status::Disabled);
        let focused_style = primary(&theme, Status::Focused);

        // Non-interactive statuses should have the same style
        assert_eq!(active_style.text_color, disabled_style.text_color);
        assert_eq!(active_style.text_color, focused_style.text_color);
        assert_eq!(
            format!("{:?}", active_style.background),
            format!("{:?}", disabled_style.background)
        );
        assert_eq!(
            format!("{:?}", active_style.background),
            format!("{:?}", focused_style.background)
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
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn catalog_style_with_different_statuses() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();

        let active_style = theme.style(&class, Status::Active);
        let hovered_style = theme.style(&class, Status::Hovered);
        let selected_style = theme.style(&class, Status::Selected);

        assert!(matches!(active_style.background, Background::Color(_)));
        assert!(matches!(hovered_style.background, Background::Color(_)));
        assert!(matches!(selected_style.background, Background::Color(_)));
    }

    #[test]
    fn border_width_is_consistent() {
        let theme = Theme::TokyoNight;

        // All statuses should have the same border width
        let active = primary(&theme, Status::Active);
        let hovered = primary(&theme, Status::Hovered);
        let selected = primary(&theme, Status::Selected);

        assert_eq!(active.border_width, 1.0);
        assert_eq!(hovered.border_width, 1.0);
        assert_eq!(selected.border_width, 1.0);
    }

    #[test]
    fn border_color_is_consistent() {
        let theme = Theme::TokyoNight;

        // All statuses should have the same border color
        let active = primary(&theme, Status::Active);
        let hovered = primary(&theme, Status::Hovered);
        let selected = primary(&theme, Status::Selected);

        assert_eq!(active.border_color, hovered.border_color);
        assert_eq!(active.border_color, selected.border_color);
    }

    #[test]
    fn style_fields_are_valid() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        // Verify all fields have valid values
        assert!(style.border_width > 0.0);
    }
}
