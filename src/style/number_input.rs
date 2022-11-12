//! Display fields that can only be filled with numeric type.
//!
//! *This API requires the following crate features to be activated: `number_input`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
use iced_style::theme::Theme;

/// The appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`NumberInput`](crate::native::number_input::NumberInput).
    pub button_background: Option<Background>,
    /// The Color of the arrows of [`NumberInput`](crate::native::number_input::NumberInput).
    pub icon_color: Color,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            button_background: None,
            icon_color: Color::BLACK,
        }
    }
}

/// The appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub trait StyleSheet {
    type Style: Default + Copy;
    /// The normal appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
    fn active(&self, style: Self::Style) -> Appearance;

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is pressed.
    fn pressed(&self, style: Self::Style) -> Appearance {
        self.active(style)
    }

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is disabled.
    fn disabled(&self, style: Self::Style) -> Appearance {
        let active = self.active(style);
        Appearance {
            button_background: active.button_background.map(|bg| match bg {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            icon_color: Color {
                a: active.icon_color.a * 0.5,
                ..active.icon_color
            },
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``NumberInput`` Styles
pub enum NumberInputStyles {
    #[default]
    Default,
}

impl StyleSheet for Theme {
    type Style = NumberInputStyles;

    fn active(&self, _style: Self::Style) -> Appearance {
        let palette = self.extended_palette();

        Appearance {
            button_background: Some(palette.primary.strong.color.into()),
            icon_color: palette.primary.strong.text,
        }
    }

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is pressed.
    fn pressed(&self, style: Self::Style) -> Appearance {
        self.active(style)
    }

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is disabled.
    fn disabled(&self, style: Self::Style) -> Appearance {
        let active = self.active(style);
        Appearance {
            button_background: active.button_background.map(|bg| match bg {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            icon_color: Color {
                a: active.icon_color.a * 0.5,
                ..active.icon_color
            },
        }
    }
}
