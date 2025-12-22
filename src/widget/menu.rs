#![allow(clippy::doc_markdown)]

//! A [`MenuBar`] widget for displaying menu trees created with [`Item`]s and [`Menu`]s
//!
//! *This API requires the following crate features to be activated: `menu`*
//!
//! ## Example 1
//!
//! ```ignore
//! use iced_widget::button;
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
//! Alterwidgetly you can use the helper macros
//!
//! ## Example 2
//! ```ignore
//! use iced_widget::button;
//! use iced_aw::menu::{Menu, Item, MenuBar};
//! use iced_aw::{menu_bar, menu_items};
//!
//! let menu_template = |items| Menu::new(items).max_width(180.0).offset(6.0);
//!
//! let menu_bar: MenuBar<'_, (), iced_widget::Theme, iced::Renderer> = menu_bar!(
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
//! );
//! ```
//!
//! Here a menu_template function/closure is used in example 2,
//! usually some properties are sync across all menus while others are not,
//! using template functions can reduce the repeated code.
//! If you find writing menu_template(menu_items!()) too cumbersome,
//! there's a menu! macro you can use to create template macros
//!
//! ## Example 3
//!
//! ```ignore
//! use iced_widget::button;
//! use iced_aw::{menu, Menu};
//!
//! macro_rules! menu_template {
//!     ($($x:tt)+) => {
//!         menu!($($x)+).max_width(180.0).offset(6.0)
//!     };
//! }
//!
//! // then you can just write
//! let m: Menu<'_, (), iced_widget::Theme, iced::Renderer> = menu_template!(
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
//! Technically you can create menu template functions with the menu! macro,
//! but turns out closures can't infer the proper generic types in this case,
//! and creating a function for it you have to write a bunch of generic annotations
//!
//! ## Example 4
//!
//! ```ignore
//! use iced_aw::{menu, Menu};
//! use iced_widget::button;
//!
//! fn menu_template<'a, Message, Theme, Renderer>(
//! menu: Menu<'a, Message, Theme, Renderer>
//! ) -> Menu<'a, Message, Theme, Renderer>
//! where
//!     Theme: iced_aw::menu::Catalog,
//!     Renderer: iced_core::Renderer,
//! {
//!     menu.max_width(180.0).offset(6.0)
//! }
//!
//! let m: Menu<'_, (), iced_widget::Theme, iced::Renderer> = menu_template(menu!(
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

pub use crate::style::menu_bar::{Catalog, Style, primary};
pub use common::{DrawPath, ScrollSpeed};
pub use menu_bar::MenuBar;
pub use menu_tree::{Item, Menu};
