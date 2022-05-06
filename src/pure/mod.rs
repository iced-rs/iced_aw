//! Stateless, pure widgets for iced

#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "badge")]
pub use badge::Badge;

<<<<<<< HEAD
#[cfg(feature = "number_input")]
pub mod number_input;
#[cfg(feature = "number_input")]
pub use number_input::NumberInput;
=======
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
>>>>>>> d1deac7f199dfc85d698fff3382111ebcaae3901
