//! Use a floating element to overlay a element over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*

use iced_widget::core::{
    self, event, layout,
    mouse::{self, Cursor},
    overlay, renderer,
    widget::Tree,
    Clipboard, Element, Event, Layout, Point, Rectangle, Shell, Size,
};

use crate::native::floating_element::{Anchor, Offset};

/// The internal overlay of a [`FloatingElement`](crate::FloatingElement) for
/// rendering a [`Element`](iced_widget::core::Element) as an overlay.
#[allow(missing_debug_implementations)]
pub struct FloatingElementOverlay<'a, Message, Renderer: core::Renderer> {
    /// The state of the element.
    state: &'a mut Tree,
    /// The floating element
    element: Element<'a, Message, Renderer>,
    /// The anchor of the element.
    anchor: &'a Anchor,
    /// The offset of the element.
    offset: &'a Offset,
}

impl<'a, Message, Renderer> FloatingElementOverlay<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: core::Renderer + 'a,
{
    /// Creates a new [`FloatingElementOverlay`] containing the given
    /// [`Element`](iced_widget::core::Element).
    pub fn new<B>(state: &'a mut Tree, element: B, anchor: &'a Anchor, offset: &'a Offset) -> Self
    where
        B: Into<Element<'a, Message, Renderer>>,
    {
        FloatingElementOverlay {
            state,
            element: element.into(),
            anchor,
            offset,
        }
    }

    /// Turns the [`FloatingElementOverlay`](FloatingElementOverlay) into an
    /// overlay [`Element`](iced_widget::core::Element) at the given target
    /// position.
    #[must_use]
    pub fn overlay(self, position: Point) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(position, Box::new(self))
    }
}

impl<'a, Message, Renderer> core::Overlay<Message, Renderer>
    for FloatingElementOverlay<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: core::Renderer + 'a,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);
        let mut element = self.element.as_widget().layout(renderer, &limits);

        match self.anchor {
            Anchor::NorthWest => element.move_to(Point::new(
                position.x + self.offset.x,
                position.y + self.offset.y,
            )),
            Anchor::NorthEast => element.move_to(Point::new(
                position.x - element.bounds().width - self.offset.x,
                position.y + self.offset.y,
            )),
            Anchor::SouthWest => element.move_to(Point::new(
                position.x + self.offset.x,
                position.y - element.bounds().height - self.offset.y,
            )),
            Anchor::SouthEast => element.move_to(Point::new(
                position.x - element.bounds().width - self.offset.x,
                position.y - element.bounds().height - self.offset.y,
            )),
            Anchor::North => element.move_to(Point::new(
                position.x + self.offset.x - element.bounds().width / 2.0,
                position.y + self.offset.y,
            )),
            Anchor::East => element.move_to(Point::new(
                position.x - element.bounds().width - self.offset.x,
                position.y - element.bounds().height / 2.0,
            )),
            Anchor::South => element.move_to(Point::new(
                position.x + self.offset.x - element.bounds().width / 2.0,
                position.y - element.bounds().height - self.offset.y,
            )),
            Anchor::West => element.move_to(Point::new(
                position.x + self.offset.x,
                position.y - element.bounds().height / 2.0,
            )),
        }

        element
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        self.element.as_widget_mut().on_event(
            self.state,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.element
            .as_widget()
            .mouse_interaction(self.state, layout, cursor, viewport, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
    ) {
        self.element.as_widget().draw(
            self.state,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        );
    }
}
