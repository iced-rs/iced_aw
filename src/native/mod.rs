//! A module fitting `iced_native`.

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