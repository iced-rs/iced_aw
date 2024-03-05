//! Bootstrap RequiredIcons.
//! Machine generated code. Do not change!

/// Bootstrap RequiredIcons
#[derive(Copy, Clone, Debug, Hash)]
pub enum Bootstrap {
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

/// Converts an Bootstrap into a char.
#[must_use]
pub const fn icon_to_char(icon: Bootstrap) -> char {
    match icon {
        Bootstrap::CaretDownFill => '\u{f217}',
        Bootstrap::CaretLeftFill => '\u{f21b}',
        Bootstrap::CaretRightFill => '\u{f21f}',
        Bootstrap::CaretUpFill => '\u{f223}',
        Bootstrap::Check => '\u{f25c}',
        Bootstrap::X => '\u{f5ae}',
    }
}

/// Converts an Bootstrap into a String.
#[must_use]
pub fn icon_to_string(icon: Bootstrap) -> String {
    icon_to_char(icon).to_string()
}

impl From<Bootstrap> for char {
    fn from(icon: Bootstrap) -> Self {
        icon_to_char(icon)
    }
}

impl From<Bootstrap> for String {
    fn from(icon: Bootstrap) -> Self {
        format!("{}", icon_to_char(icon))
    }
}

impl std::fmt::Display for Bootstrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", icon_to_char(*self))
    }
}
