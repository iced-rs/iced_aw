//! Bootstrap icons.
//! Machine generated code. Do not change!

/// Bootstrap icons
#[derive(Copy, Clone, Debug, Hash)]
pub enum Icon {
	/// x
	X,
}

/// Converts an icon into a char.
pub fn icon_to_char(icon: Icon) -> char {
	match icon {
		Icon::X => '\u{f5ae}',
	}
}
