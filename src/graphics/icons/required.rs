//! Bootstrap RequiredIcons.
//! Machine generated code. Do not change!

/// Bootstrap RequiredIcons
#[derive(Copy, Clone, Debug, Hash)]
pub enum BootstrapIcon {
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

/// Converts an BootstrapIcon into a char.
#[must_use]
pub const fn icon_to_char(icon: BootstrapIcon) -> char {
    match icon {
        BootstrapIcon::CaretDownFill => '\u{f217}',
        BootstrapIcon::CaretLeftFill => '\u{f21b}',
        BootstrapIcon::CaretRightFill => '\u{f21f}',
        BootstrapIcon::CaretUpFill => '\u{f223}',
        BootstrapIcon::Check => '\u{f25c}',
        BootstrapIcon::X => '\u{f5ae}',
    }
}

/// Converts an BootstrapIcon into a String.
#[must_use]
pub fn icon_to_string(icon: BootstrapIcon) -> String {
    icon_to_char(icon).to_string()
}

impl From<BootstrapIcon> for char {
    fn from(icon: BootstrapIcon) -> Self {
        icon_to_char(icon)
    }
}

impl From<BootstrapIcon> for String {
    fn from(icon: BootstrapIcon) -> Self {
        format!("{}", icon_to_char(icon))
    }
}

impl std::fmt::Display for BootstrapIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", icon_to_char(*self))
    }
}
