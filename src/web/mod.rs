//! Widgets for iced_web

#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "badge")]
pub use badge::Badge;

#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "card")]
pub use card::Card;

#[cfg(feature = "floating_button")]
pub mod floating_button;
#[cfg(feature = "floating_button")]
pub use floating_button::FloatingButton;