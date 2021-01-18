//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: time_picker*
use std::hash::Hash;

use crate::{
    core::clock::{
        NearestRadius, HOUR_RADIUS_PERCENTAGE, HOUR_RADIUS_PERCENTAGE_NO_SECONDS,
        MINUTE_RADIUS_PERCENTAGE, MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, PERIOD_PERCENTAGE,
        SECOND_RADIUS_PERCENTAGE,
    },
    core::{renderer::DrawEnvironment, time::Period},
    graphics::icons::Icon,
    native::{
        icon_text,
        time_picker::{State, Time},
        IconText,
    },
};
use chrono::{Duration, NaiveTime, Timelike};
use iced_graphics::{canvas, Size};
use iced_native::{
    button, column, container, event,
    layout::{self, Limits},
    mouse, overlay, row, text, Align, Button, Clipboard, Column, Container, Element, Event, Layout,
    Length, Point, Row, Text, Widget,
};

const PADDING: u16 = 10;
const SPACING: u16 = 15;
const BUTTON_SPACING: u16 = 5;

/// The overlay of the [`TimePicker`](crate::native::TimePicker).
#[allow(missing_debug_implementations)]
pub struct TimePickerOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + button::Renderer,
{
    time: &'a mut NaiveTime,
    clock_cache_needs_clearance: &'a mut bool,
    clock_cache: &'a mut canvas::Cache,
    cancel_button: Element<'a, Message, Renderer>,
    submit_button: Element<'a, Message, Renderer>,
    on_submit: &'a dyn Fn(Time) -> Message,
    use_24h: bool,
    show_seconds: bool,
    position: Point,
    style: &'a <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer> TimePickerOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a
        + self::Renderer
        + button::Renderer
        + column::Renderer
        + container::Renderer
        + icon_text::Renderer
        + row::Renderer
        + text::Renderer,
{
    /// Creates a new [`TimePickerOverlay`](TimePickerOverlay) on the given
    /// position.
    pub fn new(
        state: &'a mut State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Time) -> Message,
        use_24h: bool,
        show_seconds: bool,
        position: Point,
        style: &'a <Renderer as self::Renderer>::Style,
    ) -> Self {
        let State {
            time,
            cancel_button,
            submit_button,
            clock_cache_needs_clearance,
            clock_cache,
            ..
        } = state;

        TimePickerOverlay {
            time,
            clock_cache_needs_clearance,
            clock_cache,
            cancel_button: Button::new(cancel_button, IconText::new(Icon::X).width(Length::Fill))
                .width(Length::Fill)
                .on_press(on_cancel.clone())
                .into(),
            submit_button: Button::new(
                submit_button,
                IconText::new(Icon::Check).width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel) // Sending a fake message
            .into(),
            on_submit,
            use_24h,
            show_seconds,
            position,
            style,
        }
    }

    /// Turn this [`TimePickerOverlay`](TimePickerOverlay) into an overlay
    /// [`Element`](overlay::Element).
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(self.position, Box::new(self))
    }

