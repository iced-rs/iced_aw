//! A dummy widget that draws a quad
//!
//! *This API requires the following crate features to be activated: `quad`*

use iced_core::{
    Border, Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Shadow, Shell,
    Size, Widget,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    renderer, touch,
    widget::{
        Operation,
        tree::{self, Tree},
    },
};

use std::ops::RangeInclusive;

/// Constant Default height of `SlideBar`.
pub const DEFAULT_HEIGHT: f32 = 30.0;

/// A widget that draws a `SlideBar`
#[allow(missing_debug_implementations)]
pub struct SlideBar<'a, T, Message>
where
    Message: Clone,
{
    /// Width of the bar
    pub width: Length,
    /// Height of the bar
    pub height: Option<Length>,
    /// Color of the bar
    pub color: Color,
    /// Background color of the bar
    pub background: Option<Color>,
    /// Border radius of the bar
    pub border_radius: f32,
    /// Border width of the bar
    pub border_width: f32,
    /// Border color of the bar
    pub border_color: Color,
    /// value Range
    pub range: RangeInclusive<T>,
    /// smallest value within moveable limitations.
    step: T,
    /// Value of the bar
    value: T,
    /// Change event of the bar when a value is modified
    on_change: Box<dyn Fn(T) -> Message + 'a>,
    /// Release event when the mouse is released.
    on_release: Option<Message>,
}

impl<'a, T, Message> SlideBar<'a, T, Message>
where
    T: Copy + From<u8> + std::cmp::PartialOrd,
    Message: Clone,
{
    /// Creates a new [`SlideBar`].
    ///
    /// It expects:
    ///   * an inclusive range of possible values
    ///   * the current value of the [`SlideBar`]
    ///   * a function that will be called when the [`SlideBar`] is dragged.
    ///   * the new value of the [`SlideBar`] and must produce a `Message`.
    ///
    pub fn new<F>(range: RangeInclusive<T>, value: T, on_change: F) -> Self
    where
        F: 'a + Fn(T) -> Message,
    {
        let value = if value >= *range.start() {
            value
        } else {
            *range.start()
        };

        let value = if value <= *range.end() {
            value
        } else {
            *range.end()
        };

        Self {
            width: Length::Fill,
            height: None,
            color: Color::from([0.5; 3]),
            background: None,
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            step: T::from(1),
            value,
            range,
            on_change: Box::new(on_change),
            on_release: None,
        }
    }

    /// Sets the release message of the [`SlideBar`].
    /// This is called when the mouse is released from the slider.
    ///
    /// Typically, the user's interaction with the slider is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the slider's result, where
    /// the default `on_change` message could create too many events.
    #[must_use]
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }

    /// Sets the width of the [`SlideBar`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`SlideBar`].
    #[must_use]
    pub fn height(mut self, height: Option<Length>) -> Self {
        self.height = height;
        self
    }

    /// Sets the step size of the [`SlideBar`].
    #[must_use]
    pub fn step(mut self, step: impl Into<T>) -> Self {
        self.step = step.into();
        self
    }
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for SlideBar<'_, T, Message>
where
    T: Copy + Into<f64> + num_traits::FromPrimitive,
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height.unwrap_or(Length::Fixed(DEFAULT_HEIGHT)),
        }
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits
            .width(self.width)
            .height(self.height.unwrap_or(Length::Fixed(DEFAULT_HEIGHT)));

        let size = limits.resolve(
            self.width,
            self.height.unwrap_or(Length::Fixed(DEFAULT_HEIGHT)),
            Size::ZERO,
        );

        Node::new(size)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        update(
            event,
            layout,
            cursor,
            shell,
            tree.state.downcast_mut::<State>(),
            &mut self.value,
            &self.range,
            self.step,
            self.on_change.as_ref(),
            &self.on_release,
        );
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        // Register the widget's bounds for testing
        operation.container(None, layout.bounds());
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        draw(renderer, layout, viewport, self);
    }
}

