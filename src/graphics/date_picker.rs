//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*

use crate::native::date_picker;
pub use crate::native::date_picker::{Date, State};

/// An input element for picking dates.
///
/// This is an alias of an `iced_native` `DatePicker` with an `iced_wgpu::Renderer`.
pub use date_picker::DatePicker;
