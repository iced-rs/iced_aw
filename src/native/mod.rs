//! A module fitting `iced_native`.

pub mod overlay;

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

#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "modal")]
pub use modal::Modal;

#[cfg(feature = "tab_bar")]
pub mod tab_bar;
#[cfg(feature = "tabs")]
pub mod tabs;

#[doc(no_inline)]
#[cfg(feature = "tab_bar")]
pub use tab_bar::{TabBar, TabLabel};

#[doc(no_inline)]
#[cfg(feature = "tabs")]
pub use tabs::Tabs;