//! A context menu for showing actions on right click.
//!
use iced_widget::core::{
    self, event,
    layout::{Limits, Node},
    mouse::{self, Button, Cursor},
    overlay, renderer,
    widget::{tree, Operation, Tree},
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
};

use crate::native::overlay::ContextMenuOverlay;
pub use crate::style::context_menu::StyleSheet;

/// A context menu
///
///
/// # Example
/// ```ignore
/// # use iced::widget::{Text, Button};
/// # use iced_aw::ContextMenu;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     Action1,
/// }
///
/// let underlay = Text::new("right click me");
///
/// let cm = ContextMenu::new(
///     underlay,
///     || Button::new("action1").on_press(Message::Action1).into()
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct ContextMenu<'a, Overlay, Message, Renderer = crate::Renderer>
where
    Overlay: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: core::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The content of [`ContextMenuOverlay`].
    overlay: Overlay,
    /// The style of the [`ContextMenu`].
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Overlay, Message, Renderer> ContextMenu<'a, Overlay, Message, Renderer>
where
    Overlay: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: core::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`ContextMenu`]
    ///
    /// `underlay`: The underlying element.
    ///
    /// `overlay`: The content of [`ContextMenuOverlay`] which will be displayed when `underlay` is clicked.
    pub fn new<U>(underlay: U, overlay: Overlay) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
    {
        ContextMenu {
            underlay: underlay.into(),
            overlay,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
        }
    }

    /// Sets the style of the [`ContextMenu`].
    #[must_use]
    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Content, Message, Renderer> Widget<Message, Renderer>
    for ContextMenu<'a, Content, Message, Renderer>
where
    Content: 'a + Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay.as_widget().layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&(self.overlay)())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &(self.overlay)()]);
    }

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        let s: &mut State = state.state.downcast_mut();

        if s.show {
            let content = (self.overlay)();
            content.as_widget().diff(&mut state.children[1]);

            content
                .as_widget()
                .operate(&mut state.children[1], layout, renderer, operation);
        } else {
            self.underlay
                .as_widget()
                .operate(&mut state.children[0], layout, renderer, operation);
        }
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
        if event == Event::Mouse(mouse::Event::ButtonPressed(Button::Right)) {
            let bounds = layout.bounds();

            if cursor.is_over(bounds) {
                let s: &mut State = state.state.downcast_mut();
                s.cursor_position = cursor.position().unwrap_or_default();
                s.show = !s.show;
                return event::Status::Captured;
            }
        }

        self.underlay.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let s: &mut State = state.state.downcast_mut();

        if !s.show {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let position = s.cursor_position;
        let content = (self.overlay)();
        content.as_widget().diff(&mut state.children[1]);

        Some(
            ContextMenuOverlay::new(&mut state.children[1], content, self.style.clone(), s)
                .overlay(position),
        )
    }
}

impl<'a, Content, Message, Renderer> From<ContextMenu<'a, Content, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Content: 'a + Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(modal: ContextMenu<'a, Content, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}

/// The state of the ``context_menu``.
#[derive(Debug, Default)]
pub(crate) struct State {
    /// The visibility of the [`ContextMenu`] overlay.
    pub show: bool,
    /// Use for showing the overlay where the click was made.
    pub cursor_position: Point,
}

impl State {
    /// Creates a new [`State`] containing the given state data.
    pub const fn new() -> Self {
        Self {
            show: false,
            cursor_position: Point::ORIGIN,
        }
    }
}
