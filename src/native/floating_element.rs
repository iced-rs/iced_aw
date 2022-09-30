//! Use a floating element to overlay an element over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*
use iced_native::{
    event, mouse, overlay, Clipboard, Event, Layout, Length, Point, Rectangle, Shell,
};
use iced_native::{widget::Tree, Element, Widget};

pub mod anchor;
pub use anchor::Anchor;

pub mod offset;
pub use offset::Offset;

use super::overlay::floating_element::FloatingElementOverlay;

/// A floating element floating over some content.
///
/// # Example
/// ```
/// # use iced_native::renderer::Null;
/// # use iced_native::widget::{button, Button, Column, Text};
/// #
/// # pub type FloatingElement<'a, B, Message> = iced_aw::FloatingElement<'a, B, Message, Null>;
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
pub struct FloatingElement<'a, B, Message, Renderer>
where
    B: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
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
    element: B,
}

impl<'a, B, Message, Renderer> FloatingElement<'a, B, Message, Renderer>
where
    B: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
{
    /// Creates a new [`FloatingElement`](FloatingElement) over some content,
    /// showing the given [`Element`](iced_native::Element).
    ///
    /// It expects:
    ///     * the underlay [`Element`](iced_native::Element) on which this [`FloatingElement`](FloatingElement)
    ///         will be wrapped around.
    ///     * a function that will lazy create the [`Element`](iced_native::Element) for the overlay.
    pub fn new<U>(underlay: U, element: B) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
    {
        FloatingElement {
            anchor: Anchor::SouthEast,
            offset: 5.0.into(),
            hidden: false,
            underlay: underlay.into(),
            element,
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

impl<'a, B, Message, Renderer> Widget<Message, Renderer>
    for FloatingElement<'a, B, Message, Renderer>
where
    B: Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn children(&self) -> Vec<iced_native::widget::Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&(self.element)())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &(self.element)()]);
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
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        if self.hidden {
            return self
                .underlay
                .as_widget()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();
        let position = match self.anchor {
            Anchor::NorthWest => Point::new(0.0, 0.0),
            Anchor::NorthEast => Point::new(bounds.width, 0.0),
            Anchor::SouthWest => Point::new(0.0, bounds.height),
            Anchor::SouthEast => Point::new(bounds.width, bounds.height),
        };

        let position = Point::new(bounds.x + position.x, bounds.y + position.y);

        if state.children.len() == 2 {
            Some(
                FloatingElementOverlay::new(
                    &mut state.children[1],
                    (self.element)(),
                    &self.anchor,
                    &self.offset,
                )
                .overlay(position),
            )
        } else {
            None
        }
    }
}

impl<'a, B, Message, Renderer> From<FloatingElement<'a, B, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    B: 'a + Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn from(floating_element: FloatingElement<'a, B, Message, Renderer>) -> Self {
        Element::new(floating_element)
    }
}
