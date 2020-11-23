//! The default icon font of the widgets of this library.
use iced_native::{Font};

/// The default icon font.
pub const ICON_FONT: Font = iced::Font::External{
    name: "Icons",
    bytes: include_bytes!("./fonts/icons.ttf"),
};

/// The icons available by the default icon font.
#[derive(Copy, Clone, Debug, Hash)]
pub enum Icon {

    /// A cancel icon.
    Cancel,
}

/// Converts an icon into a char.
pub fn icon_to_char(icon: Icon) -> char {
    match icon {
        Icon::Cancel => '\u{2715}'
    }
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        icon_to_char(icon)
    }
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", icon_to_char(*self))
    }
}