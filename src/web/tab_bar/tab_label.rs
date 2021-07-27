//! A [`TabLabel`](TabLabel) showing an icon and/or a text on a tab.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*

/// A [`TabLabel`](TabLabel) showing an icon and/or a text on a tab
/// on a [`TabBar`](super::TabBar).
#[allow(missing_debug_implementations)]
#[derive(Clone, Hash)]
pub enum TabLabel {
    /// A [`TabLabel`](TabLabel) showing only a text on the tab.
    Text(String),
}
