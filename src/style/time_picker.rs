//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
#![allow(clippy::doc_markdown)]
use super::{Status, StyleFn};
use iced_core::{Background, Color, Theme};

/// The style of a [`TimePicker`](crate::widget::TimePicker).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`TimePicker`](crate::widget::TimePicker).
    pub background: Background,

    /// The border radius of the [`TimePicker`](crate::widget::TimePicker).
    pub border_radius: f32,

    /// The border width of the [`TimePicker`](crate::widget::TimePicker).
    pub border_width: f32,

    /// The border color of the [`TimePicker`](crate::widget::TimePicker).
    pub border_color: Color,

    /// The text color of the [`TimePicker`](crate::widget::TimePicker).
    pub text_color: Color,

    /// The color of the clock numbers of the
    /// [`TimePicker`](crate::widget::TimePicker).
    pub clock_number_color: Color,

    /// The background of the clock numbers of the
    /// [`TimePicker`](crate::widget::TimePicker).
    pub clock_number_background: Color,

    /// The color of the dots on the clock of the
    /// [`TimePicker`](crate::widget::TimePicker).
    pub clock_dots_color: Color,

    /// The color of the hands of the clock of the
    /// [`TimePicker`](crate::widget::TimePicker).
    pub clock_hand_color: Color,

    /// The with of the hands of the clock of the
    /// [`TimePicker](crate::widget::TimePicker).
    pub clock_hand_width: f32,
}

/// The Catalog of a [`TimePicker`](crate::widget::TimePicker).
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

/// The primary theme of a [`TimePicker`](crate::widget::TimePicker).
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
        clock_number_color: foreground.text,
        clock_number_background: palette.background.base.color,
        clock_dots_color: [0.87, 0.87, 0.87].into(),
        clock_hand_color: [0.87, 0.87, 0.87].into(),
        clock_hand_width: 3.0,
    };

    match status {
        Status::Focused => Style {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..base
        },
        Status::Hovered => Style {
            clock_number_color: palette.primary.weak.text,
            clock_number_background: palette.primary.weak.color,
            ..base
        },
        Status::Selected => Style {
            clock_number_color: palette.primary.strong.text,
            clock_number_background: palette.primary.strong.color,
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
        assert_eq!(style.clock_hand_width, 3.0);
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
    fn primary_theme_hovered() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Hovered);

        assert!(matches!(style.background, Background::Color(_)));
        assert_eq!(style.border_radius, 15.0);
        assert_eq!(style.border_width, 1.0);
    }

    #[test]
    fn primary_theme_selected() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Selected);

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
    fn focused_changes_border_color() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let focused_style = primary(&theme, Status::Focused);

        // Border color should change when focused
        assert_ne!(base_style.border_color, focused_style.border_color);
        assert_eq!(focused_style.border_color, Color::from_rgb(0.5, 0.5, 0.5));

        // Other properties should remain the same
        assert_eq!(base_style.background, focused_style.background);
        assert_eq!(base_style.border_radius, focused_style.border_radius);
        assert_eq!(base_style.border_width, focused_style.border_width);
        assert_eq!(base_style.text_color, focused_style.text_color);
    }

    #[test]
    fn hovered_changes_clock_number_styling() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let hovered_style = primary(&theme, Status::Hovered);

        // Clock number color and background should change when hovered
        assert_ne!(
            base_style.clock_number_color,
            hovered_style.clock_number_color
        );
        assert_ne!(
            base_style.clock_number_background,
            hovered_style.clock_number_background
        );

        // Other properties should remain the same
        assert_eq!(base_style.background, hovered_style.background);
        assert_eq!(base_style.border_color, hovered_style.border_color);
        assert_eq!(base_style.text_color, hovered_style.text_color);
    }

    #[test]
    fn selected_changes_clock_number_styling() {
        let theme = Theme::TokyoNight;
        let base_style = primary(&theme, Status::Active);
        let selected_style = primary(&theme, Status::Selected);

        // Clock number color and background should change when selected
        assert_ne!(
            base_style.clock_number_color,
            selected_style.clock_number_color
        );
        assert_ne!(
            base_style.clock_number_background,
            selected_style.clock_number_background
        );

        // Other properties should remain the same
        assert_eq!(base_style.background, selected_style.background);
        assert_eq!(base_style.border_color, selected_style.border_color);
        assert_eq!(base_style.text_color, selected_style.text_color);
    }

    #[test]
    fn selected_and_hovered_have_different_styles() {
        let theme = Theme::TokyoNight;
        let hovered_style = primary(&theme, Status::Hovered);
        let selected_style = primary(&theme, Status::Selected);

        // Hovered and selected should use different colors
        assert_ne!(
            hovered_style.clock_number_color,
            selected_style.clock_number_color
        );
        assert_ne!(
            hovered_style.clock_number_background,
            selected_style.clock_number_background
        );
    }

    #[test]
    fn clock_components_have_valid_styling() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        // Verify clock components have valid colors
        assert_eq!(style.clock_dots_color, [0.87, 0.87, 0.87].into());
        assert_eq!(style.clock_hand_color, [0.87, 0.87, 0.87].into());
        assert_eq!(style.clock_hand_width, 3.0);
    }

    #[test]
    fn non_interactive_statuses_use_base_style() {
        let theme = Theme::TokyoNight;
        let active_style = primary(&theme, Status::Active);
        let disabled_style = primary(&theme, Status::Disabled);

        // Non-interactive statuses should have the same style
        assert_eq!(active_style.background, disabled_style.background);
        assert_eq!(active_style.border_color, disabled_style.border_color);
        assert_eq!(active_style.text_color, disabled_style.text_color);
        assert_eq!(
            active_style.clock_number_color,
            disabled_style.clock_number_color
        );
        assert_eq!(
            active_style.clock_number_background,
            disabled_style.clock_number_background
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
        let focused_style = theme.style(&class, Status::Focused);
        let hovered_style = theme.style(&class, Status::Hovered);
        let selected_style = theme.style(&class, Status::Selected);

        assert!(matches!(active_style.background, Background::Color(_)));
        assert!(matches!(focused_style.background, Background::Color(_)));
        assert!(matches!(hovered_style.background, Background::Color(_)));
        assert!(matches!(selected_style.background, Background::Color(_)));
    }

    #[test]
    fn style_fields_are_valid() {
        let theme = Theme::TokyoNight;
        let style = primary(&theme, Status::Active);

        // Verify all fields have valid values
        assert!(style.border_radius > 0.0);
        assert!(style.border_width > 0.0);
        assert!(style.clock_hand_width > 0.0);
    }
}
