//! Displays a [`TabBar`](crate::widget::tab_bar::TabBar) to select the content
//! to be displayed.
//!
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](crate::widget::tabs::Tabs) widget instead.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*

use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme, border::Radius};

/// The appearance of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the tab bar.
    pub background: Option<Background>,

    /// The border color of the tab bar.
    pub border_color: Option<Color>,

    /// The border width of the tab bar.
    pub border_width: f32,

    /// The background of the tab labels.
    pub tab_label_background: Background,

    /// The border color of the tab labels.
    pub tab_label_border_color: Color,

    /// The border with of the tab labels.
    pub tab_label_border_width: f32,

    /// The icon color of the tab labels.
    pub icon_color: Color,

    /// The color of the closing icon border
    pub icon_background: Option<Background>,

    /// How soft/hard the corners of the icon border are
    pub icon_border_radius: Radius,

    /// The text color of the tab labels.
    pub text_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            border_color: None,
            border_width: 0.0,
            tab_label_background: Background::Color([0.87, 0.87, 0.87].into()),
            tab_label_border_color: [0.7, 0.7, 0.7].into(),
            tab_label_border_width: 1.0,
            icon_color: Color::BLACK,
            icon_background: Some(Background::Color(Color::TRANSPARENT)),
            icon_border_radius: 4.0.into(),
            text_color: Color::BLACK,
        }
    }
}
/// The Catalog of a [`TabBar`](crate::widget::tab_bar::TabBar).
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

/// The primary theme of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let mut base = Style::default();
    let palette = theme.extended_palette();

    base.text_color = palette.background.base.text;

    match status {
        Status::Disabled => {
            base.tab_label_background = Background::Color(palette.background.strong.color);
        }
        Status::Hovered => {
            base.tab_label_background = Background::Color(palette.primary.strong.color);
        }
        _ => {
            base.tab_label_background = Background::Color(palette.primary.base.color);
        }
    }

    base
}

/// The dark theme of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[must_use]
pub fn dark(_theme: &Theme, status: Status) -> Style {
    let mut base = Style {
        tab_label_background: Background::Color([0.1, 0.1, 0.1].into()),
        tab_label_border_color: [0.3, 0.3, 0.3].into(),
        icon_color: Color::WHITE,
        text_color: Color::WHITE,
        ..Default::default()
    };

    if status == Status::Disabled {
        base.tab_label_background = Background::Color([0.13, 0.13, 0.13].into());
    }

    base
}

/// The red theme of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[must_use]
pub fn red(_theme: &Theme, status: Status) -> Style {
    let mut base = Style {
        tab_label_background: Background::Color([1.0, 0.0, 0.0].into()),
        tab_label_border_color: Color::TRANSPARENT,
        tab_label_border_width: 0.0,
        icon_color: Color::WHITE,
        text_color: Color::WHITE,
        ..Default::default()
    };

    if status == Status::Disabled {
        base.tab_label_background = Background::Color([0.13, 0.13, 0.13].into());
        base.icon_color = Color::BLACK;
        base.text_color = Color::BLACK;
    }

    base
}

/// The blue theme of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[must_use]
pub fn blue(_theme: &Theme, status: Status) -> Style {
    let mut base = Style {
        tab_label_background: Background::Color([0.0, 0.0, 1.0].into()),
        tab_label_border_color: [0.0, 0.0, 1.0].into(),
        icon_color: Color::WHITE,
        text_color: Color::WHITE,
        ..Default::default()
    };

    if status == Status::Disabled {
        base.tab_label_background = Background::Color([0.5, 0.5, 1.0].into());
        base.tab_label_border_color = [0.5, 0.5, 1.0].into();
    }

    base
}

/// The blue theme of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[must_use]
pub fn green(_theme: &Theme, status: Status) -> Style {
    let mut base = Style {
        tab_label_background: Color::WHITE.into(),
        icon_color: [0.0, 0.5, 0.0].into(),
        text_color: [0.0, 0.5, 0.0].into(),
        ..Default::default()
    };

    match status {
        Status::Disabled => {
            base.icon_color = [0.7, 0.7, 0.7].into();
            base.text_color = [0.7, 0.7, 0.7].into();
            base.tab_label_border_color = [0.7, 0.7, 0.7].into();
        }
        _ => {
            base.tab_label_border_color = [0.0, 0.5, 0.0].into();
        }
    }

    base
}

