//! Displays a [`Tabs`](Tabs) widget to select the content to be displayed.
//!
//! This is a wrapper around the [`TabBar`](super::tab_bar::TabBar) widget.
//! Unlike the [`TabBar`](super::tab_bar::TabBar) widget it will also handle
//! the content of the tabs.
//!
//! *This API requires the following crate features to be activated: tabs*
use std::hash::Hash;

use iced_native::{
    column, event, row, text, Clipboard, Element, Event, Font, Layout, Length, Point, Rectangle,
    Row, Size, Widget,
};

use crate::{
    core::renderer::DrawEnvironment,
    native::{TabBar, TabLabel},
};

pub mod tab_bar_position;
pub use tab_bar_position::TabBarPosition;

/// A [`Tabs`](Tabs) widget for showing a [`TabBar`](super::tab_bar::TabBar)
/// along with the tab's content.
///
/// # Example
/// ```
/// # use iced_aw::{TabLabel};
/// # use iced_native::{renderer::Null, Text};
/// #
/// # pub type Tabs<'a, Message> = iced_aw::native::Tabs<'a, Message, Null>;
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
pub struct Tabs<'a, Message, Renderer: self::Renderer> {
    /// The [`TabBar`](crate::native::TabBar) of the [`Tabs`](Tabs).
    tab_bar: TabBar<Message, Renderer>,
    /// The vector containing the content of the tabs.
    tabs: Vec<Element<'a, Message, Renderer>>,
    /// The position of the [`TabBar`](crate::native::TabBar).
    tab_bar_position: TabBarPosition,
    /// the width of the [`Tabs`](Tabs).
    width: Length,
    /// The height of the [`Tabs`](Tabs).
    height: Length,
}

