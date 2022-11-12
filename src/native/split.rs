//! Use a split to split the available space in two parts to display two different elements.
//!
//! *This API requires the following crate features to be activated: split*
use iced_native::{
    mouse, renderer, touch, Color, Event, Layout, Length, Padding, Point, Rectangle, Shell, Size,
};
use iced_native::{
    widget::{
        tree::{self, Tag},
        Container, Row, Tree,
    },
    Element, Widget,
};

pub use crate::style::split::{Appearance, StyleSheet};

/// A split can divide the available space by half to display two different elements.
/// It can split horizontally or vertically.
///
/// # Example
/// ```
/// # use iced_aw::split::{State, Axis};
/// # use iced_native::renderer::Null;
/// # use iced_native::widget::Text;
/// #
/// # pub type Split<'a, Message> = iced_aw::Split<'a, Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     Resized(u16),
/// }
///
/// let first = Text::new("First");
/// let second = Text::new("Second");
///
/// let split = Split::new(first, second, Some(300), Axis::Vertical, Message::Resized);
/// ```
#[allow(missing_debug_implementations)]
pub struct Split<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The first element of the [`Split`](Split).
    first: Element<'a, Message, Renderer>,
    /// The second element of the [`Split`](Split).
    second: Element<'a, Message, Renderer>,
    /// The position of the divider.
    divider_position: Option<u16>,
    /// The axis to split at.
    axis: Axis,
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
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> Split<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet + iced_style::container::StyleSheet,
{
    /// Creates a new [`Split`](Split).
    ///
    /// It expects:
    ///     - The first [`Element`](Element) to display
    ///     - The second [`Element`](Element) to display
    ///     - The position of the divider. If none, the space will be split in half.
    ///     - The [`Axis`](Axis) to split at.
    ///     - The message that is send on moving the divider
    pub fn new<A, B, F>(
        first: A,
        second: B,
        divider_position: Option<u16>,
        axis: Axis,
        on_resize: F,
    ) -> Self
    where
        A: Into<Element<'a, Message, Renderer>>,
        B: Into<Element<'a, Message, Renderer>>,
        F: 'static + Fn(u16) -> Message,
    {
        Self {
            first: Container::new(first.into())
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            second: Container::new(second.into())
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            divider_position,
            axis,
            padding: 0.0,
            spacing: 5.0,
            width: Length::Fill,
            height: Length::Fill,
            min_size_first: 5,
            min_size_second: 5,
            on_resize: Box::new(on_resize),
            style: <Renderer::Theme as StyleSheet>::Style::default(),
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
    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Split<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.first), Tree::new(&self.second)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.first, &self.second]);
    }

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

        match self.axis {
            Axis::Horizontal => horizontal_split(self, renderer, limits, &space),
            Axis::Vertical => vertical_split(self, renderer, limits, &space),
        }
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> iced_native::event::Status {
        let split_state: &mut State = state.state.downcast_mut();
        let mut children = layout.children();

        let first_layout = children
            .next()
            .expect("Native: Layout should have a first layout");
        let first_status = self.first.as_widget_mut().on_event(
            &mut state.children[0],
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
                    split_state.dragging = true;
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if split_state.dragging {
                    split_state.dragging = false;
                }
            }

            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if split_state.dragging {
                    let position = match self.axis {
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
        let second_status = self.second.as_widget_mut().on_event(
            &mut state.children[1],
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
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let first_layout = children
            .next()
            .expect("Graphics: Layout should have a first layout");
        let first_mouse_interaction = self.first.as_widget().mouse_interaction(
            &state.children[0],
            first_layout,
            cursor_position,
            viewport,
            renderer,
        );
        let divider_layout = children
            .next()
            .expect("Graphics: Layout should have a divider layout");
        let divider_mouse_interaction = if divider_layout.bounds().contains(cursor_position) {
            match self.axis {
                Axis::Horizontal => mouse::Interaction::ResizingVertically,
                Axis::Vertical => mouse::Interaction::ResizingHorizontally,
            }
        } else {
            mouse::Interaction::default()
        };
        let second_layout = children
            .next()
            .expect("Graphics: Layout should have a second layout");
        let second_mouse_interaction = self.second.as_widget().mouse_interaction(
            &state.children[1],
            second_layout,
            cursor_position,
            viewport,
            renderer,
        );
        first_mouse_interaction
            .max(second_mouse_interaction)
            .max(divider_mouse_interaction)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let split_state: &State = state.state.downcast_ref();
        // TODO: clipping!
        let mut children = layout.children();

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 0.0,
                border_width: theme.active(self.style).border_width,
                border_color: theme.active(self.style).border_color,
            },
            theme
                .active(self.style)
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
                theme.hovered(self.style).first_background
            } else {
                theme.active(self.style).first_background
            }
            .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        self.first.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            first_layout,
            cursor_position,
            viewport,
        );

        let divider_layout = children
            .next()
            .expect("Graphics: Layout should have a divider layout");

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
                theme.hovered(self.style).second_background
            } else {
                theme.active(self.style).second_background
            }
            .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        self.second.as_widget().draw(
            &state.children[1],
            renderer,
            theme,
            style,
            second_layout,
            cursor_position,
            viewport,
        );

        // Divider
        let divider_style = if split_state.dragging {
            theme.dragged(self.style)
        } else if divider_layout.bounds().contains(cursor_position) {
            theme.hovered(self.style)
        } else {
            theme.active(self.style)
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

    fn overlay<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'b, Message, Renderer>> {
        let mut children = layout.children();
        let first_layout = children.next()?;
        let _divider_layout = children.next()?;
        let second_layout = children.next()?;

        let first = &self.first;
        let second = &self.second;

        // Not pretty but works to get two mutable references
        // https://stackoverflow.com/a/30075629
        let (first_state, second_state) = state.children.split_at_mut(1);

        first
            .as_widget()
            .overlay(&mut first_state[0], first_layout, renderer)
            .or_else(|| {
                second
                    .as_widget()
                    .overlay(&mut second_state[0], second_layout, renderer)
            })
    }
}

