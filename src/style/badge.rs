//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};

/// The appearance of a [`Badge`](crate::native::badge::Badge).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`Badge`](crate::native::badge::Badge).
    pub background: Background,

    /// The border radius of the [`Badge`](crate::native::badge::Badge).
    /// If no radius is specified the default one will be used.
    pub border_radius: Option<f32>,

    /// The border with of the [`Badge`](crate::native::badge::Badge).
    pub border_width: f32,

    /// The border color of the [`Badge`](crate::native::badge::Badge).
    pub border_color: Option<Color>,

    /// The default text color of the [`Badge`](crate::native::badge::Badge).
    pub text_color: Color,
}

/// The appearance of a [`Badge`](crate::native::badge::Badge).
pub trait StyleSheet {
    ///Style for the trait to use.
    type Style: Default + Copy;
    /// The normal appearance of a [`Badge`](crate::native::badge::Badge).
    fn active(&self, style: Self::Style) -> Appearance;

    /// The appearance when the [`Badge`](crate::native::badge::Badge) is hovered.
    fn hovered(&self, style: Self::Style) -> Appearance {
        self.active(style)
    }
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            background: Background::Color([0.87, 0.87, 0.87].into()),
            border_radius: None,
            border_width: 1.0,
            border_color: Some([0.8, 0.8, 0.8].into()),
            text_color: Color::BLACK,
        }
    }
}

#[cfg(feature = "colors")]
pub use predefined::*;
#[cfg(feature = "colors")]
/// Predefined styles for the [`Badge`](crate::native::Badge) widget.
mod predefined {
    use crate::style::badge::StyleSheet;
    use crate::style::{badge::Appearance, colors};

    /// The appearance with the [`primary`](colors::PRIMARY) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Primary;

    impl StyleSheet for Primary {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> super::Appearance {
            Appearance {
                background: colors::PRIMARY.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::PRIMARY.into(),
                text_color: colors::WHITE,
            }
        }
    }

    /// The appearance with the [`secondary`](colors::SECONDARY) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Secondary;

    impl StyleSheet for Secondary {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> Appearance {
            Appearance {
                background: colors::SECONDARY.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::SECONDARY.into(),
                text_color: colors::WHITE,
            }
        }
    }

    /// The appearance with the [`success`](colors::SUCCESS) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Success;

    impl StyleSheet for Success {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> Appearance {
            Appearance {
                background: colors::SUCCESS.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::SUCCESS.into(),
                text_color: colors::WHITE,
            }
        }
    }

    /// The appearance with the [`danger`](colors::DANGER) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Danger;

    impl StyleSheet for Danger {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> Appearance {
            Appearance {
                background: colors::DANGER.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::DANGER.into(),
                text_color: colors::WHITE,
            }
        }
    }

    /// The appearance with the [`warning`](colors::WARNING) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Warning;

    impl StyleSheet for Warning {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> Appearance {
            Appearance {
                background: colors::WARNING.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::WARNING.into(),
                text_color: colors::BLACK,
            }
        }
    }

    /// The appearance with the [`info`](colors::INFO) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Info;

    impl StyleSheet for Info {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> Appearance {
            Appearance {
                background: colors::INFO.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::INFO.into(),
                text_color: colors::BLACK,
            }
        }
    }

    /// The appearance with the [`light`](colors::LIGHT) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Light;

    impl StyleSheet for Light {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> Appearance {
            Appearance {
                background: colors::LIGHT.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::LIGHT.into(),
                text_color: colors::BLACK,
            }
        }
    }

    /// The appearance with the [`dark`](colors::DARK) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Dark;

    impl StyleSheet for Dark {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> Appearance {
            Appearance {
                background: colors::DARK.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::DARK.into(),
                text_color: colors::WHITE,
            }
        }
    }

    /// The appearance with the [`white`](colors::WHITE) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct White;

    impl StyleSheet for White {
        type Style = Appearance;
        fn active(&self, _style: Self::Style) -> Appearance {
            Appearance {
                background: colors::WHITE.into(),
                border_radius: None,
                border_width: 1.0,
                border_color: colors::WHITE.into(),
                text_color: colors::BLACK,
            }
        }
    }
}
