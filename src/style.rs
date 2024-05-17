//! The appearance of the widgets

pub mod colors;
pub mod status;
pub mod style_state;

#[cfg(feature = "badge")]
pub mod badge;

#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "card")]
pub use card::CardStyles;

#[cfg(feature = "color_picker")]
pub mod color_picker;
#[cfg(feature = "color_picker")]
pub use color_picker::ColorPickerStyles;

#[cfg(feature = "date_picker")]
pub mod date_picker;
#[cfg(feature = "date_picker")]
pub use date_picker::DatePickerStyle;

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
#[cfg(feature = "time_picker")]
pub use time_picker::TimePickerStyle;

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

#[cfg(feature = "menu")]
pub mod menu_bar;
#[cfg(feature = "menu")]
pub use menu_bar::MenuBarStyle;

#[cfg(feature = "spinner")]
pub mod spinner;
#[cfg(feature = "spinner")]
pub use spinner::SpinnerStyle;

#[cfg(feature = "context_menu")]
pub mod context_menu;
#[cfg(feature = "context_menu")]
pub use context_menu::ContextMenuStyle;

#[cfg(feature = "segmented_button")]
pub mod segmented_button;
#[cfg(feature = "segmented_button")]
pub use segmented_button::SegmentedButton;
