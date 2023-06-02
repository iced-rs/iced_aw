//! Displays a [`Tabs`](Tabs) widget to select the content to be displayed.
//!
//! This is a wrapper around the [`TabBar`](super::tab_bar::TabBar) widget.
//! Unlike the [`TabBar`](super::tab_bar::TabBar) widget it will also handle
//! the content of the tabs.
//!
//! *This API requires the following crate features to be activated: tabs*
use iced_native::{
    event,
    layout::{Limits, Node},
    mouse, Clipboard, Event, Font, Layout, Length, Point, Rectangle, Shell, Size,
};
use iced_native::{
    widget::{Operation, Row, Tree},
    Element, Widget,
};

use crate::{native::TabBar, style::tab_bar::StyleSheet, TabLabel};

pub mod tab_bar_position;
pub use tab_bar_position::TabBarPosition;

/// A [`Tabs`](Tabs) widget for showing a [`TabBar`](super::tab_bar::TabBar)
/// along with the tab's content.
///
/// # Example
/// ```
/// # use iced_aw::{TabLabel};
/// # use iced_native::renderer::Null;
/// # use iced_native::widget::Text;
/// # use iced_aw::native::tabs;
/// #
/// # pub type Tabs<'a, Message> = tabs::Tabs<'a, Message, TabId, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     TabSelected(TabId),
/// }
///
/// #[derive(Debug, Clone)]
/// enum TabId {
///    One,
///    Two,
///    Three,
/// }
///
/// let active_tab = TabId::One;
///
/// let tabs = Tabs::new(
///     active_tab,
///     Message::TabSelected,
/// )
/// .push(TabId::One, TabLabel::Text(String::from("One")), Text::new(String::from("One")))
/// .push(TabId::Two, TabLabel::Text(String::from("Two")), Text::new(String::from("Two")))
/// .push(TabId::Three, TabLabel::Text(String::from("Three")), Text::new(String::from("Three")));
/// ```
///
#[allow(missing_debug_implementations)]
pub struct Tabs<'a, Message, TabId, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: StyleSheet,
    TabId: Eq + Clone
{
    /// The [`TabBar`](crate::native::TabBar) of the [`Tabs`](Tabs).
    tab_bar: TabBar<Message, TabId, Renderer>,
    /// The vector containing the content of the tabs.
    tabs: Vec<Element<'a, Message, Renderer>>,
    /// The vector containing the indices of the tabs.
    indices: Vec<TabId>,
    /// The position of the [`TabBar`](crate::native::TabBar).
    tab_bar_position: TabBarPosition,
    /// the width of the [`Tabs`](Tabs).
    width: Length,
    /// The height of the [`Tabs`](Tabs).
    height: Length,
}

