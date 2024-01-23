//! widget Helpers.
//!
//!

#[allow(unused_imports)]
use iced_widget::core::{self, Color, Element};
#[allow(unused_imports)]
use std::{borrow::Cow, fmt::Display, hash::Hash};

/// Creates a [`Grid`] with the given [`GridRow`]s.
///
/// [`Grid`]: crate::Grid
/// [`GridRow`]: crate::GridRow
#[cfg(feature = "grid")]
#[macro_export]
macro_rules! grid {
    () => (
        $crate::Grid::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Grid::with_rows(vec![$($x),+])
    );
}

/// Creates a [`GridRow`] with the given widgets.
///
/// [`GridRow`]: crate::GridRow
#[cfg(feature = "grid")]
#[macro_export]
macro_rules! grid_row {
    () => (
        $crate::GridRow::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::GridRow::with_elements(vec![$(iced::Element::from($x)),+])
    );
}

/// Creates a horizontal [`Wrap`] with the given children.
///
/// [`Wrap`]: crate::Wrap
#[cfg(feature = "wrap")]
#[macro_export]
macro_rules! wrap_horizontal {
    () => (
        $crate::Wrap::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Wrap::with_elements(vec![$($crate::Element::from($x)),+])
    );
}

/// Creates a vertical [`Wrap`] with the given children.
///
/// [`Wrap`]: crate::Wrap
#[cfg(feature = "wrap")]
#[macro_export]
macro_rules! wrap_vertical {
    () => (
        $crate::Wrap::new_vertical()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Wrap::with_elements_vertical(vec![$($crate::Element::from($x)),+])
    );
}

/// Creates a [`MenuTree`] with the given children.
///
/// [`MenuTree`]: crate::MenuTree
#[cfg(feature = "menu")]
#[macro_export]
macro_rules! menu_tree {
    ($x:expr) => (
        $crate::menu::menu_tree::MenuTree::new($x)
    );
    ($x:expr, $($y:expr),+ $(,)?) => (
        $crate::menu::menu_tree::MenuTree::with_children($x, vec![$($y),+])
    );
}

/// Creates a [`MenuBar`] with the given children.
///
/// [`MenuBar`]: crate::MenuBar
#[cfg(feature = "menu")]
#[macro_export]
macro_rules! menu_bar {
    () => (
        $crate::menu::menu_bar::MenuBar::new(vec![])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::menu::menu_bar::MenuBar::new(vec![$($x),+])
    );
}

#[cfg(feature = "menu")]
/// Shortcut helper to create a [`MenuBar`] Widget.
///
/// [`MenuBar`]: crate::MenuBar
#[must_use]
pub fn menu_bar<Message, Renderer>(
    menu_roots: Vec<crate::menu::menu_tree::MenuTree<Message, Renderer>>,
) -> crate::menu::menu_bar::MenuBar<Message, Renderer>
where
    Renderer: core::Renderer,
    Renderer::Theme: crate::style::menu_bar::StyleSheet,
{
    crate::menu::menu_bar::MenuBar::new(menu_roots)
}

#[cfg(feature = "menu")]
/// Shortcut helper to create a [`MenuTree`] Widget.
///
/// [`MenuTree`]: crate::MenuTree
#[must_use]
pub fn menu_tree<'a, Message, Renderer>(
    item: impl Into<Element<'a, Message, Renderer>>,
    children: Vec<impl Into<crate::menu::menu_tree::MenuTree<'a, Message, Renderer>>>,
) -> crate::menu::menu_tree::MenuTree<'a, Message, Renderer>
where
    Renderer: core::Renderer,
{
    crate::menu::menu_tree::MenuTree::with_children(item, children)
}

#[cfg(feature = "badge")]
/// Shortcut helper to create a [`Badge`] Widget.
///
/// [`Badge`]: crate::Badge
pub fn badge<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> crate::Badge<'a, Message, Theme, Renderer>
where
    Renderer: core::Renderer,
    Theme: crate::style::badge::StyleSheet,
{
    crate::Badge::new(content)
}

