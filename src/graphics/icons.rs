//! The default icon font of the widgets of this library.

use cfg_if::cfg_if;
use iced_widget::core::Font;

cfg_if! {
    if #[cfg(feature = "icons")] {
        pub mod bootstrap;
        pub mod nerd;

        pub use bootstrap::{BootstrapIcon, icon_to_char, icon_to_string};
        /// The default icon font bytes for loading the font into iced.
        pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("./fonts/bootstrap-icons.ttf");
        /// the icon font that has all nerd fonts.
        pub const NERD_FONT_BYTES: &[u8] = include_bytes!("./fonts/nerd-icons.ttf");

        /// The bootstrap icon font.
        pub const BOOTSTRAP_FONT: Font = Font::with_name("bootstrap-icons");
        /// The nerd icon font.
        pub const NERD_FONT: Font = Font::with_name("nerd-icons");

    } else {
        pub mod required;
        pub use required::{BootstrapIcon, icon_to_char, icon_to_string};
        /// The default icon font bytes for loading the font into iced.
        pub const BOOTSTRAP_FONT_BYTES: &[u8] = include_bytes!("./fonts/required-icons.ttf");
        /// The default icon font.
        pub const BOOTSTRAP_FONT: Font = Font::with_name("required-icons");
    }

}
