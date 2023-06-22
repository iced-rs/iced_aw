use iced_native::Font;

/// `SFUIRounded` font
pub const SF_UI_ROUNDED: Font = Font::External {
    name:  "SFUIRounded",
    bytes: include_bytes!("SFUIRounded.ttf"),
};

/// `SFUIMono` font
pub const SF_UI_MONO: Font = Font::External {
    name:  "SFUIMono",
    bytes: include_bytes!("SFUIMono.ttf"),
};

