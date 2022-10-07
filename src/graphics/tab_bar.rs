//! Displays a [`TabBar`](TabBar) to select the content to be displayed.
//!
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](super::tabs) widget instead.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*
use crate::native::tab_bar;
pub use crate::style::tab_bar::{StyleSheet, TabBarStyles};
use iced_graphics::Renderer;
pub use tab_bar::tab_label::TabLabel;

/// A tab bar to show tabs.
///
/// This is an alias of an `iced_native` `TabBar` with an `iced_wgpu::Renderer`.
pub type TabBar<Message, Backend, Theme> = tab_bar::TabBar<Message, Renderer<Backend, Theme>>;
