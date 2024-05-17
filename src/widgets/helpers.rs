//! widget Helpers.
//!
//!

#[allow(unused_imports)]
use iced::{self, advanced::renderer, Color, Element};
#[cfg(feature = "number_input")]
use num_traits::bounds::Bounded;
#[allow(unused_imports)]
use std::{borrow::Cow, fmt::Display, hash::Hash, ops::RangeBounds};

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

/// Creates a vec of menu items
///
/// [`Item`]: crate::menu::Item
///
/// Syntax:
/// ```
/// menu_items!(
///     (widget)
///     (widget)
///     (widget, menu)
///     (widget)
///     (widget, menu)
///     (widget)
///     ...
/// )
/// ```
#[cfg(feature = "menu")]
#[macro_export]
macro_rules! menu_items {
    ($($x:tt)+) => {
        {
            macro_rules! wrap_item {
                (($i:expr , $m:expr)) => (
                    $crate::menu::Item::with_menu($i, $m)
                );
                (($i:expr)) => (
                    $crate::menu::Item::new($i)
                );
            }

            vec![ $( wrap_item!($x) ),+ ]
        }
    }
}

/// Creates a [`Menu`] with the given items.
///
/// [`Menu`]: crate::menu::Menu
///
/// Syntax:
/// ```
/// menu!(
///     (widget)
///     (widget)
///     (widget, menu)
///     (widget)
///     (widget, menu)
///     (widget)
///     ...
/// )
/// ```
#[cfg(feature = "menu")]
#[macro_export]
macro_rules! menu {
    ($($x:tt)+) => {
        $crate::menu::Menu::new( $crate::menu_items!( $($x)+ ) )
    }
}

/// Creates a [`MenuBar`] with the given children.
///
/// [`MenuBar`]: crate::menu::MenuBar
///
/// Syntax:
/// ```
/// menu_bar!(
///     (widget, menu)
///     (widget, menu)
///     (widget, menu)
///     ...
/// )
/// ```
#[cfg(feature = "menu")]
#[macro_export]
macro_rules! menu_bar {
    ($(($x:expr, $m:expr))+) => (
        $crate::menu::MenuBar::new(vec![ $( Item::with_menu($x, $m) ),+ ])
    );
}

#[cfg(feature = "badge")]
/// Shortcut helper to create a [`Badge`] Widget.
///
/// [`Badge`]: crate::Badge
pub fn badge<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> crate::Badge<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
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
    Renderer: renderer::Renderer,
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
    underlay: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
    on_cancel: Message,
    on_submit: F,
) -> crate::ColorPicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::color_picker::StyleSheet
        + iced::widget::button::StyleSheet
        + iced::widget::text::StyleSheet,
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
    underlay: impl Into<Element<'a, Message, Theme, iced::Renderer>>,
    on_cancel: Message,
    on_submit: F,
) -> crate::DatePicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::date_picker::StyleSheet
        + iced::widget::button::StyleSheet
        + iced::widget::text::StyleSheet
        + iced::widget::container::StyleSheet,
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
        + iced::widget::button::StyleSheet
        + iced::widget::text::StyleSheet,
    U: Into<Element<'a, Message, Theme, iced::Renderer>>,
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
    Renderer: renderer::Renderer,
{
    crate::FloatingElement::new(underlay, element)
}

#[cfg(feature = "grid")]
/// Shortcut helper to create a [`Grid`] Widget.
///
/// [`Grid`]: crate::grid::Grid
#[must_use]
pub fn grid<Message, Theme, Renderer>(
    rows: Vec<crate::GridRow<Message, Theme, Renderer>>,
) -> crate::Grid<Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
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
    Renderer: renderer::Renderer,
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
    Renderer: renderer::Renderer,
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
    Renderer: renderer::Renderer,
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
    Renderer: renderer::Renderer,
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
    bounds: impl RangeBounds<T>,
    on_changed: F,
) -> crate::NumberInput<'a, T, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: crate::style::number_input::StyleSheet
        + iced::widget::text_input::StyleSheet
        + iced::widget::container::StyleSheet
        + iced::widget::text::StyleSheet,
    F: 'static + Fn(T) -> Message + Copy,
    T: 'static
        + num_traits::Num
        + num_traits::NumAssignOps
        + PartialOrd
        + std::fmt::Display
        + std::str::FromStr
        + Copy
        + Bounded,
{
    crate::NumberInput::new(value, bounds, on_changed)
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
    font: iced::Font,
) -> crate::SelectionList<'a, T, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a
        + crate::style::selection_list::StyleSheet
        + iced::widget::container::StyleSheet
        + iced::widget::scrollable::StyleSheet,
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
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: 'a
        + crate::style::selection_list::StyleSheet
        + iced::widget::container::StyleSheet
        + iced::widget::scrollable::StyleSheet,
    T: Clone + Display + Eq + Hash + Bounded,
    [T]: ToOwned<Owned = Vec<T>>,
{
    crate::SelectionList::new(options, on_selected)
}
