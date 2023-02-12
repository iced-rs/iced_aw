//! A [`MenuBar`] widget for displaying [`MenuTree`]s
//!
//! //! *This API requires the following crate features to be activated: `menu`*

mod menu_tree;
mod menu_bar;
mod menu;
mod flex;

pub use menu_tree::MenuTree;
pub use menu_bar::MenuBar;
pub use menu::{PathHighlight, ItemWidth, ItemHeight};
pub use crate::style::menu_bar::{Appearance, StyleSheet};