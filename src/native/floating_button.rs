//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_button`*
use std::hash::Hash;

use iced_native::{
    button, event, overlay, Button, Clipboard, Element, Event, Layout, Length, Point, Rectangle,
    Widget,
};

pub mod anchor;
pub use anchor::Anchor;

pub mod offset;
pub use offset::Offset;

use super::overlay::floating_button::FloatingButtonOverlay;

/// A floating button floating over some content.
///
/// # Example
/// ```
/// # use iced_native::{button, Button, Column, renderer::Null, Text};
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
pub struct FloatingButton<'a, B, Message, Renderer>
where
    B: Fn(&mut button::State) -> Button<'_, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::button::Renderer,
{
    /// The state of the button.
    state: &'a mut button::State,
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

impl<'a, B, Message, Renderer> FloatingButton<'a, B, Message, Renderer>
where
    B: Fn(&mut button::State) -> Button<'_, Message, Renderer>,
    Message: Clone,
    Renderer: iced_native::button::Renderer,
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
    pub fn new<U>(state: &'a mut button::State, underlay: U, button: B) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
    {
        FloatingButton {
            state,
            anchor: Anchor::SouthEast,
            offset: 5.0.into(),
            hidden: false,
            underlay: underlay.into(),
            button,
        }
    }

    /// Sets the [`Anchor`](Anchor) of the [`FloatingButton`](FloatingButton).
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the [`Offset`](Offset) of the [`FloatingButton`](FloatingButton).
    pub fn offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Offset>,
    {
        self.offset = offset.into();
        self
    }

    /// Hide or unhide the [`Button`](iced_native::button::Button) on the
    /// [`FloatingButton`](FloatingButton).
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }
}

impl<'a, B, Message, Renderer> Widget<Message, Renderer>
    for FloatingButton<'a, B, Message, Renderer>
where
    B: 'a + Fn(&mut button::State) -> Button<'_, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::button::Renderer,
{
    fn width(&self) -> Length {
        self.underlay.width()
    }

    fn height(&self) -> Length {
        self.underlay.height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.underlay.layout(renderer, limits)
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
        self.underlay.on_event(
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
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        self.underlay
            .draw(renderer, defaults, layout, cursor_position, viewport)
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.anchor.hash(state);
        (self.offset.x as u32).hash(state);
        (self.offset.y as u32).hash(state);
        self.hidden.hash(state);
        self.underlay.hash_layout(state);
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        if self.hidden {
            return None;
        }

        let bounds = layout.bounds();
        let position = match self.anchor {
            Anchor::NorthWest => Point::new(0.0, 0.0),
            Anchor::NorthEast => Point::new(bounds.width, 0.0),
            Anchor::SouthWest => Point::new(0.0, bounds.height),
            Anchor::SouthEast => Point::new(bounds.width, bounds.height),
        };

        let position = Point::new(bounds.x + position.x, bounds.y + position.y);

        Some(
            FloatingButtonOverlay::new(&mut self.state, &self.button, &self.anchor, &self.offset)
                .overlay(position),
        )
    }
}

impl<'a, B, Message, Renderer> From<FloatingButton<'a, B, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    B: 'a + Fn(&mut button::State) -> Button<'_, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::button::Renderer,
{
    fn from(floating_button: FloatingButton<'a, B, Message, Renderer>) -> Self {
        Element::new(floating_button)
    }
}
