//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: modal*
use iced_widget::core::{
    self, event, layout,
    mouse::{self, Cursor},
    overlay, renderer,
    widget::{Operation, Tree},
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
};

use super::overlay::modal::ModalOverlay;

pub use crate::style::modal::StyleSheet;

/// A modal content as an overlay.
///
/// Can be used in combination with the [`Card`](crate::card::Card)
/// widget to form dialog elements.
///
/// # Example
/// ```
/// # use core::Renderer::Null;
/// # use iced_native::widget::Text;
/// # use iced_aw::native::{ modal};
/// #
/// # pub type Modal<'a, Message>
/// #  = modal::Modal<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     CloseModal,
/// }
///
/// let modal = Modal::new(
///     false,
///     Text::new("Underlay"),
///     || Text::new("Overlay").into()
/// )
/// .backdrop(Message::CloseModal);
/// ```
#[allow(missing_debug_implementations)]
pub struct Modal<'a, Message, Renderer = crate::Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Show the modal.
    show_modal: bool,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The content of teh [`ModalOverlay`](ModalOverlay).
    content: Element<'a, Message, Renderer>,
    /// The optional message that will be send when the user clicked on the backdrop.
    backdrop: Option<Message>,
    /// The optional message that will be send when the ESC key was pressed.
    esc: Option<Message>,
    /// The style of the [`ModalOverlay`](ModalOverlay).
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> Modal<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
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
    pub fn new<U, C>(show_modal: bool, underlay: U, content: C) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        C: Into<Element<'a, Message, Renderer>>,
    {
        Modal {
            show_modal,
            underlay: underlay.into(),
            content: content.into(),
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

impl<'a, Message, Renderer> Widget<Message, Renderer> for Modal<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.content]);
    }

    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.underlay.as_widget().layout(renderer, limits)
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
        if !self.show_modal {
            return self.underlay.as_widget_mut().on_event(
                &mut state.children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if !self.show_modal {
            return self.underlay.as_widget().mouse_interaction(
                &state.children[0],
                layout,
                cursor,
                viewport,
                renderer,
            );
        }

        mouse::Interaction::default()
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

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        if self.show_modal {
            let bounds = layout.bounds();
            let position = Point::new(bounds.x, bounds.y);
            self.content.as_widget().diff(&mut state.children[1]);

            Some(overlay::Element::new(
                position,
                Box::new(ModalOverlay::new(
                    &mut state.children[1],
                    &mut self.content,
                    self.backdrop.clone(),
                    self.esc.clone(),
                    self.style,
                )),
            ))
        } else {
            self.underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer)
        }
    }

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        if self.show_modal {
            self.content.as_widget().diff(&mut state.children[1]);

            self.content
                .as_widget()
                .operate(&mut state.children[1], layout, renderer, operation);
        } else {
            self.underlay
                .as_widget()
                .operate(&mut state.children[0], layout, renderer, operation);
        }
    }
}

impl<'a, Message, Renderer> From<Modal<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(modal: Modal<'a, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}

/// The state of the modal.
#[derive(Debug, Default)]
pub struct State<S> {
    /// The visibility of the [`Modal`](Modal) overlay.
    show: bool,
    /// The state of the content of the [`Modal`](Modal) overlay.
    state: S,
}

impl<S> State<S> {
    /// Creates a new [`State`](State) containing the given state data.
    pub const fn new(s: S) -> Self {
        Self {
            show: false,
            state: s,
        }
    }

    /// Setting this to true shows the modal (the modal is open), false means
    /// the modal is hidden (closed).
    pub fn show(&mut self, b: bool) {
        self.show = b;
    }

    /// See if this modal will be shown or not.
    pub const fn is_shown(&self) -> bool {
        self.show
    }

    /// Get a mutable reference to the inner state data.
    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.state
    }

    /// Get a reference to the inner state data.
    pub const fn inner(&self) -> &S {
        &self.state
    }
}
