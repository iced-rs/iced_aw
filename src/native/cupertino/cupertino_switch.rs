use crate::native::cupertino::cupertino_colors::{secondary_system_fill, system_green};

use iced_widget::{
    canvas::{Cache, Fill, Geometry, Path},
    core::{
        event,
        layout::{Limits, Node},
        mouse::{self, Cursor},
        overlay, renderer, touch,
        widget::tree::{State, Tag, Tree},
        window, Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Renderer as _,
        Shell, Size, Vector, Widget,
    },
    graphics::geometry::Renderer as _,
    renderer::Renderer,
};

/**
 * `CupertinoSwitch`
 *
 * See
 *
 * 1. [Flutter Cupertino Switch](https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/cupertino/switch.dart)
 * 2. [Flutter Cupertino Widgets](https://docs.flutter.dev/development/ui/widgets/cupertino)
 *
 * (1) for a couple constants, and colours.
 *
 * The examples folder (`examples/cupertino/cupertino_switch`) has a full example of usage.
 *
 */
#[allow(missing_debug_implementations)]
pub struct CupertinoSwitch<Message>
where
    Message: Clone,
{
    width: Length,
    height: Length,
    active_colour: Color,
    focus_colour: Color,
    thumb_colour: Color,
    track_colour: Color,
    apply_theme: bool, // TODO //
    on_changed: Option<Box<dyn Fn(bool) -> Message>>,

    /// The `CupertinoSwitch`'s value (true or false)
    pub value: bool,
    // drag_start_behaviour: bool, // TODO //
}

// A note about constants:
// -----------------------
// Currently, this widget is not dynamic in `width` and `height`. Making it
// dynamic in size would affect the `draw` and `on_event` methods.
//
// 1) The sizes of the rectangle and circles would have to change ( `draw` )
// 2) The frame count may need to change ( `draw` )
// 3) The "hit box" for the button would have to change ( `on_event` )
//
const ANIMATION_FRAME_COUNT: usize = 40;

#[derive(Debug)]
struct SwitchState {
    animation_frame: usize,
    bounds: Rectangle,
    prev_value: bool,
    published: bool,
    switch: Cache,
    toggle_staged: bool,
}

impl<Message> Default for CupertinoSwitch<Message>
where
    Message: Clone,
{
    fn default() -> Self {
        Self {
            width: Length::Fixed(180.0),
            height: Length::Fixed(180.0),
            active_colour: system_green(1.0),
            focus_colour: system_green(0.8),
            thumb_colour: Color::WHITE,
            track_colour: secondary_system_fill(),
            apply_theme: false,
            on_changed: None,
            value: true,
            // drag_start_behaviour: false, // TODO //
        }
    }
}

