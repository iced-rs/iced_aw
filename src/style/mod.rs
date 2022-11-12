//! The appearance of the widgets

pub mod style_state;

#[cfg(feature = "colors")]
pub mod colors;

#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "badge")]
pub use badge::BadgeStyles;

#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "card")]
pub use card::CardStyles;

#[cfg(feature = "color_picker")]
pub mod color_picker;

#[cfg(feature = "date_picker")]
pub mod date_picker;

#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "modal")]
pub use modal::ModalStyles;

#[cfg(feature = "tab_bar")]
pub mod tab_bar;
#[cfg(feature = "tab_bar")]
pub use tab_bar::TabBarStyles;

#[cfg(feature = "time_picker")]
pub mod time_picker;

#[cfg(feature = "number_input")]
pub mod number_input;
#[cfg(feature = "number_input")]
pub use number_input::NumberInputStyles;

#[cfg(feature = "selection_list")]
pub mod selection_list;
#[cfg(feature = "selection_list")]
pub use selection_list::SelectionListStyles;

#[cfg(feature = "split")]
pub mod split;
#[cfg(feature = "split")]
pub use split::SplitStyles;
