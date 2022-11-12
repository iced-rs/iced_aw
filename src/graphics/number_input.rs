//! Display fields that can only be filled with numeric type.
//!
//! A [`NumberInput`].
use iced_graphics::Renderer;

use crate::native::number_input;
pub use crate::style::number_input::{Appearance, StyleSheet};

/// A field that can only be filled with numeric type.
///
/// This is an alias of an `iced_native` number input with an `iced_wgpu::Renderer`.
pub type NumberInput<'a, T, Message, Backend, Theme> =
    number_input::NumberInput<'a, T, Message, Renderer<Backend, Theme>>;
