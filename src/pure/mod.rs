//! Stateless, pure widgets for iced

pub mod overlay;

#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "badge")]
pub use badge::Badge;

#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "card")]
pub use card::Card;

#[cfg(feature = "color_picker")]
pub mod color_picker;
#[cfg(feature = "color_picker")]
pub use color_picker::ColorPicker;

#[cfg(feature = "date_picker")]
pub mod date_picker;
#[cfg(feature = "date_picker")]
pub use date_picker::DatePicker;

#[cfg(feature = "floating_element")]
pub mod floating_element;
#[cfg(feature = "floating_element")]
pub use floating_element::FloatingElement;
