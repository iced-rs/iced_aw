//! Use a floating element to overlay a element over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*

use crate::native::floating_element::{Anchor, Offset};

use iced_widget::core::{
    self, event, layout,
    mouse::{self, Cursor},
    overlay, renderer,
    widget::Tree,
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector,
};

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
    /// The bounds of the underlay element.
    underlay_bounds: Rectangle,
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
        underlay_bounds: Rectangle,
    ) -> Self {
        FloatingElementOverlay {
            state,
            element,
            anchor,
            offset,
            underlay_bounds,
        }
    }
}

impl<'a, 'b, Message, Renderer> core::Overlay<Message, Renderer>
    for FloatingElementOverlay<'a, 'b, Message, Renderer>
where
    Renderer: core::Renderer,
{
    fn layout(
        &mut self,
        renderer: &Renderer,
        _bounds: Size,
        position: Point,
        _translation: Vector,
    ) -> layout::Node {
        // Constrain overlay to fit inside the underlay's bounds
        let limits = layout::Limits::new(Size::ZERO, self.underlay_bounds.size())
            .width(Length::Fill)
            .height(Length::Fill);
        let node = self
            .element
            .as_widget()
            .layout(self.state, renderer, &limits);

        let position = match self.anchor {
            Anchor::NorthWest => Point::new(position.x + self.offset.x, position.y + self.offset.y),
            Anchor::NorthEast => Point::new(
                position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                position.y + self.offset.y,
            ),
            Anchor::SouthWest => Point::new(
                position.x + self.offset.x,
                position.y + self.underlay_bounds.height - node.bounds().height - self.offset.y,
            ),
            Anchor::SouthEast => Point::new(
                position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                position.y + self.underlay_bounds.height - node.bounds().height - self.offset.y,
            ),
            Anchor::North => Point::new(
                position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0
                    + self.offset.x,
                position.y + self.offset.y,
            ),
            Anchor::East => Point::new(
                position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0
                    + self.offset.y,
            ),
            Anchor::South => Point::new(
                position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0
                    + self.offset.x,
                position.y + self.underlay_bounds.height - node.bounds().height - self.offset.y,
            ),
            Anchor::West => Point::new(
                position.x + self.offset.x,
                position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0
                    + self.offset.y,
            ),
        };

        node.move_to(position)
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
        let bounds = layout.bounds();
        self.element
            .as_widget()
            .draw(self.state, renderer, theme, style, layout, cursor, &bounds);
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        self.element
            .as_widget_mut()
            .overlay(self.state, layout, renderer)
    }
}