/// Processes an [`Event`] and updates the [`State`] of a [`SlideBar`]
/// accordingly.
#[allow(clippy::too_many_arguments)]
pub fn update<Message, T>(
    event: &Event,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
    state: &mut State,
    value: &mut T,
    range: &RangeInclusive<T>,
    step: T,
    on_change: &dyn Fn(T) -> Message,
    on_release: &Option<Message>,
) where
    T: Copy + Into<f64> + num_traits::FromPrimitive,
    Message: Clone,
{
    let is_dragging = state.is_dragging;

    let mut change = |cursor_position: Point| {
        let bounds = layout.bounds();
        let new_value = if cursor_position.x <= bounds.x {
            *range.start()
        } else if cursor_position.x >= bounds.x + bounds.width {
            *range.end()
        } else {
            let step = step.into();
            let start = (*range.start()).into();
            let end = (*range.end()).into();

            let percent = f64::from(cursor_position.x - bounds.x) / f64::from(bounds.width);

            let steps = (percent * (end - start) / step).round();
            let value = steps * step + start;

            if let Some(value) = T::from_f64(value) {
                value
            } else {
                return;
            }
        };

        if ((*value).into() - new_value.into()).abs() > f64::EPSILON {
            shell.publish((on_change)(new_value));

            *value = new_value;
        }
    };

    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerPressed { .. }) => {
            if let Some(cursor_position) = cursor.position_over(layout.bounds()) {
                change(cursor_position);
                state.is_dragging = true;

                shell.capture_event();
            }
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
            if is_dragging {
                if let Some(on_release) = on_release.clone() {
                    shell.publish(on_release);
                }
                state.is_dragging = false;

                shell.capture_event();
            }
        }
        Event::Mouse(mouse::Event::CursorMoved { .. })
        | Event::Touch(touch::Event::FingerMoved { .. }) => {
            if is_dragging {
                let _ = cursor.position().map(change);

                shell.capture_event();
            }
        }
        _ => {}
    }
}

/// Draws a [`SlideBar`].
pub fn draw<T, R, Message>(
    renderer: &mut R,
    layout: Layout<'_>,
    viewport: &Rectangle,
    slider: &SlideBar<T, Message>,
) where
    T: Into<f64> + Copy,
    Message: Clone,
    R: renderer::Renderer,
{
    let bounds = layout.bounds();
    let value = slider.value.into() as f32;
    let (range_start, range_end) = {
        let (start, end) = slider.range.clone().into_inner();

        (start.into() as f32, end.into() as f32)
    };

    let active_progress_bounds = if range_start >= range_end {
        Rectangle {
            width: 0.0,
            ..bounds
        }
    } else {
        Rectangle {
            width: bounds.width * (value - range_start) / (range_end - range_start),
            ..bounds
        }
    };

    let background = slider.background.unwrap_or_else(|| Color::from([1.0; 3]));

    if bounds.intersects(viewport) {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: slider.border_radius.into(),
                    width: slider.border_width,
                    color: slider.border_color,
                },
                shadow: Shadow::default(),
                ..Default::default()
            },
            background,
        );
    }

    if active_progress_bounds.intersects(viewport) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: active_progress_bounds,
                border: Border {
                    radius: slider.border_radius.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
                ..Default::default()
            },
            slider.color,
        );
    }
}

impl<'a, T, Message, Theme, Renderer> From<SlideBar<'a, T, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    T: 'a + Copy + Into<f64> + num_traits::FromPrimitive,
    Renderer: 'a + renderer::Renderer,
    Message: 'a + Clone,
    Theme: 'a,
{
    fn from(value: SlideBar<'a, T, Message>) -> Self {
        Self::new(value)
    }
}

/// The local state of a [`SlideBar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_dragging: bool,
}

impl State {
    /// Creates a new [`State`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    enum Message {
        Changed(#[allow(dead_code)] u32),
    }

    #[test]
    fn test_slide_bar_new() {
        let slider = SlideBar::new(0..=100, 50, Message::Changed);
        assert_eq!(slider.value, 50);
        assert_eq!(*slider.range.start(), 0);
        assert_eq!(*slider.range.end(), 100);
    }

    #[test]
    fn test_slide_bar_value_clamped_to_range() {
        // Value below range
        let slider = SlideBar::new(10..=100, 5, Message::Changed);
        assert_eq!(slider.value, 10);

        // Value above range
        let slider = SlideBar::new(0..=50, 100, Message::Changed);
        assert_eq!(slider.value, 50);

        // Value within range
        let slider = SlideBar::new(0..=100, 50, Message::Changed);
        assert_eq!(slider.value, 50);
    }

    #[test]
    fn test_slide_bar_with_step() {
        let slider = SlideBar::new(0u32..=100, 50, Message::Changed).step(10u32);
        assert_eq!(slider.step, 10);
    }

    #[test]
    fn test_slide_bar_with_on_release() {
        let slider = SlideBar::new(0..=100, 50, Message::Changed).on_release(Message::Changed(0));
        assert!(slider.on_release.is_some());
    }

    #[test]
    fn test_slide_bar_with_width() {
        let slider = SlideBar::new(0..=100, 50, Message::Changed).width(Length::Fixed(300.0));
        assert_eq!(slider.width, Length::Fixed(300.0));
    }

    #[test]
    fn test_slide_bar_with_height() {
        let slider = SlideBar::new(0..=100, 50, Message::Changed).height(Some(Length::Fixed(50.0)));
        assert_eq!(slider.height, Some(Length::Fixed(50.0)));
    }

    #[test]
    fn test_state_new() {
        let state = State::new();
        assert!(!state.is_dragging);
    }

    #[test]
    fn test_state_default() {
        let state = State::default();
        assert!(!state.is_dragging);
    }
}
