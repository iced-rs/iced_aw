//! Use a floating element to overlay a element over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*
use iced_native::{event, layout::Limits, overlay, Clipboard, Event, Layout, Point, Shell, Size};
use iced_native::{widget::Tree, Element};

use crate::native::floating_element::{Anchor, Offset};

/// The internal overlay of a [`FloatingElement`](crate::FloatingElement) for
/// rendering a [`Element`](iced_native::Element) as an overlay.
#[allow(missing_debug_implementations)]
pub struct FloatingElementOverlay<'a, Message: Clone, Renderer: iced_native::Renderer> {
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
    Message: Clone + 'a,
    Renderer: iced_native::Renderer + 'a,
{
    /// Creates a new [`FloatingElementOverlay`] containing the given
    /// [`Element`](iced_native::Element).
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
    /// overlay [`Element`](iced_native::overlay::Element) at the given target
    /// position.
    #[must_use]
    pub fn overlay(self, position: Point) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(position, Box::new(self))
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for FloatingElementOverlay<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_native::Renderer + 'a,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(Size::ZERO, bounds);
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
        }

        element
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        self.element.as_widget_mut().on_event(
            self.state,
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
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &iced_graphics::Rectangle,
        renderer: &Renderer,
    ) -> iced_native::mouse::Interaction {
        self.element.as_widget().mouse_interaction(
            self.state,
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
    ) {
        self.element.as_widget().draw(
            self.state,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            &layout.bounds(),
        );
    }
}
