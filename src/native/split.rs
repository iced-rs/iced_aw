//! Use a split to split the available space in two parts to display two different elements.
//!
//! *This API requires the following crate features to be activated: split*
use std::hash::Hash;

use iced_native::{
    mouse, renderer, touch,
    widget::{Container, Row},
    Color, Element, Event, Layout, Length, Padding, Point, Rectangle, Shell, Size, Widget,
};

pub use crate::style::split::{Style, StyleSheet};

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
pub struct Split<'a, Message, Renderer> {
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
    /// The minimum size of the first element of the [`Split`](Split).
    min_size_first: u16,
    /// The minimum size of the second element of the [`Split`](Split).
    min_size_second: u16,
    /// The message that is send when the divider of the [`Split`](Split) is moved.
    on_resize: Box<dyn Fn(u16) -> Message>,
    /// The style of the [`Split`](Split).
    style_sheet: Box<dyn StyleSheet>,
}

impl<'a, Message, Renderer> Split<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + iced_native::Renderer,
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
            min_size_first: 5,
            min_size_second: 5,
            on_resize: Box::new(on_resize),
            style_sheet: std::boxed::Box::default(),
        }
    }

    /// Sets the padding of the [`Split`](Split) around the inner elements.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the spacing of the [`Split`](Split) between the elements.
    /// This will also be the width of the divider.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the width of the [`Split`](Split).
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Split`](Split).
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the minimum size of the first element of the [`Split`](Split).
    #[must_use]
    pub fn min_size_first(mut self, size: u16) -> Self {
        self.min_size_first = size;
        self
    }

    /// Sets the minimum size of the second element of the [`Split`](Split).
    #[must_use]
    pub fn min_size_second(mut self, size: u16) -> Self {
        self.min_size_second = size;
        self
    }

    /// Sets the style of the [`Split`](Split).
    #[must_use]
    pub fn style(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Split<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: 'a + iced_native::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
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

        match self.state.axis {
            Axis::Horizontal => horizontal_split(self, renderer, limits, &space),
            Axis::Vertical => vertical_split(self, renderer, limits, &space),
        }
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut Shell<Message>,
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
            shell,
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

                    shell.publish((self.on_resize)(position as u16));
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
            shell,
        );

        first_status.merge(second_status)
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let _first_layout = children
            .next()
            .expect("Graphics: Layout should have a first layout");
        let first_mouse_interaction =
            self.first
                .mouse_interaction(layout, cursor_position, viewport, renderer);
        let divider_layout = children
            .next()
            .expect("Graphics: Layout should have a divider layout");
        let divider_mouse_interaction = if divider_layout.bounds().contains(cursor_position) {
            match self.state.axis {
                Axis::Horizontal => mouse::Interaction::ResizingVertically,
                Axis::Vertical => mouse::Interaction::ResizingHorizontally,
            }
        } else {
            mouse::Interaction::default()
        };
        let _second_layout = children
            .next()
            .expect("Graphics: Layout should have a second layout");
        let second_mouse_interaction =
            self.second
                .mouse_interaction(layout, cursor_position, viewport, renderer);
        first_mouse_interaction
            .max(second_mouse_interaction)
            .max(divider_mouse_interaction)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) {
        // TODO: clipping!
        let mut children = layout.children();

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 0.0,
                border_width: self.style_sheet.active().border_width,
                border_color: self.style_sheet.active().border_color,
            },
            self.style_sheet
                .active()
                .background
                .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        let first_layout = children
            .next()
            .expect("Graphics: Layout should have a first layout");

        // First
        renderer.fill_quad(
            renderer::Quad {
                bounds: first_layout.bounds(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            if first_layout.bounds().contains(cursor_position) {
                self.style_sheet.hovered().first_background
            } else {
                self.style_sheet.active().first_background
            }
            .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        if first_layout.children().count() > 0 {
            self.first
                .draw(renderer, style, first_layout, cursor_position, viewport);
        }

        // Second
        let second_layout = children
            .next()
            .expect("Graphics: Layout should have a second layout");

        renderer.fill_quad(
            renderer::Quad {
                bounds: second_layout.bounds(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            if second_layout.bounds().contains(cursor_position) {
                self.style_sheet.hovered().second_background
            } else {
                self.style_sheet.active().second_background
            }
            .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        if second_layout.children().count() > 0 {
            self.second
                .draw(renderer, style, second_layout, cursor_position, viewport);
        }

        // Divider
        let divider_layout = children
            .next()
            .expect("Graphics: Layout should have a divider layout");

        let divider_style = if self.state.dragging {
            self.style_sheet.dragged()
        } else if divider_layout.bounds().contains(cursor_position) {
            self.style_sheet.hovered()
        } else {
            self.style_sheet.active()
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: divider_layout.bounds(),
                border_radius: 0.0,
                border_width: divider_style.divider_border_width,
                border_color: divider_style.divider_border_color,
            },
            divider_style.divider_background,
        );
    }

    fn overlay(
        &mut self,
        layout: iced_native::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'_, Message, Renderer>> {
        let mut children = layout.children();
        let first_layout = children.next()?;
        let _divider_layout = children.next()?;
        let second_layout = children.next()?;

        let first = &mut self.first;
        let second = &mut self.second;

        first
            .overlay(first_layout, renderer)
            .or_else(move || second.overlay(second_layout, renderer))
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.state.divider_position.hash(state);
        self.first.hash_layout(state);
        self.second.hash_layout(state);
    }
}

/// Do a horizontal split.
fn horizontal_split<'a, Message, Renderer: iced_native::Renderer>(
    split: &Split<'a, Message, Renderer>,
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    space: &iced_native::layout::Node,
) -> iced_native::layout::Node {
    if space.bounds().height
        < split.spacing + f32::from(split.min_size_first + split.min_size_second)
    {
        return iced_native::layout::Node::with_children(
            space.bounds().size(),
            vec![
                split.first.layout(
                    renderer,
                    &limits.clone().shrink(Size::new(0.0, space.bounds().height)),
                ),
                iced_native::layout::Node::new(Size::new(space.bounds().height, split.spacing)),
                split.second.layout(
                    renderer,
                    &limits.clone().shrink(Size::new(0.0, space.bounds().width)),
                ),
            ],
        );
    }

    let divider_position = split
        .state
        .divider_position
        .unwrap_or_else(|| (space.bounds().height / 2.0) as u16)
        .max((split.spacing / 2.0) as u16);
    let divider_position = (divider_position - (split.spacing / 2.0) as u16).clamp(
        split.min_size_first,
        space.bounds().height as u16 - split.min_size_second - split.spacing as u16,
    );

    let padding = Padding::from(split.padding as u16);
    let first_limits = limits
        .clone()
        .shrink(Size::new(
            0.0,
            space.bounds().height - f32::from(divider_position),
        ))
        .pad(padding);
    let mut first = split.first.layout(renderer, &first_limits);
    first.move_to(Point::new(
        space.bounds().x + split.padding,
        space.bounds().y + split.padding,
    ));

    let mut divider =
        iced_native::layout::Node::new(Size::new(space.bounds().width, split.spacing));
    divider.move_to(Point::new(space.bounds().x, f32::from(divider_position)));

    let second_limits = limits
        .clone()
        .shrink(Size::new(0.0, f32::from(divider_position) + split.spacing))
        .pad(padding);
    let mut second = split.second.layout(renderer, &second_limits);
    second.move_to(Point::new(
        space.bounds().x + split.padding,
        space.bounds().y + f32::from(divider_position) + split.spacing + split.padding,
    ));

    iced_native::layout::Node::with_children(space.bounds().size(), vec![first, divider, second])
}

/// Do a vertical split.
fn vertical_split<'a, Message, Renderer: iced_native::Renderer>(
    split: &Split<'a, Message, Renderer>,
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    space: &iced_native::layout::Node,
) -> iced_native::layout::Node {
    if space.bounds().width
        < split.spacing + f32::from(split.min_size_first + split.min_size_second)
    {
        return iced_native::layout::Node::with_children(
            space.bounds().size(),
            vec![
                split.first.layout(
                    renderer,
                    &limits.clone().shrink(Size::new(space.bounds().width, 0.0)),
                ),
                iced_native::layout::Node::new(Size::new(split.spacing, space.bounds().height)),
                split.second.layout(
                    renderer,
                    &limits.clone().shrink(Size::new(space.bounds().width, 0.0)),
                ),
            ],
        );
    }

    let divider_position = split
        .state
        .divider_position
        .unwrap_or_else(|| (space.bounds().width / 2.0) as u16)
        .max((split.spacing / 2.0) as u16);
    let divider_position = (divider_position - (split.spacing / 2.0) as u16).clamp(
        split.min_size_first,
        space.bounds().width as u16 - split.min_size_second - split.spacing as u16,
    );

    let padding = Padding::from(split.padding as u16);
    let first_limits = limits
        .clone()
        .shrink(Size::new(
            space.bounds().width - f32::from(divider_position),
            0.0,
        ))
        .pad(padding);
    let mut first = split.first.layout(renderer, &first_limits);
    first.move_to(Point::new(
        space.bounds().x + split.padding,
        space.bounds().y + split.padding,
    ));

    let mut divider =
        iced_native::layout::Node::new(Size::new(split.spacing, space.bounds().height));
    divider.move_to(Point::new(f32::from(divider_position), space.bounds().y));

    let second_limits = limits
        .clone()
        .shrink(Size::new(f32::from(divider_position) + split.spacing, 0.0))
        .pad(padding);
    let mut second = split.second.layout(renderer, &second_limits);
    second.move_to(Point::new(
        space.bounds().x + f32::from(divider_position) + split.spacing + split.padding,
        space.bounds().y + split.padding,
    ));

    iced_native::layout::Node::with_children(space.bounds().size(), vec![first, divider, second])
}

impl<'a, Message, Renderer> From<Split<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer,
    Message: 'a + Clone,
{
    fn from(split_pane: Split<'a, Message, Renderer>) -> Self {
        Element::new(split_pane)
    }
}

/// The state of a [`Split`](Split).
#[derive(Clone, Debug, Default)]
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

    /// Gets the position of the divider.
    #[must_use]
    pub const fn divider_position(&self) -> Option<u16> {
        self.divider_position
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

impl Default for Axis {
    fn default() -> Self {
        Self::Vertical
    }
}
