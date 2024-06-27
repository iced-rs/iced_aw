//! `iced_aw_core`.
use cfg_if::cfg_if;

#[cfg(feature = "date_picker")]
pub mod date;

#[cfg(feature = "time_picker")]
pub mod clock;

#[cfg(feature = "color_picker")]
pub mod color;

pub mod overlay;

pub mod renderer;

#[cfg(feature = "time_picker")]
pub mod time;

#[cfg(feature = "drop_down")]
pub mod offset;

#[cfg(feature = "drop_down")]
pub mod alignment;

pub mod icons;

cfg_if! {
    if #[cfg(feature = "icons")] {
        pub use icons::{BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES, NERD_FONT, NERD_FONT_BYTES, Bootstrap, Nerd, bootstrap, nerd};
    } else {
        pub use icons::{BOOTSTRAP_FONT, BOOTSTRAP_FONT_BYTES, Bootstrap, bootstrap};
    }
}
