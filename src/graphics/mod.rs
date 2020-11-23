//! A module fitting `iced_graphics`.

pub mod icons;

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