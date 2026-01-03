//! A button widget optimized for use in menus and overlays.
//!
//! Unlike the standard button, this widget explicitly manages its own hover state
//! to ensure proper visual feedback in overlay contexts like context menus.
//!
//! *This API requires the following crate features to be activated: `menu_button`*

use iced_core::{
    Border, Color, Element, Length, Point, Rectangle, Size, Vector, Widget,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer,
    widget::{Operation, Tree, tree},
};

/// A button designed for use in menus and overlays.
///
/// # Example
/// ```ignore
/// use iced_aw::MenuButton;
/// use iced::widget::text;
///
/// #[derive(Clone)]
/// enum Message {
///     ItemClicked,
/// }
///
/// let button = MenuButton::new(text("Menu Item"))
///     .on_press(Message::ItemClicked);
/// ```
pub struct MenuButton<'a, Message, Theme = iced_widget::Theme, Renderer = iced_widget::Renderer>
where
    Theme: iced_widget::button::Catalog,
    Renderer: renderer::Renderer,
{
    /// The content displayed inside the button
    content: Element<'a, Message, Theme, Renderer>,
    /// The message produced when the button is pressed
    on_press: Option<Message>,
    /// The width of the button
    width: Length,
    /// The padding inside the button
    padding: f32,
}

impl<'a, Message: Clone, Theme, Renderer> MenuButton<'a, Message, Theme, Renderer>
where
    Theme: iced_widget::button::Catalog,
    Renderer: renderer::Renderer,
{
    /// Creates a new [`MenuButton`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            on_press: None,
            width: Length::Fill,
            padding: 8.0,
        }
    }

    /// Sets the message that will be produced when the button is pressed.
    #[must_use]
    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    /// Sets the width of the button.
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the padding of the button.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

/// The local state of a [`MenuButton`].
#[derive(Default)]
struct State {
    /// Whether the button is currently hovered
    is_hovered: bool,
    /// Whether the button is currently pressed
    is_pressed: bool,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for MenuButton<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: iced_widget::button::Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
    ) -> Node {
        let padding = iced_core::Padding::from(self.padding);
        let limits = limits.width(self.width).shrink(padding);

        let mut content = self.content.as_widget_mut().layout(
            &mut tree.children[0],
            renderer,
            &limits,
        );

        let size = limits.resolve(self.width, Length::Shrink, content.size()).expand(padding);
        content.move_to_mut(Point::new(self.padding, self.padding));

        Node::with_children(size, vec![content])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();

        // Determine background color based on state
        let background_color = if self.on_press.is_none() {
            // Disabled state - transparent
            Color::TRANSPARENT
        } else if state.is_pressed {
            // Pressed state - use theme's strong color
            Color::from_rgb(0.4, 0.4, 0.5)
        } else if state.is_hovered {
            // Hovered state - subtle highlight
            Color::from_rgba(0.5, 0.5, 0.6, 0.3)
        } else {
            // Normal state - transparent
            Color::TRANSPARENT
        };

        // Draw background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: (4.0).into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            },
            background_color,
        );

        // Draw content
        let content_layout = layout.children().next().expect("MenuButton should have content layout");
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            viewport,
        );
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        // Update hover state
        let old_hovered = state.is_hovered;
        state.is_hovered = cursor.is_over(bounds) && self.on_press.is_some();

        if old_hovered != state.is_hovered {
            shell.request_redraw();
        }

        // Handle mouse button events
        if let iced_core::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if state.is_hovered {
                state.is_pressed = true;
                shell.request_redraw();
            }
        }

        if let iced_core::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = event {
            if state.is_pressed {
                state.is_pressed = false;

                if state.is_hovered {
                    if let Some(ref message) = self.on_press {
                        shell.publish(message.clone());
                    }
                }
                shell.request_redraw();
            }
        }

        // Forward events to content
        let content_layout = layout.children().next().expect("MenuButton should have content layout");
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            content_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: iced_core::Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();

        if cursor.is_over(bounds) && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let content_layout = layout.children().next().expect("MenuButton should have content layout");
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            content_layout,
            renderer,
            operation,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: iced_core::Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let content_layout = layout.children().next().expect("MenuButton should have content layout");
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            content_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<MenuButton<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + iced_widget::button::Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(button: MenuButton<'a, Message, Theme, Renderer>) -> Self {
        Element::new(button)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    enum TestMessage {
        Pressed,
    }

    #[test]
    fn menu_button_can_be_created() {
        let _button: MenuButton<TestMessage> = MenuButton::new(
            iced_widget::text::Text::new("Test")
        )
            .on_press(TestMessage::Pressed)
            .width(Length::Fill)
            .padding(10.0);
    }

    #[test]
    fn menu_button_state_defaults_to_not_hovered() {
        let state = State::default();
        assert!(!state.is_hovered);
        assert!(!state.is_pressed);
    }

    #[test]
    fn menu_button_has_correct_default_width() {
        let button: MenuButton<TestMessage> = MenuButton::new(iced_widget::text::Text::new("Test"));
        assert_eq!(button.width, Length::Fill);
    }

    #[test]
    fn menu_button_has_correct_default_padding() {
        let button: MenuButton<TestMessage> = MenuButton::new(iced_widget::text::Text::new("Test"));
        assert_eq!(button.padding, 8.0);
    }

    #[test]
    fn menu_button_width_can_be_customized() {
        let button: MenuButton<TestMessage> = MenuButton::new(iced_widget::text::Text::new("Test"))
            .width(Length::Fixed(100.0));
        assert_eq!(button.width, Length::Fixed(100.0));
    }

    #[test]
    fn menu_button_padding_can_be_customized() {
        let button: MenuButton<TestMessage> = MenuButton::new(iced_widget::text::Text::new("Test"))
            .padding(15.0);
        assert_eq!(button.padding, 15.0);
    }

    #[test]
    fn menu_button_without_on_press_has_no_message() {
        let button: MenuButton<TestMessage> = MenuButton::new(iced_widget::text::Text::new("Test"));
        assert!(button.on_press.is_none());
    }

    #[test]
    fn menu_button_with_on_press_has_message() {
        let button: MenuButton<TestMessage> = MenuButton::new(iced_widget::text::Text::new("Test"))
            .on_press(TestMessage::Pressed);
        assert!(button.on_press.is_some());
    }

    #[test]
    fn menu_button_methods_can_be_chained() {
        let _button: MenuButton<TestMessage> = MenuButton::new(iced_widget::text::Text::new("Test"))
            .on_press(TestMessage::Pressed)
            .width(Length::Fixed(150.0))
            .padding(12.0);
    }
}
