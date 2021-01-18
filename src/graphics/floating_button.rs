//! TODO

use iced_graphics::Renderer;

use crate::native::floating_button;
pub use floating_button::{Anchor, Offset};

/// TODO
pub type FloatingButton<'a, B, Message, Backend> =
    floating_button::FloatingButton<'a, B, Message, Renderer<Backend>>;
