//! Predefined styles for a [`Button`](iced_native::button::Button).
//!
//! *This API requires the following crate features to be activated: colors*

#[cfg(feature = "colors")]
pub use predefined::*;
#[cfg(feature = "colors")]
/// Predefined styles for the [`Button`](iced_native::Button) widget.
mod predefined {
    use iced_style::button::{Style, StyleSheet};

    use crate::style::colors;

    /// The appearance with the [`primary`](colors::PRIMARY) color of a
    /// [`Button`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct Primary;

    impl StyleSheet for Primary {
        fn active(&self) -> Style {
            Style {
                background: colors::PRIMARY.into(),
                text_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`secondary`](colors::SECONDARY) color of a
    /// [`Button`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct Secondary;

    impl StyleSheet for Secondary {
        fn active(&self) -> Style {
            Style {
                background: colors::SECONDARY.into(),
                text_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`success`](colors::SUCCESS) color of a
    /// [`Button`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct Success;

    impl StyleSheet for Success {
        fn active(&self) -> Style {
            Style {
                background: colors::SUCCESS.into(),
                text_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`danger`](colors::DANGER) color of a
    /// [`Button`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct Danger;

    impl StyleSheet for Danger {
        fn active(&self) -> Style {
            Style {
                background: colors::DANGER.into(),
                text_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`warning`](colors::WARNING) color of a
    /// [`Warning`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct Warning;

    impl StyleSheet for Warning {
        fn active(&self) -> Style {
            Style {
                background: colors::WARNING.into(),
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`info`](colors::INFO) color of a
    /// [`Button`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct Info;

    impl StyleSheet for Info {
        fn active(&self) -> Style {
            Style {
                background: colors::INFO.into(),
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`light`](colors::LIGHT) color of a
    /// [`Button`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct Light;

    impl StyleSheet for Light {
        fn active(&self) -> Style {
            Style {
                background: colors::LIGHT.into(),
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`dark`](colors::DARK) color of a
    /// [`Button`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct Dark;

    impl StyleSheet for Dark {
        fn active(&self) -> Style {
            Style {
                background: colors::DARK.into(),
                text_color: colors::WHITE,
                ..Style::default()
            }
        }
    }

    /// The appearance with the [`white`](colors::WHITE) color of a
    /// [`Button`](iced_native::button::Button).
    #[derive(Clone, Copy, Debug)]
    pub struct White;

    impl StyleSheet for White {
        fn active(&self) -> Style {
            Style {
                background: colors::WHITE.into(),
                ..Style::default()
            }
        }
    }
}
