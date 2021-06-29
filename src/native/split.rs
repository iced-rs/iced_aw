//! Use a split to split the available space in two parts to display two different elements.
//!
//! *This API requires the following crate features to be activated: split*
use std::hash::Hash;

use iced_native::{
    container, mouse, row, touch, Container, Element, Event, Length, Point, Row, Size, Widget,
};

use crate::core::renderer::DrawEnvironment;

/// A split can divide the available space by half to display two different elements.
/// It can split horizontally or vertically.
///
/// # Example
/// ```
/// # use iced_aw::split::{State, Axis};
/// # use iced_native::{Text, renderer::Null};
/// #
/// # pub type Split<'a, Message> = iced_aw::native::Split<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     Resized(u16),    
/// }
///
/// let mut state = State::new(Some(300), Axis::Vertical);
/// let first = Text::new("First");
/// let second = Text::new("Second");
///
/// let split = Split::new(&mut state, first, second, Message::Resized);
/// ```
#[allow(missing_debug_implementations)]
pub struct Split<'a, Message, Renderer: self::Renderer> {
    /// The state of the [`Split`](Split).
    state: &'a mut State,
    /// The first element of the [`Split`](Split).
    first: Element<'a, Message, Renderer>,
    /// The second element of the [`Split`](Split).
    second: Element<'a, Message, Renderer>,
    /// The padding around the elements of the [`Split`](Split).
    padding: f32,
    /// The spacing between the elements of the [`Split`](Split).
    /// This is also the width of the divider.
    spacing: f32,
    /// The width of the [`Split`](Split).
    width: Length,
    /// The height of the [`Split`](Split).
    height: Length,
    /// The message that is send when the divider of the [`Split`](Split) is moved.
    on_resize: Box<dyn Fn(u16) -> Message>,
    /// The style of the [`Split`](Split).
    style: <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer> Split<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + self::Renderer + container::Renderer,
{
    /// Creates a new [`Split`](Split).
    ///
    /// It expects:
    ///     - The [`State`](State) of the [`Split`](Split)
    ///     - The first [`Element`](Element) to display
    ///     - The second [`Element`](Element) to display
    ///     - The message that is send on moving the divider
    pub fn new<A, B, F>(state: &'a mut State, first: A, second: B, on_resize: F) -> Self
    where
        A: Into<Element<'a, Message, Renderer>>,
        B: Into<Element<'a, Message, Renderer>>,
        F: 'static + Fn(u16) -> Message,
    {
        Self {
            state,
            first: Container::new(first.into())
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            second: Container::new(second.into())
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            padding: 0.0,
            spacing: 5.0,
            width: Length::Fill,
            height: Length::Fill,
            on_resize: Box::new(on_resize),
            style: <Renderer as self::Renderer>::Style::default(),
        }
    }

    /// Sets the padding of the [`Split`](Split) around the inner elements.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the spacing of the [`Split`](Split) between the elements.
    /// This will also be the width of the divider.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the width of the [`Split`](Split).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Split`](Split).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Split<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer + row::Renderer,
{
    fn width(&self) -> iced_native::Length {
        self.width
    }

    fn height(&self) -> iced_native::Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let space = Row::<Message, Renderer>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .layout(renderer, limits);

        // TODO: clean up
        match self.state.axis {
            Axis::Horizontal => {
                let divider_position = self
                    .state
                    .divider_position
                    .unwrap_or_else(|| ((space.bounds().height) / 2.0) as u16);
                let divider_position = (divider_position.max(self.spacing as u16)
                    - (self.spacing / 2.0) as u16)
                    .clamp(
                        (self.spacing * 2.0) as u16,
                        (space.bounds().height - self.spacing * 2.0) as u16,
                    );

                let first_limits = limits
                    .clone()
                    .max_height(u32::from(divider_position))
                    .pad(self.padding);
                let mut first = self.first.layout(renderer, &first_limits);
                first.move_to(Point::new(self.padding, self.padding));

                let mut divider =
                    iced_native::layout::Node::new(Size::new(space.bounds().width, self.spacing));
                divider.move_to(Point::new(space.bounds().x, f32::from(divider_position)));

                let second_limits = limits
                    .clone()
                    .shrink(Size::new(
                        0.0,
                        first.bounds().height + (self.spacing / 2.0) + self.padding * 2.0,
                    ))
                    .pad(self.padding);
                let mut second = self.second.layout(renderer, &second_limits);
                second.move_to(Point::new(
                    first.bounds().x,
                    first.bounds().y + first.bounds().height + self.spacing + self.padding * 2.0,
                ));

                iced_native::layout::Node::with_children(
                    Size::new(space.bounds().width, space.bounds().height),
                    vec![first, divider, second],
                )
            }
            Axis::Vertical => {
                let divider_position = self
                    .state
                    .divider_position
                    .unwrap_or_else(|| ((space.bounds().width - self.spacing) / 2.0) as u16);
                let divider_position = (divider_position.max(self.spacing as u16)
                    - (self.spacing / 2.0) as u16)
                    .clamp(
                        (self.spacing * 2.0) as u16,
                        (space.bounds().width - self.spacing * 2.0) as u16,
                    );

                let first_limits = limits
                    .clone()
                    .max_width(u32::from(divider_position))
                    .pad(self.padding);
                let mut first = self.first.layout(renderer, &first_limits);
                first.move_to(Point::new(self.padding, self.padding));

                let mut divider =
                    iced_native::layout::Node::new(Size::new(self.spacing, space.bounds().height));
                divider.move_to(Point::new(f32::from(divider_position), space.bounds().y));

                let second_limits = limits
                    .clone()
                    .shrink(Size::new(
                        first.bounds().width + (self.spacing / 2.0) + self.padding * 2.0,
                        0.0,
                    ))
                    .pad(self.padding);
                let mut second = self.second.layout(renderer, &second_limits);
                second.move_to(Point::new(
                    first.bounds().x + first.bounds().width + self.spacing + self.padding * 2.0,
                    first.bounds().y,
                ));

                iced_native::layout::Node::with_children(
                    Size::new(space.bounds().width, space.bounds().width),
                    vec![first, divider, second],
                )
            }
        }
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        messages: &mut Vec<Message>,
    ) -> iced_native::event::Status {
        let mut children = layout.children();

        let first_layout = children
            .next()
            .expect("Native: Layout should have a first layout");
        let first_status = self.first.on_event(
            event.clone(),
            first_layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        );

        let divider_layout = children
            .next()
            .expect("Native: Layout should have a divider layout");
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if divider_layout.bounds().contains(cursor_position) {
                    self.state.dragging = true;
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if self.state.dragging {
                    self.state.dragging = false;
                }
            }

            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if self.state.dragging {
                    let position = match self.state.axis {
                        Axis::Horizontal => position.y,
                        Axis::Vertical => position.x,
                    };

                    messages.push((self.on_resize)(position as u16));
                }
            }

            _ => {}
        }

        let second_layout = children
            .next()
            .expect("Native: Layout should have a second layout");
        let second_status = self.second.on_event(
            event,
            second_layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        );

        first_status.merge(second_status)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) -> Renderer::Output {
        self::Renderer::draw(
            renderer,
            DrawEnvironment {
                defaults,
                layout,
                cursor_position,
                style_sheet: &self.style,
                viewport: Some(viewport),
                focus: (),
            },
            &self.first,
            &self.second,
            self.state.dragging,
            self.state.axis,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.state.divider_position.hash(state);
    }
}

