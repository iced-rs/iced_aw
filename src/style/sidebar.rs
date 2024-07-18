//! This is the style for [`Sidebar`](crate::widgets::sidebar::Sidebar) and
//! [`SidebarWithContent`](crate::widgets::sidebar::SidebarWithContent).
//!
//! *This API requires the following crate features to be activated: `sidebar`*

use super::{Status, StyleFn};
use iced::{border::Radius, Background, Color, Theme};

/// The appearance of a [`Sidebar`](crate::widgets::sidebar::Sidebar).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the sidebar.
    pub background: Option<Background>,

    /// The border color of the sidebar.
    pub border_color: Option<Color>,

    /// The border width of the sidebar.
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
/// The Catalog of a [`Sidebar`](crate::widgets::sidebar::Sidebar).
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

/// The primary theme of a [`Sidebar`](crate::widgets::sidebar::Sidebar).
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

/// The dark theme of a [`Sidebar`](crate::widgets::sidebar::Sidebar).
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

/// The red theme of a [`Sidebar`](crate::widgets::sidebar::Sidebar).
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

/// The blue theme of a [`Sidebar`](crate::widgets::sidebar::Sidebar).
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

/// The blue theme of a [`Sidebar`](crate::widgets::sidebar::Sidebar).
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

/// The purple theme of a [`Sidebar`](crate::widgets::sidebar::Sidebar).
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