#[cfg(feature = "card")]
/// Shortcut helper to create a [`Card`] Widget.
///
/// [`Card`]: crate::Card
pub fn card<'a, Message, Theme, Renderer>(
    head: impl Into<Element<'a, Message, Theme, Renderer>>,
    body: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> crate::Card<'a, Message, Theme, Renderer>
where
    Renderer: core::Renderer,
    Theme: crate::style::card::StyleSheet,
{
    crate::Card::new(head, body)
}

#[cfg(feature = "color_picker")]
/// Shortcut helper to create a [`ColorPicker`] Widget.
///
/// [`ColorPicker`]: crate::ColorPicker
pub fn color_picker<'a, Message, Theme, F>(
    show_picker: bool,
    color: Color,
    underlay: impl Into<Element<'a, Message, Theme, iced_widget::renderer::Renderer>>,
    on_cancel: Message,
    on_submit: F,
) -> crate::ColorPicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::color_picker::StyleSheet
        + iced_widget::button::StyleSheet
        + iced_widget::text::StyleSheet,
    F: 'static + Fn(Color) -> Message,
{
    crate::ColorPicker::new(show_picker, color, underlay, on_cancel, on_submit)
}

#[cfg(feature = "date_picker")]
/// Shortcut helper to create a [`DatePicker`] Widget.
///
/// [`DatePicker`]: crate::DatePicker
pub fn date_picker<'a, Message, Theme, F>(
    show_picker: bool,
    date: impl Into<crate::core::date::Date>,
    underlay: impl Into<Element<'a, Message, Theme, iced_widget::renderer::Renderer>>,
    on_cancel: Message,
    on_submit: F,
) -> crate::DatePicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::date_picker::StyleSheet
        + iced_widget::button::StyleSheet
        + iced_widget::text::StyleSheet
        + iced_widget::container::StyleSheet,
    F: 'static + Fn(crate::core::date::Date) -> Message,
{
    crate::DatePicker::new(show_picker, date, underlay, on_cancel, on_submit)
}

#[cfg(feature = "time_picker")]
/// Shortcut helper to create a [`DatePicker`] Widget.
///
/// [`DatePicker`]: crate::DatePicker
pub fn time_picker<'a, Message, Theme, U, F>(
    show_picker: bool,
    time: impl Into<crate::core::time::Time>,
    underlay: U,
    on_cancel: Message,
    on_submit: F,
) -> crate::TimePicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::time_picker::StyleSheet
        + iced_widget::button::StyleSheet
        + iced_widget::text::StyleSheet,
    U: Into<Element<'a, Message, Theme, iced_widget::renderer::Renderer>>,
    F: 'static + Fn(crate::core::time::Time) -> Message,
{
    crate::TimePicker::new(show_picker, time, underlay, on_cancel, on_submit)
}

#[cfg(feature = "floating_element")]
/// Shortcut helper to create a [`FloatingElement`] Widget.
///
/// [`FloatingElement`]: crate::FloatingElement
pub fn floating_element<'a, Message, Theme, Renderer>(
    underlay: impl Into<Element<'a, Message, Theme, Renderer>>,
    element: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> crate::FloatingElement<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: core::Renderer,
{
    crate::FloatingElement::new(underlay, element)
}

#[cfg(feature = "grid")]
/// Shortcut helper to create a [`Grid`] Widget.
///
/// [`Grid`]: crate::grid::Grid
#[must_use]
pub fn grid<Message, Renderer>(
    rows: Vec<crate::GridRow<'_, Message, Renderer>>,
) -> crate::Grid<'_, Message, Renderer>
where
    Renderer: core::Renderer,
{
    crate::Grid::with_rows(rows)
}

#[cfg(feature = "grid")]
/// Shortcut helper to create a [`GridRow`] for the [`Grid`] Widget.
///
/// [`GridRow`]: crate::GridRow
/// [`Grid`]: crate::Grid
#[must_use]
pub fn grid_row<'a, Message, Theme, Renderer>(
    elements: Vec<impl Into<Element<'a, Message, Theme, Renderer>>>,
) -> crate::GridRow<'a, Message, Theme, Renderer>
where
    Renderer: core::Renderer,
{
    crate::GridRow::with_elements(elements)
}

