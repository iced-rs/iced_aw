//! A module fitting `iced_graphics`.

pub mod icons;

/// The default cupertino font bytes for loading the font into the system.
#[cfg(feature = "cupertino")]
pub const SF_UI_ROUNDED_BYTES: &[u8] = include_bytes!("./fonts/SFUIRounded.ttf");

/// The default cupertino font for alerts and button.
#[cfg(feature = "cupertino")]
pub const SF_UI_ROUNDED: iced_widget::core::Font =
    iced_widget::core::Font::with_name("sfuiRounded");
