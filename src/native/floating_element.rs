//! Use a floating element to overlay an element over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*

use iced_widget::core::{
    self, event, layout,
    mouse::{self, Cursor},
    overlay, renderer,
    widget::{Operation, Tree},
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
};

pub mod anchor;
pub use anchor::Anchor;

pub mod offset;
pub use offset::Offset;

use super::overlay::floating_element::FloatingElementOverlay;

/// A floating element floating over some content.
///
/// # Example
/// ```
/// # use iced_widget::graphics::renderer::Null;
/// # use iced_widget::{button, Button, Column, Text};
/// # use iced_aw::native::floating_element;
/// #
/// # pub type FloatingElement<'a, B, Message> = floating_element::FloatingElement<'a, B, Message>;
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
pub struct FloatingElement<'a, Message, Renderer = crate::Renderer>
where
    Renderer: core::Renderer,
{
    /// The anchor of the element.
    anchor: Anchor,
    /// The offset of the element.
    offset: Offset,
    /// The visibility of the element.
    hidden: bool,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The floating element of the [`FloatingElementOverlay`](FloatingElementOverlay).
    element: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> FloatingElement<'a, Message, Renderer>
where
    Renderer: core::Renderer,
{
    /// Creates a new [`FloatingElement`](FloatingElement) over some content,
    /// showing the given [`Element`](iced_native::Element).
    ///
    /// It expects:
    ///     * the underlay [`Element`](iced_native::Element) on which this [`FloatingElement`](FloatingElement)
    ///         will be wrapped around.
    ///     * a function that will lazy create the [`Element`](iced_native::Element) for the overlay.
    pub fn new<U, B>(underlay: U, element: B) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        B: Into<Element<'a, Message, Renderer>>,
    {
        FloatingElement {
            anchor: Anchor::SouthEast,
            offset: 5.0.into(),
            hidden: false,
            underlay: underlay.into(),
            element: element.into(),
        }
    }

    /// Sets the [`Anchor`](Anchor) of the [`FloatingElement`](FloatingElement).
    #[must_use]
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the [`Offset`](Offset) of the [`FloatingElement`](FloatingElement).
    #[must_use]
    pub fn offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Offset>,
    {
        self.offset = offset.into();
        self
    }

    /// Hide or unhide the [`Element`](iced_native::Element) on the
    /// [`FloatingElement`](FloatingElement).
    #[must_use]
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for FloatingElement<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: core::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.element)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.element]);
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
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        if self.hidden {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        if state.children.len() == 2 {
            let bounds = layout.bounds();

            let position = match self.anchor {
                Anchor::NorthWest => Point::new(0.0, 0.0),
                Anchor::NorthEast => Point::new(bounds.width, 0.0),
                Anchor::SouthWest => Point::new(0.0, bounds.height),
                Anchor::SouthEast => Point::new(bounds.width, bounds.height),
                Anchor::North => Point::new(bounds.center_x(), 0.0),
                Anchor::East => Point::new(bounds.width, bounds.center_y()),
                Anchor::South => Point::new(bounds.center_x(), bounds.height),
                Anchor::West => Point::new(0.0, bounds.center_y()),
            };

            let position = Point::new(bounds.x + position.x, bounds.y + position.y);

            Some(overlay::Element::new(
                position,
                Box::new(FloatingElementOverlay::new(
                    &mut state.children[1],
                    &mut self.element,
                    &self.anchor,
                    &self.offset,
                    layout.bounds().size(),
                )),
            ))
        } else {
            None
        }
    }
}

impl<'a, Message, Renderer> From<FloatingElement<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + core::Renderer,
{
    fn from(floating_element: FloatingElement<'a, Message, Renderer>) -> Self {
        Element::new(floating_element)
    }
}
