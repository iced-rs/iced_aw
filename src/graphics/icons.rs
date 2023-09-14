//! The default icon font of the widgets of this library.

use iced_widget::core::Font;

#[cfg(feature = "icons")]
mod bootstrap;
#[cfg(feature = "icons")]
pub use bootstrap::*;

#[cfg(not(feature = "icons"))]
mod required;
#[cfg(not(feature = "icons"))]
pub use required::*;

/// The default icon font bytes for loading the font into iced.
#[cfg(feature = "icons")]
pub const ICON_FONT_BYTES: &[u8] = include_bytes!("./fonts/bootstrap-icons.ttf");

/// The default icon font bytes for loading the font into iced.
#[cfg(not(feature = "icons"))]
pub const ICON_FONT_BYTES: &[u8] = include_bytes!("./fonts/required-icons.ttf");

/// The default icon font.
#[cfg(feature = "icons")]
pub const ICON_FONT: Font = Font::with_name("bootstrap-icons");

/// The default icon font.
#[cfg(not(feature = "icons"))]
pub const ICON_FONT: Font = Font::with_name("required-icons");

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        icon_to_char(icon)
    }
}

impl From<Icon> for String {
    fn from(icon: Icon) -> Self {
        format!("{}", icon_to_char(icon))
    }
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", icon_to_char(*self))
    }
}
