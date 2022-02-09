//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: modal*

use std::hash::Hash;

use iced_native::{
    event, keyboard, layout::Limits, mouse, overlay, renderer, touch, widget::Container, Clipboard,
    Color, Element, Event, Layout, Length, Point, Size, Widget,
};

use crate::style::modal::{Style, StyleSheet};

/// The overlay of the modal.
#[allow(missing_debug_implementations)]
pub struct ModalOverlay<'a, State, Content, Message, Renderer>
where
    State: 'a,
    Content: Fn(&'a mut State) -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    /// The state of the [`ModalOverlay`](ModalOverlay).
    state: &'a mut State,
    /// The content of the [`Overlay`](Overlay).
    content: Content,
    /// The optional message that will be send when the user clicks on the backdrop.
    backdrop: Option<Message>,
    /// The optional message that will be send when the ESC key was pressed.
    esc: Option<Message>,
    /// The style of the [`Overlay`](Overlay).
    style_sheet: &'a Box<dyn StyleSheet + 'a>,
}

impl<'a, State, Content, Message, Renderer> ModalOverlay<'a, State, Content, Message, Renderer>
where
    State: 'a,
    Content: Fn(&mut State) -> Element<'_, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
{
    /// Creates a new [`ModalOverlay`](ModalOverlay).
    pub fn new(
        state: &'a mut State,
        content: Content,
        backdrop: Option<Message>,
        esc: Option<Message>,
        style_sheet: &'a Box<dyn StyleSheet + 'a>,
    ) -> Self {
        ModalOverlay {
            state,
            content,
            backdrop,
            esc,
            style_sheet,
        }
    }

    /// Turn this [`ModalOverlay`] into an overlay
    /// [`Element`](iced_native::overlay::Element).
    pub fn overlay(self, position: Point) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(position, Box::new(Overlay::new(self)))
    }
}

/// The [`Overlay`](Overlay) of the [`Modal`](crate::native::Modal).
struct Overlay<'a, Message, Renderer: iced_native::Renderer> {
    /// The content of the [`Overlay`](Overlay).
    content: Element<'a, Message, Renderer>,
    /// The optional message that will be send when the user clicks on the backdrop.
    backdrop: Option<Message>,
    /// The optional message that will be send when the ESC key was pressed.
    esc: Option<Message>,
    /// The style of the [`Overlay`](Overlay).
    style_sheet: &'a Box<dyn StyleSheet + 'a>,
}

impl<'a, Message, Renderer> Overlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    /// Creates a new [`Overlay`](Overlay) from the given [`ModalOverlay`](ModalOverlay).
    pub fn new<State, Content>(modal: ModalOverlay<'a, State, Content, Message, Renderer>) -> Self
    where
        Content: Fn(&mut State) -> Element<'_, Message, Renderer>,
    {
        let ModalOverlay {
            state,
            content,
            backdrop,
            esc,
            style_sheet,
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
            style_sheet,
        }
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for Overlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
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
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        // TODO clean this up
        let esc_status = self
            .esc
            .as_ref()
            .map_or(event::Status::Ignored, |esc| match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                    if key_code == keyboard::KeyCode::Escape {
                        messages.push(esc.to_owned());
                        event::Status::Captured
                    } else {
                        event::Status::Ignored
                    }
                }
                _ => event::Status::Ignored,
            });

        let backdrop_status = self.backdrop.as_ref().zip(layout.children().next()).map_or(
            event::Status::Ignored,
            |(backdrop, layout)| match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if layout.bounds().contains(cursor_position) {
                        event::Status::Ignored
                    } else {
                        messages.push(backdrop.to_owned());
                        event::Status::Captured
                    }
                }
                _ => event::Status::Ignored,
            },
        );

        match esc_status.merge(backdrop_status) {
            event::Status::Ignored => self.content.on_event(
                event,
                layout,
                cursor_position,
                renderer,
                clipboard,
                messages,
            ),
            event::Status::Captured => event::Status::Captured,
        }
    }

    fn mouse_interaction(
        &self,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &iced_graphics::Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        todo!()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
    ) {
        let bounds = layout.bounds();

        let style_sheet = self.style_sheet.active();

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            style_sheet.background,
        );

        // Modal
        self.content
            .draw(renderer, style, layout, cursor_position, &bounds);
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher, position: Point) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
        self.content.hash_layout(state);
    }
}
