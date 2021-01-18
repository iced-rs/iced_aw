//! Use a floating button to overlay a button over some content
//! 
//! *This API requires the following crate features to be activated: floating_button*

use std::hash::Hash;

use iced_native::{
    button, event, layout::Limits, overlay, Button, Clipboard, Event,
    Layout, Point, Size, Widget, 
};

use crate::native::floating_button::{Anchor, Offset};

/// The internal overlay of a [`FloatingButton`](crate::native::FloatingButton) for
/// rendering a [`Button`](iced_native::button::Button) as an overlay.
#[allow(missing_debug_implementations)]
pub struct FloatingButtonOverlay<'a, B, Message: Clone, Renderer: iced_native::button::Renderer>
where
    B: Fn(&'a mut button::State) -> Button<'a, Message, Renderer>,
{
    state: &'a mut button::State,
    button: B,
    anchor: &'a Anchor,
    offset: &'a Offset,
}

impl<'a, B, Message, Renderer> FloatingButtonOverlay<'a, B, Message, Renderer>
where
    B: Fn(&mut button::State) -> Button<'_, Message, Renderer>,
    Message: Clone + 'a,
    Renderer: iced_native::button::Renderer + 'a,
{
    /// Creates a new [`FloatingButtonOverlay`] containing the given
    /// [`Button`](iced_native::button::Button).
    pub fn new(
        state: &'a mut button::State,
        button: B,
        anchor: &'a Anchor,
        offset: &'a Offset,
    ) -> Self
    {
        FloatingButtonOverlay {
            state,
            button,
            anchor,
            offset,
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
            Box::new(Overlay::new(self))
        )
    }
}

struct Overlay<'a, Message, Renderer: iced_native::button::Renderer> {
    anchor: &'a Anchor,
    offset: &'a Offset,
    button: Button<'a, Message, Renderer>,
}

impl<'a, Message, Renderer: iced_native::button::Renderer> Overlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a,
{
    pub fn new<B>(floating_button: FloatingButtonOverlay<'a, B, Message, Renderer>) -> Self
    where
        B: Fn(&mut button::State) -> Button<'_, Message, Renderer>,
    {
        let FloatingButtonOverlay {
            state,
            button,
            anchor,
            offset,
        } = floating_button;

        Self {
            anchor,
            offset,
            button: button(state),
        }
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for Overlay<'a, Message, Renderer>
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
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>
    ) -> event::Status {
        self.button.on_event(
            event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        )
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
