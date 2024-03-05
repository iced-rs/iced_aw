//! Use a floating element to overlay a element over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*

use crate::widgets::floating_element::{Anchor, Offset};

use iced::{
    advanced::{
        layout::{Limits, Node},
        overlay, renderer,
        widget::Tree,
        Clipboard, Layout, Overlay, Shell,
    },
    event,
    mouse::{self, Cursor},
    Element, Event, Length, Point, Rectangle, Size, Vector,
};

/// The internal overlay of a [`FloatingElement`](crate::FloatingElement) for
/// rendering a [`Element`](iced_widget::core::Element) as an overlay.
#[allow(missing_debug_implementations)]
pub struct FloatingElementOverlay<'a, 'b, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    // The position of the element
    position: Point,
    /// The state of the element.
    state: &'b mut Tree,
    /// The floating element
    element: &'b mut Element<'a, Message, Theme, Renderer>,
    /// The anchor of the element.
    anchor: &'b Anchor,
    /// The offset of the element.
    offset: &'b Offset,
    /// The bounds of the underlay element.
    underlay_bounds: Rectangle,
}

impl<'a, 'b, Message, Theme, Renderer> FloatingElementOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Creates a new [`FloatingElementOverlay`] containing the given
    /// [`Element`](iced_widget::core::Element).
    pub fn new(
        position: Point,
        state: &'b mut Tree,
        element: &'b mut Element<'a, Message, Theme, Renderer>,
        anchor: &'b Anchor,
        offset: &'b Offset,
        underlay_bounds: Rectangle,
    ) -> Self {
        FloatingElementOverlay {
            position,
            state,
            element,
            anchor,
            offset,
            underlay_bounds,
        }
    }
}

impl<'a, 'b, Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
    for FloatingElementOverlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> Node {
        // Constrain overlay to fit inside the underlay's bounds
        let limits = Limits::new(Size::ZERO, self.underlay_bounds.size())
            .width(Length::Fill)
            .height(Length::Fill);
        let node = self
            .element
            .as_widget()
            .layout(self.state, renderer, &limits);

        let position = match self.anchor {
            Anchor::NorthWest => Point::new(
                self.position.x + self.offset.x,
                self.position.y + self.offset.y,
            ),
            Anchor::NorthEast => Point::new(
                self.position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                self.position.y + self.offset.y,
            ),
            Anchor::SouthWest => Point::new(
                self.position.x + self.offset.x,
                self.position.y + self.underlay_bounds.height
                    - node.bounds().height
                    - self.offset.y,
            ),
            Anchor::SouthEast => Point::new(
                self.position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                self.position.y + self.underlay_bounds.height
                    - node.bounds().height
                    - self.offset.y,
            ),
            Anchor::North => Point::new(
                self.position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0
                    + self.offset.x,
                self.position.y + self.offset.y,
            ),
            Anchor::East => Point::new(
                self.position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                self.position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0
                    + self.offset.y,
            ),
            Anchor::South => Point::new(
                self.position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0
                    + self.offset.x,
                self.position.y + self.underlay_bounds.height
                    - node.bounds().height
                    - self.offset.y,
            ),
            Anchor::West => Point::new(
                self.position.x + self.offset.x,
                self.position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0
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
        theme: &Theme,
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
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        self.element
            .as_widget_mut()
            .overlay(self.state, layout, renderer, Vector::ZERO)
    }
}
