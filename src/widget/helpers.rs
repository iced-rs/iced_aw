//! widget Helpers.
//!
//!
#[cfg(feature = "selection_list")]
use crate::style::{Status, StyleFn};
#[allow(unused_imports)]
use iced_core::{self, Color, Element, Padding, renderer};

#[cfg(feature = "number_input")]
use num_traits::bounds::Bounded;
#[allow(unused_imports)]
use std::{borrow::Cow, fmt::Display, hash::Hash, ops::RangeBounds};

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

/// Creates a vec of menu [`Item`]s
///
/// [`Item`]: crate::menu::Item
///
/// Syntax:
/// ```ignore
/// menu_items!(
///     (widget),
///     (widget, menu),
///     expression,
///     ...
/// )
/// ```
#[cfg(feature = "menu")]
#[macro_export]
macro_rules! menu_items {
    // base case
    (@process [$($output:expr,)*]) => {
        vec![$($output),*]
    };

    // rule for (widget, menu)
    (@process [$($output:expr,)*] ($i:expr, $m:expr) , $($rest:tt)*) => {
        $crate::menu_items!(@process [$($output,)* $crate::menu::Item::with_menu($i, $m),] $($rest)*)
    };
    (@process [$($output:expr,)*] ($i:expr, $m:expr)) => {
        $crate::menu_items!(@process [$($output,)* $crate::menu::Item::with_menu($i, $m),])
    };

    // rule for (widget)
    (@process [$($output:expr,)*] ($i:expr) , $($rest:tt)*) => {
        $crate::menu_items!(@process [$($output,)* $crate::menu::Item::new($i),] $($rest)*)
    };
    (@process [$($output:expr,)*] ($i:expr)) => {
        $crate::menu_items!(@process [$($output,)* $crate::menu::Item::new($i),])
    };

    // rule for expr
    (@process [$($output:expr,)*] $item:expr , $($rest:tt)*) => {
        $crate::menu_items!(@process [$($output,)* $item,] $($rest)*)
    };
    (@process [$($output:expr,)*] $item:expr) => {
        $crate::menu_items!(@process [$($output,)* $item,])
    };

    // entry point
    ($($input:tt)*) => {
        $crate::menu_items!(@process [] $($input)*)
    };
}

/// Creates a [`Menu`] with the given items.
///
/// [`Menu`]: crate::menu::Menu
///
/// Syntax:
/// ```ignore
/// menu!(
///     (widget),
///     (widget, menu),
///     expression,
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
/// ```ignore
/// menu_bar!(
///     (widget),
///     (widget, menu),
///     expression,
///     ...
/// )
/// ```
#[cfg(feature = "menu")]
#[macro_export]
macro_rules! menu_bar {
    ($($input:tt)+) => (
        $crate::menu::MenuBar::new(menu_items!( $($input)+ ))
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
    Theme: crate::style::badge::Catalog,
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
    Theme: crate::style::card::Catalog,
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
    underlay: impl Into<Element<'a, Message, Theme, iced_widget::Renderer>>,
    on_cancel: Message,
    on_submit: F,
) -> crate::ColorPicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::color_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog,
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
    underlay: impl Into<Element<'a, Message, Theme, iced_widget::Renderer>>,
    on_cancel: Message,
    on_submit: F,
) -> crate::DatePicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog
        + iced_widget::container::Catalog,
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
        + crate::style::time_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog,
    U: Into<Element<'a, Message, Theme, iced_widget::Renderer>>,
    F: 'static + Fn(crate::core::time::Time) -> Message,
{
    crate::TimePicker::new(show_picker, time, underlay, on_cancel, on_submit)
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

#[cfg(feature = "number_input")]
/// Shortcut helper to create a [`NumberInput`] Widget.
///
/// [`NumberInput`]: crate::NumberInput
#[must_use]
pub fn number_input<'a, T, Message, Theme, Renderer, F>(
    value: &T,
    bounds: impl RangeBounds<T>,
    on_change: F,
) -> crate::NumberInput<'a, T, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_core::text::Renderer<Font = iced_core::Font>,
    Theme: crate::style::number_input::ExtendedCatalog,
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
    crate::NumberInput::new(value, bounds, on_change)
}

#[cfg(feature = "typed_input")]
/// Shortcut helper to create a [`TypedInput`] Widget.
///
/// [`TypedInput`]: crate::TypedInput
#[must_use]
pub fn typed_input<'a, T, Message, Theme, Renderer, F>(
    value: &T,
    on_change: F,
) -> crate::TypedInput<'a, T, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced_core::text::Renderer<Font = iced_core::Font>,
    Theme: iced_widget::text_input::Catalog,
    F: 'static + Fn(T) -> Message + Copy,
    T: 'static + std::fmt::Display + std::str::FromStr + Clone,
{
    crate::TypedInput::new("", value).on_input(on_change)
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
    padding: impl Into<Padding>,
    style: impl Fn(&Theme, Status) -> crate::style::selection_list::Style + 'a + Clone,
    selected: Option<usize>,
    font: iced_core::Font,
) -> crate::SelectionList<'a, T, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
    Theme: 'a
        + crate::style::selection_list::Catalog
        + iced_widget::container::Catalog
        + iced_widget::scrollable::Catalog,
    T: Clone + Display + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
    <Theme as crate::style::selection_list::Catalog>::Class<'a>:
        From<StyleFn<'a, Theme, crate::style::selection_list::Style>>,
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
    Renderer: 'a + renderer::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
    Theme: 'a
        + crate::style::selection_list::Catalog
        + iced_widget::container::Catalog
        + iced_widget::scrollable::Catalog,
    T: Clone + Display + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
{
    crate::SelectionList::new(options, on_selected)
}
