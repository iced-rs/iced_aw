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
pub struct FloatingElementOverlay<'a, 'b, Message, Renderer: core::Renderer> {
    /// The state of the element.
    state: &'b mut Tree,
    /// The floating element
    element: &'b mut Element<'a, Message, Renderer>,
    /// The anchor of the element.
    anchor: &'b Anchor,
    /// The offset of the element.
    offset: &'b Offset,
}

impl<'a, 'b, Message, Renderer> FloatingElementOverlay<'a, 'b, Message, Renderer>
where
    Renderer: core::Renderer,
{
    /// Creates a new [`FloatingElementOverlay`] containing the given
    /// [`Element`](iced_widget::core::Element).
    pub fn new(
        state: &'b mut Tree,
        element: &'b mut Element<'a, Message, Renderer>,
        anchor: &'b Anchor,
        offset: &'b Offset,
    ) -> Self {
        FloatingElementOverlay {
            state,
            element,
            anchor,
            offset,
        }
    }
}

impl<'a, 'b, Message, Renderer> core::Overlay<Message, Renderer>
    for FloatingElementOverlay<'a, 'b, Message, Renderer>
where
    Renderer: core::Renderer,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);
        let element = self.element.as_widget().layout(renderer, &limits);

        let size = match self.anchor {
            Anchor::NorthWest | Anchor::North => Size::new(
                position.x + self.offset.x + element.bounds().width,
                position.y + self.offset.y + element.bounds().height,
            ),
            Anchor::NorthEast => Size::new(
                position.x - self.offset.x,
                position.y + self.offset.y + element.bounds().height,
            ),
            Anchor::SouthWest | Anchor::South => Size::new(
                position.x + self.offset.x + element.bounds().width,
                position.y - self.offset.y,
            ),
            Anchor::SouthEast => Size::new(position.x - self.offset.x, position.y - self.offset.y),
            Anchor::East => Size::new(
                position.x - self.offset.x,
                position.y + element.bounds().height / 2.0,
            ),
            Anchor::West => Size::new(
                position.x + self.offset.x + element.bounds().width,
                position.y + element.bounds().height / 2.0,
            ),
        };

        let position = match self.anchor {
            Anchor::NorthWest => Point::new(position.x + self.offset.x, position.y + self.offset.y),
            Anchor::NorthEast => Point::new(
                position.x - element.bounds().width - self.offset.x,
                position.y + self.offset.y,
            ),
            Anchor::SouthWest => Point::new(
                position.x + self.offset.x,
                position.y - element.bounds().height - self.offset.y,
            ),
            Anchor::SouthEast => Point::new(
                position.x - element.bounds().width - self.offset.x,
                position.y - element.bounds().height - self.offset.y,
            ),
            Anchor::North => Point::new(
                position.x + self.offset.x - element.bounds().width / 2.0,
                position.y + self.offset.y,
            ),
            Anchor::East => Point::new(
                position.x - element.bounds().width - self.offset.x,
                position.y - element.bounds().height / 2.0,
            ),
            Anchor::South => Point::new(
                position.x + self.offset.x - element.bounds().width / 2.0,
                position.y - element.bounds().height - self.offset.y,
            ),
            Anchor::West => Point::new(
                position.x + self.offset.x,
                position.y - element.bounds().height / 2.0,
            ),
        };

        //element.move_to(position);
        let mut node = layout::Node::with_children(size, vec![element]);
        node.move_to(position);
        node
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
            layout
                .children()
                .next()
                .expect("Native: Layout should have a content layout."),
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
        self.element.as_widget().mouse_interaction(
            self.state,
            layout
                .children()
                .next()
                .expect("Native: Layout should have a content layout."),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
    ) {
        let bounds = layout.bounds();
        self.element.as_widget().draw(
            self.state,
            renderer,
            theme,
            style,
            layout
                .children()
                .next()
                .expect("Native: Layout should have a content layout."),
            cursor,
            &bounds,
        );
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        self.element
            .as_widget_mut()
            .overlay(self.state, layout.children().next()?, renderer)
    }
}
