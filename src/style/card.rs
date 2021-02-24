//! Displays a [`Card`](crate::native::Card).
//!
//! *This API requires the following crate features to be activated: card*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
#[cfg(target_arch = "wasm32")]
use iced_web::{Background, Color};

/// The appearance of a [`Card`](crate::native::card::Card).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`Card`](crate::native::card::Card).
    pub background: Background,

    /// The border radius of the [`Card`](crate::native::card::Card).
    pub border_radius: f32,

    /// The border width of the [`Card`](crate::native::card::Card).
    pub border_width: f32,

    /// The border color of the [`Card`](crate::native::card::Card).
    pub border_color: Color,

    /// The background of the head of the [`Card`](crate::native::card::Card).
    pub head_background: Background,

    /// The text color of the head of the [`Card`](crate::native::card::Card).
    pub head_text_color: Color,

    /// The background of the body of the [`Card`](crate::native::card::Card).
    pub body_background: Background,

    /// The text color of the body of the [`Card`](crate::native::card::Card).
    pub body_text_color: Color,

    /// The background of the foot of the [`Card`](crate::native::card::Card).
    pub foot_background: Background,

    /// The text color of the foot of the [`Card`](crate::native::card::Card).
    pub foot_text_color: Color,

    /// The color of the close icon of the [`Card`](crate::native::card::Card).
    pub close_color: Color,
}

/// The appearance of a [`Card`](crate::native::card::Card).
pub trait StyleSheet {
    /// The normal appearance of a [`Card`](crate::native::card::Card).
    fn active(&self) -> Style;
}

/// The default appearance of a [`Card`](crate::native::card::Card).
#[derive(Clone, Copy, Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            background: Color::WHITE.into(),
            border_radius: 10.0, //32.0,
            border_width: 1.0,
            border_color: [0.87, 0.87, 0.87].into(), //Color::BLACK.into(),
            head_background: Background::Color([0.87, 0.87, 0.87].into()),
            head_text_color: Color::BLACK,
            body_background: Color::TRANSPARENT.into(),
            body_text_color: Color::BLACK,
            foot_background: Color::TRANSPARENT.into(),
            foot_text_color: Color::BLACK,
            close_color: Color::BLACK,
        }
    }
}

#[allow(clippy::use_self)]
impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

#[allow(clippy::use_self)]
impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}

impl std::default::Default for Style {
    fn default() -> Self {
        Default.active()
    }
}

#[cfg(feature = "colors")]
pub use predefined::*;
#[cfg(feature = "colors")]
/// Predefined styles for the [`Card`](crate::native::Card) widget.
mod predefined {
    use crate::style::{
        card::{Style, StyleSheet},
        colors,
    };

    /// The appearance with the [`primary`](colors::PRIMARY) head background of
    /// a [`Card`](crate::native::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct Primary;

    impl StyleSheet for Primary {
        fn active(&self) -> Style {
            Style {
                border_color: colors::PRIMARY,
                head_background: colors::PRIMARY.into(),
                head_text_color: colors::WHITE,
                close_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`secondary`](colors::SECONDARY) head background
    /// of a [`Card`](crate::native::card::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct Secondary;

    impl StyleSheet for Secondary {
        fn active(&self) -> Style {
            Style {
                border_color: colors::SECONDARY,
                head_background: colors::SECONDARY.into(),
                head_text_color: colors::WHITE,
                close_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`success`](colors::SUCCESS) head background of
    /// a [`Card`](crate::native::card::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct Success;

    impl StyleSheet for Success {
        fn active(&self) -> Style {
            Style {
                border_color: colors::SUCCESS,
                head_background: colors::SUCCESS.into(),
                head_text_color: colors::WHITE,
                close_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`danger`](colors::DANGER) head background of a
    /// [`Card`](crate::native::card::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct Danger;

    impl StyleSheet for Danger {
        fn active(&self) -> Style {
            Style {
                border_color: colors::DANGER,
                head_background: colors::DANGER.into(),
                head_text_color: colors::WHITE,
                close_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`warning`](colors::WARNING) head background of
    /// a [`Card`](crate::native::card::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct Warning;

    impl StyleSheet for Warning {
        fn active(&self) -> Style {
            Style {
                border_color: colors::WARNING,
                head_background: colors::WARNING.into(),
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`info`](colors::INFO) head background of a
    /// [`Card`](crate::native::card::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct Info;

    impl StyleSheet for Info {
        fn active(&self) -> Style {
            Style {
                border_color: colors::INFO,
                head_background: colors::INFO.into(),
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`light`](colors::LIGHT) head background of a
    /// [`Card`](crate::native::card::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct Light;

    impl StyleSheet for Light {
        fn active(&self) -> Style {
            Style {
                border_color: colors::LIGHT,
                head_background: colors::LIGHT.into(),
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`dark`](colors::DARK) head background of a
    /// [`Card`](crate::native::card::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct Dark;

    impl StyleSheet for Dark {
        fn active(&self) -> Style {
            Style {
                border_color: colors::DARK,
                head_background: colors::DARK.into(),
                head_text_color: colors::WHITE,
                close_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`white`](colors::WHITE) head background of a
    /// [`Card`](crate::native::card::Card).
    #[derive(Clone, Copy, Debug)]
    pub struct White;

    impl StyleSheet for White {
        fn active(&self) -> Style {
            Style {
                border_color: colors::WHITE,
                head_background: colors::WHITE.into(),
                ..Style::default()
            }
        }
    }
}
