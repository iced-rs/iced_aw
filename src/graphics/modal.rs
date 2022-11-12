//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*

use iced_graphics::Renderer;

use crate::native::modal;
pub use crate::native::modal::State;
pub use crate::style::modal::{ModalStyles, StyleSheet};

/// A modal content as an overlay.
///
/// This is an alias of an `iced_native` Modal with an `iced_wgpu::Renderer`.
pub type Modal<'a, Content, Message, Backend, Theme> =
    modal::Modal<'a, Content, Message, Renderer<Backend, Theme>>;
