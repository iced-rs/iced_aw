//! Use a floating button to overlay a button over some content
//! 
//! *This API requires the following crate features to be activated: floating_button*

use std::hash::Hash;

use iced_native::{
    Button, Clipboard, Event, Layout, Point, Size,
    Widget, event, layout::Limits, mouse, overlay
};

use crate::native::floating_button::{Anchor, Offset};

/// The internal overlay of a [`FloatingButton`](crate::native::FloatingButton) for
/// rendering a [`Button`](iced_native::button::Button) as an overlay.
#[allow(missing_debug_implementations)]
pub struct FloatingButtonOverlay<'a, Message: Clone, Renderer: iced_native::button::Renderer> {
    button: &'a Button<'a, Message, Renderer>,
    anchor: &'a Anchor,
    offset: &'a Offset,
    on_press: Option<Message>,
}

impl<'a, Message, Renderer> FloatingButtonOverlay<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_native::button::Renderer + 'a
{
    /// Creates a new [`FloatingButtonOverlay`] containing the given
    /// [`Button`](iced_native::button::Button).
    pub fn new(
        button: &'a Button<'a, Message, Renderer>,
        anchor: &'a Anchor,
        offset: &'a Offset,
        on_press: Option<Message>,
    ) -> Self
    {
        FloatingButtonOverlay {
            button,
            anchor,
            offset,
            on_press,
        }
    }

    /// Turns the [`FloatingButtonOverlay`](FloatingButtonOverlay) into an
    /// overlay [`Element`](iced_native::overlay::Element) at the given target
    /// position.
    pub fn overlay(
        self,
        position: Point,
    ) -> overlay::Element<'a, Message, Renderer>
    {
        overlay::Element::new(
            position,
            Box::new(self)
        )
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for FloatingButtonOverlay<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_native::button::Renderer + 'a,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(
            Size::ZERO,
            bounds,
        );
        let mut button = self.button.layout(renderer, &limits);

        match self.anchor {
            Anchor::NorthWest => button.move_to(
                Point::new(
                    position.x + self.offset.x,
                    position.y + self.offset.y,
                )
            ),
            Anchor::NorthEast => button.move_to(
                Point::new(
                    position.x - button.bounds().width - self.offset.x,
                    position.y + self.offset.y,
                )
            ),
            Anchor::SouthWest => button.move_to(
                Point::new(
                    position.x + self.offset.x,
                    position.y - button.bounds().height - self.offset.y,
                )
            ),
            Anchor::SouthEast => button.move_to(
                Point::new(
                    position.x - button.bounds().width - self.offset.x,
                    position.y - button.bounds().height - self.offset.y,
                )
            )
        }

        button
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>
    ) -> event::Status {
        // TODO: I'll burn in hell because of this...
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();
                    
                    if bounds.contains(cursor_position) {
                        return event::Status::Captured;
                    }
                }
            },
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(on_press) = self.on_press.clone() {
                    let bounds = layout.bounds();
                        
                        if bounds.contains(cursor_position) {
                            messages.push(on_press);
                            return event::Status::Captured;
                        }
                }
            },
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        self.button.draw(
            renderer,
            defaults,
            layout,
            cursor_position,
            &layout.bounds(),
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher, position: Point) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
        self.button.hash_layout(state);
    }
}
