//! A module fitting `iced_graphics`.

pub mod icons;

#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "badge")]
pub use badge::Badge;

#[cfg(feature = "floating_button")]
pub mod floating_button;
#[cfg(feature = "floating_button")]
pub use floating_button::FloatingButton;

#[cfg(feature = "tab_bar")]
pub mod tab_bar;
#[cfg(feature = "tabs")]
pub mod tabs;

#[doc(no_inline)]
#[cfg(feature = "tab_bar")]
pub use tab_bar::TabBar;

#[doc(no_inline)]
#[cfg(feature = "tabs")]
pub use tabs::Tabs;