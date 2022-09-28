//! Use a grid as an input element for creating grids.
//!
//! *This API requires the following crate features to be activated: `grid`*
use iced_graphics::Renderer;

use crate::native::grid;

/// A container that distributes its contents in a grid.
///
/// This is an alias of an `iced_native` `Grid` with a default `iced_wgpu::Renderer`.
pub type Grid<'a, Message, Backend, Theme> = grid::Grid<'a, Message, Renderer<Backend, Theme>>;
