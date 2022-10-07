//! Displays a [`TabBar`](crate::native::tab_bar::TabBar) to select the content
//! to be displayed.
//!
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](crate::native::tabs::Tabs) widget instead.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
use iced_style::Theme;

/// The appearance of a [`TabBar`](crate::native::tab_bar::TabBar).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
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

    /// The text color of the tab labels.
    pub text_color: Color,
}

/// The appearance of a [`TabBar`](crate::native::tab_bar::TabBar).
pub trait StyleSheet {
    ///Style for the trait to use.
    type Style: Default + Copy;

    /// The normal appearance0of a tab bar and its tab labels.
    ///
    /// `is_active` is true if the tab is selected.
    fn active(&self, style: Self::Style, is_active: bool) -> Appearance;

    /// The appearance when the tab bar and/or a tab label is hovered.
    ///
    /// `is_active` is true if the tab is selected.
    fn hovered(&self, style: Self::Style, is_active: bool) -> Appearance;
}

#[derive(Clone, Copy, Debug, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``TabBar`` Styles
pub enum TabBarStyles {
    #[default]
    Default,
}

impl StyleSheet for Theme {
    type Style = TabBarStyles;

    fn active(&self, _style: Self::Style, is_active: bool) -> Appearance {
        Appearance {
            background: None,
            border_color: None,
            border_width: 0.0,
            tab_label_background: if is_active {
                Background::Color([0.9, 0.9, 0.9].into())
            } else {
                Background::Color([0.87, 0.87, 0.87].into())
            },
            tab_label_border_color: [0.7, 0.7, 0.7].into(),
            tab_label_border_width: 1.0,
            icon_color: Color::BLACK,
            text_color: Color::BLACK,
        }
    }

    fn hovered(&self, style: Self::Style, is_active: bool) -> Appearance {
        Appearance {
            tab_label_background: Background::Color([0.9, 0.9, 0.9].into()),
            ..self.active(style, is_active)
        }
    }
}
