//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: modal*

use std::hash::Hash;

use iced_native::{
    event, keyboard, layout::Limits, mouse, overlay, touch, Clipboard, Container, Element, Event,
    Layout, Length, Point, Size,
};

use crate::core::renderer::DrawEnvironment;

/// The overlay of the modal.
#[allow(missing_debug_implementations)]
pub struct ModalOverlay<'a, State, Content, Message, Renderer>
where
    State: 'a,
    Content: Fn(&'a mut State) -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer,
{
    state: &'a mut State,
    content: Content,
    backdrop: Option<Message>,
    esc: Option<Message>,
    style: &'a <Renderer as self::Renderer>::Style,
}

impl<'a, State, Content, Message, Renderer> ModalOverlay<'a, State, Content, Message, Renderer>
where
    State: 'a,
    Content: Fn(&mut State) -> Element<'_, Message, Renderer>,
    Message: Clone,
    Renderer: self::Renderer + iced_native::container::Renderer,
{
    /// Creates a new [`ModalOverlay`](ModalOverlay).
    pub fn new(
        state: &'a mut State,
        content: Content,
        backdrop: Option<Message>,
        esc: Option<Message>,
        style: &'a <Renderer as self::Renderer>::Style,
    ) -> Self {
        ModalOverlay {
            state,
            content,
            backdrop,
            esc,
            style,
        }
    }

    /// Turn this [`ModalOverlay`] into an overlay
    /// [`Element`](iced_native::overlay::Element).
    pub fn overlay(self, position: Point) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(position, Box::new(Overlay::new(self)))
    }
}

struct Overlay<'a, Message, Renderer: self::Renderer> {
    content: Element<'a, Message, Renderer>,
    backdrop: Option<Message>,
    esc: Option<Message>,
    style: &'a <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer> Overlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + iced_native::container::Renderer,
{
    pub fn new<State, Content>(modal: ModalOverlay<'a, State, Content, Message, Renderer>) -> Self
    where
        Content: Fn(&mut State) -> Element<'_, Message, Renderer>,
    {
        let ModalOverlay {
            state,
            content,
            backdrop,
            esc,
            style,
        } = modal;

        Self {
            content: Container::new(content(state))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into(),
            backdrop,
            esc,
            style,
        }
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for Overlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(Size::ZERO, bounds);

        let mut content = self.content.layout(renderer, &limits);

        content.move_to(position);

        content
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        // TODO clean this up
        let esc_status = self
            .esc
            .as_ref()
            .map(|esc| match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                    if key_code == keyboard::KeyCode::Escape {
                        messages.push(esc.to_owned());
                        event::Status::Captured
                    } else {
                        event::Status::Ignored
                    }
                }
                _ => event::Status::Ignored,
            })
            .unwrap_or(event::Status::Ignored);

        let backdrop_status = self
            .backdrop
            .as_ref()
            .zip(layout.children().next())
            .map(|(backdrop, layout)| match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if !layout.bounds().contains(cursor_position) {
                        messages.push(backdrop.to_owned());
                        event::Status::Captured
                    } else {
                        event::Status::Ignored
                    }
                }
                _ => event::Status::Ignored,
            })
            .unwrap_or(event::Status::Ignored);

        match esc_status.merge(backdrop_status) {
            event::Status::Ignored => self.content.on_event(
                event,
                layout,
                cursor_position,
                messages,
                renderer,
                clipboard,
            ),
            event::Status::Captured => event::Status::Captured,
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        renderer.draw(
            DrawEnvironment {
                defaults,
                layout,
                cursor_position,
                style_sheet: self.style,
                viewport: None,
                focus: (),
            },
            &self.content,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher, position: Point) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
        self.content.hash_layout(state);
    }
}

/// The renderer of a [`ModalOverlay`](ModalOverlay).
///
/// Your renderer will need to implement this trait before being
/// able to use a [`Modal`](crate::native::Modal) in your user interface.
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`ModalOverlay`](ModalOverlay).
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<Self::Defaults, Self::Style, ()>,
        modal: &Element<'_, Message, Self>,
    ) -> Self::Output;
}

#[cfg(debug_assertions)]
impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<Self::Defaults, Self::Style, ()>,
        _modal: &Element<'_, Message, Self>,
    ) -> Self::Output {
    }
}
