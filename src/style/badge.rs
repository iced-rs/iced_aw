//! Use a badge for color highlighting important information.
//! 
//! *This API requires the following crate features to be activated: badge*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
#[cfg(target_arch = "wasm32")]
use iced_web::{Background, Color};

/// The appearance of a [`Badge`](crate::native::badge::Badge).
#[allow(missing_debug_implementations)]
pub struct Style {
    /// The background of the [`Badge`](crate::native::badge::Badge).
    pub background: Background,

    /// The border color of the [`Badge`](crate::native::badge::Badge).
    pub border_color: Option<Color>,

    /// The border with of the [`Badge`](crate::native::badge::Badge).
    pub border_width: u16,
}

/// The appearance of a [`Badge`](crate::native::badge::Badge).
pub trait StyleSheet {
    /// The normal appearance of a [`Badge`](crate::native::badge::Badge).
    fn active(&self) -> Style;

    /// The appearance when the [`Badge`](crate::native::badge::Badge) is hovered.
    fn hovered(&self) -> Style;
}

/// The default appearance of the [`Badge`](crate::native::badge::Badge).
#[derive(Debug)]
pub struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            background: Background::Color([0.87, 0.87, 0.87].into()),
            border_color: Some([0.8, 0.8, 0.8].into()),
            border_width: 1,
        }
    }

    fn hovered(&self) -> Style {
        self.active()
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}

#[cfg(feature = "colors")]
pub use predefined::*;
#[cfg(feature = "colors")]
mod predefined {
    use crate::style::{badge::Style, colors};
    use crate::style::badge::StyleSheet;

    /// The appearance with the [`primary`](colors::PRIMARY) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Primary;

    impl StyleSheet for Primary {
        fn active(&self) -> super::Style {
            Style {
                background: colors::PRIMARY.into(),
                border_color: colors::PRIMARY.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> super::Style {
            self.active()
        }
    }

    /// The appearance with the [`secondary`](colors::SECONDARY) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Secondary;

    impl StyleSheet for Secondary {
        fn active(&self) -> Style {
            Style {
                background: colors::SECONDARY.into(),
                border_color: colors::SECONDARY.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> Style {
            self.active()
        }
    }

    /// The appearance with the [`success`](colors::SUCCESS) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Success;

    impl StyleSheet for Success {
        fn active(&self) -> Style {
            Style {
                background: colors::SUCCESS.into(),
                border_color: colors::SUCCESS.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> Style {
            self.active()
        }
    }

    /// The appearance with the [`danger`](colors::DANGER) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Danger;

    impl StyleSheet for Danger {
        fn active(&self) -> Style {
            Style {
                background: colors::DANGER.into(),
                border_color: colors::DANGER.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> Style {
            self.active()
        }
    }

    /// The appearance with the [`warning`](colors::WARNING) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Warning;

    impl StyleSheet for Warning {
        fn active(&self) -> Style {
            Style {
                background: colors::WARNING.into(),
                border_color: colors::WARNING.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> Style {
            self.active()
        }
    }

    /// The appearance with the [`info`](colors::INFO) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Info;

    impl StyleSheet for Info {
        fn active(&self) -> Style {
            Style {
                background: colors::INFO.into(),
                border_color: colors::INFO.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> Style {
            self.active()
        }
    }

    /// The appearance with the [`light`](colors::LIGHT) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Light;

    impl StyleSheet for Light {
        fn active(&self) -> Style {
            Style {
                background: colors::LIGHT.into(),
                border_color: colors::LIGHT.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> Style {
            self.active()
        }
    }

    /// The appearance with the [`dark`](colors::DARK) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct Dark;

    impl StyleSheet for Dark {
        fn active(&self) -> Style {
            Style {
                background: colors::DARK.into(),
                border_color: colors::DARK.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> Style {
            self.active()
        }
    }

    /// The appearance with the [`white`](colors::WHITE) color of a
    /// [`Badge`](crate::native::badge::Badge).
    #[derive(Clone, Copy, Debug)]
    pub struct White;

    impl StyleSheet for White {
        fn active(&self) -> Style {
            Style {
                background: colors::WHITE.into(),
                border_color: colors::WHITE.into(),
                border_width: 1,
            }
        }

        fn hovered(&self) -> Style {
            self.active()
        }
    }
}