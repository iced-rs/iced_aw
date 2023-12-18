//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: modal*

use super::overlay::modal::ModalOverlay;

use iced_widget::core::{
    self, alignment, event,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer,
    widget::{Operation, Tree},
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
};

pub use crate::style::modal::StyleSheet;

/// A modal content as an overlay.
///
/// Can be used in combination with the [`Card`](crate::card::Card)
/// widget to form dialog elements.
///
/// # Example
/// ```ignore
/// # use iced::widget::Text;
/// # use iced_aw::Modal;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     CloseModal,
/// }
///
/// let modal = Modal::new(
///     Text::new("Underlay"),
///     Some(Text::new("Overlay")),
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
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The optional content of the [`ModalOverlay`].
    overlay: Option<Element<'a, Message, Renderer>>,
    /// The optional message that will be send when the user clicked on the backdrop.
    backdrop: Option<Message>,
    /// The optional message that will be send when the ESC key was pressed.
    esc: Option<Message>,
    /// The style of the [`ModalOverlay`].
    style: <Renderer::Theme as StyleSheet>::Style,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
}

impl<'a, Message, Renderer> Modal<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`Modal`] wrapping the underlying element to show some content as an overlay.
    ///
    /// If [`overlay`] is `Some`, the contained [`Element`] is shown over the underlying element.
    /// If [`overlay`] is `None`, only the underlying element is shown.
    ///
    /// It expects:
    ///     * the underlay [`Element`] on which this [`Modal`] will be wrapped around.
    ///     * the optional overlay [`Element`] of the [`Modal`].
    pub fn new(
        underlay: impl Into<Element<'a, Message, Renderer>>,
        overlay: Option<impl Into<Element<'a, Message, Renderer>>>,
    ) -> Self {
        Modal {
            underlay: underlay.into(),
            overlay: overlay.map(Into::into),
            backdrop: None,
            esc: None,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Center,
        }
    }

    /// Sets the content alignment for the horizontal axis of the [`Modal`].
    #[must_use]
    pub fn align_x(mut self, alignment: alignment::Horizontal) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the content alignment for the vertical axis of the [`Modal`].
    #[must_use]
    pub fn align_y(mut self, alignment: alignment::Vertical) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the message that will be produced when the backdrop of the
    /// [`Modal`] is clicked.
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

    /// Sets the style of the [`Modal`].
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
        self.overlay.as_ref().map_or_else(
            || vec![Tree::new(&self.underlay)],
            |overlay| vec![Tree::new(&self.underlay), Tree::new(overlay)],
        )
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(overlay) = &self.overlay {
            tree.diff_children(&[&self.underlay, overlay]);
        } else {
            tree.diff_children(&[&self.underlay]);
        }
    }

    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().height()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
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
        if self.overlay.is_none() {
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
        if self.overlay.is_none() {
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
        if let Some(overlay) = &mut self.overlay {
            let bounds = layout.bounds();
            let position = Point::new(bounds.x, bounds.y);
            overlay.as_widget().diff(&mut state.children[1]);

            Some(overlay::Element::new(
                position,
                Box::new(ModalOverlay::new(
                    &mut state.children[1],
                    overlay,
                    self.backdrop.clone(),
                    self.esc.clone(),
                    self.style.clone(),
                    self.horizontal_alignment,
                    self.vertical_alignment,
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
        if let Some(overlay) = &self.overlay {
            overlay.as_widget().diff(&mut state.children[1]);

            overlay
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
    /// The visibility of the [`Modal`] overlay.
    show: bool,
    /// The state of the content of the [`Modal`] overlay.
    state: S,
}

impl<S> State<S> {
    /// Creates a new [`State`] containing the given state data.
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
