//! A menu widget

mod menu_tree;
mod menu_bar;
mod flex;

pub use menu_tree::MenuTree;
pub use menu_bar::{MenuBar, PathHighlight};
pub use crate::style::menu_bar::{Appearance, StyleSheet};