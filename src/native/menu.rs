//! A [`MenuBar`] widget for displaying [`MenuTree`]s
//!
//! *This API requires the following crate features to be activated: `menu`*
//! 
//! # Example
//! 
//! ```
//! use iced::widget::button;
//! use iced_aw::menu::{MenuTree, MenuBar};
//! 
//! let sub_2 = MenuTree::with_children(
//!     button("Sub Menu 2"),
//!     vec![
//!         MenuTree::new(button("item_1")),
//!         MenuTree::new(button("item_2")),
//!         MenuTree::new(button("item_3")),
//!     ]
//! );
//! 
//! let sub_1 = MenuTree::with_children(
//!     button("Sub Menu 1"),
//!     vec![
//!         MenuTree::new(button("item_1")),
//!         sub_2,
//!         MenuTree::new(button("item_2")),
//!         MenuTree::new(button("item_3")),
//!     ]
//! );
//! 
//! 
//! let root_1 = MenuTree::with_children(
//!     button("Menu 1"),
//!     vec![
//!         MenuTree::new(button("item_1")),
//!         MenuTree::new(button("item_2")),
//!         sub_1,
//!         MenuTree::new(button("item_3")),
//!     ]
//! );
//! 
//! let root_2 = MenuTree::with_children(
//!     button("Menu 2"),
//!     vec![
//!         MenuTree::new(button("item_1")),
//!         MenuTree::new(button("item_2")),
//!         MenuTree::new(button("item_3")),
//!     ]
//! );
//! 
//! let menu_bar = MenuBar::new(vec![root_1, root_2]);
//! 
//! ```
//! 

mod menu_tree;
mod menu_bar;
mod menu;
mod flex;

pub use menu_tree::MenuTree;
pub use menu_bar::MenuBar;
pub use menu::{PathHighlight, ItemWidth, ItemHeight};
pub use crate::style::menu_bar::{Appearance, StyleSheet};