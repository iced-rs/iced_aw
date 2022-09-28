//! Display fields that can only be filled with numeric type.
//!
//! *This API requires the following crate features to be activated: `number_input`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};

/// The appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The background of the [`NumberInput`](crate::native::number_input::NumberInput).
    pub button_background: Option<Background>,
    /// The Color of the arrows of [`NumberInput`](crate::native::number_input::NumberInput).
    pub icon_color: Color,
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            button_background: None,
            icon_color: Color::BLACK,
        }
    }
}

/// The appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
pub trait StyleSheet {
    type Appearance: std::default::Default + Copy;
    /// The normal appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
    fn active(&self) -> Appearance;

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is pressed.
    fn pressed(&self) -> Appearance {
        self.active()
    }

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is disabled.
    fn disabled(&self) -> Appearance {
        let active = self.active();
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

/// The default appearance of the [`NumberInput`](crate::native::number_input::NumberInput).
#[derive(Clone, Copy, Debug)]
struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Appearance {
        Appearance::default()
    }
}
