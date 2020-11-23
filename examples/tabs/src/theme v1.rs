use iced::{tab_bar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Default,
    Red,
    Blue,
    Green,
    Purple,
}

impl Theme {
    pub const ALL: [Theme; 5] = [
        Theme::Default,
        Theme::Red,
        Theme::Blue,
        Theme::Green,
        Theme::Purple,
    ];
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Default
    }
}

impl From<Theme> for String {
    fn from(theme: Theme) -> Self {
        String::from(match theme {
            Theme::Default => "Default",
            Theme::Red => "Red",
            Theme::Blue => "Blue",
            Theme::Green => "Green",
            Theme::Purple => "Purple",
        })
    }
}

impl From<Theme> for Box<dyn tab_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Default => Default::default(),
            Theme::Red => red::TabBar.into(),
            Theme::Blue => blue::TabBar.into(),
            Theme::Green => green::TabBar.into(),
            Theme::Purple => purple::TabBar.into(),
        }
    }
}

mod red {
    use iced::{Background, Color, tab_bar::{self, Style}};

    pub struct TabBar;

    impl tab_bar::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tab_bar::Style {
            Style {
                background: if is_selected {
                    Background::Color([1.0, 0.0, 0.0].into())
                } else {
                    Background::Color(Color::WHITE).into()
                },
                border_color: Color::TRANSPARENT,
                border_width: 0,
                icon_color: if is_selected {
                    Color::WHITE
                } else {
                    Color::BLACK
                },
                text_color: if is_selected {
                    Color::WHITE
                } else {
                    Color::BLACK
                },
            }
        }

        fn hovered(&self, _is_selected: bool) -> tab_bar::Style {
            Style {
                background: Background::Color([1.0, 0.0, 0.0].into()),
                border_color: Color::TRANSPARENT,
                border_width: 0,
                icon_color: Color::WHITE,
                text_color: Color::WHITE,
            }
        }
    }
}

mod blue {
    use iced::{Background, Color, tab_bar::{self, Style}};

    pub struct TabBar;

    impl tab_bar::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tab_bar::Style {
            Style {
                background: if is_selected {
                    Background::Color([0.0, 0.0, 1.0].into())
                } else {
                    Background::Color([0.5, 0.5, 1.0].into())
                },
                border_color: if is_selected {
                    [0.0, 0.0, 1.0].into()
                } else {
                    [0.5, 0.5, 1.0].into()
                },
                border_width: 1,
                icon_color: Color::WHITE,
                text_color: Color::WHITE,
            }
        }

        fn hovered(&self, _is_selected: bool) -> tab_bar::Style {
            Style {
                background: Background::Color([0.0, 0.0, 1.0].into()),
                border_color: [0.0, 0.0, 1.0].into(),
                border_width: 1,
                icon_color: Color::WHITE,
                text_color: Color::WHITE,
            }
        }
    }
}

mod green {
    use iced::{Color, tab_bar::{self, Style}};

    pub struct TabBar;

    impl tab_bar::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tab_bar::Style {
            let color = if is_selected {
                [0.0, 0.5, 0.0]
            } else {
                [0.7, 0.7, 0.7]
            };

            Style {
                background: Color::WHITE.into(),
                border_color: color.into(),
                border_width: 1,
                icon_color: color.into(),
                text_color: color.into(),
            }
        }

        fn hovered(&self, _is_selected: bool) -> tab_bar::Style {
            let color = [0.0, 0.4, 0.0];

            Style {
                background: Color::WHITE.into(),
                border_color: color.into(),
                border_width: 1,
                icon_color: color.into(),
                text_color: color.into(),
            }
        }
    }
}

mod purple {
    use iced::{Color, tab_bar::{self, Style}};

    pub struct TabBar;

    impl tab_bar::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tab_bar::Style {
            Style {
                background: Color::WHITE.into(),
                border_color: Color::TRANSPARENT,
                border_width: 0,
                icon_color: if is_selected {
                    [0.7, 0.0, 1.0].into()
                } else {
                    Color::BLACK
                },
                text_color: if is_selected {
                    [0.7, 0.0, 1.0].into()
                } else {
                    Color::BLACK
                }
            }
        }

        fn hovered(&self, _is_selected: bool) -> tab_bar::Style {
            Style {
                background: Color::WHITE.into(),
                border_color: Color::TRANSPARENT,
                border_width: 0,
                icon_color: [0.7, 0.0, 1.0].into(),
                text_color: [0.7, 0.0, 1.0].into(),
            }
        }
    }
}