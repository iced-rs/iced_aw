//! Use a floating element to overlay an element over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*

use super::overlay::floating_element::FloatingElementOverlay;

use iced::{
    self,
    advanced::{
        layout::{Limits, Node},
        overlay, renderer,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    event,
    mouse::{self, Cursor},
    Element, Event, Length, Rectangle, Size, Vector,
};

pub mod anchor;
pub use anchor::Anchor;
pub mod offset;
pub use offset::Offset;

/// A floating element floating over some content.
///
/// # Example
/// ```ignore
/// # use iced::widget::{button, Button, Column, Text};
/// # use iced_aw::native::FloatingElement;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// let content = Column::new();
/// let floating_element = FloatingElement::new(
///     content,
///     || Button::new(Text::new("Press Me!"))
///         .on_press(Message::ButtonPressed)
///         .into()
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct FloatingElement<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer,
{
    /// The anchor of the element.
    anchor: Anchor,
    /// The offset of the element.
    offset: Offset,
    /// The visibility of the element.
    hidden: bool,
    /// The underlying element.
    underlay: Element<'a, Message, Theme, Renderer>,
    /// The floating element of the [`FloatingElementOverlay`].
    element: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> FloatingElement<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Creates a new [`FloatingElement`] over some content,
    /// showing the given [`Element`].
    ///
    /// It expects:
    ///     * the underlay [`Element`] on which this [`FloatingElement`]
    ///         will be wrapped around.
    ///     * a function that will lazily create the [`Element`] for the overlay.
    pub fn new<U, B>(underlay: U, element: B) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
        B: Into<Element<'a, Message, Theme, Renderer>>,
    {
        FloatingElement {
            anchor: Anchor::SouthEast,
            offset: 5.0.into(),
            hidden: false,
            underlay: underlay.into(),
            element: element.into(),
        }
    }

    /// Sets the [`Anchor`] of the [`FloatingElement`].
    #[must_use]
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Hide or unhide the [`Element`] on the [`FloatingElement`].
    #[must_use]
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }

    /// Sets the [`Offset`] of the [`FloatingElement`].
    #[must_use]
    pub fn offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Offset>,
    {
        self.offset = offset.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for FloatingElement<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.element)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.element]);
    }

    fn size(&self) -> Size<Length> {
        self.underlay.as_widget().size()
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

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        self.underlay
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        if self.hidden {
            return self.underlay.as_widget_mut().overlay(
                &mut state.children[0],
                layout,
                renderer,
                translation,
            );
        }

        if state.children.len() == 2 {
            let bounds = layout.bounds();

            Some(overlay::Element::new(Box::new(
                FloatingElementOverlay::new(
                    layout.position() + translation,
                    &mut state.children[1],
                    &mut self.element,
                    &self.anchor,
                    &self.offset,
                    bounds,
                ),
            )))
        } else {
            None
        }
    }
}

impl<'a, Message, Theme, Renderer> From<FloatingElement<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
    Theme: 'a,
{
    fn from(floating_element: FloatingElement<'a, Message, Theme, Renderer>) -> Self {
        Element::new(floating_element)
    }
}
