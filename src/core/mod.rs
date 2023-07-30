//! A module fitting `iced_core`.

#[cfg(feature = "date_picker")]
//#[cfg(all(feature = "date_picker", not(target_arch = "wasm32")))]
pub mod date;

#[cfg(feature = "time_picker")]
pub mod clock;

#[cfg(feature = "color_picker")]
pub mod color;

pub mod overlay;

pub mod renderer;

#[cfg(feature = "time_picker")]
pub mod time;
