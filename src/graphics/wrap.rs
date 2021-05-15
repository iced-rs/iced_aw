//! A widget that displays its children in multiple horizontal or vertical runs.
//!
//! *This API requires the following crate features to be activated: `wrap`*
use crate::native::wrap;

/// A widget that displays its children in multiple horizontal or vertical runs.
///
/// This is an alias of an `iced_native` `Wrap` with an `iced_wgpu::Renderer`.
pub type Wrap<'a, B, Message, Direction> = wrap::Wrap<'a, B, Message, Direction>;
