//! widget Helpers.
//!
//!

use iced_native::Element;
use iced_style::Color;

/// Creates a [`Grid`] with the given children.
///
/// [`Grid`]: iced_aw::Grid
#[macro_export]
macro_rules! grid {
    () => (
        $crate::Grid::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Grid::with_children(vec![$($crate::Element::from($x)),+])
    );
}

/// Creates a [`MenuTree`] with the given children.
///
/// [`MenuTree`]: iced_aw::MenuTree
#[macro_export]
macro_rules! menu_tree {
    ($x:expr) => (
        $crate::MenuTree::new($x)
    );
    ($x:expr, $($y:expr),+ $(,)?) => (
        $crate::MenuTree::with_children($x, vec![$($y),+])
    );
}

/// Creates a [`MenuBar`] with the given children.
///
/// [`MenuBar`]: iced_aw::MenuBar
#[macro_export]
macro_rules! menu_bar {
    () => (
        $crate::MenuBar::new(vec![])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::MenuBar::new(vec![$($x),+])
    );
}

#[cfg(feature = "menu")]
/// Shortcut helper to create a Card Widget.
#[must_use]
pub fn menu_bar<Message, Renderer>(
    menu_roots: Vec<crate::MenuTree<Message, Renderer>>,
) -> crate::MenuBar<Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: crate::style::menu_bar::StyleSheet,
{
    crate::MenuBar::new(menu_roots)
}

#[cfg(feature = "menu")]
/// Shortcut helper to create a Card Widget.
#[must_use]
pub fn menu_tree<'a, Message, Renderer>(
    item: impl Into<Element<'a, Message, Renderer>>,
    children: Vec<impl Into<crate::MenuTree<'a, Message, Renderer>>>,
) -> crate::MenuTree<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    crate::MenuTree::with_children(item, children)
}

#[cfg(feature = "badge")]
/// Shortcut helper to create a Badge Widget.
pub fn badge<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Renderer>>,
) -> crate::Badge<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: crate::style::badge::StyleSheet,
{
    crate::Badge::new(content)
}

#[cfg(feature = "card")]
/// Shortcut helper to create a Card Widget.
pub fn card<'a, Message, Renderer>(
    head: impl Into<Element<'a, Message, Renderer>>,
    body: impl Into<Element<'a, Message, Renderer>>,
) -> crate::Card<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: crate::style::card::StyleSheet,
{
    crate::Card::new(head, body)
}

#[cfg(feature = "color_picker")]
/// Shortcut helper to create a ``ColorPicker`` Widget.
pub fn color_picker<'a, Message, B, Theme, F>(
    show_picker: bool,
    color: Color,
    underlay: impl Into<Element<'a, Message, iced_graphics::Renderer<B, Theme>>>,
    on_cancel: Message,
    on_submit: F,
) -> crate::ColorPicker<'a, Message, B, Theme>
where
    Message: 'a + Clone,
    B: 'a + iced_graphics::Backend + iced_graphics::backend::Text,
    Theme: 'a
        + crate::style::color_picker::StyleSheet
        + iced_style::button::StyleSheet
        + iced_style::text::StyleSheet,
    F: 'static + Fn(Color) -> Message,
{
    crate::ColorPicker::new(show_picker, color, underlay, on_cancel, on_submit)
}

#[cfg(feature = "date_picker")]
/// Shortcut helper to create a ``ColorPicker`` Widget.
pub fn date_picker<'a, Message, B, Theme, F>(
    show_picker: bool,
    date: impl Into<crate::core::date::Date>,
    underlay: impl Into<Element<'a, Message, iced_graphics::Renderer<B, Theme>>>,
    on_cancel: Message,
    on_submit: F,
) -> crate::DatePicker<'a, Message, B, Theme>
where
    Message: 'a + Clone,
    B: 'a + iced_graphics::Backend + iced_graphics::backend::Text,
    Theme: 'a
        + crate::style::date_picker::StyleSheet
        + iced_style::button::StyleSheet
        + iced_style::text::StyleSheet
        + iced_style::container::StyleSheet,
    F: 'static + Fn(crate::core::date::Date) -> Message,
{
    crate::DatePicker::new(show_picker, date, underlay, on_cancel, on_submit)
}

#[cfg(feature = "floating_element")]
/// Shortcut helper to create a Card Widget.
pub fn floating_element<'a, Message, Renderer, B>(
    underlay: impl Into<Element<'a, Message, Renderer>>,
    element: B,
) -> crate::FloatingElement<'a, B, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced_native::Renderer,
    B: Fn() -> Element<'a, Message, Renderer>,
{
    crate::FloatingElement::new(underlay, element)
}

#[cfg(feature = "grid")]
/// Shortcut helper to create a Card Widget.
#[must_use]
pub fn grid<Message, Renderer>(
    children: Vec<Element<Message, Renderer>>,
) -> crate::Grid<Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    crate::Grid::with_children(children)
}

#[cfg(feature = "icon_text")]
/// Shortcut helper to create a Card Widget.
#[must_use]
pub fn icon_text<Renderer>(label: impl Into<String>) -> crate::IconText<Renderer>
where
    Renderer: iced_native::text::Renderer,
{
    crate::IconText::new(label)
}
