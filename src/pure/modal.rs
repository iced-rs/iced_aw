//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: modal*
use iced_native::{event, mouse, Clipboard, Event, Layout, Length, Point, Rectangle, Shell};
use iced_pure::{widget::Tree, Element, Widget};

use super::overlay::modal::ModalOverlay;

pub use crate::style::modal::{Style, StyleSheet};

/// A modal content as an overlay.
///
/// Can be used in combination with the [`Card`](crate::native::card::Card)
/// widget to form dialog elements.
///
/// # Example
/// ```
/// # use iced_aw::native::modal;
/// # use iced_native::{widget::Text, renderer::Null};
/// #
/// # pub type Modal<'a, S, Content, Message>
/// #  = iced_aw::native::Modal<'a, Message, S, Content, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     CloseModal,
/// }
///
/// let mut state = modal::State::new(());
///
/// let modal = Modal::new(
///     &mut state,
///     Text::new("Underlay"),
///     |_state| Text::new("Overlay").into()
/// )
/// .backdrop(Message::CloseModal);
/// ```
#[allow(missing_debug_implementations)]
pub struct Modal<'a, Content, Message, Renderer>
where
    Content: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
{
    /// Show the modal.
    show_modal: bool,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The content of teh [`ModalOverlay`](ModalOverlay).
    content: Content,
    /// The optional message that will be send when the user clicked on the backdrop.
    backdrop: Option<Message>,
    /// The optional message that will be send when the ESC key was pressed.
    esc: Option<Message>,
    /// The style of the [`ModalOverlay`](ModalOverlay).
    style_sheet: Box<dyn StyleSheet>,
}

impl<'a, Content, Message, Renderer> Modal<'a, Content, Message, Renderer>
where
    Content: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
{
    /// Creates a new [`Modal`](Modal) wrapping the underlying element to
    /// show some content as an overlay.
    ///
    /// `state` is the content's state, assigned at the creation of the
    /// overlying content.
    ///
    /// It expects:
    ///     * a mutable reference to the content's [`State`](State) of the [`Modal`](Modal).
    ///     * the underlay [`Element`](iced_native::Element) on which this [`Modal`](Modal)
    ///         will be wrapped around.
    ///     * the content [`Element`](iced_native::Element) of the [`Modal`](Modal).
    pub fn new<U>(show_modal: bool, underlay: U, content: Content) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
    {
        Modal {
            show_modal,
            underlay: underlay.into(),
            content,
            backdrop: None,
            esc: None,
            style_sheet: std::boxed::Box::default(),
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
    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

impl<'a, Content, Message, Renderer> Widget<Message, Renderer>
    for Modal<'a, Content, Message, Renderer>
where
    Content: 'a + Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn children(&self) -> Vec<iced_pure::widget::Tree> {
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

    fn draw(
        &self,
        state: &iced_pure::widget::Tree,
        renderer: &mut Renderer,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            style,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'b, Message, Renderer>> {
        if !self.show_modal {
            return self
                .underlay
                .as_widget()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.x, bounds.y);

        Some(
            ModalOverlay::new(
                &mut state.children[1],
                (self.content)(),
                self.backdrop.clone(),
                self.esc.clone(),
                &self.style_sheet,
            )
            .overlay(position),
        )
    }
}

impl<'a, Content, Message, Renderer> From<Modal<'a, Content, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Content: 'a + Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn from(modal: Modal<'a, Content, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}
