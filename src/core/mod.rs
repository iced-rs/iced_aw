//! A module fitting `iced_core`.

#[cfg(feature = "date_picker")]
pub mod date;

#[cfg(all(feature = "time_picker", not(target_arch = "wasm32")))]
pub mod clock;
#[cfg(feature = "time_picker")]
pub mod time;