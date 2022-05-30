//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_button`*
use iced_native::{
    event, mouse, overlay, Clipboard, Event, Layout, Length, Point, Rectangle, Shell,
};
use iced_pure::{widget::Tree, Element, Widget};

pub use crate::native::floating_button::{Anchor, Offset};

use super::overlay::floating_element::FloatingElementOverlay;

/// A floating button floating over some content.
///
/// # Example
/// ```
/// # use iced_native::{widget::{button, Button, Column, Text}, renderer::Null};
/// #
/// # pub type FloatingButton<'a, B, Message> = iced_aw::native::FloatingButton<'a, B, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// let mut button_state = button::State::default();
///
/// let content = Column::new();
/// let floating_button = FloatingButton::new(
///     &mut button_state,
///     content,
///     |state| Button::new(state, Text::new("Press Me!"))
///         .on_press(Message::ButtonPressed)
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct FloatingElement<'a, B, Message, Renderer>
where
    B: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
{
    /// The anchor of the button.
    anchor: Anchor,
    /// The offset of the button.
    offset: Offset,
    /// The visibility of the button.
    hidden: bool,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer>,
    /// The floating button of the [`FloatingButtonOverlay`](FloatingButtonOverlay).
    button: B,
}

impl<'a, B, Message, Renderer> FloatingElement<'a, B, Message, Renderer>
where
    B: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::Renderer,
{
    /// Creates a new [`FloatingButton`](FloatingButton) over some content,
    /// showing the given [`Button`](iced_native::button::Button).
    ///
    /// It expects:
    ///     * a mutable reference to the [`FloatingButton`](FloatingButton)'s
    ///         [`State`](iced_native::button::State).
    ///     * the underlay [`Element`](iced_native::Element) on which this [`FloatingButton`](FloatingButton)
    ///         will be wrapped around.
    ///     * a function that will lazy create the [`Button`](iced_native::Button) for the overlay.
    pub fn new<U>(underlay: U, button: B) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
    {
        FloatingElement {
            anchor: Anchor::SouthEast,
            offset: 5.0.into(),
            hidden: false,
            underlay: underlay.into(),
            button,
        }
    }

    /// Sets the [`Anchor`](Anchor) of the [`FloatingButton`](FloatingButton).
    #[must_use]
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the [`Offset`](Offset) of the [`FloatingButton`](FloatingButton).
    #[must_use]
    pub fn offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Offset>,
    {
        self.offset = offset.into();
        self
    }

    /// Hide or unhide the [`Button`](iced_native::button::Button) on the
    /// [`FloatingButton`](FloatingButton).
    #[must_use]
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }
}

impl<'a, B, Message, Renderer> Widget<Message, Renderer>
    for FloatingElement<'a, B, Message, Renderer>
where
    B: Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn children(&self) -> Vec<iced_pure::widget::Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&(self.button)())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &(self.button)()]);
    }

    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.underlay.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.underlay.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &iced_pure::widget::Tree,
        renderer: &mut Renderer,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            style,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        if self.hidden {
            return self
                .underlay
                .as_widget()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();
        let position = match self.anchor {
            Anchor::NorthWest => Point::new(0.0, 0.0),
            Anchor::NorthEast => Point::new(bounds.width, 0.0),
            Anchor::SouthWest => Point::new(0.0, bounds.height),
            Anchor::SouthEast => Point::new(bounds.width, bounds.height),
        };

        let position = Point::new(bounds.x + position.x, bounds.y + position.y);

        if state.children.len() == 2 {
            Some(
                FloatingElementOverlay::new(
                    &mut state.children[1],
                    (self.button)(),
                    &self.anchor,
                    &self.offset,
                )
                .overlay(position),
            )
        } else {
            None
        }
    }
}

impl<'a, B, Message, Renderer> From<FloatingElement<'a, B, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    B: 'a + Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn from(floating_button: FloatingElement<'a, B, Message, Renderer>) -> Self {
        Element::new(floating_button)
    }
}