impl<Message> CupertinoSwitch<Message>
where
    Message: Clone,
{
    /// Creates a new [`CupertinoSwitch`] widget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the active colour of the [`CupertinoSwitch`].
    #[must_use]
    pub fn active_colour(mut self, colour: Color) -> Self {
        self.active_colour = colour;
        self
    }

    /// Sets `apply_theme` of the [`CupertinoSwitch`].
    #[must_use]
    pub fn apply_theme(mut self, theme: bool) -> Self {
        self.apply_theme = theme;
        self
    }

    /// Sets the focus colour of the [`CupertinoSwitch`].
    #[must_use]
    pub fn focus_colour(mut self, colour: Color) -> Self {
        self.focus_colour = colour;
        self
    }

    /// Sets the height of the [`CupertinoSwitch`].
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets `on_changed` of the [`CupertinoSwitch`].
    #[must_use]
    pub fn on_changed(mut self, on_changed: Option<Box<dyn Fn(bool) -> Message>>) -> Self {
        self.on_changed = on_changed;
        self
    }

    /// Sets the thumb colour of the [`CupertinoSwitch`].
    #[must_use]
    pub fn thumb_colour(mut self, colour: Color) -> Self {
        self.thumb_colour = colour;
        self
    }

    /// Sets the value of the [`CupertinoSwitch`].
    #[must_use]
    pub fn value(mut self, value: bool) -> Self {
        self.value = value;
        self
    }

    /// Sets the width of the [`CupertinoSwitch`].
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<Message, Theme> Widget<Message, Renderer<Theme>> for CupertinoSwitch<Message>
where
    Message: Clone,
{
    fn width(&self) -> Length {
        self.width
    }
    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer<Theme>, limits: &Limits) -> Node {
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
        renderer: &mut Renderer<Theme>,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let state: &SwitchState = state.state.downcast_ref::<SwitchState>();

        // TODO //
        // let width:  f32 = self.width;
        // let height: f32 = self.height;
        let width: f32 = 40.0;
        let height: f32 = 40.0;
        let radius: f32 = width / 2.0;
        let padding: f32 = 1.5;

        let bounds: Rectangle = layout.bounds();

        let switch: Geometry = state.switch.draw(
            renderer,
            Size {
                width: viewport.width,
                height: viewport.height,
            },
            |frame| {
                frame.translate(Vector::new(
                    bounds.x + 2.0 * width - 10.0,
                    bounds.y + 2.0 * width - 15.0,
                ));

                let new_index: usize = state.animation_frame;

                if self.value {
                    frame.fill_rectangle(
                        Point::ORIGIN,
                        Size { width, height },
                        Fill::from(self.active_colour),
                    );

                    frame.fill(
                        &Path::circle(
                            Point {
                                x: width,
                                y: height / 2.0,
                            },
                            radius,
                        ),
                        Fill::from(self.active_colour),
                    );

                    frame.fill(
                        &Path::circle(
                            Point {
                                x: 0.0,
                                y: height / 2.0,
                            },
                            radius,
                        ),
                        Fill::from(self.active_colour),
                    );

                    // Subtract `padding` to leave a slight gap //
                    frame.fill(
                        &Path::circle(
                            Point {
                                x: width - padding - new_index as f32,
                                y: height / 2.0,
                            },
                            radius - padding,
                        ),
                        Fill::from(Color::WHITE),
                    );
                } else {
                    frame.fill_rectangle(
                        Point::ORIGIN,
                        Size { width, height },
                        Fill::from(self.track_colour),
                    );

                    frame.fill(
                        &Path::circle(
                            Point {
                                x: width,
                                y: height / 2.0,
                            },
                            radius,
                        ),
                        Fill::from(self.track_colour),
                    );

                    frame.fill(
                        &Path::circle(
                            Point {
                                x: 0.0,
                                y: height / 2.0,
                            },
                            radius,
                        ),
                        Fill::from(self.track_colour),
                    );

                    // Subtract `padding` to leave a slight gap //
                    frame.fill(
                        &Path::circle(
                            Point {
                                x: 0.0 + padding + new_index as f32,
                                y: height / 2.0,
                            },
                            radius - padding,
                        ),
                        Fill::from(Color::WHITE),
                    );
                }
            },
        );

        // A useful debugging tool for element position... //
        // renderer.draw_primitive(Primitive::Quad {
        //     bounds:        state.bounds,
        //     background:    iced_graphics::Background::Color(Color::TRANSPARENT),
        //     border_radius: [1.0, 1.0, 1.0, 1.0],
        //     border_width:  5.0,
        //     border_color:  Color::BLACK,
        // });
        //

        let translation = Vector::new(0.0, 0.0);
        renderer.with_translation(translation, |renderer| {
            renderer.draw(vec![switch]);
        });
    }

    fn tag(&self) -> Tag {
        Tag::of::<SwitchState>()
    }

    fn state(&self) -> State {
        State::new(SwitchState {
            animation_frame: 0,
            bounds: Rectangle::default(),
            prev_value: self.value,
            published: false,
            switch: Cache::default(),
            toggle_staged: false,
        })
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        _layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer<Theme>,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state: &mut SwitchState = state.state.downcast_mut::<SwitchState>();

        match event {
            Event::Window(_id, window::Event::RedrawRequested(_now)) => {
                if state.toggle_staged {
                    state.animation_frame += 1;

                    // This machinery is built to accommodate for the most bizarre
                    // behaviour that only happens when `shell.publish` is called...
                    if state.published && self.value != state.prev_value {
                        self.value = !self.value;
                    }

                    if state.animation_frame >= ANIMATION_FRAME_COUNT {
                        self.value = !self.value;
                        state.toggle_staged = false;
                        state.animation_frame = 0;
                        state.published = false;
                    }

                    state.switch.clear();
                    shell.request_redraw(window::RedrawRequest::NextFrame);
                }

                return event::Status::Captured;
            }

            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let cur_pos = cursor.position().unwrap_or_default();
                // TODO: Make these calculations not hard-coded //
                let hit_x: bool =
                    ((state.bounds.x + 50.0)..(state.bounds.x + 125.0)).contains(&cur_pos.x);

                let hit_y: bool =
                    ((state.bounds.y + 70.0)..(state.bounds.y + 100.0)).contains(&cur_pos.y);

                if hit_x && hit_y {
                    state.toggle_staged = true;
                    state.animation_frame = 0;

                    if self.on_changed.as_ref().is_some() {
                        shell.publish((self
                            .on_changed
                            .as_ref()
                            .expect("Unable to retrieve the changed message"))(
                            !self.value
                        ));

                        state.prev_value = self.value;
                        state.published = true;
                    }

                    return event::Status::Captured;
                }
            }

            _ => {}
        }

        event::Status::Ignored
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer<Theme>,
    ) -> Option<overlay::Element<'b, Message, Renderer<Theme>>> {
        let state: &mut SwitchState = state.state.downcast_mut::<SwitchState>();

        state.bounds = layout.bounds();

        None
    }
}

impl<'a, Message, Theme> From<CupertinoSwitch<Message>> for Element<'a, Message, Renderer<Theme>>
where
    Message: Clone + 'a,
{
    fn from(switch: CupertinoSwitch<Message>) -> Self {
        Self::new(switch)
    }
}
