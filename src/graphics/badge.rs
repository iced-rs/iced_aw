//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use iced_graphics::Renderer;

use crate::native::badge;
pub use crate::style::badge::{Appearance, StyleSheet};

/// A badge for color highlighting small information.
///
/// This is an alias of an `iced_native` Badge with an `iced_wgpu::Renderer`.
pub type Badge<'a, Message, Backend, Theme> = badge::Badge<'a, Message, Renderer<Backend, Theme>>;
