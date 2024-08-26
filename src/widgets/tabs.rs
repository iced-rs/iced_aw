//! Displays a [`Tabs`] widget to select the content to be displayed.
//!
//! This is a wrapper around the [`TabBar`](super::tab_bar::TabBar) widget.
//! Unlike the [`TabBar`](super::tab_bar::TabBar) widget it will also handle
//! the content of the tabs.
//!
//! *This API requires the following crate features to be activated: tabs*

pub mod tab_bar_position;
pub use crate::tab_bar::Position;
use crate::{
    style::{
        tab_bar::{Catalog, Style},
        Status, StyleFn,
    },
    widgets::tab_bar::TabBar,
    TabLabel,
};

use iced::{
    advanced::{
        layout::{Limits, Node},
        overlay, renderer,
        widget::{
            tree::{State, Tag},
            Operation, Tree,
        },
        Clipboard, Layout, Shell, Widget,
    },
    event,
    mouse::{self, Cursor},
    widget::{text, Row},
    Element, Event, Font, Length, Padding, Pixels, Point, Rectangle, Size, Vector,
};

pub use tab_bar_position::TabBarPosition;

/// A [`Tabs`] widget for showing a [`TabBar`](super::tab_bar::TabBar)
/// along with the tab's content.
///
/// # Example
/// ```ignore
/// # use iced_aw::{TabLabel, tabs::Tabs};
/// # use iced::widget::Text;
/// #
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
/// let tabs = Tabs::new(Message::TabSelected)
/// .push(TabId::One, TabLabel::Text(String::from("One")), Text::new(String::from("One")))
/// .push(TabId::Two, TabLabel::Text(String::from("Two")), Text::new(String::from("Two")))
/// .push(TabId::Three, TabLabel::Text(String::from("Three")), Text::new(String::from("Three")))
/// .set_active_tab(&TabId::Two);Theme
/// ```
///
#[allow(missing_debug_implementations)]
pub struct Tabs<'a, Message, TabId, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer,
    Theme: Catalog,
    TabId: Eq + Clone,
{
    /// The [`TabBar`](crate::widgets::TabBar) of the [`Tabs`].
    tab_bar: TabBar<'a, Message, TabId, Theme, Renderer>,
    /// The vector containing the content of the tabs.
    tabs: Vec<Element<'a, Message, Theme, Renderer>>,
    /// The vector containing the indices of the tabs.
    indices: Vec<TabId>,
    /// The position of the [`TabBar`](crate::widgets::TabBar).
    tab_bar_position: TabBarPosition,
    /// The position of the [`TabBar`](crate::widgets::TabBar) Icon.
    tab_icon_position: Position,
    /// the width of the [`Tabs`].
    width: Length,
    /// The height of the [`Tabs`].
    height: Length,
}

