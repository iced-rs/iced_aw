//! Display fields that can only be filled with numeric type.
//!
//! *This API requires the following crate features to be activated: `number_input`*
#[cfg(not(target_arch = "wasm32"))]
use iced_native::{Background, Color};
#[cfg(target_arch = "wasm32")]
use iced_web::{Background, Color};

/// The appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`NumberInput`](crate::native::number_input::NumberInput).
    pub button_background: Option<Background>,
    /// The Color of the arrows of [`NumberInput`](crate::native::number_input::NumberInput).
    pub icon_color: Color,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            button_background: None,
            icon_color: Color::BLACK,
        }
    }
}

/// The appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
pub trait StyleSheet {
    /// The normal appearance of a [`NumberInput`](crate::native::number_input::NumberInput).
    fn active(&self) -> Style;

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is pressed.
    fn pressed(&self) -> Style {
        self.active()
    }

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is disabled.
    fn disabled(&self) -> Style {
        let active = self.active();
        Style {
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
            ..active
        }
    }
}

/// The default appearance of the [`NumberInput`](crate::native::number_input::NumberInput).
#[derive(Clone, Copy, Debug)]
struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::default()
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
