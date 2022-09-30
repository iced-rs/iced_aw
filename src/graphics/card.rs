//! Displays a [`Card`](Card).
//!
//! *This API requires the following crate features to be activated: card*
use iced_graphics::Renderer;

use crate::native::card;
pub use crate::style::card::{Appearance, StyleSheet};

/// A card consisting of a head, body and optional foot.
///
/// This is an alias of an `iced_native` Card with an `iced_wgpu::Renderer`.
pub type Card<'a, Message, Backend, Theme> = card::Card<'a, Message, Renderer<Backend, Theme>>;
