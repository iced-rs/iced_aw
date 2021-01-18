//! The default icon font of the widgets of this library.
use iced_graphics::Font;

#[cfg(feature = "icons")]
mod bootstrap;
#[cfg(feature = "icons")]
pub use bootstrap::*;

#[cfg(not(feature = "icons"))]
mod required;
#[cfg(not(feature = "icons"))]
pub use required::*;

/// The default icon font.
#[cfg(feature = "icons")]
pub const ICON_FONT: Font = iced_native::Font::External {
    name: "Icons",
    bytes: include_bytes!("./fonts/bootstrap-icons.ttf"),
};

/// The default icon font.
#[cfg(not(feature = "icons"))]
pub const ICON_FONT: Font = iced_native::Font::External {
    name: "Icons",
    bytes: include_bytes!("./fonts/required-icons.ttf"),
};

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
