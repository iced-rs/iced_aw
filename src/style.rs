//! The appearance of the widgets

pub mod colors;
pub mod status;
pub mod style_state;

pub use status::{Status, StyleFn};

#[cfg(feature = "badge")]
pub mod badge;

#[cfg(feature = "card")]
pub mod card;

#[cfg(feature = "color_picker")]
pub mod color_picker;

#[cfg(feature = "date_picker")]
pub mod date_picker;

#[cfg(feature = "tab_bar")]
pub mod tab_bar;

#[cfg(feature = "time_picker")]
pub mod time_picker;

#[cfg(feature = "number_input")]
pub mod number_input;

#[cfg(feature = "selection_list")]
pub mod selection_list;

#[cfg(feature = "menu")]
pub mod menu_bar;

#[cfg(feature = "context_menu")]
pub mod context_menu;

#[cfg(feature = "sidebar")]
pub mod sidebar;
