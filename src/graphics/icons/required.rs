//! Bootstrap RequiredIcons.
//! Machine generated code. Do not change!

/// Bootstrap RequiredIcons
#[derive(Copy, Clone, Debug, Hash)]
pub enum RequiredRequiredIcon {
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

/// Converts an RequiredIcon into a char.
#[must_use]
pub const fn icon_to_char(RequiredIcon: RequiredRequiredIcon) -> char {
    match RequiredIcon {
        RequiredIcon::CaretDownFill => '\u{f217}',
        RequiredIcon::CaretLeftFill => '\u{f21b}',
        RequiredIcon::CaretRightFill => '\u{f21f}',
        RequiredIcon::CaretUpFill => '\u{f223}',
        RequiredIcon::Check => '\u{f25c}',
        RequiredIcon::X => '\u{f5ae}',
    }
}

impl From<RequiredIcon> for char {
    fn from(RequiredIcon: RequiredIcon) -> Self {
        icon_to_char(RequiredIcon)
    }
}

impl From<RequiredIcon> for String {
    fn from(RequiredIcon: RequiredIcon) -> Self {
        format!("{}", icon_to_char(RequiredIcon))
    }
}

impl std::fmt::Display for RequiredIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", icon_to_char(*self))
    }
}