/// The purple theme of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[must_use]
pub fn purple(_theme: &Theme, status: Status) -> Style {
    let mut base = Style {
        tab_label_background: Color::WHITE.into(),
        tab_label_border_color: Color::TRANSPARENT,
        icon_color: [0.7, 0.0, 1.0].into(),
        text_color: [0.7, 0.0, 1.0].into(),
        ..Default::default()
    };

    if status == Status::Disabled {
        base.icon_color = Color::BLACK;
        base.text_color = Color::BLACK;
    }

    base
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::{Background, Theme};

    #[test]
    fn style_default() {
        let style = Style::default();

        assert!(style.background.is_none());
        assert!(style.border_color.is_none());
        assert_eq!(style.border_width, 0.0);
        assert!(matches!(style.tab_label_background, Background::Color(_)));
        assert_eq!(style.tab_label_border_width, 1.0);
        assert_eq!(style.icon_color, Color::BLACK);
        assert!(style.icon_background.is_some());
        assert_eq!(style.text_color, Color::BLACK);
    }

    #[test]
    fn primary_theme_active() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        assert!(matches!(style.tab_label_background, Background::Color(_)));
    }

    #[test]
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(matches!(style.tab_label_background, Background::Color(_)));
    }

    #[test]
    fn primary_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Disabled);

        assert!(matches!(style.tab_label_background, Background::Color(_)));
    }

    #[test]
    fn primary_theme_focused() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Focused);

        assert!(matches!(style.tab_label_background, Background::Color(_)));
    }

    #[test]
    fn primary_changes_background_by_status() {
        let theme = Theme::TokyoNight;
        let active = primary(&theme, Status::Active);
        let hovered = primary(&theme, Status::Hovered);
        let disabled = primary(&theme, Status::Disabled);

        // Different statuses should have different backgrounds
        assert_ne!(
            format!("{:?}", active.tab_label_background),
            format!("{:?}", hovered.tab_label_background)
        );
        assert_ne!(
            format!("{:?}", active.tab_label_background),
            format!("{:?}", disabled.tab_label_background)
        );
    }

    #[test]
    fn dark_theme_active() {
        let theme = Theme::TokyoNight;
        let style = dark(&theme, Status::Active);

        assert_eq!(style.icon_color, Color::WHITE);
        assert_eq!(style.text_color, Color::WHITE);
    }

    #[test]
    fn dark_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = dark(&theme, Status::Disabled);

        assert_eq!(style.icon_color, Color::WHITE);
        assert_eq!(style.text_color, Color::WHITE);
    }

    #[test]
    fn dark_theme_changes_background_when_disabled() {
        let theme = Theme::TokyoNight;
        let active = dark(&theme, Status::Active);
        let disabled = dark(&theme, Status::Disabled);

        assert_ne!(
            format!("{:?}", active.tab_label_background),
            format!("{:?}", disabled.tab_label_background)
        );
    }

    #[test]
    fn red_theme_active() {
        let theme = Theme::TokyoNight;
        let style = red(&theme, Status::Active);

        assert_eq!(
            style.tab_label_background,
            Background::Color([1.0, 0.0, 0.0].into())
        );
        assert_eq!(style.icon_color, Color::WHITE);
        assert_eq!(style.text_color, Color::WHITE);
        assert_eq!(style.tab_label_border_width, 0.0);
    }

    #[test]
    fn red_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = red(&theme, Status::Disabled);

        assert_eq!(style.icon_color, Color::BLACK);
        assert_eq!(style.text_color, Color::BLACK);
    }

    #[test]
    fn blue_theme_active() {
        let theme = Theme::TokyoNight;
        let style = blue(&theme, Status::Active);

        assert_eq!(
            style.tab_label_background,
            Background::Color([0.0, 0.0, 1.0].into())
        );
        assert_eq!(style.icon_color, Color::WHITE);
        assert_eq!(style.text_color, Color::WHITE);
    }

    #[test]
    fn blue_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = blue(&theme, Status::Disabled);

        assert_eq!(
            style.tab_label_background,
            Background::Color([0.5, 0.5, 1.0].into())
        );
        assert_eq!(style.tab_label_border_color, [0.5, 0.5, 1.0].into());
    }

    #[test]
    fn green_theme_active() {
        let theme = Theme::TokyoNight;
        let style = green(&theme, Status::Active);

        assert_eq!(style.tab_label_background, Background::Color(Color::WHITE));
        assert_eq!(style.icon_color, [0.0, 0.5, 0.0].into());
        assert_eq!(style.text_color, [0.0, 0.5, 0.0].into());
        assert_eq!(style.tab_label_border_color, [0.0, 0.5, 0.0].into());
    }

    #[test]
    fn green_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = green(&theme, Status::Disabled);

        assert_eq!(style.icon_color, [0.7, 0.7, 0.7].into());
        assert_eq!(style.text_color, [0.7, 0.7, 0.7].into());
        assert_eq!(style.tab_label_border_color, [0.7, 0.7, 0.7].into());
    }

    #[test]
    fn purple_theme_active() {
        let theme = Theme::TokyoNight;
        let style = purple(&theme, Status::Active);

        assert_eq!(style.tab_label_background, Background::Color(Color::WHITE));
        assert_eq!(style.icon_color, [0.7, 0.0, 1.0].into());
        assert_eq!(style.text_color, [0.7, 0.0, 1.0].into());
        assert_eq!(style.tab_label_border_color, Color::TRANSPARENT);
    }

    #[test]
    fn purple_theme_disabled() {
        let theme = Theme::TokyoNight;
        let style = purple(&theme, Status::Disabled);

        assert_eq!(style.icon_color, Color::BLACK);
        assert_eq!(style.text_color, Color::BLACK);
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

        assert!(matches!(style.tab_label_background, Background::Color(_)));
    }

    #[test]
    fn catalog_style_with_different_statuses() {
        let theme = Theme::TokyoNight;
        let class = <Theme as Catalog>::default();

        let active = theme.style(&class, Status::Active);
        let hovered = theme.style(&class, Status::Hovered);
        let disabled = theme.style(&class, Status::Disabled);

        assert!(matches!(active.tab_label_background, Background::Color(_)));
        assert!(matches!(hovered.tab_label_background, Background::Color(_)));
        assert!(matches!(
            disabled.tab_label_background,
            Background::Color(_)
        ));
    }

    #[test]
    fn icon_border_radius_is_set() {
        let style = Style::default();
        // Icon border radius should be set to 4.0 (all corners)
        assert_eq!(style.icon_border_radius.top_left, 4.0);
        assert_eq!(style.icon_border_radius.top_right, 4.0);
        assert_eq!(style.icon_border_radius.bottom_left, 4.0);
        assert_eq!(style.icon_border_radius.bottom_right, 4.0);
    }

    #[test]
    fn different_themes_produce_different_styles() {
        let theme = Theme::TokyoNight;
        let status = Status::Active;

        let _primary_style = primary(&theme, status);
        let dark_style = dark(&theme, status);
        let red_style = red(&theme, status);
        let blue_style = blue(&theme, status);
        let green_style = green(&theme, status);
        let purple_style = purple(&theme, status);

        // All themes should produce different background styles
        assert_ne!(
            format!("{:?}", dark_style.tab_label_background),
            format!("{:?}", red_style.tab_label_background)
        );
        assert_ne!(
            format!("{:?}", red_style.tab_label_background),
            format!("{:?}", blue_style.tab_label_background)
        );
        assert_ne!(
            format!("{:?}", blue_style.tab_label_background),
            format!("{:?}", green_style.tab_label_background)
        );

        // Text colors also differ between some themes
        assert_ne!(green_style.text_color, purple_style.text_color);
    }
}
