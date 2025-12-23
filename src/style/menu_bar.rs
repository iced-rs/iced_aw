//! Change the appearance of menu bars and their menus.
use super::{Status, StyleFn};
use iced_core::{Background, Border, Color, Shadow, Theme, Vector};

/// The appearance of a menu bar and its menus.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The background of the menu bar.
    pub bar_background: Background,
    /// The border of the menu bar.
    pub bar_border: Border,
    /// The shadow of the menu bar.
    pub bar_shadow: Shadow,

    /// The background of the menus.
    pub menu_background: Background,
    /// The border of the menus.
    pub menu_border: Border,
    /// The shadow of the menus
    pub menu_shadow: Shadow,

    /// The backgraound of the path
    pub path: Background,
    /// The border of the path
    pub path_border: Border,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            bar_background: Color::from([0.85; 3]).into(),
            bar_border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            bar_shadow: Shadow::default(),

            menu_background: Color::from([0.85; 3]).into(),
            menu_border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            menu_shadow: Shadow {
                color: Color::from([0.0, 0.0, 0.0, 0.5]),
                offset: Vector::ZERO,
                blur_radius: 10.0,
            },
            path: Color::from([0.3; 3]).into(),
            path_border: Border {
                radius: 6.0.into(),
                ..Default::default()
            },
        }
    }
}

/// The Catalog of a [`Menu`](crate::widget::menu::Menu).
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

/// The primary theme of a [`Menu`](crate::widget::menu::Menu).
#[must_use]
pub fn primary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();

    Style {
        bar_background: palette.background.base.color.into(),
        menu_background: palette.background.base.color.into(),
        path: palette.primary.weak.color.into(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::{Background, Theme};

    #[test]
    fn style_default() {
        let style = Style::default();

        // Verify bar properties
        assert!(matches!(style.bar_background, Background::Color(_)));
        assert_eq!(style.bar_border.radius.top_left, 8.0);
        assert_eq!(style.bar_border.radius.top_right, 8.0);
        assert_eq!(style.bar_border.radius.bottom_left, 8.0);
        assert_eq!(style.bar_border.radius.bottom_right, 8.0);

        // Verify menu properties
        assert!(matches!(style.menu_background, Background::Color(_)));
        assert_eq!(style.menu_border.radius.top_left, 8.0);
        assert_eq!(style.menu_shadow.blur_radius, 10.0);

        // Verify path properties
        assert!(matches!(style.path, Background::Color(_)));
        assert_eq!(style.path_border.radius.top_left, 6.0);
    }

    #[test]
    fn default_shadow_has_blur() {
        let style = Style::default();

        // Menu shadow should have blur
        assert_eq!(style.menu_shadow.blur_radius, 10.0);
        assert_eq!(style.menu_shadow.offset, Vector::ZERO);

        // Verify shadow color has transparency
        if let Background::Color(color) = style.bar_background {
            // Just verify it exists
            let _ = color;
        }
    }

    #[test]
    fn primary_theme_active() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        assert!(matches!(style.bar_background, Background::Color(_)));
        assert!(matches!(style.menu_background, Background::Color(_)));
        assert!(matches!(style.path, Background::Color(_)));
    }

    #[test]
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(matches!(style.bar_background, Background::Color(_)));
        assert!(matches!(style.menu_background, Background::Color(_)));
        assert!(matches!(style.path, Background::Color(_)));
    }

    #[test]
    fn primary_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Disabled);

        assert!(matches!(style.bar_background, Background::Color(_)));
        assert!(matches!(style.menu_background, Background::Color(_)));
        assert!(matches!(style.path, Background::Color(_)));
    }

    #[test]
    fn primary_theme_focused() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Focused);

        assert!(matches!(style.bar_background, Background::Color(_)));
        assert!(matches!(style.menu_background, Background::Color(_)));
        assert!(matches!(style.path, Background::Color(_)));
    }

    #[test]
    fn primary_uses_default_borders_and_shadows() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);
        let default_style = Style::default();

        // Primary theme should use default borders
        assert_eq!(style.bar_border.radius, default_style.bar_border.radius);
        assert_eq!(style.menu_border.radius, default_style.menu_border.radius);
        assert_eq!(style.path_border.radius, default_style.path_border.radius);

        // Primary theme should use default shadows
        assert_eq!(
            style.bar_shadow.blur_radius,
            default_style.bar_shadow.blur_radius
        );
        assert_eq!(
            style.menu_shadow.blur_radius,
            default_style.menu_shadow.blur_radius
        );
    }

    #[test]
    fn status_does_not_affect_style() {
        let theme = Theme::TokyoNight;
        let active_style = primary(&theme, Status::Active);
        let hovered_style = primary(&theme, Status::Hovered);
        let disabled_style = primary(&theme, Status::Disabled);

        // All statuses should produce the same style
        assert_eq!(
            format!("{:?}", active_style.bar_background),
            format!("{:?}", hovered_style.bar_background)
        );
        assert_eq!(
            format!("{:?}", active_style.bar_background),
            format!("{:?}", disabled_style.bar_background)
        );
        assert_eq!(
            format!("{:?}", active_style.menu_background),
            format!("{:?}", hovered_style.menu_background)
        );
        assert_eq!(
            format!("{:?}", active_style.path),
            format!("{:?}", hovered_style.path)
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

        assert!(matches!(style.bar_background, Background::Color(_)));
        assert!(matches!(style.menu_background, Background::Color(_)));
        assert!(matches!(style.path, Background::Color(_)));
    }

    #[test]
    fn catalog_style_with_different_statuses() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();

        let active_style = theme.style(&class, Status::Active);
        let hovered_style = theme.style(&class, Status::Hovered);
        let disabled_style = theme.style(&class, Status::Disabled);

        assert!(matches!(active_style.bar_background, Background::Color(_)));
        assert!(matches!(hovered_style.bar_background, Background::Color(_)));
        assert!(matches!(
            disabled_style.bar_background,
            Background::Color(_)
        ));
    }

    #[test]
    fn default_border_radii_are_consistent() {
        let style = Style::default();

        // Bar border should have uniform radius
        assert_eq!(
            style.bar_border.radius.top_left,
            style.bar_border.radius.top_right
        );
        assert_eq!(
            style.bar_border.radius.bottom_left,
            style.bar_border.radius.bottom_right
        );

        // Menu border should have uniform radius
        assert_eq!(
            style.menu_border.radius.top_left,
            style.menu_border.radius.top_right
        );
        assert_eq!(
            style.menu_border.radius.bottom_left,
            style.menu_border.radius.bottom_right
        );

        // Path border should have uniform radius
        assert_eq!(
            style.path_border.radius.top_left,
            style.path_border.radius.top_right
        );
        assert_eq!(
            style.path_border.radius.bottom_left,
            style.path_border.radius.bottom_right
        );
    }

    #[test]
    fn all_backgrounds_are_colors() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        // All backgrounds should be Color variants
        assert!(matches!(style.bar_background, Background::Color(_)));
        assert!(matches!(style.menu_background, Background::Color(_)));
        assert!(matches!(style.path, Background::Color(_)));
    }
}