impl<'a, Message, TabId, Theme, Renderer> Tabs<'a, Message, TabId, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = Font>,
    Theme: Catalog + text::Catalog,
    TabId: Eq + Clone,
{
    /// Creates a new [`Tabs`] widget with the index of the selected tab and a
    /// specified message which will be send when a tab is selected by the user.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn new<F>(on_select: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        Self::new_with_tabs(Vec::new(), on_select)
    }

    /// Similar to `new` but with a given Vector of the
    /// [`TabLabel`](super::tab_bar::TabLabel) along with the tab's content.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * a vector containing the [`TabLabel`]s along with the content
    ///         [`Element`]s of the [`Tabs`].
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn new_with_tabs<F>(
        tabs: impl IntoIterator<Item = (TabId, TabLabel, Element<'a, Message, Theme, Renderer>)>,
        on_select: F,
    ) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        let tabs = tabs.into_iter();
        let n_tabs = tabs.size_hint().0;

        let mut tab_labels = Vec::with_capacity(n_tabs);
        let mut elements = Vec::with_capacity(n_tabs);
        let mut indices = Vec::with_capacity(n_tabs);

        for (id, tab_label, element) in tabs {
            tab_labels.push((id.clone(), tab_label));
            indices.push(id);
            elements.push(element);
        }

        Tabs {
            tab_bar: TabBar::with_tab_labels(tab_labels, on_select),
            tabs: elements,
            indices,
            tab_bar_position: TabBarPosition::Top,
            tab_icon_position: Position::Left,
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    /// Sets the size of the close icon of the
    /// [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn close_size(mut self, close_size: f32) -> Self {
        self.tab_bar = self.tab_bar.close_size(close_size);
        self
    }

    /// Sets the Tabs Icon render Position
    /// [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_icon_position(mut self, position: Position) -> Self {
        self.tab_icon_position = position;
        self
    }

    /// Sets the height of the [`Tabs`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the font of the icons of the
    /// [`TabLabel`](super::tab_bar::TabLabel)s of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn icon_font(mut self, font: Font) -> Self {
        self.tab_bar = self.tab_bar.icon_font(font);
        self
    }

    /// Sets the icon size of the [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.tab_bar = self.tab_bar.icon_size(icon_size);
        self
    }

    /// Sets the message that will be produced when the close icon of a tab
    /// on the [`TabBar`] is pressed.
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

    /// Pushes a [`TabLabel`](super::tab_bar::TabLabel) along with the tabs
    /// content to the [`Tabs`].
    #[must_use]
    pub fn push<E>(mut self, id: TabId, tab_label: TabLabel, element: E) -> Self
    where
        E: Into<Element<'a, Message, Theme, Renderer>>,
    {
        self.tab_bar = self
            .tab_bar
            .push(id.clone(), tab_label)
            .set_position(self.tab_icon_position);
        self.tabs.push(element.into());
        self.indices.push(id);
        self
    }

    /// Sets the active tab of the [`Tabs`] using the ``TabId``.
    #[must_use]
    pub fn set_active_tab(mut self, id: &TabId) -> Self {
        self.tab_bar = self.tab_bar.set_active_tab(id);
        self
    }

    /// Sets the height of the [`TabBar`](super::tab_bar::TabBar) of the [`Tabs`].
    #[must_use]
    pub fn tab_bar_height(mut self, height: Length) -> Self {
        self.tab_bar = self.tab_bar.height(height);
        self
    }

    /// Sets the maximum height of the [`TabBar`](super::tab_bar::TabBar) of the [`Tabs`].
    #[must_use]
    pub fn tab_bar_max_height(mut self, max_height: f32) -> Self {
        self.tab_bar = self.tab_bar.max_height(max_height);
        self
    }

    /// Sets the width of the [`TabBar`](super::tab_bar::TabBar) of the [`Tabs`].
    #[must_use]
    pub fn tab_bar_width(mut self, width: Length) -> Self {
        self.tab_bar = self.tab_bar.width(width);
        self
    }

    /// Sets the [`TabBarPosition`] of the [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_bar_position(mut self, position: TabBarPosition) -> Self {
        self.tab_bar_position = position;
        self
    }

    /// Sets the style of the [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_bar_style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.tab_bar = self.tab_bar.style(style);
        self
    }

    /// Sets the padding of the tabs of the [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_label_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.tab_bar = self.tab_bar.padding(padding);
        self
    }

    /// Sets the spacing between the tabs of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn tab_label_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.tab_bar = self.tab_bar.spacing(spacing);
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

    /// Sets the text size of the [`TabLabel`](super::tab_bar::TabLabel) of the
    /// [`TabBar`](super::tab_bar::TabBar).
    #[must_use]
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.tab_bar = self.tab_bar.text_size(text_size);
        self
    }

    /// Sets the width of the [`Tabs`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message, TabId, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Tabs<'a, Message, TabId, Theme, Renderer>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = Font>,
    Theme: Catalog + text::Catalog,
    TabId: Eq + Clone,
{
    fn children(&self) -> Vec<Tree> {
        let tabs = Tree {
            tag: Tag::stateless(),
            state: State::None,
            children: self.tabs.iter().map(Tree::new).collect(),
        };

        let bar = Tree {
            tag: self.tab_bar.tag(),
            state: self.tab_bar.state(),
            children: self.tab_bar.children(),
        };

        vec![bar, tabs]
    }

    fn diff(&self, tree: &mut Tree) {
        if tree.children.is_empty() {
            tree.children = self.children();
        }

        if let Some(tabs) = tree.children.get_mut(1) {
            tabs.diff_children(&self.tabs);
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let tab_bar_limits = limits.width(self.width).height(Length::Shrink);
        let mut tab_bar_node =
            self.tab_bar
                .layout(&mut tree.children[0], renderer, &tab_bar_limits);

        let tab_content_limits = limits
            .width(self.width)
            .height(self.height)
            .shrink([0.0, tab_bar_node.size().height]);

        let mut tab_content_node =
            if let Some(element) = self.tabs.get(self.tab_bar.get_active_tab_idx()) {
                element.as_widget().layout(
                    &mut tree.children[1].children[self.tab_bar.get_active_tab_idx()],
                    renderer,
                    &tab_content_limits,
                )
            } else {
                Row::<Message, Theme, Renderer>::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .layout(tree, renderer, &tab_content_limits)
            };

        let tab_bar_bounds = tab_bar_node.bounds();
        tab_bar_node = tab_bar_node.move_to(Point::new(
            tab_bar_bounds.x,
            tab_bar_bounds.y
                + match self.tab_bar_position {
                    TabBarPosition::Top => 0.0,
                    TabBarPosition::Bottom => tab_content_node.bounds().height,
                },
        ));

        let tab_content_bounds = tab_content_node.bounds();
        tab_content_node = tab_content_node.move_to(Point::new(
            tab_content_bounds.x,
            tab_content_bounds.y
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
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut children = layout.children();
        let (tab_bar_layout, tab_content_layout) = match self.tab_bar_position {
            TabBarPosition::Top => {
                let tab_bar_layout = children
                    .next()
                    .expect("widgets: Layout should have a TabBar layout at top position");
                let tab_content_layout = children
                    .next()
                    .expect("widgets: Layout should have a tab content layout at top position");
                (tab_bar_layout, tab_content_layout)
            }
            TabBarPosition::Bottom => {
                let tab_content_layout = children
                    .next()
                    .expect("widgets: Layout should have a tab content layout at bottom position");
                let tab_bar_layout = children
                    .next()
                    .expect("widgets: Layout should have a TabBar layout at bottom position");
                (tab_bar_layout, tab_content_layout)
            }
        };

        let status_tab_bar = self.tab_bar.on_event(
            &mut Tree::empty(),
            event.clone(),
            tab_bar_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        let idx = self.tab_bar.get_active_tab_idx();
        let status_element = self
            .tabs
            .get_mut(idx)
            .map_or(event::Status::Ignored, |element| {
                element.as_widget_mut().on_event(
                    &mut state.children[1].children[idx],
                    event,
                    tab_content_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            });

        status_tab_bar.merge(status_element)
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        // Tab bar
        let mut children = layout.children();
        let tab_bar_layout = match self.tab_bar_position {
            TabBarPosition::Top => children
                .next()
                .expect("widgets: There should be a TabBar at the top position"),
            TabBarPosition::Bottom => children
                .last()
                .expect("widgets: There should be a TabBar at the bottom position"),
        };

        let mut mouse_interaction = mouse::Interaction::default();
        let new_mouse_interaction = self.tab_bar.mouse_interaction(
            &Tree::empty(),
            tab_bar_layout,
            cursor,
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
                &state.children[1].children[idx],
                tab_content_layout,
                cursor,
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
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let tab_bar_layout = match self.tab_bar_position {
            TabBarPosition::Top => children
                .next()
                .expect("widgets: There should be a TabBar at the top position"),
            TabBarPosition::Bottom => children
                .last()
                .expect("widgets: There should be a TabBar at the bottom position"),
        };

        self.tab_bar.draw(
            &Tree::empty(),
            renderer,
            theme,
            style,
            tab_bar_layout,
            cursor,
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
                &state.children[1].children[idx],
                renderer,
                theme,
                style,
                tab_content_layout,
                cursor,
                viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
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
                        &mut state.children[1].children[idx],
                        layout,
                        renderer,
                        translation,
                    )
                })
        })
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let active_tab = self.tab_bar.get_active_tab_idx();
        operation.container(None, layout.bounds(), &mut |operation| {
            self.tabs[active_tab].as_widget().operate(
                &mut tree.children[1].children[active_tab],
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

impl<'a, Message, TabId, Theme, Renderer> From<Tabs<'a, Message, TabId, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = Font>,
    Theme: 'a + Catalog + text::Catalog,
    Message: 'a,
    TabId: 'a + Eq + Clone,
{
    fn from(tabs: Tabs<'a, Message, TabId, Theme, Renderer>) -> Self {
        Element::new(tabs)
    }
}
