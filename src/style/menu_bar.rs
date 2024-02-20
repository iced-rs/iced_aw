//! Change the appearance of menu bars and their menus.
use iced_widget::{
    core::{
        Color, Border, Background, Shadow, Padding, Vector
    }, 
    style::Theme
};

/// The appearance of a menu bar and its menus.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The background of the menus.
    pub background: Background,
    /// The border of the menus.
    pub border: Border,
    /// The shadow of the menus
    pub shadow: Shadow,
    /// Expand the background
    pub background_expand: Padding,
}
impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            background: Color::from([0.85; 3]).into(),
            border: Border{
                radius: [6.0; 4].into(),
                ..Default::default()
            },
            shadow: Shadow{
                color: Color::from([0.0, 0.0, 0.0, 0.5]),
                offset: Vector::ZERO,
                blur_radius: 10.0,
            },
            background_expand: [6; 4].into(),
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

/// The style of a menu bar and its menus
#[derive(Default)]
#[allow(missing_debug_implementations)]
pub enum MenuBarStyle {
    /// The default style.
    #[default]
    Default,
    /// A [`Theme`] that uses a `Custom` palette.
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl From<fn(&Theme) -> Appearance> for MenuBarStyle {
    fn from(f: fn(&Theme) -> Appearance) -> Self {
        Self::Custom(Box::new(f))
    }
}

impl StyleSheet for fn(&Theme) -> Appearance {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        (self)(style)
    }
}

impl StyleSheet for Theme {
    type Style = MenuBarStyle;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        match style {
            MenuBarStyle::Default => Appearance {
                background: palette.background.base.color.into(),
                // border: Border{
                //     color: palette.background.weak.color.into(),
                //     width: 1.0,
                //     radius: [6.0; 4].into(),
                // },
                ..Default::default()
            },
            MenuBarStyle::Custom(c) => c.appearance(self),
        }
    }
}
