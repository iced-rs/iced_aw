//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_button`*
use iced_graphics::Renderer;

use crate::native::floating_button;
pub use floating_button::{Anchor, Offset};

/// A floating button floating over some content.
///
/// This is an alias of an `iced_native` `FloatingButton` with an `iced_wgpu::Renderer`.
pub type FloatingButton<'a, B, Message, Backend> =
    floating_button::FloatingButton<'a, B, Message, Renderer<Backend>>;
