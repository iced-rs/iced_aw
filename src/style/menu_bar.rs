//! Change the appearance of menu bars and their menus.
use iced_native::Color;
use iced_style::Theme;

/// The appearance of a menu bar and its menus.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The background color of the menu bar and its menus.
    pub background: Color,
    /// The border width of the menu bar and its menus.
    pub border_width: f32,
    /// The border radius of the menu bar and its menus.
    pub border_radius: [f32;4],
    /// The border [`Color`] of the menu bar and its menus.
    pub border_color: Color,
    /// The expand value of the menus' background
    pub background_expand: [u16;4],
    /// The highlighted path [`Color`] of the the menu bar and its menus.
    pub path: Color,
}
impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            background: Color::from([0.85;3]),
            border_width: 1.0,
            border_radius: [4.0;4],
            border_color: Color::from([0.5;3]),
            background_expand: [4;4],
            path: Color::from([0.0, 0.0, 0.0, 0.3]),
        }
    }
}

/// The style sheet of a menu bar and its menus.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the [`Appearance`] of a menu bar and its menus.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}


#[derive(Default)]
/// The style of a menu bar and its menus
#[allow(missing_debug_implementations)]
pub enum MenuBarStyle{
    /// The default style.
    #[default]
    Default,
    /// A [`Theme`] that uses a [`Custom`] palette.
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}
impl StyleSheet for Theme{
    type Style = MenuBarStyle;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        Appearance{
            background: palette.background.base.color,
            border_width: 1.0,
            border_radius: [6.0;4],
            border_color: palette.background.weak.color,
            background_expand: [6;4],
            path: palette.primary.weak.color,
        }
    }
}