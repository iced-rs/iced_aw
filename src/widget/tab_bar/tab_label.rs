//! A [`TabLabel`] showing an icon and/or a text on a tab.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*

/// A [`TabLabel`] showing an icon and/or a text on a tab
/// on a [`TabBar`](super::TabBar).
#[allow(missing_debug_implementations)]
#[derive(Clone, Hash)]
pub enum TabLabel {
    /// A [`TabLabel`] showing only an icon on the tab.
    Icon(char),

    /// A [`TabLabel`] showing only a text on the tab.
    Text(String),

    /// A [`TabLabel`] showing an icon and a text on the tab.
    IconText(char, String),
    // TODO: Support any element as a label.
}

impl From<char> for TabLabel {
    fn from(value: char) -> Self {
        Self::Icon(value)
    }
}

impl From<&str> for TabLabel {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<String> for TabLabel {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<(char, &str)> for TabLabel {
    fn from(value: (char, &str)) -> Self {
        Self::IconText(value.0, value.1.to_owned())
    }
}

impl From<(char, String)> for TabLabel {
    fn from(value: (char, String)) -> Self {
        Self::IconText(value.0, value.1)
    }
}
