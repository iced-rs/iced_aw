//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use super::{Status, StyleFn, colors};
use iced_core::{Background, Color, Theme, theme::palette};

/// The style of a [`Badge`](crate::widget::badge::Badge).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`Badge`](crate::widget::badge::Badge).
    pub background: Background,

    /// The border radius of the [`Badge`](crate::widget::badge::Badge).
    /// If no radius is specified the default one will be used.
    pub border_radius: Option<f32>,

    /// The border with of the [`Badge`](crate::widget::badge::Badge).
    pub border_width: f32,

    /// The border color of the [`Badge`](crate::widget::badge::Badge).
    pub border_color: Option<Color>,

    /// The default text color of the [`Badge`](crate::widget::badge::Badge).
    pub text_color: Color,
}

/// The Catalog of a [`Badge`](crate::widget::badge::Badge).
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

/// The primary theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.primary.strong);

    match status {
        Status::Hovered => Style {
            background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
        _ => base,
    }
}

/// The secondary theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.secondary.strong);

    match status {
        Status::Hovered => Style {
            background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
        _ => base,
    }
}

/// The success theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn success(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.success.strong);

    match status {
        Status::Hovered => Style {
            background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
        _ => base,
    }
}

/// The danger theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.danger.strong);

    match status {
        Status::Hovered => Style {
            background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
        _ => base,
    }
}

/// The warning theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn warning(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::WARNING, colors::BLACK);

    match status {
        Status::Disabled => disabled(base),
        _ => base,
    }
}

/// The info theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn info(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::INFO, colors::BLACK);

    match status {
        Status::Disabled => disabled(base),
        _ => base,
    }
}

/// The light theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn light(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::LIGHT, colors::BLACK);

    match status {
        Status::Disabled => disabled(base),
        _ => base,
    }
}

/// The dark theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn dark(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::DARK, colors::WHITE);

    match status {
        Status::Disabled => disabled(base),
        _ => base,
    }
}

/// The white theme of a [`Badge`](crate::widget::badge::Badge).
#[must_use]
pub fn white(_theme: &Theme, status: Status) -> Style {
    let base = from_color(colors::WHITE, colors::BLACK);

    match status {
        Status::Disabled => disabled(base),
        _ => base,
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

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::{Background, Color, Theme};

    #[test]
    fn style_default() {
        let style = Style::default();
        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_radius.is_none());
        assert_eq!(style.border_width, 1.0);
        assert!(style.border_color.is_some());
        assert_eq!(style.text_color, Color::BLACK);
    }

    #[test]
    fn from_color_creates_style() {
        let color = Color::from_rgb(1.0, 0.0, 0.0);
        let text = Color::WHITE;
        let style = from_color(color, text);

        assert_eq!(style.background, Background::Color(color));
        assert_eq!(style.border_color, Some(color));
        assert_eq!(style.text_color, text);
        assert_eq!(style.border_width, 1.0);
        assert!(style.border_radius.is_none());
    }

    #[test]
    fn disabled_scales_alpha() {
        let base = Style {
            background: Background::Color(Color::from_rgb(1.0, 0.0, 0.0)),
            border_radius: Some(5.0),
            border_width: 2.0,
            border_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            text_color: Color::WHITE,
        };

        let disabled_style = disabled(base);

        // Background and text color should have scaled alpha
        assert!(matches!(disabled_style.background, Background::Color(_)));
        assert_eq!(disabled_style.border_radius, Some(5.0));
        assert_eq!(disabled_style.border_width, 2.0);
        assert_eq!(
            disabled_style.border_color,
            Some(Color::from_rgb(0.5, 0.5, 0.5))
        );
    }

    #[test]
    fn primary_theme_active() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn primary_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn secondary_theme_active() {
        let theme = Theme::TokyoNight;
        let style = secondary(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn secondary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = secondary(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn secondary_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = secondary(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn success_theme_active() {
        let theme = Theme::TokyoNight;
        let style = success(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn success_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = success(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn success_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = success(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn danger_theme_active() {
        let theme = Theme::TokyoNight;
        let style = danger(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn danger_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = danger(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn danger_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = danger(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn warning_theme_active() {
        let theme = Theme::TokyoNight;
        let style = warning(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
        assert_eq!(style.text_color, colors::BLACK);
    }

    #[test]
    fn warning_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = warning(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn info_theme_active() {
        let theme = Theme::TokyoNight;
        let style = info(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
        assert_eq!(style.text_color, colors::BLACK);
    }

    #[test]
    fn info_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = info(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn light_theme_active() {
        let theme = Theme::TokyoNight;
        let style = light(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
        assert_eq!(style.text_color, colors::BLACK);
    }

    #[test]
    fn light_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = light(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn dark_theme_active() {
        let theme = Theme::TokyoNight;
        let style = dark(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
        assert_eq!(style.text_color, colors::WHITE);
    }

    #[test]
    fn dark_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = dark(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
    }

    #[test]
    fn white_theme_active() {
        let theme = Theme::TokyoNight;
        let style = white(&theme, Status::Active);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
        assert_eq!(style.text_color, colors::BLACK);
    }

    #[test]
    fn white_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = white(&theme, Status::Disabled);

        assert!(matches!(style.background, Background::Color(_)));
        assert!(style.border_color.is_some());
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
        assert!(style.border_color.is_some());
    }
}
