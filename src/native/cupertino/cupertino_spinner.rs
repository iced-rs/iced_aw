use iced_graphics::{Backend, Renderer};
use iced_native::event::Status;
use iced_native::layout::{Limits, Node};
use iced_native::renderer::Style;
use iced_native::widget::{
    tree::{State, Tag},
    Tree,
};

use iced_native::{
    window, Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Shell, Size,
    Vector, Widget,
};

use iced_graphics::widget::canvas::{stroke, Cache, Geometry, LineCap, Path, Stroke};

use std::f32::consts::PI;

const HAND_COUNT: usize = 8;
const ALPHAS: [u16; 8] = [47, 47, 47, 47, 72, 97, 122, 147];

/**
 * `CupertinoSpinner`
 *
 * See
 *
 * 1. [Flutter Activity Indicator](https://github.com/flutter/flutter/blob/0b451b6dfd6de73ff89d89081c33d0f971db1872/packages/flutter/lib/src/cupertino/activity_indicator.dart)
 * 2. [Flutter Cupertino Widgets](https://docs.flutter.dev/development/ui/widgets/cupertino)
 *
 * for reference. The Flutter source is used for constants. The implementation for this widget
 * pulls together ideas from:
 *
 *     1. the mainline Clock example
 *     2. the existing `iced_aw` Spinner
 *     3. the Flutter Activity Indicator above
 *     4. the QR Code widget
 *
 * See the examples folder (`examples/cupertino/cupertino_spinner`) for a full example of usage.
 *
 */
#[allow(missing_debug_implementations)]
#[derive(Debug)]
pub struct CupertinoSpinner {
    width: Length,
    height: Length,
    radius: f32,
}

struct SpinnerState {
    now: time::OffsetDateTime,
    spinner: Cache,
}

impl Default for CupertinoSpinner {
    fn default() -> Self {
        Self {
            width: Length::Fixed(20.0),
            height: Length::Fixed(20.0),
            radius: 20.0,
        }
    }
}

impl CupertinoSpinner {
    /// Creates a new [`CupertinoSpinner`] widget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the width of the [`CupertinoSpinner`](CupertinoSpinner).
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`CupertinoSpinner`](CupertinoSpinner).
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the radius of the [`CupertinoSpinner`](CupertinoSpinner).
    /// NOTE: While you _can_ tweak the radius, the scale may be all out of whack if not using a
    /// number close to the default of `20.0`.
    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl<Message, B, T> Widget<Message, Renderer<B, T>> for CupertinoSpinner
where
    B: Backend,
{
    fn width(&self) -> Length {
        self.width
    }
    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer<B, T>, limits: &Limits) -> Node {
        Node::new(
            limits
                .width(self.width)
                .height(self.height)
                .resolve(Size::new(f32::INFINITY, f32::INFINITY)),
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer<B, T>,
        _theme: &T,
        _style: &Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let state: &SpinnerState = state.state.downcast_ref::<SpinnerState>();

        let spinner: Geometry = state.spinner.draw(layout.bounds().size(), |frame| {
            let center = frame.center();
            let radius = self.radius;
            let width = radius / 5.0;

            let mut hands: Vec<(Path, _)> = vec![];

            for i in ALPHAS {
                hands.push((
                    Path::line(Point::new(0.0, radius / 3.0), Point::new(0.0, radius / 1.5)),
                    move || -> Stroke {
                        // The `60.0` is to shift the original black to dark grey //
                        gen_stroke(
                            width,
                            Color::from_rgba(0.0, 0.0, 0.0, f32::from(i) / (60.0 + 147.0)),
                        )
                    },
                ));
            }

            frame.translate(Vector::new(center.x, center.y));

            // Populate the spinner with 8 hands and make them spin! //
            // The `(HAND_COUNT - i - 1)` block is to make the spinning
            // clockwise. For counterclockwise, leave it at `i`.
            frame.with_save(|frame| {
                let new_index: usize = (state.now.millisecond() / 125 % 8) as usize;

                for i in 0..HAND_COUNT {
                    frame.rotate(hand_rotation(45, 360));

                    frame.stroke(
                        &hands[i].0,
                        hands[((HAND_COUNT - i - 1) + new_index) % 8].1(),
                    );
                }
            });
            //
        });

        renderer.draw_primitive(spinner.into_primitive());
    }

    fn tag(&self) -> Tag {
        Tag::of::<SpinnerState>()
    }

    fn state(&self) -> State {
        State::new(SpinnerState {
            now: time::OffsetDateTime::now_local()
                .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            spinner: Cache::default(),
        })
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _renderer: &Renderer<B, T>,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> Status {
        let state: &mut SpinnerState = state.state.downcast_mut::<SpinnerState>();

        if let Event::Window(window::Event::RedrawRequested(_now)) = &event {
            // if is_visible(&bounds) {
            state.now = time::OffsetDateTime::now_local()
                .unwrap_or_else(|_| time::OffsetDateTime::now_utc());

            state.spinner.clear();
            shell.request_redraw(window::RedrawRequest::NextFrame);
            return Status::Captured;
        }

        Status::Ignored
    }
}

impl<'a, Message, B, T> From<CupertinoSpinner> for Element<'a, Message, Renderer<B, T>>
where
    B: Backend,
{
    fn from(spinner: CupertinoSpinner) -> Self {
        Self::new(spinner)
    }
}

fn gen_stroke(width: f32, color: Color) -> Stroke<'static> {
    Stroke {
        width,
        style: stroke::Style::Solid(color),
        line_cap: LineCap::Round,
        ..Stroke::default()
    }
}

const K: f32 = PI * 2.0;

fn hand_rotation(n: u16, total: u16) -> f32 {
    let turns = f32::from(n) / f32::from(total);

    K * turns
}
