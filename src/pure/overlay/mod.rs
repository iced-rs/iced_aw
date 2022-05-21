//! Display interactive elements on top of other widgets.

#[cfg(feature = "floating_element")]
pub mod floating_element;
#[cfg(feature = "floating_element")]
pub use floating_element::FloatingElementOverlay;
