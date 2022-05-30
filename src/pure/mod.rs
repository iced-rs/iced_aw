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

#[cfg(feature = "grid")]
pub mod grid;
#[cfg(feature = "grid")]
pub use grid::Grid;

#[cfg(feature = "icon_text")]
pub mod icon_text;
#[cfg(feature = "icon_text")]
pub use icon_text::IconText;

#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "modal")]
pub use modal::Modal;

#[cfg(feature = "tab_bar")]
pub mod tab_bar;
#[cfg(feature = "tab_bar")]
pub use tab_bar::{TabBar, TabLabel};

#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "tabs")]
pub use tabs::{TabBarPosition, Tabs};

#[cfg(feature = "time_picker")]
pub mod time_picker;
#[cfg(feature = "time_picker")]
pub use time_picker::TimePicker;
