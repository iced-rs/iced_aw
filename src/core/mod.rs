//! A module fitting `iced_core`.

#[cfg(feature = "date_picker")]
//#[cfg(all(feature = "date_picker", not(target_arch = "wasm32")))]
pub mod date;

#[cfg(all(feature = "time_picker", not(target_arch = "wasm32")))]
pub mod clock;

#[cfg(all(feature = "color_picker", not(target_arch = "wasm32")))]
pub mod color;

#[cfg(not(target_arch = "wasm32"))]
pub mod overlay;

#[cfg(not(target_arch = "wasm32"))]
pub mod renderer;

#[cfg(feature = "time_picker")]
pub mod time;
