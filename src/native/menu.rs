//! A [`MenuBar`] widget for displaying menu trees created with [`Item`]s and [`Menu`]s
//!
//! *This API requires the following crate features to be activated: `menu`*
//!
//! ## Example 1
//!
//! ```ignore
//! use iced::widget::button;
//! use iced_aw::menu::{Item, Menu, MenuBar};
//!
//! let sub_2 = Item::with_menu(
//!     button("Sub Menu 2"),
//!     Menu::new([
//!         Item::new(button("item_1")),
//!         Item::new(button("item_2")),
//!         Item::new(button("item_3")),
//!     ].into())
//! );
//!
//! let sub_1 = Item::with_menu(
//!     button("Sub Menu 1"),
//!     Menu::new([
//!         Item::new(button("item_1")),
//!         sub_2,
//!         Item::new(button("item_2")),
//!         Item::new(button("item_3")),
//!     ].into())
//! );
//!
//! let root_1 = Item::with_menu(
//!     button("Menu 1"),
//!     Menu::new([
//!         Item::new(button("item_1")),
//!         Item::new(button("item_2")),
//!         sub_1,
//!         Item::new(button("item_3")),
//!     ].into())
//! );
//!
//! let root_2 = Item::with_menu(
//!     button("Menu 2"),
//!     Menu::new([
//!         Item::new(button("item_1")),
//!         Item::new(button("item_2")),
//!         Item::new(button("item_3")),
//!     ].into())
//! );
//!
//! let menu_bar = MenuBar::new(vec![root_1, root_2]);
//!
//! ```
//!
//! Alternatively you can use the helper macros
//!
//! ## Example 2
//! ```
//! use iced::widget::button;
//! use iced_aw::menu::{Menu}
//! use iced_aw::{menu_bar, menu_items}
//!
//! let menu_template = |items| Menu::new(items).max_width(180.0).offset(6.0);
//!
//! let menu_bar = menu_bar!(
//!     (button("Menu 1"),menu_template(menu_items!(
//!         (button("item_1"))
//!         (button("item_2"))
//!         (button("Sub Menu 1"), menu_template(menu_items!(
//!             (button("item_1"))
//!             (button("Sub Menu 2"), menu_template(menu_items!(
//!                 (button("item_1"))
//!                 (button("item_2"))
//!                 (button("item_3"))
//!             )))
//!             (button("item_2"))
//!             (button("item_3"))
//!         )))
//!         (button("item_3"))
//!     )))
//!     (button("Menu 2"), menu_template(menu_items!(
//!         (button("item_1"))
//!         (button("item_2"))
//!         (button("item_3"))
//!     )))
//! )
//! ```
//!
//! Notice a menu_template function/closure is used in example 2. Usually some properties are sync across all menus while others are not,
//! template function is one way to do that. If you find writing menu_template(menu_items!()) cumbersome,
//! there is a menu! macro you can use to create template macros
//!
//! # Example 3
//!
//! ```
//! use iced_aw::{menu};
//!
//! macro_rules! menu_template {
//!     ($($x:tt)+) => {
//!         menu!($($x)+).max_width(180.0).offset(6.0)
//!     };
//! }
//!
//! // then you can just write
//! let m = menu_template!(
//!     (button("item_1"))
//!     (button("item_2"))
//!     (button("sub menu"), menu_template!(
//!         (button("item_1"))
//!         (button("item_2"))
//!     ))
//!     (button("item_3"))
//! );
//! ```
//!
//! Technically You can create menu template functions with the menu! macro,
//! but turns out closures can't infer the generic types,
//! and creating a function for it involves writing a ton of generic annotations
//! ```
//! fn menu_template<'a, Message, Theme, Renderer>(
//! menu: Menu<'a, Message, Theme, Renderer>
//! ) -> Menu<'a, Message, Theme, Renderer>
//! where
//!     Theme: iced_aw::menu::StyleSheet,
//!     Renderer: iced::advanced::Renderer,
//! {
//!     menu.max_width(180.0).offset(6.0)
//! }
//!
//! let m = menu_template(menu!(
//!     (button("item_1"))
//!     (button("item_2"))
//!     (button("item_3"))
//! ));
//! ```
//!
//! For a more detailed example please
//! take a look at the menu example in the iced_aw repo
//!

mod common;
mod flex;
mod menu_bar;
mod menu_bar_overlay;
mod menu_tree;

pub use crate::style::menu_bar::{Appearance, StyleSheet};
pub use menu_bar::MenuBar;
pub use menu_tree::{Item, Menu};