/// Do a horizontal split.
fn horizontal_split<'a, Message, Renderer>(
    split: &Split<'a, Message, Renderer>,
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    space: &iced_native::layout::Node,
) -> iced_native::layout::Node
where
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    if space.bounds().height
        < split.spacing + f32::from(split.min_size_first + split.min_size_second)
    {
        return iced_native::layout::Node::with_children(
            space.bounds().size(),
            vec![
                split.first.as_widget().layout(
                    renderer,
                    &limits.clone().shrink(Size::new(0.0, space.bounds().height)),
                ),
                iced_native::layout::Node::new(Size::new(space.bounds().height, split.spacing)),
                split.second.as_widget().layout(
                    renderer,
                    &limits.clone().shrink(Size::new(0.0, space.bounds().width)),
                ),
            ],
        );
    }

    let divider_position = split
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
    let mut first = split.first.as_widget().layout(renderer, &first_limits);
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
    let mut second = split.second.as_widget().layout(renderer, &second_limits);
    second.move_to(Point::new(
        space.bounds().x + split.padding,
        space.bounds().y + f32::from(divider_position) + split.spacing + split.padding,
    ));

    iced_native::layout::Node::with_children(space.bounds().size(), vec![first, divider, second])
}

/// Do a vertical split.
fn vertical_split<'a, Message, Renderer>(
    split: &Split<'a, Message, Renderer>,
    renderer: &Renderer,
    limits: &iced_native::layout::Limits,
    space: &iced_native::layout::Node,
) -> iced_native::layout::Node
where
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    if space.bounds().width
        < split.spacing + f32::from(split.min_size_first + split.min_size_second)
    {
        return iced_native::layout::Node::with_children(
            space.bounds().size(),
            vec![
                split.first.as_widget().layout(
                    renderer,
                    &limits.clone().shrink(Size::new(space.bounds().width, 0.0)),
                ),
                iced_native::layout::Node::new(Size::new(split.spacing, space.bounds().height)),
                split.second.as_widget().layout(
                    renderer,
                    &limits.clone().shrink(Size::new(space.bounds().width, 0.0)),
                ),
            ],
        );
    }

    let divider_position = split
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
    let mut first = split.first.as_widget().layout(renderer, &first_limits);
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
    let mut second = split.second.as_widget().layout(renderer, &second_limits);
    second.move_to(Point::new(
        space.bounds().x + f32::from(divider_position) + split.spacing + split.padding,
        space.bounds().y + split.padding,
    ));

    iced_native::layout::Node::with_children(space.bounds().size(), vec![first, divider, second])
}

impl<'a, Message, Renderer> From<Split<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(split_pane: Split<'a, Message, Renderer>) -> Self {
        Element::new(split_pane)
    }
}

/// The state of a [`Split`](Split).
#[derive(Clone, Debug, Default)]
pub struct State {
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
    pub const fn new() -> Self {
        Self { dragging: false }
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
