//! Display fields that can only be filled with numeric type.
//!
//! A [`NumberInput`] has some local [`State`].
use iced_graphics::Renderer;

use crate::native::number_input;
pub use crate::native::number_input::State;
pub use crate::style::number_input::{Style, StyleSheet};

/// A field that can only be filled with numeric type.
///
/// This is an alias of an `iced_native` number input with an `iced_wgpu::Renderer`.
pub type NumberInput<'a, T, Message, Backend> =
    number_input::NumberInput<'a, T, Message, Renderer<Backend>>;
