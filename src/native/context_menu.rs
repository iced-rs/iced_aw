//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: modal*
use iced_native::{
    event, mouse::{self, Button},
    widget::{Operation, Tree, tree},
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
};

use crate::native::overlay::ContextMenuOverlay;

pub use crate::style::context_menu::StyleSheet;

use super::overlay;

/// A modal content as an overlay.
///
/// Can be used in combination with the [`Card`](crate::card::Card)
/// widget to form dialog elements.
///
/// # Example
/// ```
/// # use iced_native::renderer::Null;
/// # use iced_native::widget::Text;
/// # use iced_aw::native::modal;
/// #
/// # pub type Modal<'a, Content, Message>
/// #  = modal::Modal<'a, Message, Content, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     CloseModal,
/// }
///
/// let modal = Modal::new(
///     true,
///     Text::new("Underlay"),
///     || Text::new("Overlay").into()
/// )
/// .backdrop(Message::CloseModal);
/// ```
#[allow(missing_debug_implementations)]
pub struct ContextMenu<'a, Content, Message, Renderer>
where
    Content: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The content of teh [`ModalOverlay`](ModalOverlay).
    content: Content,
    /// The optional message that will be send when the user clicked on the backdrop.
    backdrop: Option<Message>,
    /// The optional message that will be send when the ESC key was pressed.
    esc: Option<Message>,
    /// The style of the [`ModalOverlay`](ModalOverlay).
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Content, Message, Renderer> ContextMenu<'a, Content, Message, Renderer>
where
    Content: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`Modal`](Modal) wrapping the underlying element to
    /// show some content as an overlay.
    ///
    /// `state` is the content's state, assigned at the creation of the
    /// overlying content.
    ///
    /// It expects:
    ///     * if the overlay of the date picker is visible.
    ///     * the underlay [`Element`](iced_native::Element) on which this [`Modal`](Modal)
    ///         will be wrapped around.
    ///     * the content [`Element`](iced_native::Element) of the [`Modal`](Modal).
    pub fn new<U>(underlay: U, content: Content) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
    {
        ContextMenu {
            underlay: underlay.into(),
            content,
            backdrop: None,
            esc: None,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
        }
    }

    /// Sets the message that will be produced when the backdrop of the
    /// [`Modal`](Modal) is clicked.
    #[must_use]
    pub fn backdrop(mut self, message: Message) -> Self {
        self.backdrop = Some(message);
        self
    }

    /// Sets the message that will be produced when the Escape Key is
    /// pressed when the modal is open.
    ///
    /// This can be used to close the modal on ESC.
    #[must_use]
    pub fn on_esc(mut self, message: Message) -> Self {
        self.esc = Some(message);
        self
    }

    /// Sets the style of the [`Modal`](Modal).
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
    fn children(&self) -> Vec<iced_native::widget::Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&(self.content)())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &(self.content)()]);
    }

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

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
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

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
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

        let bounds = layout.bounds();
        let position = Point::new(bounds.x, bounds.y);
        let content = (self.content)();
        content.as_widget().diff(&mut state.children[1]);

        Some(
            ContextMenuOverlay::new(
                &mut state.children[1],
                content,
                self.backdrop.clone(),
                self.esc.clone(),
                self.style,
                s,
            )
            .overlay(position),
        )
    }

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        let s: &mut State = state.state.downcast_mut();

        if s.show    {
            let content = (self.content)();
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
/// The state of the modal.
#[derive(Debug, Default)]
pub(crate) struct State{
    /// The visibility of the [`Modal`](Modal) overlay.
    pub show: bool,
}

impl State {
    /// Creates a new [`State`](State) containing the given state data.
    pub const fn new() -> Self {
        Self {
            show: false,
        }
    }

}
