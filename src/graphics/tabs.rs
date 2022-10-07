//! Displays a [`Tabs`](Tabs) widget to select the content to be displayed.
//!
//! This is a wrapper around the [`TabBar`](super::tab_bar::TabBar) widget.
//! Unlike the [`TabBar`](super::tab_bar::TabBar) widget it will also handle
//! the content of the tabs.
//!
//! *This API requires the following crate features to be activated: tabs*
use crate::native::tabs;
pub use crate::style::tab_bar::{StyleSheet, TabBarStyles};
use iced_graphics::Renderer;
pub use tabs::tab_bar_position::TabBarPosition;

/// A [`Tabs`](Tabs) widget for showing a [`TabBar`](super::tab_bar::TabBar)
/// along with the tab's content.
///
/// This is an alias of an `iced_native` Tabs widget with an
/// `iced_wgpu::Renderer`.
pub type Tabs<'a, Message, Backend, Theme> = tabs::Tabs<'a, Message, Renderer<Backend, Theme>>;
