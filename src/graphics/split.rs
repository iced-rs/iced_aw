//! Use a split to split the available space in two parts to display two different elements.
//!
//! *This API requires the following crate features to be activated: split*
use iced_graphics::Renderer;

use crate::native::split;
pub use crate::native::split::{Axis, State};
pub use crate::style::split::{Appearance, StyleSheet};

/// A split can divide the available space by half to display two different elements.
/// It can split horizontally or vertically.
///
/// This is an alias of an `iced_native` Split with an `iced_wgpu::Renderer`.
pub type Split<'a, Message, Backend, Theme> = split::Split<'a, Message, Renderer<Backend, Theme>>;