#[cfg(feature = "wrap")]
/// Shortcut helper to create a horizontal [`Wrap`] Widget.
///
/// [`Wrap`]: crate::Wrap
#[must_use]
pub fn wrap_horizontal<Message, Theme, Renderer>(
    children: Vec<Element<Message, Theme, Renderer>>,
) -> crate::Wrap<Message, crate::direction::Horizontal, Theme, Renderer>
where
    Renderer: core::Renderer,
{
    crate::Wrap::with_elements(children)
}

#[cfg(feature = "wrap")]
/// Shortcut helper to create a vertical [`Wrap`] Widget.
///
/// [`Wrap`]: crate::Wrap
#[must_use]
pub fn wrap_vertical<Message, Theme, Renderer>(
    children: Vec<Element<Message, Theme, Renderer>>,
) -> crate::Wrap<Message, crate::direction::Vertical, Theme, Renderer>
where
    Renderer: core::Renderer,
{
    crate::Wrap::with_elements_vertical(children)
}

#[cfg(feature = "modal")]
/// Shortcut helper to create a [`Modal`] Widget.
///
/// [`Modal`]: crate::Modal
#[must_use]
pub fn modal<'a, Message, Theme, Renderer>(
    underlay: impl Into<Element<'a, Message, Theme, Renderer>>,
    overlay: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
) -> crate::Modal<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
    Theme: crate::style::modal::StyleSheet,
{
    crate::Modal::new(underlay, overlay)
}

#[cfg(feature = "number_input")]
/// Shortcut helper to create a [`NumberInput`] Widget.
///
/// [`NumberInput`]: crate::NumberInput
#[must_use]
pub fn number_input<'a, T, Message, Theme, Renderer, F>(
    value: T,
    max: T,
    on_changed: F,
) -> crate::NumberInput<'a, T, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: core::text::Renderer<Font = core::Font>,
    Theme: crate::style::number_input::StyleSheet
        + iced_widget::text_input::StyleSheet
        + iced_widget::container::StyleSheet
        + iced_widget::text::StyleSheet,
    F: 'static + Fn(T) -> Message + Copy,
    T: 'static
        + num_traits::Num
        + num_traits::NumAssignOps
        + PartialOrd
        + std::fmt::Display
        + std::str::FromStr
        + Copy,
{
    crate::NumberInput::new(value, max, on_changed)
}

#[cfg(feature = "selection_list")]
/// Shortcut helper to create a [`SelectionList`] Widget.
///
/// [`SelectionList`]: crate::SelectionList
#[must_use]
pub fn selection_list_with<'a, T, Message, Theme, Renderer>(
    options: &'a [T],
    on_selected: impl Fn(usize, T) -> Message + 'static,
    text_size: f32,
    padding: f32,
    style: <Theme as crate::style::selection_list::StyleSheet>::Style,
    selected: Option<usize>,
    font: iced_widget::runtime::Font,
) -> crate::SelectionList<'a, T, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer + core::text::Renderer<Font = core::Font>,
    Theme: 'a
        + crate::style::selection_list::StyleSheet
        + iced_widget::container::StyleSheet
        + iced_widget::scrollable::StyleSheet,
    T: Clone + Display + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
{
    crate::SelectionList::new_with(
        options,
        on_selected,
        text_size,
        padding,
        style,
        selected,
        font,
    )
}

#[cfg(feature = "selection_list")]
/// Shortcut helper to create a [`SelectionList`] Widget.
///
/// [`SelectionList`]: crate::SelectionList
#[must_use]
pub fn selection_list<'a, T, Message, Theme, Renderer>(
    options: &'a [T],
    on_selected: impl Fn(usize, T) -> Message + 'static,
) -> crate::SelectionList<'a, T, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer + core::text::Renderer<Font = core::Font>,
    Theme: 'a
        + crate::style::selection_list::StyleSheet
        + iced_widget::container::StyleSheet
        + iced_widget::scrollable::StyleSheet,
    T: Clone + Display + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
{
    crate::SelectionList::new(options, on_selected)
}
