//! Display a list of selectable values.
use iced_graphics::Renderer;

pub use crate::native::selection_list::{self, list};
pub use crate::style::selection_list::{SelectionListStyles, StyleSheet};

/// A widget allowing the selection of a single value from a list of options.
pub type SelectionList<'a, T, Message, Backend, Theme> =
    selection_list::SelectionList<'a, T, Message, Renderer<Backend, Theme>>;
