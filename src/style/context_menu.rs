//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme};

/// The style of a [`ContextMenu`](crate::widget::ContextMenu).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`ContextMenu`](crate::widget::ContextMenu).
    ///
    /// This is used to color the backdrop of the modal.
    pub background: Background,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Background::Color([0.87, 0.87, 0.87, 0.30].into()),
        }
    }
}

/// The Catalog of a [`ContextMenu`](crate::widget::ContextMenu).
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

/// The primary theme of a [`ContextMenu`](crate::widget::ContextMenu).
#[must_use]
pub fn primary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Background::Color(Color {
            a: 0f32,
            ..palette.background.base.color
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::{Background, Theme};

    #[test]
    fn style_default() {
        let style = Style::default();
        assert!(matches!(style.background, Background::Color(_)));

        // Verify the default has a semi-transparent background
        if let Background::Color(color) = style.background {
            assert_eq!(color.a, 0.30);
        }
    }

    #[test]
    fn primary_theme_active() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
    }

    #[test]
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
    }

    #[test]
    fn primary_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
    }

    #[test]
    fn primary_theme_focused() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Focused);

        assert!(matches!(style.background, Background::Color(_)));
    }

    #[test]
    fn primary_theme_has_transparent_background() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        // The primary theme should create a transparent background
        #[allow(clippy::panic)]
        if let Background::Color(color) = style.background {
            assert_eq!(
                color.a, 0.0,
                "Primary theme should have fully transparent background"
            );
        } else {
            panic!("Expected Background::Color");
        }
    }

    #[test]
    fn status_does_not_affect_style() {
        let theme = Theme::TokyoNight;
        let active_style = primary(&theme, Status::Active);
        let hovered_style = primary(&theme, Status::Hovered);
        let disabled_style = primary(&theme, Status::Disabled);
        let focused_style = primary(&theme, Status::Focused);

        // All statuses should produce the same style
        assert_eq!(
            format!("{:?}", active_style.background),
            format!("{:?}", hovered_style.background)
        );
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
    }

    #[test]
    fn catalog_style_with_different_statuses() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();

        let active_style = theme.style(&class, Status::Active);
        let hovered_style = theme.style(&class, Status::Hovered);
        let disabled_style = theme.style(&class, Status::Disabled);

        assert!(matches!(active_style.background, Background::Color(_)));
        assert!(matches!(hovered_style.background, Background::Color(_)));
        assert!(matches!(disabled_style.background, Background::Color(_)));
    }
}
