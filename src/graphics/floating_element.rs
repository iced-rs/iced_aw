//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*
use iced_graphics::Renderer;

use crate::native::floating_element;
pub use floating_element::{Anchor, Offset};

/// A floating button floating over some content.
///
/// This is an alias of an `iced_native` `FloatingElement` with an `iced_wgpu::Renderer`.
pub type FloatingElement<'a, B, Message, Backend, Theme> =
    floating_element::FloatingElement<'a, B, Message, Renderer<Backend, Theme>>;
