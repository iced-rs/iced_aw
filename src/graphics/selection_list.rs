//! Display a list of selectable values.
use iced_graphics::Renderer;

pub use crate::native::selection_list::{self, list, State};
pub use crate::style::selection_list::{Style, StyleSheet};

/// A widget allowing the selection of a single value from a list of options.
pub type SelectionList<'a, T, Message, Backend> =
    selection_list::SelectionList<'a, T, Message, Renderer<Backend>>;
