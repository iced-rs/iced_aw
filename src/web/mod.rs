//! Widgets for iced_web

#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "badge")]
pub use badge::Badge;

#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "card")]
pub use card::Card;

#[cfg(feature = "date_picker")]
pub mod date_picker;
#[cfg(feature = "date_picker")]
pub use date_picker::DatePicker;

#[cfg(feature = "floating_button")]
pub mod floating_button;
#[cfg(feature = "floating_button")]
pub use floating_button::FloatingButton;

#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "modal")]
pub use modal::Modal;