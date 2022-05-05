//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*

use crate::native::time_picker;
pub use crate::native::time_picker::{Period, State, Time};

/// An input element for picking times.
///
/// This is an alias of an `iced_native` `TimePicker` with an `iced_wgpu::Renderer`.
pub use time_picker::TimePicker;
