use iced_aw::tabs;

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

impl From<Theme> for Box<dyn tabs::StyleSheet> {
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
    use iced::{Background, Color};
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = if is_selected {
                Background::Color([1.0, 0.0, 0.0].into())
            } else {
                Background::Color(Color::WHITE)
            };

            let text_color = if is_selected {
                Color::WHITE
            } else {
                Color::BLACK
            };

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background,
                tab_label_border_color: Color::TRANSPARENT,
                tab_label_border_width: 0.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = Background::Color([1.0, 0.0, 0.0].into());
            let text_color = Color::WHITE;

            Style {
                tab_label_background,
                icon_color: text_color,
                text_color,
                ..self.active(is_selected)
            }
        }
    }
}

mod blue {
    use iced::{Background, Color};
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = if is_selected {
                Background::Color([0.0, 0.0, 1.0].into())
            } else {
                Background::Color([0.5, 0.5, 1.0].into())
            };

            let tab_label_border_color = if is_selected {
                [0.0, 0.0, 1.0].into()
            } else {
                [0.5, 0.5, 1.0].into()
            };

            let text_color = Color::WHITE;

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background,
                tab_label_border_color,
                tab_label_border_width: 1.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let tab_label_background = Background::Color([0.0, 0.0, 1.0].into());
            let tab_label_border_color = [0.0, 0.0, 1.0].into();

            Style {
                tab_label_background,
                tab_label_border_color,
                ..self.active(is_selected)
            }
        }
    }
}

mod green {
    use iced::Color;
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let color = if is_selected {
                [0.0, 0.5, 0.0]
            } else {
                [0.7, 0.7, 0.7]
            }
            .into();

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background: Color::WHITE.into(),
                tab_label_border_color: color,
                tab_label_border_width: 1.0,
                icon_color: color,
                text_color: color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let color = [0.0, 0.4, 0.0].into();

            Style {
                tab_label_border_color: color,
                icon_color: color,
                text_color: color,
                ..self.active(is_selected)
            }
        }
    }
}

mod purple {
    use iced::Color;
    use iced_aw::tabs::{self, Style};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let text_color = if is_selected {
                [0.7, 0.0, 1.0].into()
            } else {
                Color::BLACK
            };

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background: Color::WHITE.into(),
                tab_label_border_color: Color::TRANSPARENT,
                tab_label_border_width: 0.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let text_color = [0.7, 0.0, 1.0].into();

            Style {
                icon_color: text_color,
                text_color,
                ..self.active(is_selected)
            }
        }
    }
}
