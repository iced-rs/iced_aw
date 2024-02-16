//! Displays a [`TabBar`](crate::native::tab_bar::TabBar) to select the content
//! to be displayed.
//!
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](crate::native::tabs::Tabs) widget instead.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*

use std::rc::Rc;

use iced::{border::Radius, Background, Color, Theme};

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

    /// The color of the closing icon border
    pub icon_background: Option<Background>,

    /// How soft/hard the corners of the icon border are
    pub icon_border_radius: Radius,

    /// The text color of the tab labels.
    pub text_color: Color,
}

/// The appearance of a [`TabBar`](crate::native::tab_bar::TabBar).
pub trait StyleSheet {
    ///Style for the trait to use.
    type Style: Default;

    /// The normal appearance of a tab bar and its tab labels.
    ///
    /// `is_active` is true if the tab is selected.
    fn active(&self, style: &Self::Style, is_active: bool) -> Appearance;

    /// The appearance when the tab bar and/or a tab label is hovered.
    ///
    /// `is_active` is true if the tab is selected.
    fn hovered(&self, style: &Self::Style, is_active: bool) -> Appearance;
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
            icon_background: Some(Background::Color(Color::TRANSPARENT)),
            icon_border_radius: 4.0.into(),
            text_color: Color::BLACK,
        }
    }
}

#[derive(Clone, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``TabBar`` Styles
pub enum TabBarStyles {
    #[default]
    Default,
    Dark,
    Red,
    Blue,
    Green,
    Purple,
    Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl TabBarStyles {
    /// Creates a custom [`TabBarStyles`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Rc::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = TabBarStyles;

    fn active(&self, style: &Self::Style, is_active: bool) -> Appearance {
        let mut appearance = Appearance::default();

        match style {
            TabBarStyles::Default => {
                appearance.tab_label_background = if is_active {
                    Background::Color([0.9, 0.9, 0.9].into())
                } else {
                    Background::Color([0.87, 0.87, 0.87].into())
                };
            }
            TabBarStyles::Dark => {
                appearance.tab_label_background = if is_active {
                    Background::Color([0.1, 0.1, 0.1].into())
                } else {
                    Background::Color([0.13, 0.13, 0.13].into())
                };
                appearance.tab_label_border_color = [0.3, 0.3, 0.3].into();
                appearance.icon_color = Color::WHITE;
                appearance.text_color = Color::WHITE;
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
            TabBarStyles::Custom(custom) => return custom.active(self, is_active),
        }

        appearance
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> Appearance {
        match style {
            TabBarStyles::Default => Appearance {
                tab_label_background: Background::Color([0.9, 0.9, 0.9].into()),
                ..self.active(style, is_active)
            },
            TabBarStyles::Dark => Appearance {
                tab_label_background: Background::Color([0.1, 0.1, 0.1].into()),
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
            TabBarStyles::Custom(custom) => custom.hovered(self, is_active),
        }
    }
}