    /// The event handling for the clock.
    fn on_event_clock(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut clock_status = event::Status::Ignored;
        if layout.bounds().contains(cursor_position) {
            *self.clock_cache_needs_clearance = true;
            self.clock_cache.clear();
        } else if *self.clock_cache_needs_clearance {
            self.clock_cache.clear();
            *self.clock_cache_needs_clearance = false;
        }

        let clock_bounds = layout.bounds();
        if clock_bounds.contains(cursor_position) {
            let center = clock_bounds.center();
            let radius = clock_bounds.width.min(clock_bounds.height) * 0.5;

            let period_radius = radius * PERIOD_PERCENTAGE;

            let (hour_radius, minute_radius, second_radius) = if self.show_seconds {
                (
                    radius * HOUR_RADIUS_PERCENTAGE,
                    radius * MINUTE_RADIUS_PERCENTAGE,
                    radius * SECOND_RADIUS_PERCENTAGE,
                )
            } else {
                (
                    radius * HOUR_RADIUS_PERCENTAGE_NO_SECONDS,
                    radius * MINUTE_RADIUS_PERCENTAGE_NO_SECONDS,
                    f32::MAX,
                )
            };

            let nearest_radius = crate::core::clock::nearest_radius(
                &if self.show_seconds {
                    vec![
                        (period_radius, NearestRadius::Period),
                        (hour_radius, NearestRadius::Hour),
                        (minute_radius, NearestRadius::Minute),
                        (second_radius, NearestRadius::Second),
                    ]
                } else {
                    vec![
                        (period_radius, NearestRadius::Period),
                        (hour_radius, NearestRadius::Hour),
                        (minute_radius, NearestRadius::Minute),
                    ]
                },
                cursor_position,
                center,
            );

            clock_status = match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    match nearest_radius {
                        NearestRadius::Period => {
                            let (pm, hour) = self.time.hour12();
                            let hour = if hour == 12 {
                                if pm {
                                    12
                                } else {
                                    0
                                }
                            } else {
                                hour
                            };

                            *self.time = self
                                .time
                                .with_hour(if pm && hour != 12 { hour } else { hour + 12 } % 24)
                                .unwrap();
                            event::Status::Captured
                        }
                        NearestRadius::Hour => {
                            let hour_points =
                                crate::core::clock::circle_points(hour_radius, center, 12);
                            let nearest_point =
                                crate::core::clock::nearest_point(&hour_points, cursor_position);

                            let (pm, _) = self.time.hour12();

                            *self.time = self
                                .time
                                .with_hour((nearest_point as u32 + if pm { 12 } else { 0 }) % 24)
                                .unwrap();
                            event::Status::Captured
                        }
                        NearestRadius::Minute => {
                            let minute_points =
                                crate::core::clock::circle_points(minute_radius, center, 60);
                            let nearest_point =
                                crate::core::clock::nearest_point(&minute_points, cursor_position);

                            *self.time = self.time.with_minute(nearest_point as u32).unwrap();
                            event::Status::Captured
                        }
                        NearestRadius::Second => {
                            let second_points =
                                crate::core::clock::circle_points(second_radius, center, 60);
                            let nearest_point =
                                crate::core::clock::nearest_point(&second_points, cursor_position);

                            *self.time = self.time.with_second(nearest_point as u32).unwrap();
                            event::Status::Captured
                        }
                        _ => event::Status::Ignored,
                    }
                }
                _ => event::Status::Ignored,
            };
        }

        clock_status
    }

    /// The event handling for the digital clock.
    fn on_event_digital_clock(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut digital_clock_children = layout.children();

        if !self.use_24h {
            // Placeholder
            let _ = digital_clock_children.next();
        }

        let hour_layout = digital_clock_children.next().unwrap();
        let mut hour_children = hour_layout.children();

        let hour_up_arrow = hour_children.next().unwrap();
        let _ = hour_children.next();
        let hour_down_arrow = hour_children.next().unwrap();

        let _ = digital_clock_children.next();

        let minute_layout = digital_clock_children.next().unwrap();
        let mut minute_children = minute_layout.children();

        let minute_up_arrow = minute_children.next().unwrap();
        let _ = minute_children.next();
        let minute_down_arrow = minute_children.next().unwrap();

        let digital_clock_status = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let mut status = event::Status::Ignored;

                if hour_up_arrow.bounds().contains(cursor_position) {
                    *self.time = self.time.overflowing_add_signed(Duration::hours(1)).0;

                    status = event::Status::Captured;
                }
                if hour_down_arrow.bounds().contains(cursor_position) {
                    *self.time = self.time.overflowing_sub_signed(Duration::hours(1)).0;

                    status = event::Status::Captured;
                }

                if minute_up_arrow.bounds().contains(cursor_position) {
                    *self.time = self.time.overflowing_add_signed(Duration::minutes(1)).0;

                    status = event::Status::Captured;
                }
                if minute_down_arrow.bounds().contains(cursor_position) {
                    *self.time = self.time.overflowing_sub_signed(Duration::minutes(1)).0;

                    status = event::Status::Captured;
                }

                status
            }
            _ => event::Status::Ignored,
        };

        let second_status = if self.show_seconds {
            let _ = digital_clock_children.next();

            let second_layout = digital_clock_children.next().unwrap();
            let mut second_children = second_layout.children();

            let second_up_arrow = second_children.next().unwrap();
            let _ = second_children.next();
            let second_down_arrow = second_children.next().unwrap();

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    let mut status = event::Status::Ignored;

                    if second_up_arrow.bounds().contains(cursor_position) {
                        *self.time = self.time.overflowing_add_signed(Duration::seconds(1)).0;

                        status = event::Status::Captured;
                    }
                    if second_down_arrow.bounds().contains(cursor_position) {
                        *self.time = self.time.overflowing_sub_signed(Duration::seconds(1)).0;

                        status = event::Status::Captured;
                    }

                    status
                }
                _ => event::Status::Ignored,
            }
        } else {
            event::Status::Ignored
        };

        let digital_clock_status = digital_clock_status.merge(second_status);

        if digital_clock_status == event::Status::Captured {
            self.clock_cache.clear()
        }

        digital_clock_status
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for TimePickerOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a
        + self::Renderer
        + button::Renderer
        + column::Renderer
        + container::Renderer
        + icon_text::Renderer
        + row::Renderer
        + text::Renderer,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .pad(PADDING as f32)
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(300)
            .max_height(350);

        let arrow_size = text::Renderer::default_size(renderer);
        let font_size = (1.2 * (text::Renderer::default_size(renderer) as f32)) as u16;

        // Digital Clock
        let digital_clock_limits = limits;

        let mut digital_clock_row = Row::<(), Renderer>::new()
            .align_items(Align::Center)
            .height(Length::Shrink)
            .width(Length::Shrink)
            .spacing(1);

        if !self.use_24h {
            digital_clock_row = digital_clock_row.push(
                Column::new() // Just a placeholder
                    .height(Length::Shrink)
                    .push(Text::new("AM").size(font_size)),
            )
        }

        digital_clock_row = digital_clock_row
            .push(
                // Hour
                Column::new()
                    .align_items(Align::Center)
                    .height(Length::Shrink)
                    .push(
                        // Up Hour arrow
                        Row::new()
                            .width(Length::Units(arrow_size))
                            .height(Length::Units(arrow_size)),
                    )
                    .push(Text::new(format!("{:02}", self.time.hour())).size(font_size))
                    .push(
                        // Down Hour arrow
                        Row::new()
                            .width(Length::Units(arrow_size))
                            .height(Length::Units(arrow_size)),
                    ),
            )
            .push(
                Column::new()
                    .height(Length::Shrink)
                    .push(Text::new(":").size(font_size)),
            )
            .push(
                Column::new()
                    .align_items(Align::Center)
                    .height(Length::Shrink)
                    .push(
                        // Up Minute arrow
                        Row::new()
                            .width(Length::Units(arrow_size))
                            .height(Length::Units(arrow_size)),
                    )
                    .push(Text::new(format!("{:02}", self.time.hour())).size(font_size))
                    .push(
                        // Down Minute arrow
                        Row::new()
                            .width(Length::Units(arrow_size))
                            .height(Length::Units(arrow_size)),
                    ),
            );

        if self.show_seconds {
            digital_clock_row = digital_clock_row
                .push(
                    Column::new()
                        .height(Length::Shrink)
                        .push(Text::new(":").size(font_size)),
                )
                .push(
                    Column::new()
                        .align_items(Align::Center)
                        .height(Length::Shrink)
                        .push(
                            // Up Minute arrow
                            Row::new()
                                .width(Length::Units(arrow_size))
                                .height(Length::Units(arrow_size)),
                        )
                        .push(Text::new(format!("{:02}", self.time.hour())).size(font_size))
                        .push(
                            // Down Minute arrow
                            Row::new()
                                .width(Length::Units(arrow_size))
                                .height(Length::Units(arrow_size)),
                        ),
                )
        }

        if !self.use_24h {
            digital_clock_row = digital_clock_row.push(
                Column::new()
                    .height(Length::Shrink)
                    .push(Text::new("AM").size(font_size)),
            );
        }

        let mut digital_clock = Container::new(digital_clock_row)
            .width(Length::Fill)
            .height(Length::Shrink)
            .center_x()
            .center_y()
            .layout(renderer, &digital_clock_limits);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(
            0.0,
            digital_clock.bounds().height + cancel_button.bounds().height + 2.0 * SPACING as f32,
        ));

        // Clock-Canvas
        let mut clock = Row::<(), Renderer>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .layout(renderer, &limits);

        clock.move_to(Point::new(
            clock.bounds().x + PADDING as f32,
            clock.bounds().y + PADDING as f32,
        ));

        digital_clock.move_to(Point::new(
            digital_clock.bounds().x + PADDING as f32,
            digital_clock.bounds().y + PADDING as f32 + SPACING as f32 + clock.bounds().height,
        ));

        // Buttons
        let cancel_limits = limits
            .clone()
            .max_width(((clock.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32);

        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let submit_limits = limits
            .clone()
            .max_width(((clock.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32);

        let mut submit_button = self.submit_button.layout(renderer, &submit_limits);

        cancel_button.move_to(Point {
            x: cancel_button.bounds().x + PADDING as f32,
            y: cancel_button.bounds().y
                + clock.bounds().height
                + PADDING as f32
                + digital_clock.bounds().height
                + 2.0 * SPACING as f32,
        });

        submit_button.move_to(Point {
            x: submit_button.bounds().x + clock.bounds().width - submit_button.bounds().width
                + PADDING as f32,
            y: submit_button.bounds().y
                + clock.bounds().height
                + PADDING as f32
                + digital_clock.bounds().height
                + 2.0 * SPACING as f32,
        });

        let mut node = layout::Node::with_children(
            Size::new(
                clock.bounds().width + (2.0 * PADDING as f32),
                clock.bounds().height
                    + digital_clock.bounds().height
                    + cancel_button.bounds().height
                    + (2.0 * PADDING as f32)
                    + 2.0 * SPACING as f32,
            ),
            vec![clock, digital_clock, cancel_button, submit_button],
        );

        node.move_to(Point::new(
            (position.x - node.size().width / 2.0).max(0.0),
            (position.y - node.size().height / 2.0).max(0.0),
        ));

        node.move_to(Point::new(
            if node.bounds().x + node.bounds().width > bounds.width {
                (node.bounds().x - (node.bounds().width - (bounds.width - node.bounds().x)))
                    .max(0.0)
            } else {
                node.bounds().x
            },
            //node.bounds().x,
            if node.bounds().y + node.bounds().height > bounds.height {
                (node.bounds().y - (node.bounds().height - (bounds.height - node.bounds().y)))
                    .max(0.0)
            } else {
                node.bounds().y
            },
        ));

        node
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
        let mut children = layout.children();

        // Clock canvas
        let clock_layout = children.next().unwrap();
        let clock_status = self.on_event_clock(
            event.clone(),
            clock_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Digital clock ------------------
        let digital_clock_layout = children.next().unwrap().children().next().unwrap();
        let digital_clock_status = self.on_event_digital_clock(
            event.clone(),
            digital_clock_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Buttons ------------------------
        let cancel_button_layout = children.next().unwrap();

        let cancel_status = self.cancel_button.on_event(
            event.clone(),
            cancel_button_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        let submit_button_layout = children.next().unwrap();

        let mut fake_messages: Vec<Message> = Vec::new();

        let submit_status = self.submit_button.on_event(
            event,
            submit_button_layout,
            cursor_position,
            //messages,
            &mut fake_messages,
            renderer,
            clipboard,
        );

        if !fake_messages.is_empty() {
            let (hour, period) = if self.use_24h {
                (self.time.hour(), Period::H24)
            } else {
                let (period, hour) = self.time.hour12();
                (hour, if period { Period::Pm } else { Period::Am })
            };

            let time = if self.show_seconds {
                Time::Hms {
                    hour,
                    minute: self.time.minute(),
                    second: self.time.second(),
                    period,
                }
            } else {
                Time::Hm {
                    hour,
                    minute: self.time.minute(),
                    period,
                }
            };

            messages.push((self.on_submit)(time))
        }

        clock_status
            .merge(digital_clock_status)
            .merge(cancel_status)
            .merge(submit_status)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        <Renderer as self::Renderer>::draw(
            renderer,
            DrawEnvironment {
                defaults,
                layout,
                cursor_position,
                style_sheet: &self.style,
                viewport: None,
            },
            &self.time,
            &self.clock_cache,
            &self.cancel_button,
            &self.submit_button,
            self.use_24h,
            self.show_seconds,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher, position: Point) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
    }
}

/// The renderer of a [`TimePickerOverlay`](TimePickerOverlay).
///
/// Your renderer fill need to implement this trait before being
/// able to use a [`TimePicker`](crate::native::TimePicker) in your user
/// interface.
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`TimePickerOverlay`](TimePickerOverlay).
    #[allow(clippy::too_many_arguments)]
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<Self::Defaults, Self::Style>,
        time: &NaiveTime,
        clock_cache: &canvas::Cache,
        cancel_button: &Element<'_, Message, Self>,
        submit_button: &Element<'_, Message, Self>,
        use_24h: bool,
        show_seconds: bool,
    ) -> Self::Output;
}

#[cfg(debug_assertions)]
impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<Self::Defaults, Self::Style>,
        _time: &NaiveTime,
        _clock_cache: &canvas::Cache,
        _cancel_button: &Element<'_, Message, Self>,
        _submit_button: &Element<'_, Message, Self>,
        _use_24h: bool,
        _show_seconds: bool,
    ) -> Self::Output {
    }
}
