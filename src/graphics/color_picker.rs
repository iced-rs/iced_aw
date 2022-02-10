//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use crate::native::color_picker;
pub use crate::native::color_picker::State;

/// An input element for picking colors.
///
/// This is an alias of an `iced_native` `ColorPicker` with an `iced_wgpu::Renderer`.
pub use color_picker::ColorPicker;
