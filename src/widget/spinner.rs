//! A spinner to suggest something is loading.
use iced_core::{
    layout::{Limits, Node},
    mouse::Cursor,
    renderer,
    time::{Duration, Instant},
    widget::{
        tree::{State, Tag},
        Tree,
    },
    window, Border, Clipboard, Color, Element, Event, Layout, Length, Rectangle, Shell, Size,
    Vector, Widget,
};

/// A spinner widget, a circle spinning around the center of the widget.
#[allow(missing_debug_implementations)]
pub struct Spinner {
    /// The width of the [`Spinner`].
    width: Length,
    /// The height of the [`Spinner`].
    height: Length,
    /// The rate of the [`Spinner`].
    rate: Duration,
    /// The radius of the spinning circle.
    circle_radius: f32,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            width: Length::Fixed(20.0),
            height: Length::Fixed(20.0),
            rate: Duration::from_secs_f32(1.0),
            circle_radius: 2.0,
        }
    }
}

impl Spinner {
    /// Creates a new [`Spinner`] widget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the width of the [`Spinner`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Spinner`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the circle radius of the spinning circle.
    #[must_use]
    pub fn circle_radius(mut self, radius: f32) -> Self {
        self.circle_radius = radius;
        self
    }
}

struct SpinnerState {
    last_update: Instant,
    t: f32,
}

fn is_visible(bounds: &Rectangle) -> bool {
    bounds.width > 0.0 && bounds.height > 0.0
}

fn fill_circle(
    renderer: &mut impl renderer::Renderer,
    position: Vector,
    radius: f32,
    color: Color,
) {
    if radius > 0. {
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: position.x,
                    y: position.y,
                    width: radius * 2.0,
                    height: radius * 2.0,
                },
                border: Border {
                    radius: radius.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            },
            color,
        );
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Spinner
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.width(self.width).height(self.height).resolve(
            self.width,
            self.height,
            Size::new(f32::INFINITY, f32::INFINITY),
        ))
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        if !is_visible(&bounds) {
            return;
        }

        let size = if bounds.width < bounds.height {
            bounds.width
        } else {
            bounds.height
        } / 2.0;
        let state = state.state.downcast_ref::<SpinnerState>();
        let center = bounds.center();
        let distance_from_center = size - self.circle_radius;
        let (y, x) = (state.t * std::f32::consts::PI * 2.0).sin_cos();
        let position = Vector::new(
            center.x + x * distance_from_center - self.circle_radius,
            center.y + y * distance_from_center - self.circle_radius,
        );

        fill_circle(renderer, position, self.circle_radius, style.text_color);
    }

    fn tag(&self) -> Tag {
        Tag::of::<SpinnerState>()
    }

    fn state(&self) -> State {
        State::new(SpinnerState {
            last_update: Instant::now(),
            t: 0.0,
        })
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        _cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        const FRAMES_PER_SECOND: u64 = 60;

        let bounds = layout.bounds();

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            if is_visible(&bounds) {
                let state = state.state.downcast_mut::<SpinnerState>();
                let duration = (*now - state.last_update).as_secs_f32();
                let increment = if self.rate == Duration::ZERO {
                    0.0
                } else {
                    duration * 1.0 / self.rate.as_secs_f32()
                };

                state.t += increment;

                if state.t > 1.0 {
                    state.t -= 1.0;
                }

                shell.request_redraw_at(window::RedrawRequest::At(
                    *now + Duration::from_millis(1000 / FRAMES_PER_SECOND),
                ));
                state.last_update = *now;
            }
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Spinner> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
{
    fn from(spinner: Spinner) -> Self {
        Self::new(spinner)
    }
}
