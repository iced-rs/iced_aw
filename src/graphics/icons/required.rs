//! Bootstrap icons.
//! Machine generated code. Do not change!

/// Bootstrap icons
#[derive(Copy, Clone, Debug, Hash)]
pub enum Icon {
	/// caret-left-fill
	CaretLeftFill,
	/// caret-right-fill
	CaretRightFill,
	/// check
	Check,
	/// x
	X,
}

/// Converts an icon into a char.
pub fn icon_to_char(icon: Icon) -> char {
	match icon {
		Icon::CaretLeftFill => '\u{f21b}',
		Icon::CaretRightFill => '\u{f21f}',
		Icon::Check => '\u{f25c}',
		Icon::X => '\u{f5ae}',
	}
}