/// The renderer of a [`Split`](Split).
///
/// Your renderer will need to implement this trait before being
/// able to use a [`Split`](Split) in your user interface.
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Split`](Split).
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<Self::Defaults, Self::Style, ()>,
        first: &Element<'_, Message, Self>,
        second: &Element<'_, Message, Self>,
        dragging: bool,
        axis: Axis,
    ) -> Self::Output;
}

#[cfg(debug_assertions)]
impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<Self::Defaults, Self::Style, ()>,
        _first: &Element<'_, Message, Self>,
        _second: &Element<'_, Message, Self>,
        _dragging: bool,
        _axis: Axis,
    ) -> Self::Output {
    }
}

impl<'a, Message, Renderer> From<Split<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + row::Renderer,
    Message: 'a + Clone,
{
    fn from(split_pane: Split<'a, Message, Renderer>) -> Self {
        Element::new(split_pane)
    }
}

/// The state of a [`Split`](Split).
#[derive(Clone, Debug)]
pub struct State {
    /// The position of the divider.
    divider_position: Option<u16>,
    /// The axis to split at.
    axis: Axis,
    /// If the divider is dragged by the user.
    dragging: bool,
}

impl State {
    /// Creates a new [`State`](State) for a [`Split`](Split).
    ///
    /// It expects:
    ///     - The optional position of the divider. If none, the available space will be split in half.
    ///     - The [`Axis`](Axis) to split at.
    #[must_use]
    pub const fn new(divider_position: Option<u16>, axis: Axis) -> Self {
        Self {
            divider_position,
            axis,
            dragging: false,
        }
    }

    /// Sets the position of the divider of the [`State`](State).
    pub fn set_divider_position(&mut self, position: u16) {
        self.divider_position = Some(position);
    }
}

/// The axis to split at.
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    /// Split horizontally.
    Horizontal,
    /// Split vertically.
    Vertical,
}
