//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_button`*
use iced_native::{event, layout::Limits, overlay, Clipboard, Event, Layout, Point, Shell, Size};
use iced_pure::{widget::Tree, Element};

use crate::native::floating_button::{Anchor, Offset};

/// The internal overlay of a [`FloatingButton`](crate::native::FloatingButton) for
/// rendering a [`Button`](iced_native::button::Button) as an overlay.
#[allow(missing_debug_implementations)]
pub struct FloatingElementOverlay<'a, Message: Clone, Renderer: iced_native::Renderer> {
    /// The state of the button.
    state: &'a mut Tree,
    /// The floating button
    button: Element<'a, Message, Renderer>,
    /// The anchor of the button.
    anchor: &'a Anchor,
    /// The offset of the button.
    offset: &'a Offset,
}

impl<'a, Message, Renderer> FloatingElementOverlay<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_native::Renderer + 'a,
{
    /// Creates a new [`FloatingButtonOverlay`] containing the given
    /// [`Button`](iced_native::button::Button).
    pub fn new<B>(state: &'a mut Tree, button: B, anchor: &'a Anchor, offset: &'a Offset) -> Self
    where
        B: Into<Element<'a, Message, Renderer>>,
    {
        FloatingElementOverlay {
            state,
            button: button.into(),
            anchor,
            offset,
        }
    }

    /// Turns the [`FloatingButtonOverlay`](FloatingButtonOverlay) into an
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
        let mut button = self.button.as_widget().layout(renderer, &limits);

        match self.anchor {
            Anchor::NorthWest => button.move_to(Point::new(
                position.x + self.offset.x,
                position.y + self.offset.y,
            )),
            Anchor::NorthEast => button.move_to(Point::new(
                position.x - button.bounds().width - self.offset.x,
                position.y + self.offset.y,
            )),
            Anchor::SouthWest => button.move_to(Point::new(
                position.x + self.offset.x,
                position.y - button.bounds().height - self.offset.y,
            )),
            Anchor::SouthEast => button.move_to(Point::new(
                position.x - button.bounds().width - self.offset.x,
                position.y - button.bounds().height - self.offset.y,
            )),
        }

        button
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
        self.button.as_widget_mut().on_event(
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
        self.button.as_widget().mouse_interaction(
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
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
    ) {
        self.button.as_widget().draw(
            self.state,
            renderer,
            style,
            layout,
            cursor_position,
            &layout.bounds(),
        );
    }
}