impl<'a, Message, TabId, Renderer> Tabs<'a, Message, TabId, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: StyleSheet + iced_style::text::StyleSheet,
    TabId: Eq + Clone,
{
    /// Creates a new [`Tabs`](Tabs) widget with the index of the selected tab
    /// and a specified message which will be send when a tab is selected by
    /// the user.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn new<F>(active_tab: TabId, on_select: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
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
        active_tab: TabId,
        tabs: Vec<(TabId, TabLabel, Element<'a, Message, Renderer>)>,
        on_select: F,
    ) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        let mut tab_labels = Vec::with_capacity(tabs.len());
        let mut elements = Vec::with_capacity(tabs.len());
        let mut indices = Vec::with_capacity(tabs.len());

        for (id,tab_label,  element) in tabs {
            tab_labels.push((id.clone(), tab_label));
            indices.push(id);
            elements.push(element);
        }

        Tabs {
            tab_bar: TabBar::width_tab_labels(active_tab, tab_labels, on_select),
            tabs: elements,
            indices,
            tab_bar_position: TabBarPosition::Top,
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    /// Sets the message that will be produced when the close icon of a tab
    /// on the [`TabBar`](TabBar) is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the tabs.
    #[must_use]
    pub fn on_close<F>(mut self, on_close: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        self.tab_bar = self.tab_bar.on_close(on_close);
        self
    }

    /// Sets the width of the [`Tabs`](Tabs).
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Tabs`](Tabs).
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the width of the [`TabBar`](super::tab_bar::TabBar) of the
    /// [`Tabs`](Tabs).
    #[must_use]
    pub fn tab_bar_width(mut self, width: Length) -> Self {
        self.tab_bar = self.tab_bar.width(width);
        self
    }

    /// Sets the height of the [`TabBar`](super::tab_bar::TabBar) of the
    /// [`Tabs`](Tabs).
    #[must_use]
    pub fn tab_bar_height(mut self, height: Length) -> Self {
        self.tab_bar = self.tab_bar.height(height);
        self
    }

    /// Sets the maximum height of the [`TabBar`](super::tab_bar::TabBar) of the
    /// [`Tabs`](Tabs).
    #[must_use]
    pub fn tab_bar_max_height(mut self, max_height: f32) -> Self {
        self.tab_bar = self.tab_bar.max_height(max_height);
        self
    }

    /// Sets the icon size of the [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.tab_bar = self.tab_bar.icon_size(icon_size);
        self
    }

    /// Sets the text size of the [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.tab_bar = self.tab_bar.text_size(text_size);
        self
    }

    /// Sets the size of the close icon of the
    /// [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn close_size(mut self, close_size: f32) -> Self {
        self.tab_bar = self.tab_bar.close_size(close_size);
        self
    }

    /// Sets the padding of the tabs of the [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_label_padding(mut self, padding: f32) -> Self {
        self.tab_bar = self.tab_bar.padding(padding);
        self
    }

    /// Sets the spacing between the tabs of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_label_spacing(mut self, spacing: f32) -> Self {
        self.tab_bar = self.tab_bar.spacing(spacing);
        self
    }

    /// Sets the font of the icons of the
    /// [`TabLabel`](super::tab_bar::TabLabel)s of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn icon_font(mut self, icon_font: Font) -> Self {
        self.tab_bar = self.tab_bar.icon_font(icon_font);
        self
    }

    /// Sets the font of the text of the
    /// [`TabLabel`](super::tab_bar::TabLabel)s of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn text_font(mut self, text_font: Font) -> Self {
        self.tab_bar = self.tab_bar.text_font(text_font);
        self
    }

    /// Sets the style of the [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_bar_style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.tab_bar = self.tab_bar.style(style);
        self
    }

    /// Sets the [`TabBarPosition`](TabBarPosition) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_bar_position(mut self, position: TabBarPosition) -> Self {
        self.tab_bar_position = position;
        self
    }

    /// Pushes a [`TabLabel`](super::tab_bar::TabLabel) along with the tabs
    /// content to the [`Tabs`](Tabs).
    #[must_use]
    pub fn push<E>(mut self, id: TabId, tab_label: TabLabel, element: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.tab_bar = self.tab_bar.push(id.clone(), tab_label);
        self.tabs.push(element.into());
        self.indices.push(id);
        self
    }
}

impl<'a, Message, TabId, Renderer> Widget<Message, Renderer> for Tabs<'a, Message, TabId, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: StyleSheet + iced_style::text::StyleSheet,
    TabId: Eq + Clone,
{
    fn children(&self) -> Vec<Tree> {
        self.tabs.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.tabs);
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let tab_bar_limits = limits.width(self.width).height(self.tab_bar.get_height());

        let mut tab_bar_node = self.tab_bar.layout(renderer, &tab_bar_limits);

        let tab_content_limits = limits
            .clone()
            .shrink(Size::new(0.0, tab_bar_node.size().height))
            .width(self.width)
            .height(self.height);

        let mut tab_content_node = self.tabs.get(self.tab_bar.get_active_tab_idx()).map_or_else(
            || {
                Row::<Message, Renderer>::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .layout(renderer, &tab_content_limits)
            },
            |element| element.as_widget().layout(renderer, &tab_content_limits),
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

        Node::with_children(
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
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
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
            &mut Tree::empty(),
            event.clone(),
            tab_bar_layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        );
        let idx = self.tab_bar.get_active_tab_idx();
        let status_element = self.tabs.get_mut(idx).map_or(
            event::Status::Ignored,
            |element| {
                element.as_widget_mut().on_event(
                    &mut state.children[idx],
                    event,
                    tab_content_layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    shell,
                )
            },
        );

        status_tab_bar.merge(status_element)
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        // Tab bar
        let mut children = layout.children();
        let tab_bar_layout = match self.tab_bar_position {
            TabBarPosition::Top => children
                .next()
                .expect("Native: There should be a TabBar at the top position"),
            TabBarPosition::Bottom => children
                .last()
                .expect("Native: There should be a TabBar at the bottom position"),
        };

        let mut mouse_interaction = mouse::Interaction::default();
        let new_mouse_interaction = self.tab_bar.mouse_interaction(
            &Tree::empty(),
            tab_bar_layout,
            cursor_position,
            viewport,
            renderer,
        );

        if new_mouse_interaction > mouse_interaction {
            mouse_interaction = new_mouse_interaction;
        }

        // Tab content
        let mut children = layout.children();
        let tab_content_layout = match self.tab_bar_position {
            TabBarPosition::Top => children
                .last()
                .expect("Graphics: There should be a TabBar at the top position"),
            TabBarPosition::Bottom => children
                .next()
                .expect("Graphics: There should be a TabBar at the bottom position"),
        };
        let idx = self.tab_bar.get_active_tab_idx();
        if let Some(element) = self.tabs.get(idx) {
            let new_mouse_interaction = element.as_widget().mouse_interaction(
                &state.children[idx],
                tab_content_layout,
                cursor_position,
                viewport,
                renderer,
            );

            if new_mouse_interaction > mouse_interaction {
                mouse_interaction = new_mouse_interaction;
            }
        }

        mouse_interaction
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let tab_bar_layout = match self.tab_bar_position {
            TabBarPosition::Top => children
                .next()
                .expect("Native: There should be a TabBar at the top position"),
            TabBarPosition::Bottom => children
                .last()
                .expect("Native: There should be a TabBar at the bottom position"),
        };

        self.tab_bar.draw(
            &Tree::empty(),
            renderer,
            theme,
            style,
            tab_bar_layout,
            cursor_position,
            viewport,
        );

        let mut children = layout.children();

        let tab_content_layout = match self.tab_bar_position {
            TabBarPosition::Top => children
                .last()
                .expect("Graphics: There should be a TabBar at the top position"),
            TabBarPosition::Bottom => children
                .next()
                .expect("Graphics: There should be a TabBar at the bottom position"),
        };

        let idx = self.tab_bar.get_active_tab_idx();
        if let Some(element) = self.tabs.get(idx) {
            element.as_widget().draw(
                &state.children[idx],
                renderer,
                theme,
                style,
                tab_content_layout,
                cursor_position,
                viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'b, Message, Renderer>> {
        let layout = match self.tab_bar_position {
            TabBarPosition::Top => layout.children().nth(1),
            TabBarPosition::Bottom => layout.children().next(),
        };

        layout.and_then(|layout| {
            let idx = self.tab_bar.get_active_tab_idx();
            self.tabs
                .get_mut(idx)
                .map(Element::as_widget_mut)
                .and_then(|w| {
                    w.overlay(
                        &mut state.children[idx],
                        layout,
                        renderer,
                    )
                })
        })
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        let active_tab = self.tab_bar.get_active_tab_idx();
        operation.container(None, &mut |operation| {
            self.tabs[active_tab].as_widget().operate(
                &mut tree.children[active_tab],
                layout
                    .children()
                    .nth(1)
                    .expect("TabBar is 0th child, contents are 1st node"),
                renderer,
                operation,
            );
        });
    }
}

impl<'a, Message, TabId, Renderer> From<Tabs<'a, Message, TabId, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
    Renderer::Theme: StyleSheet + iced_style::text::StyleSheet,
    Message: 'a,
    TabId: 'a + Eq + Clone,
{
    fn from(tabs: Tabs<'a, Message, TabId, Renderer>) -> Self {
        Element::new(tabs)
    }
}
