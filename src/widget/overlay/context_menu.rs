//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: ``context_menu``*
use crate::context_menu;
pub use crate::style::{
    context_menu::{Catalog, Style},
    status::{self, StyleFn},
};

use iced_core::{
    Border, Clipboard, Color, Element, Event, Layout, Point, Rectangle, Shell, Size, keyboard,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer, touch,
    widget::Tree,
    window,
};

/// The overlay of the [`ContextMenu`](crate::widget::ContextMenu).
#[allow(missing_debug_implementations)]
pub struct ContextMenuOverlay<
    'a,
    'b,
    Message,
    Theme = iced_widget::Theme,
    Renderer = iced_widget::Renderer,
> where
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
    /// The state shared between [`ContextMenu`](crate::widget::ContextMenu) and [`ContextMenuOverlay`].
    state: &'a mut context_menu::State,
    /// The viewport of the overlay
    viewport: Rectangle,
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
        viewport: Rectangle,
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
            viewport,
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
            .as_widget_mut()
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

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: (0.0).into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            },
            style_sheet.background,
        );

        let content_layout = layout
            .children()
            .next()
            .expect("widget: Layout should have a content layout.");

        // Draw content directly without a background quad
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            &self.viewport,
        );
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let layout_children = layout
            .children()
            .next()
            .expect("widget: Layout should have a content layout.");

        let mut forward_event_to_children = true;
        let mut capture_event = false;

        match &event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if *key == keyboard::Key::Named(keyboard::key::Named::Escape) {
                    self.state.show = false;
                    forward_event_to_children = false;
                    shell.capture_event();
                }
            }

            Event::Mouse(mouse::Event::ButtonPressed(
                mouse::Button::Left | mouse::Button::Right,
            ))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout_children.bounds()) {
                    capture_event = true;
                } else {
                    self.state.show = false;
                    forward_event_to_children = false;
                    shell.request_redraw();
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                // close when released because because button send message on release
                self.state.show = false;

                capture_event = true;
            }

            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                // Track hover state for manual highlighting
                let old_bounds = self.state.hovered_bounds;
                self.state.hovered_bounds = if cursor.is_over(layout_children.bounds()) {
                    cursor.position().map(|_pos| {
                        // Find which child is hovered - we'll refine this in draw
                        layout_children.bounds()
                    })
                } else {
                    None
                };

                if old_bounds != self.state.hovered_bounds {
                    shell.request_redraw();
                }
            }

            Event::Window(window::Event::Resized { .. }) => {
                self.state.show = false;
                forward_event_to_children = false;
                capture_event = true;
            }

            _ => {}
        }

        if forward_event_to_children {
            self.content.as_widget_mut().update(
                self.tree,
                event,
                layout_children,
                cursor,
                renderer,
                clipboard,
                shell,
                &self.viewport,
            );
        }
        if capture_event {
            shell.capture_event();
        }
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation,
    ) {
        let content_layout = layout
            .children()
            .next()
            .expect("widget: Layout should have a content layout.");

        self.content
            .as_widget_mut()
            .operate(self.tree, content_layout, renderer, operation);
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,

        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout
                .children()
                .next()
                .expect("widget: Layout should have a content layout."),
            cursor,
            &self.viewport,
            renderer,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_core::Point;
    use iced_widget::text::Text;

    #[derive(Clone)]
    enum TestMessage {}

    fn create_test_tree() -> Tree {
        Tree::empty()
    }

    fn create_test_content()
    -> Element<'static, TestMessage, iced_widget::Theme, iced_widget::Renderer> {
        Text::new("Test content").into()
    }

    fn create_test_state() -> crate::context_menu::State {
        crate::context_menu::State::new()
    }

    #[test]
    fn context_menu_overlay_new_creates_instance() {
        // Test basic creation of overlay
        let mut tree = create_test_tree();
        let content = create_test_content();
        let class = &<iced_widget::Theme as Catalog>::default();
        let mut state = create_test_state();
        let position = Point::new(100.0, 100.0);
        let viewport = Rectangle::new(Point::ORIGIN, iced_core::Size::new(800.0, 600.0));

        let overlay = ContextMenuOverlay::new(position, &mut tree, content, class, &mut state, viewport);

        assert_eq!(overlay.position, position);
    }

    #[test]
    fn context_menu_overlay_creates_with_different_positions() {
        // Test overlay creation with various positions
        let mut tree1 = create_test_tree();
        let mut tree2 = create_test_tree();
        let mut tree3 = create_test_tree();
        let content1 = create_test_content();
        let content2 = create_test_content();
        let content3 = create_test_content();
        let class = &<iced_widget::Theme as Catalog>::default();
        let mut state1 = create_test_state();
        let mut state2 = create_test_state();
        let mut state3 = create_test_state();
        let viewport = Rectangle::new(Point::ORIGIN, iced_core::Size::new(800.0, 600.0));

        let overlay1 =
            ContextMenuOverlay::new(Point::ORIGIN, &mut tree1, content1, class, &mut state1, viewport);
        let overlay2 = ContextMenuOverlay::new(
            Point::new(500.0, 300.0),
            &mut tree2,
            content2,
            class,
            &mut state2,
            viewport,
        );
        let overlay3 = ContextMenuOverlay::new(
            Point::new(1000.0, 800.0),
            &mut tree3,
            content3,
            class,
            &mut state3,
            viewport,
        );

        assert_eq!(overlay1.position, Point::ORIGIN);
        assert_eq!(overlay2.position, Point::new(500.0, 300.0));
        assert_eq!(overlay3.position, Point::new(1000.0, 800.0));
    }

    #[test]
    fn context_menu_overlay_converts_to_overlay_element() {
        // Test conversion to overlay::Element
        let mut tree = create_test_tree();
        let content = create_test_content();
        let class = &<iced_widget::Theme as Catalog>::default();
        let mut state = create_test_state();
        let position = Point::new(100.0, 100.0);
        let viewport = Rectangle::new(Point::ORIGIN, iced_core::Size::new(800.0, 600.0));

        let overlay = ContextMenuOverlay::new(position, &mut tree, content, class, &mut state, viewport);
        let _overlay_element = overlay.overlay();

        // If we get here without panic, the conversion worked
    }

    #[test]
    fn context_menu_overlay_state_can_be_shown() {
        // Test that state can be set to shown
        let mut state = create_test_state();
        assert!(!state.show);

        state.show = true;
        assert!(state.show);
    }

    #[test]
    fn context_menu_overlay_state_can_be_hidden() {
        // Test that state can be set to hidden
        let mut state = create_test_state();
        state.show = true;
        assert!(state.show);

        state.show = false;
        assert!(!state.show);
    }

    #[test]
    fn context_menu_overlay_state_tracks_cursor_position() {
        // Test that state can track cursor position for overlay placement
        let mut state = create_test_state();
        assert_eq!(state.cursor_position, Point::ORIGIN);

        state.cursor_position = Point::new(123.4, 567.8);
        assert_eq!(state.cursor_position.x, 123.4);
        assert_eq!(state.cursor_position.y, 567.8);
    }

    #[test]
    fn context_menu_overlay_state_show_and_position_update_together() {
        // Test that state can track both show status and cursor position
        // This simulates what happens during a right-click event
        let mut state = create_test_state();
        assert!(!state.show);
        assert_eq!(state.cursor_position, Point::ORIGIN);

        // Simulate showing the overlay at a specific position
        state.show = true;
        state.cursor_position = Point::new(200.0, 300.0);

        assert!(state.show);
        assert_eq!(state.cursor_position, Point::new(200.0, 300.0));

        // Simulate hiding the overlay (position stays the same)
        state.show = false;
        assert!(!state.show);
        assert_eq!(state.cursor_position, Point::new(200.0, 300.0));
    }

    #[test]
    fn context_menu_overlay_position_can_be_negative() {
        // Test that overlay can be positioned with negative coordinates
        // (Though layout() will adjust it, the initial position can be negative)
        let mut tree = create_test_tree();
        let content = create_test_content();
        let class = &<iced_widget::Theme as Catalog>::default();
        let mut state = create_test_state();
        let position = Point::new(-50.0, -100.0);
        let viewport = Rectangle::new(Point::ORIGIN, iced_core::Size::new(800.0, 600.0));

        let overlay = ContextMenuOverlay::new(position, &mut tree, content, class, &mut state, viewport);

        assert_eq!(overlay.position.x, -50.0);
        assert_eq!(overlay.position.y, -100.0);
    }

    #[test]
    fn context_menu_overlay_position_accuracy() {
        // Test that position is stored with floating-point precision
        let mut tree = create_test_tree();
        let content = create_test_content();
        let class = &<iced_widget::Theme as Catalog>::default();
        let mut state = create_test_state();
        let position = Point::new(123.456, 789.012);
        let viewport = Rectangle::new(Point::ORIGIN, iced_core::Size::new(800.0, 600.0));

        let overlay = ContextMenuOverlay::new(position, &mut tree, content, class, &mut state, viewport);

        assert_eq!(overlay.position.x, 123.456);
        assert_eq!(overlay.position.y, 789.012);
    }
}
