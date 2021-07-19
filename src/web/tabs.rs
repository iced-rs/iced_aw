//! Displays a [`Tabs`](Tabs) widget to select the content to be displayed.
//!
//! This is a wrapper around the [`TabBar`](super::tab_bar::TabBar) widget.
//! Unlike the [`TabBar`](super::tab_bar::TabBar) widget it will also handle
//! the content of the tabs.
//!
//! *This API requires the following crate features to be activated: tabs*
use dodrio::bumpalo;
use iced_web::{css, Align, Background, Bus, Css, Element, Length, Widget};

pub mod tab_bar_position;
pub use tab_bar_position::TabBarPosition;

pub use crate::style::tab_bar::{Style, StyleSheet};
use crate::{TabBar, TabLabel};

/// A [`Tabs`](Tabs) widget for showing a [`TabBar`](super::tab_bar::TabBar)
/// along with the tab's content.
///
/// # Example
/// ```
/// # use iced_aw::{TabLabel};
/// # use iced_web::Text;
/// #[derive(Debug, Clone)]
/// enum Message {
///     TabSelected(usize),
/// }
///
/// let active_tab = 0;
///
/// let tabs = Tabs::new(
///     active_tab,
///     Message::TabSelected,
/// )
/// .push(TabLabel::Text(String::from("One")), Text::new(String::from("One")))
/// .push(TabLabel::Text(String::from("Two")), Text::new(String::from("Two")))
/// .push(TabLabel::Text(String::from("Three")), Text::new(String::from("Three")));
/// ```
///
#[allow(missing_debug_implementations)]
pub struct Tabs<'a, Message> {
    tab_bar: TabBar<Message>,
    tabs: Vec<Element<'a, Message>>,
    tab_bar_position: TabBarPosition,
    width: Length,
    height: Length,
}

impl<'a, Message> Tabs<'a, Message> {
    /// Creates a new [`Tabs`](Tabs) widget with the index of the selected tab
    /// and a specified message which will be send when a tab is selected by
    /// the user.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn new<F>(active_tab: usize, on_select: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        Self::with_tabs(active_tab, Vec::new(), on_select)
    }

    /// Similar to `new` but with a given Vector of the
    /// [`TabLabel`](super::tab_bar::TabLabel) along with the tab's content.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * a vector containing the [`TabLabel`](TabLabel)s along with the content
    ///         [`Element`](iced_native::Element)s of the [`Tabs`](Tabs).
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn with_tabs<F>(
        active_tab: usize,
        tabs: Vec<(TabLabel, Element<'a, Message>)>,
        on_select: F,
    ) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        let mut tab_labels = Vec::with_capacity(tabs.len());
        let mut elements = Vec::with_capacity(tabs.len());

        for (tab_label, element) in tabs {
            tab_labels.push(tab_label);
            elements.push(element);
        }

        Tabs {
            tab_bar: TabBar::width_tab_labels(active_tab, tab_labels, on_select),
            tabs: elements,
            tab_bar_position: TabBarPosition::Top,
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    /// Sets the message that will be produced when the close icon of a tab
    /// on the [`TabBar`](TabBar) is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the tabs.
    pub fn on_close<F>(mut self, on_close: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        self.tab_bar = self.tab_bar.on_close(on_close);
        self
    }

    /// Sets the width of the [`Tabs`](Tabs).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Tabs`](Tabs).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the width of the [`TabBar`](super::tab_bar::TabBar) of the
    /// [`Tabs`](Tabs).
    pub fn tab_bar_width(mut self, width: Length) -> Self {
        self.tab_bar = self.tab_bar.width(width);
        self
    }

    /// Sets the height of the [`TabBar`](super::tab_bar::TabBar) of the
    /// [`Tabs`](Tabs).
    pub fn tab_bar_height(mut self, height: Length) -> Self {
        self.tab_bar = self.tab_bar.height(height);
        self
    }

    /// Sets the maximum height of the [`TabBar`](super::tab_bar::TabBar) of the
    /// [`Tabs`](Tabs).
    pub fn tab_bar_max_height(mut self, max_height: u32) -> Self {
        self.tab_bar = self.tab_bar.max_height(max_height);
        self
    }

    /// Sets the text size of the [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.tab_bar = self.tab_bar.text_size(text_size);
        self
    }

    /// Sets the size of the close icon of the
    /// [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    pub fn close_size(mut self, close_size: u16) -> Self {
        self.tab_bar = self.tab_bar.close_size(close_size);
        self
    }

    /// Sets the padding of the tabs of the [`TabBar`](super::tab_bar::TabBar).
    pub fn tab_label_padding(mut self, padding: u16) -> Self {
        self.tab_bar = self.tab_bar.padding(padding);
        self
    }

    /// Sets the spacing between the tabs of the
    /// [`TabBar`](super::tab_bar::TabBar).
    pub fn tab_label_spacing(mut self, spacing: u16) -> Self {
        self.tab_bar = self.tab_bar.spacing(spacing);
        self
    }

    /// Sets the style of the [`TabBar`](super::tab_bar::TabBar).
    pub fn tab_bar_style<T>(mut self, style: T) -> Self
    where
        T: Into<Box<dyn StyleSheet>>,
    {
        self.tab_bar = self.tab_bar.style(style);
        self
    }

    /// Sets the [`TabBarPosition`](TabBarPosition) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    pub fn tab_bar_position(mut self, position: TabBarPosition) -> Self {
        self.tab_bar_position = position;
        self
    }

    /// Pushes a [`TabLabel`](super::tab_bar::TabLabel) along with the tabs
    /// content to the [`Tabs`](Tabs).
    pub fn push<E>(mut self, tab_label: TabLabel, element: E) -> Self
    where
        E: Into<Element<'a, Message>>,
    {
        self.tab_bar = self.tab_bar.push(tab_label);
        self.tabs.push(element.into());
        self
    }
}

impl<'a, Message> Widget<Message> for Tabs<'a, Message>
where
    Message: 'static + Clone,
{
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        bus: &Bus<Message>,
        style_sheet: &mut Css<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        let tab_bar = self.tab_bar.node(bump, bus, style_sheet);
        let content = self.tabs[self.tab_bar.get_active_tab()].node(bump, bus, style_sheet);

        let node = div(bump)
            .attr(
                "style",
                bumpalo::format!(
                    in bump,
                    "width: {}, height: {}",
                    css::length(self.width),
                    css::length(self.height)
                )
                .into_bump_str(),
            )
            .children(match self.tab_bar_position {
                TabBarPosition::Top => vec![tab_bar, content],
                TabBarPosition::Bottom => vec![content, tab_bar],
            });

        node.finish()
    }
}

impl<'a, Message> From<Tabs<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(tabs: Tabs<'a, Message>) -> Self {
        Element::new(tabs)
    }
}
