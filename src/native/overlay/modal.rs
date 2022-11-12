//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: modal*
use iced_graphics::Vector;
use iced_native::{
    event, keyboard, layout::Limits, mouse, overlay, renderer, touch, Clipboard, Color, Event,
    Layout, Point, Shell, Size,
};
use iced_native::{widget::Tree, Element};

use crate::style::modal::StyleSheet;

/// The overlay of the modal.
#[allow(missing_debug_implementations)]
pub struct ModalOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The state of the [`ModalOverlay`](ModalOverlay).
    state: &'a mut Tree,
    /// The content of the [`ModalOverlay`](ModalOverlay).
    content: Element<'a, Message, Renderer>,
    /// The optional message that will be send when the user clicks on the backdrop.
    backdrop: Option<Message>,
    /// The optional message that will be send when the ESC key was pressed.
    esc: Option<Message>,
    /// The style of the [`ModalOverlay`](ModalOverlay).
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> ModalOverlay<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`ModalOverlay`](ModalOverlay).
    pub fn new<C>(
        state: &'a mut Tree,
        content: C,
        backdrop: Option<Message>,
        esc: Option<Message>,
        style: <Renderer::Theme as StyleSheet>::Style,
    ) -> Self
    where
        C: Into<Element<'a, Message, Renderer>>,
    {
        ModalOverlay {
            state,
            content: content.into(),
            backdrop,
            esc,
            style,
        }
    }

    /// Turn this [`ModalOverlay`] into an overlay
    /// [`Element`](iced_native::overlay::Element).
    pub fn overlay(self, position: Point) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(position, Box::new(self))
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for ModalOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(Size::ZERO, bounds);

        let mut content = self.content.as_widget().layout(renderer, &limits);

        // Center position
        let max_size = limits.max();
        let container_half_width = max_size.width / 2.0;
        let container_half_height = max_size.height / 2.0;
        let content_half_width = content.bounds().width / 2.0;
        let content_half_height = content.bounds().height / 2.0;

        let position = position
            + Vector::new(
                container_half_width - content_half_width,
                container_half_height - content_half_height,
            );

        content.move_to(position);

        iced_native::layout::Node::with_children(max_size, vec![content])
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
        // TODO clean this up
        let esc_status = self
            .esc
            .as_ref()
            .map_or(event::Status::Ignored, |esc| match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                    if key_code == keyboard::KeyCode::Escape {
                        shell.publish(esc.to_owned());
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
                        shell.publish(backdrop.to_owned());
                        event::Status::Captured
                    }
                }
                _ => event::Status::Ignored,
            },
        );

        match esc_status.merge(backdrop_status) {
            event::Status::Ignored => self.content.as_widget_mut().on_event(
                self.state,
                event,
                layout
                    .children()
                    .next()
                    .expect("Native: Layout should have a content layout."),
                cursor_position,
                renderer,
                clipboard,
                shell,
            ),
            event::Status::Captured => event::Status::Captured,
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &iced_graphics::Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.state,
            layout
                .children()
                .next()
                .expect("Native: Layout should have a content layout."),
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
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
    ) {
        let bounds = layout.bounds();

        let style_sheet = theme.active(self.style);

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

        let content_layout = layout
            .children()
            .next()
            .expect("Native: Layout should have a content layout.");

        // Modal
        self.content.as_widget().draw(
            self.state,
            renderer,
            theme,
            style,
            content_layout,
            cursor_position,
            &bounds,
        );
    }
}
