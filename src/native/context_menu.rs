//! A context menu for showing actions on right click.
//!
use iced_native::{
    Clipboard, Element,
    event,
    Event, Layout, Length, mouse::{self, Button}, Point, Rectangle, Shell, widget::{Operation, Tree, tree}, Widget,
};

use crate::native::overlay::ContextMenuOverlay;
pub use crate::style::context_menu::StyleSheet;

/// A context menu
///
///
/// # Example
/// ```
/// # use iced_native::renderer::Null;
/// # use iced_native::widget::Text;
/// # use iced_native::widget::Button;
/// # use iced_aw::native::context_menu;
/// #
/// # pub type ContextMenu<'a, Content, Message>
/// #  = context_menu::ContextMenu<'a, Message, Content, Null>;
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
pub struct ContextMenu<'a, Overlay, Message, Renderer>
    where
        Overlay: Fn() -> Element<'a, Message, Renderer>,
        Message: Clone,
        Renderer: iced_native::Renderer,
        Renderer::Theme: StyleSheet,
{
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The content of [`ContextMenuOverlay`](ContextMenuOverlay).
    overlay: Overlay,
    /// The style of the [`ContextMenu`](ContextMenu).
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Overlay, Message, Renderer> ContextMenu<'a, Overlay, Message, Renderer>
    where
        Overlay: Fn() -> Element<'a, Message, Renderer>,
        Message: Clone,
        Renderer: iced_native::Renderer,
        Renderer::Theme: StyleSheet,
{
    /// Creates a new [`ContextMenu`](ContextMenu)
    ///
    /// `underlay`: The underlying element.
    ///
    /// `overlay`: The content of [`ContextMenuOverlay`](ContextMenuOverlay) which will be displayed when `underlay` is clicked.
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


    /// Sets the style of the [`ContextMenu`](ContextMenu).
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
        Renderer: 'a + iced_native::Renderer,
        Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.underlay.as_widget().layout(renderer, limits)
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
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor_position,
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
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        if let Event::Mouse(mouse::Event::ButtonPressed(Button::Right)) = event {
            let bounds = layout.bounds();

            if bounds.contains(cursor_position) {
                let s: &mut State = state.state.downcast_mut();
                s.cursor_position = cursor_position;
                s.show = !s.show;
                return event::Status::Captured;
            }
        }


        self.underlay.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'b, Message, Renderer>> {
        let s: &mut State = state.state.downcast_mut();


        if !s.show {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let position = s.cursor_position.clone();
        let content = (self.overlay)();
        content.as_widget().diff(&mut state.children[1]);

        Some(
            ContextMenuOverlay::new(
                &mut state.children[1],
                content,
                self.style,
                s,
            )
                .overlay(position),
        )
    }
}

impl<'a, Content, Message, Renderer> From<ContextMenu<'a, Content, Message, Renderer>>
for Element<'a, Message, Renderer>
    where
        Content: 'a + Fn() -> Element<'a, Message, Renderer>,
        Message: 'a + Clone,
        Renderer: 'a + iced_native::Renderer,
        Renderer::Theme: StyleSheet,
{
    fn from(modal: ContextMenu<'a, Content, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}

/// The state of the context_menu.
#[derive(Debug, Default)]
pub(crate) struct State {
    /// The visibility of the [`ContextMenu`](ContextMenu) overlay.
    pub show: bool,
    /// Use for showing the overlay where the click was made.
    pub cursor_position: Point,
}

impl State {
    /// Creates a new [`State`](State) containing the given state data.
    pub const fn new() -> Self {
        Self {
            show: false,
            cursor_position: Point::ORIGIN,
        }
    }
}
