//! A [`TabBarPosition`](TabBarPosition) for defining the position of a
//! [`TabBar`](crate::native::tab_bar::TabBar).
//!
//! *This API requires the following crate features to be activated: tabs*

/// A [`TabBarPosition`](TabBarPosition) for defining the position of a
/// [`TabBar`](crate::native::tab_bar::TabBar).
#[derive(Clone, Hash)]
#[allow(missing_debug_implementations)]
pub enum TabBarPosition {
    /// A [`TabBarPosition`] for placing the
    /// [`TabBar`](crate::native::tab_bar::TabBar) on top of its content.
    Top,

    /// A [`TabBarPosition`] for placing the
    /// [`TabBar`](crate::native::tab_bar::TabBar) on bottom of its content.
    Bottom,
}
