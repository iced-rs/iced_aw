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

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background: None,
            border_color: None,
            border_width: 0.0,
            tab_label_background: Background::Color([0.87, 0.87, 0.87].into()),
            tab_label_border_color: [0.7, 0.7, 0.7].into(),
            tab_label_border_width: 1.0,
            icon_color: Color::BLACK,
            text_color: Color::BLACK,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``TabBar`` Styles
pub enum TabBarStyles {
    #[default]
    Default,
    Red,
    Blue,
    Green,
    Purple,
}

impl From<TabBarStyles> for String {
    fn from(style: TabBarStyles) -> Self {
        Self::from(match style {
            TabBarStyles::Default => "Default",
            TabBarStyles::Red => "Red",
            TabBarStyles::Blue => "Blue",
            TabBarStyles::Green => "Green",
            TabBarStyles::Purple => "Purple",
        })
    }
}

impl StyleSheet for Theme {
    type Style = TabBarStyles;

    fn active(&self, style: Self::Style, is_active: bool) -> Appearance {
        let mut appearance = Appearance::default();

        match style {
            TabBarStyles::Default => {
                appearance.tab_label_background = if is_active {
                    Background::Color([0.9, 0.9, 0.9].into())
                } else {
                    Background::Color([0.87, 0.87, 0.87].into())
                };
            }
            TabBarStyles::Red => {
                let text_color = if is_active {
                    Color::WHITE
                } else {
                    Color::BLACK
                };

                appearance.tab_label_background = if is_active {
                    Background::Color([1.0, 0.0, 0.0].into())
                } else {
                    Background::Color(Color::WHITE)
                };
                appearance.tab_label_border_width = 0.0;
                appearance.tab_label_border_color = Color::TRANSPARENT;
                appearance.icon_color = text_color;
                appearance.text_color = text_color;
            }
            TabBarStyles::Blue => {
                appearance.tab_label_background = if is_active {
                    Background::Color([0.0, 0.0, 1.0].into())
                } else {
                    Background::Color([0.5, 0.5, 1.0].into())
                };
                appearance.tab_label_border_color = if is_active {
                    [0.0, 0.0, 1.0].into()
                } else {
                    [0.5, 0.5, 1.0].into()
                };
                appearance.icon_color = Color::WHITE;
                appearance.text_color = Color::WHITE;
            }
            TabBarStyles::Green => {
                let color = if is_active {
                    [0.0, 0.5, 0.0]
                } else {
                    [0.7, 0.7, 0.7]
                }
                .into();

                appearance.tab_label_border_color = color;
                appearance.tab_label_background = Color::WHITE.into();
                appearance.icon_color = color;
                appearance.text_color = color;
            }
            TabBarStyles::Purple => {
                let text_color = if is_active {
                    [0.7, 0.0, 1.0].into()
                } else {
                    Color::BLACK
                };

                appearance.tab_label_background = Color::WHITE.into();
                appearance.tab_label_border_color = Color::TRANSPARENT;
                appearance.tab_label_border_width = 0.0;
                appearance.icon_color = text_color;
                appearance.text_color = text_color;
            }
        }

        appearance
    }

    fn hovered(&self, style: Self::Style, is_active: bool) -> Appearance {
        match style {
            TabBarStyles::Default => Appearance {
                tab_label_background: Background::Color([0.9, 0.9, 0.9].into()),
                ..self.active(style, is_active)
            },
            TabBarStyles::Red => Appearance {
                tab_label_background: Background::Color([1.0, 0.0, 0.0].into()),
                icon_color: Color::WHITE,
                text_color: Color::WHITE,
                ..self.active(style, is_active)
            },
            TabBarStyles::Blue => Appearance {
                tab_label_background: Background::Color([0.0, 0.0, 1.0].into()),
                tab_label_border_color: [0.0, 0.0, 1.0].into(),
                ..self.active(style, is_active)
            },
            TabBarStyles::Green => Appearance {
                tab_label_border_color: [0.0, 0.4, 0.0].into(),
                icon_color: [0.0, 0.4, 0.0].into(),
                text_color: [0.0, 0.4, 0.0].into(),
                ..self.active(style, is_active)
            },
            TabBarStyles::Purple => {
                let text_color = [0.7, 0.0, 1.0].into();
                Appearance {
                    icon_color: text_color,
                    text_color,
                    ..self.active(style, is_active)
                }
            }
        }
    }
}