impl<'a, Message, Renderer> Tabs<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
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
        tabs: Vec<(TabLabel, Element<'a, Message, Renderer>)>,
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

    /// Sets the icon size of the [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    pub fn icon_size(mut self, icon_size: u16) -> Self {
        self.tab_bar = self.tab_bar.icon_size(icon_size);
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

    /// Sets the font of the icons of the
    /// [`TabLabel`](super::tab_bar::TabLabel)s of the
    /// [`TabBar`](super::tab_bar::TabBar).
    pub fn icon_font(mut self, icon_font: Font) -> Self {
        self.tab_bar = self.tab_bar.icon_font(icon_font);
        self
    }

    /// Sets the font of the text of the
    /// [`TabLabel`](super::tab_bar::TabLabel)s of the
    /// [`TabBar`](super::tab_bar::TabBar).
    pub fn text_font(mut self, text_font: Font) -> Self {
        self.tab_bar = self.tab_bar.text_font(text_font);
        self
    }

    /// Sets the style of the [`TabBar`](super::tab_bar::TabBar).
    pub fn tab_bar_style<T>(mut self, style: T) -> Self
    where
        T: Into<<Renderer as crate::native::tab_bar::Renderer>::Style>,
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
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.tab_bar = self.tab_bar.push(tab_label);
        self.tabs.push(element.into());
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Tabs<'a, Message, Renderer>
where
    Renderer: self::Renderer + column::Renderer + text::Renderer + row::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let tab_bar_limits = limits
            .clone()
            .width(self.width)
            .height(self.tab_bar.get_height());

        let mut tab_bar_node = self.tab_bar.layout(renderer, &tab_bar_limits);

        let tab_content_limits = limits
            .clone()
            .shrink(Size::new(0.0, tab_bar_node.size().height))
            .width(self.width)
            .height(self.height);

        let mut tab_content_node = self.tabs.get(self.tab_bar.get_active_tab()).map_or_else(
            || {
                Row::<Message, Renderer>::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .layout(renderer, &tab_content_limits)
            },
            |element| element.layout(renderer, &tab_content_limits),
        );

        tab_bar_node.move_to(Point::new(
            tab_bar_node.bounds().x,
            tab_bar_node.bounds().y
                + match self.tab_bar_position {
                    TabBarPosition::Top => 0.0,
                    TabBarPosition::Bottom => tab_content_node.bounds().height,
                },
        ));

        tab_content_node.move_to(Point::new(
            tab_content_node.bounds().x,
            tab_content_node.bounds().y
                + match self.tab_bar_position {
                    TabBarPosition::Top => tab_bar_node.bounds().height,
                    TabBarPosition::Bottom => 0.0,
                },
        ));

        iced_native::layout::Node::with_children(
            Size::new(
                tab_content_node.size().width,
                tab_bar_node.size().height + tab_content_node.size().height,
            ),
            match self.tab_bar_position {
                TabBarPosition::Top => vec![tab_bar_node, tab_content_node],
                TabBarPosition::Bottom => vec![tab_content_node, tab_bar_node],
            },
        )
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        let mut children = layout.children();
        let (tab_bar_layout, tab_content_layout) = match self.tab_bar_position {
            TabBarPosition::Top => {
                let tab_bar_layout = children
                    .next()
                    .expect("Native: Layout should have a TabBar layout at top position");
                let tab_content_layout = children
                    .next()
                    .expect("Native: Layout should have a tab content layout at top position");
                (tab_bar_layout, tab_content_layout)
            }
            TabBarPosition::Bottom => {
                let tab_content_layout = children
                    .next()
                    .expect("Native: Layout should have a tab content layout at bottom position");
                let tab_bar_layout = children
                    .next()
                    .expect("Native: Layout should have a TabBar layout at bottom position");
                (tab_bar_layout, tab_content_layout)
            }
        };

        let status_tab_bar = self.tab_bar.on_event(
            event.clone(),
            tab_bar_layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        );

        let status_element = self.tabs.get_mut(self.tab_bar.get_active_tab()).map_or(
            event::Status::Ignored,
            |element| {
                element.on_event(
                    event,
                    tab_content_layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    messages,
                )
            },
        );

        status_tab_bar.merge(status_element)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        let mut children = layout.children();
        let tab_bar_layout = match self.tab_bar_position {
            TabBarPosition::Top => children
                .next()
                .expect("Native: There should be a TabBar at the top position"),
            TabBarPosition::Bottom => children
                .last()
                .expect("Native: There should be a TabBar at the bottom position"),
        };

        let tab_bar = self.tab_bar.draw(
            renderer,
            defaults,
            tab_bar_layout,
            cursor_position,
            viewport,
        );

        self::Renderer::draw(
            renderer,
            DrawEnvironment {
                defaults,
                layout,
                cursor_position,
                style_sheet: &(),
                viewport: Some(viewport),
                focus: (),
            },
            self.tab_bar.get_active_tab(),
            tab_bar,
            &self.tabs,
            &self.tab_bar_position,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.tab_bar.hash_layout(state);
        self.tabs.iter().for_each(|tab| tab.hash_layout(state));
        self.tab_bar_position.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

/// The renderer of a [`Tabs`](Tabs) widget.
///
/// Your renderer will need to implement this trait before being able to
/// use a [`Tabs`](Tabs) widget in your user interface.
pub trait Renderer: iced_native::Renderer + crate::native::tab_bar::Renderer {
    /// Draws a [`Tabs`](Tabs) widget.
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, (), ()>,
        active_tab: usize,
        tab_bar: Self::Output,
        tabs: &[Element<'_, Message, Self>],
        tab_bar_position: &TabBarPosition,
    ) -> Self::Output;
}

#[cfg(debug_assertions)]
impl Renderer for iced_native::renderer::Null {
    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<'_, Self::Defaults, Self::Style, ()>,
        _active_tab: usize,
        _tab_bar: Self::Output,
        _tabs: &[Element<'_, Message, Self>],
        _tab_bar_position: &TabBarPosition,
    ) -> Self::Output {
    }
}

impl<'a, Message, Renderer> From<Tabs<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + column::Renderer + text::Renderer + row::Renderer,
    Message: 'a,
{
    fn from(tabs: Tabs<'a, Message, Renderer>) -> Self {
        Element::new(tabs)
    }
}
