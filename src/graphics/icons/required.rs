//! Bootstrap icons.
//! Machine generated code. Do not change!

/// Bootstrap icons
#[derive(Copy, Clone, Debug, Hash)]
pub enum Icon {
    /// caret-down-fill
    CaretDownFill,
    /// caret-left-fill
    CaretLeftFill,
    /// caret-right-fill
    CaretRightFill,
    /// caret-up-fill
    CaretUpFill,
    /// check
    Check,
    /// x
    X,
}

/// Converts an icon into a char.
#[must_use]
pub const fn icon_to_char(icon: Icon) -> char {
    match icon {
        Icon::CaretDownFill => '\u{f217}',
        Icon::CaretLeftFill => '\u{f21b}',
        Icon::CaretRightFill => '\u{f21f}',
        Icon::CaretUpFill => '\u{f223}',
        Icon::Check => '\u{f25c}',
        Icon::X => '\u{f5ae}',
    }
}
