//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: ``context_menu``*
use crate::context_menu;
pub use crate::style::{
    context_menu::{Catalog, Style},
    status::{self, StyleFn},
};

use iced::{
    advanced::{
        layout::{Limits, Node},
        overlay, renderer,
        widget::Tree,
        Clipboard, Layout, Shell,
    },
    event::Status,
    keyboard,
    mouse::{self, Cursor},
    touch, window, Border, Color, Element, Event, Point, Rectangle, Shadow, Size,
};

/// The overlay of the [`ContextMenu`](crate::widgets::ContextMenu).
#[allow(missing_debug_implementations)]
pub struct ContextMenuOverlay<'a, 'b, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: Catalog,
    'b: 'a,
{
    // The position of the element
    position: Point,
    /// The state of the [`ContextMenuOverlay`].
    tree: &'a mut Tree,
    /// The content of the [`ContextMenuOverlay`].
    content: Element<'a, Message, Theme, Renderer>,
    /// The style of the [`ContextMenuOverlay`].
    class: &'a Theme::Class<'b>,
    /// The state shared between [`ContextMenu`](crate::widgets::ContextMenu) and [`ContextMenuOverlay`].
    state: &'a mut context_menu::State,
}

impl<'a, 'b, Message, Theme, Renderer> ContextMenuOverlay<'a, 'b, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
    Theme: 'a + Catalog,
    'b: 'a,
{
    /// Creates a new [`ContextMenuOverlay`].
    pub(crate) fn new<C>(
        position: Point,
        tree: &'a mut Tree,
        content: C,
        class: &'a <Theme as Catalog>::Class<'b>,
        state: &'a mut context_menu::State,
    ) -> Self
    where
        C: Into<Element<'a, Message, Theme, Renderer>>,
    {
        ContextMenuOverlay {
            position,
            tree,
            content: content.into(),
            class,
            state,
        }
    }

    /// Turn this [`ContextMenuOverlay`] into an overlay [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(self))
    }
}

impl<'a, 'b, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for ContextMenuOverlay<'a, 'b, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: 'a + Catalog,
    'b: 'a,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        let limits = Limits::new(Size::ZERO, bounds);
        let max_size = limits.max();

        let mut content = self
            .content
            .as_widget()
            .layout(self.tree, renderer, &limits);

        // Try to stay inside borders
        let mut position = self.position;
        if position.x + content.size().width > bounds.width {
            position.x = f32::max(0.0, position.x - content.size().width);
        }
        if position.y + content.size().height > bounds.height {
            position.y = f32::max(0.0, position.y - content.size().height);
        }

        content.move_to_mut(position);

        Node::with_children(max_size, vec![content])
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

        let style_sheet = theme.style(self.class, status::Status::Active);

        // Background
        if (bounds.width > 0.) && (bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: (0.0).into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
                },
                style_sheet.background,
            );
        }

        let content_layout = layout
            .children()
            .next()
            .expect("widgets: Layout should have a content layout.");

        // Modal
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            &bounds,
        );
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> Status {
        let layout_children = layout
            .children()
            .next()
            .expect("widgets: Layout should have a content layout.");

        let mut forward_event_to_children = true;

        let status = match &event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if *key == keyboard::Key::Named(keyboard::key::Named::Escape) {
                    self.state.show = false;
                    forward_event_to_children = false;
                    Status::Captured
                } else {
                    Status::Ignored
                }
            }

            Event::Mouse(mouse::Event::ButtonPressed(
                mouse::Button::Left | mouse::Button::Right,
            ))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if !cursor.is_over(layout_children.bounds()) {
                    self.state.show = false;
                    forward_event_to_children = false;
                }
                Status::Captured
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                // close when released because because button send message on release
                self.state.show = false;
                Status::Captured
            }

            Event::Window(window::Event::Resized { .. }) => {
                self.state.show = false;
                forward_event_to_children = false;
                Status::Captured
            }

            _ => Status::Ignored,
        };

        let child_status = if forward_event_to_children {
            self.content.as_widget_mut().on_event(
                self.tree,
                event,
                layout_children,
                cursor,
                renderer,
                clipboard,
                shell,
                &layout.bounds(),
            )
        } else {
            Status::Ignored
        };

        match child_status {
            Status::Ignored => status,
            Status::Captured => Status::Captured,
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout
                .children()
                .next()
                .expect("widgets: Layout should have a content layout."),
            cursor,
            viewport,
            renderer,
        )
    }
}
