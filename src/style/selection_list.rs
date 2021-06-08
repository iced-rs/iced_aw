//! Selection List Redirects a Menu Style.
use iced_style::menu;

/// A set of rules that dictate the style of a container.
pub trait StyleSheet {
    /// The normal appearance of a [`ListMenu`]
    fn menu(&self) -> menu::Style;
}

/// Default holder
#[derive(Clone, Copy, Debug)]
struct Default;

impl StyleSheet for Default {
    fn menu(&self) -> menu::Style {
        menu::Style::default()
    }
}
#[allow(clippy::use_self)]
impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

#[allow(clippy::use_self)]
impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
