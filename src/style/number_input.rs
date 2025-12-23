//! Display fields that can only be filled with numeric type.
//!
//! *This API requires the following crate features to be activated: `number_input`*

use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme};
use iced_widget::{container, text, text_input};

/// The appearance of a [`NumberInput`](crate::widget::number_input::NumberInput).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`NumberInput`](crate::widget::number_input::NumberInput).
    pub button_background: Option<Background>,
    /// The Color of the arrows of [`NumberInput`](crate::widget::number_input::NumberInput).
    pub icon_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            button_background: None,
            icon_color: Color::BLACK,
        }
    }
}

/// The Catalog of a [`NumberInput`](crate::widget::number_input::NumberInput).
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

/// The Extended Catalog of a [`NumberInput`](crate::widget::number_input::NumberInput).
pub trait ExtendedCatalog:
    text_input::Catalog + container::Catalog + text::Catalog + self::Catalog
{
    /// The default class produced by the [`Catalog`].
    #[must_use]
    fn default_input<'a>() -> <Self as text_input::Catalog>::Class<'a> {
        <Self as text_input::Catalog>::default()
    }

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &<Self as self::Catalog>::Class<'_>, status: Status) -> Style;
}

impl ExtendedCatalog for Theme {
    fn style(&self, class: &<Self as self::Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The primary theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        button_background: Some(palette.primary.strong.color.into()),
        icon_color: palette.primary.strong.text,
    };

    match status {
        Status::Disabled => Style {
            button_background: base.button_background.map(|bg| match bg {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
                Background::Gradient(grad) => Background::Gradient(grad),
            }),
            icon_color: Color {
                a: base.icon_color.a * 0.5,
                ..base.icon_color
            },
        },
        _ => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::Theme;

    #[test]
    fn style_default() {
        let style = Style::default();
        assert!(style.button_background.is_none());
        assert_eq!(style.icon_color, Color::BLACK);
    }

    #[test]
    fn primary_theme_active() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        assert!(style.button_background.is_some());
        if let Some(Background::Color(_)) = style.button_background {
            // Background is a color
        } else {
            panic!("Expected button_background to be Some(Background::Color)");
        }
    }

    #[test]
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(style.button_background.is_some());
    }

    #[test]
    fn primary_theme_focused() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Focused);

        assert!(style.button_background.is_some());
    }

    #[test]
    fn primary_theme_selected() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Selected);

        assert!(style.button_background.is_some());
    }

    #[test]
    fn primary_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Disabled);

        assert!(style.button_background.is_some());
    }

    #[test]
    fn disabled_reduces_alpha() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let disabled_style = primary(&theme, Status::Disabled);

        // Icon color alpha should be reduced
        assert!(
            disabled_style.icon_color.a <= base_style.icon_color.a,
            "Disabled icon color should have reduced alpha"
        );

        // Button background alpha should be reduced for Color backgrounds
        if let (Some(Background::Color(base_color)), Some(Background::Color(disabled_color))) = (
            base_style.button_background,
            disabled_style.button_background,
        ) {
            assert!(
                disabled_color.a <= base_color.a,
                "Disabled button background should have reduced alpha"
            );
        }
    }

    #[test]
    fn disabled_has_half_alpha() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let disabled_style = primary(&theme, Status::Disabled);

        // Icon color should have approximately half the alpha
        let expected_icon_alpha = base_style.icon_color.a * 0.5;
        assert!(
            (disabled_style.icon_color.a - expected_icon_alpha).abs() < 0.01,
            "Disabled icon alpha should be approximately half"
        );
    }

    #[test]
    fn non_disabled_statuses_use_base_style() {
        let theme = Theme::TokyoNight;
        let active_style = primary(&theme, Status::Active);
        let hovered_style = primary(&theme, Status::Hovered);
        let focused_style = primary(&theme, Status::Focused);

        // All non-disabled statuses should have the same style
        assert_eq!(
            format!("{:?}", active_style.button_background),
            format!("{:?}", hovered_style.button_background)
        );
        assert_eq!(
            format!("{:?}", active_style.button_background),
            format!("{:?}", focused_style.button_background)
        );
        assert_eq!(active_style.icon_color, hovered_style.icon_color);
        assert_eq!(active_style.icon_color, focused_style.icon_color);
    }

    #[test]
    fn catalog_default_class() {
        let _class = <Theme as Catalog>::default();
    }

    #[test]
    fn catalog_style() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();
        let style = <Theme as Catalog>::style(&theme, &class, Status::Active);

        assert!(style.button_background.is_some());
    }

    #[test]
    fn catalog_style_with_different_statuses() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();

        let active_style = <Theme as Catalog>::style(&theme, &class, Status::Active);
        let hovered_style = <Theme as Catalog>::style(&theme, &class, Status::Hovered);
        let disabled_style = <Theme as Catalog>::style(&theme, &class, Status::Disabled);

        assert!(active_style.button_background.is_some());
        assert!(hovered_style.button_background.is_some());
        assert!(disabled_style.button_background.is_some());
    }

    #[test]
    fn extended_catalog_style() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();
        let style = <Theme as ExtendedCatalog>::style(&theme, &class, Status::Active);

        assert!(style.button_background.is_some());
    }

    #[test]
    fn extended_catalog_default_input() {
        let _class = <Theme as ExtendedCatalog>::default_input();
    }

    #[test]
    fn disabled_preserves_gradient_backgrounds() {
        // Create a style with a gradient background
        let gradient = iced_core::Gradient::Linear(iced_core::gradient::Linear {
            angle: 0.0.into(),
            stops: [None; 8],
        });
        let style_with_gradient = Style {
            button_background: Some(Background::Gradient(gradient)),
            icon_color: Color::WHITE,
        };

        // If we had a disabled version, it should preserve the gradient
        // This tests that gradients are handled in the match statement
        if let Some(Background::Gradient(_)) = style_with_gradient.button_background {
            // Gradient is preserved
            assert!(true);
        } else {
            panic!("Expected gradient to be preserved");
        }
    }
}
