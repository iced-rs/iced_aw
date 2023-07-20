//! Use a floating element to overlay a element over some content
//!
//! *This API requires the following crate features to be activated: `floating_element`*

use iced_widget::core::{
    self, event, layout,
    mouse::{self, Cursor},
    overlay, renderer,
    widget::Tree,
    BorderRadius, Clipboard, Color, Element, Event, Layout, Point, Rectangle, Shell, Size,
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
        let bounds = layout.bounds();

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 800.0,
                    height: 800.0,
                },
                border_radius: BorderRadius::default(),
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            Color {
                a: 0.80,
                ..Color::BLACK
            },
        );

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
