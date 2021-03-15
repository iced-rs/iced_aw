//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
use std::hash::Hash;

use crate::{
    core::clock::{
        NearestRadius, HOUR_RADIUS_PERCENTAGE, HOUR_RADIUS_PERCENTAGE_NO_SECONDS,
        MINUTE_RADIUS_PERCENTAGE, MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, PERIOD_PERCENTAGE,
        SECOND_RADIUS_PERCENTAGE,
    },
    core::{overlay::Position, renderer::DrawEnvironment, time::Period},
    graphics::icons::Icon,
    native::{
        icon_text,
        time_picker::{self, Time},
        IconText,
    },
};
use chrono::{Duration, Local, NaiveTime, Timelike};
use iced_graphics::{canvas, Size};
use iced_native::{
    button, column, container, event, keyboard,
    layout::{self, Limits},
    mouse, overlay, row, text, touch, Align, Button, Clipboard, Column, Container, Element, Event,
    Layout, Length, Point, Row, Text, Widget,
};

/// The padding around the elements.
const PADDING: u16 = 10;
/// The spacing between the elements.
const SPACING: u16 = 15;
/// The spacing between the buttons.
const BUTTON_SPACING: u16 = 5;

/// The overlay of the [`TimePicker`](crate::native::TimePicker).
#[allow(missing_debug_implementations)]
pub struct TimePickerOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + button::Renderer,
{
    /// The state of the [`TimePickerOverlay`](TimePickerOverlay).
    state: &'a mut State,
    /// The cancel button of the [`TimePickerOverlay`](TimePickerOverlay).
    cancel_button: Element<'a, Message, Renderer>,
    /// The submit button of the [`TimePickerOverlay`](TimePickerOverlay).
    submit_button: Element<'a, Message, Renderer>,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`] is pressed.
    on_submit: &'a dyn Fn(Time) -> Message,
    /// The position of the [`TimePickerOverlay`](TimePickerOverlay).
    position: Point,
    /// The style of the [`TimePickerOverlay`](TimePickerOverlay).
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
        state: &'a mut time_picker::State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Time) -> Message,
        position: Point,
        style: &'a <Renderer as self::Renderer>::Style,
    ) -> Self {
        let time_picker::State {
            overlay_state,
            cancel_button,
            submit_button,
            ..
        } = state;

        TimePickerOverlay {
            state: overlay_state,
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
            position,
            style,
        }
    }

    /// Turn this [`TimePickerOverlay`](TimePickerOverlay) into an overlay
    /// [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(self.position, Box::new(self))
    }

    /// The event handling for the clock.
    #[allow(clippy::too_many_lines)]
    fn on_event_clock(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
        if layout.bounds().contains(cursor_position) {
            self.state.clock_cache_needs_clearance = true;
            self.state.clock_cache.clear();
        } else if self.state.clock_cache_needs_clearance {
            self.state.clock_cache.clear();
            self.state.clock_cache_needs_clearance = false;
        }

        let clock_bounds = layout.bounds();
        if clock_bounds.contains(cursor_position) {
            let center = clock_bounds.center();
            let radius = clock_bounds.width.min(clock_bounds.height) * 0.5;

            let period_radius = radius * PERIOD_PERCENTAGE;

            let (hour_radius, minute_radius, second_radius) = if self.state.show_seconds {
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
                &if self.state.show_seconds {
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

            let clock_clicked_status = match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => match nearest_radius {
                    NearestRadius::Period => {
                        let (pm, hour) = self.state.time.hour12();
                        let hour = if hour == 12 {
                            if pm {
                                12
                            } else {
                                0
                            }
                        } else {
                            hour
                        };

                        self.state.time = self
                            .state
                            .time
                            .with_hour(if pm && hour != 12 { hour } else { hour + 12 } % 24)
                            .expect("New time with hour should be valid");
                        event::Status::Captured
                    }
                    NearestRadius::Hour => {
                        self.state.focus = Focus::DigitalHour;
                        self.state.clock_dragged = ClockDragged::Hour;
                        event::Status::Captured
                    }
                    NearestRadius::Minute => {
                        self.state.focus = Focus::DigitalMinute;
                        self.state.clock_dragged = ClockDragged::Minute;
                        event::Status::Captured
                    }
                    NearestRadius::Second => {
                        self.state.focus = Focus::DigitalSecond;
                        self.state.clock_dragged = ClockDragged::Second;
                        event::Status::Captured
                    }
                    NearestRadius::None => event::Status::Ignored,
                },
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerLifted { .. })
                | Event::Touch(touch::Event::FingerLost { .. }) => {
                    self.state.clock_dragged = ClockDragged::None;
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            };

            let clock_dragged_status = match self.state.clock_dragged {
                ClockDragged::Hour => {
                    let hour_points = crate::core::clock::circle_points(hour_radius, center, 12);
                    let nearest_point =
                        crate::core::clock::nearest_point(&hour_points, cursor_position);

                    let (pm, _) = self.state.time.hour12();

                    self.state.time = self
                        .state
                        .time
                        .with_hour((nearest_point as u32 + if pm { 12 } else { 0 }) % 24)
                        .expect("New time with hour should be valid");
                    event::Status::Captured
                }
                ClockDragged::Minute => {
                    let minute_points =
                        crate::core::clock::circle_points(minute_radius, center, 60);
                    let nearest_point =
                        crate::core::clock::nearest_point(&minute_points, cursor_position);

                    self.state.time = self
                        .state
                        .time
                        .with_minute(nearest_point as u32)
                        .expect("New time with minute should be valid");
                    event::Status::Captured
                }
                ClockDragged::Second => {
                    let second_points =
                        crate::core::clock::circle_points(second_radius, center, 60);
                    let nearest_point =
                        crate::core::clock::nearest_point(&second_points, cursor_position);

                    self.state.time = self
                        .state
                        .time
                        .with_second(nearest_point as u32)
                        .expect("New time with second should be valid");
                    event::Status::Captured
                }
                ClockDragged::None => event::Status::Ignored,
            };

            clock_clicked_status.merge(clock_dragged_status)
        } else {
            match event {
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerLifted { .. })
                | Event::Touch(touch::Event::FingerLost { .. }) => {
                    self.state.clock_dragged = ClockDragged::None;
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            }
        }
    }

    /// The event handling for the digital clock.
    #[allow(clippy::too_many_lines)]
    fn on_event_digital_clock(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
        let mut digital_clock_children = layout.children();

        if !self.state.use_24h {
            // Placeholder
            let _ = digital_clock_children.next();
        }

        let hour_layout = digital_clock_children
            .next()
            .expect("Native: Layout should have a hour layout");
        let mut hour_children = hour_layout.children();

        let hour_up_arrow = hour_children
            .next()
            .expect("Native: Layout should have an up arrow for hours");
        let _ = hour_children.next();
        let hour_down_arrow = hour_children
            .next()
            .expect("Native: Layout should have a down arrow for hours");

        let _ = digital_clock_children.next();

        let minute_layout = digital_clock_children
            .next()
            .expect("Native: Layout should have a minute layout");
        let mut minute_children = minute_layout.children();

        let minute_up_arrow = minute_children
            .next()
            .expect("Native: Layout should have an up arrow for minutes");
        let _ = minute_children.next();
        let minute_down_arrow = minute_children
            .next()
            .expect("Native: Layout should have a down arrow for minutes");

        let calculate_time = |time: &mut NaiveTime,
                              up_arrow: Layout<'_>,
                              down_arrow: Layout<'_>,
                              duration: Duration| {
            if up_arrow.bounds().contains(cursor_position) {
                *time += duration;
                event::Status::Captured
            } else if down_arrow.bounds().contains(cursor_position) {
                *time -= duration;
                event::Status::Captured
            } else {
                event::Status::Ignored
            }
        };

        let digital_clock_status = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if hour_layout.bounds().contains(cursor_position) {
                    self.state.focus = Focus::DigitalHour;

                    calculate_time(
                        &mut self.state.time,
                        hour_up_arrow,
                        hour_down_arrow,
                        Duration::hours(1),
                    )
                } else if minute_layout.bounds().contains(cursor_position) {
                    self.state.focus = Focus::DigitalMinute;

                    calculate_time(
                        &mut self.state.time,
                        minute_up_arrow,
                        minute_down_arrow,
                        Duration::minutes(1),
                    )
                } else {
                    event::Status::Ignored
                }
            }
            _ => event::Status::Ignored,
        };

        let second_status = if self.state.show_seconds {
            let _ = digital_clock_children.next();

            let second_layout = digital_clock_children
                .next()
                .expect("Native: Layout should have a second layout");
            let mut second_children = second_layout.children();

            let second_up_arrow = second_children
                .next()
                .expect("Native: Layout should have an up arrow for seconds");
            let _ = second_children.next();
            let second_down_arrow = second_children
                .next()
                .expect("Native: Layout should have a down arrow for seconds");

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if second_layout.bounds().contains(cursor_position) {
                        self.state.focus = Focus::DigitalSecond;

                        calculate_time(
                            &mut self.state.time,
                            second_up_arrow,
                            second_down_arrow,
                            Duration::seconds(1),
                        )
                    } else {
                        event::Status::Ignored
                    }
                }
                _ => event::Status::Ignored,
            }
        } else {
            event::Status::Ignored
        };

        let digital_clock_status = digital_clock_status.merge(second_status);

        if digital_clock_status == event::Status::Captured {
            self.state.clock_cache.clear()
        }

        digital_clock_status
    }

    /// The event handling for the keyboard input.
    fn on_event_keyboard(
        &mut self,
        event: &Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
            let mut status = event::Status::Ignored;

            if let keyboard::KeyCode::Tab = key_code {
                if self.state.keyboard_modifiers.shift {
                    self.state.focus = self.state.focus.previous(self.state.show_seconds);
                } else {
                    self.state.focus = self.state.focus.next(self.state.show_seconds);
                }
            } else {
                let mut keyboard_handle =
                    |key_code: &keyboard::KeyCode, time: &mut NaiveTime, duration: Duration| {
                        match key_code {
                            keyboard::KeyCode::Left | keyboard::KeyCode::Down => {
                                *time -= duration;
                                status = event::Status::Captured;
                            }
                            keyboard::KeyCode::Right | keyboard::KeyCode::Up => {
                                *time += duration;
                                status = event::Status::Captured;
                            }
                            _ => {}
                        }
                    };

                match self.state.focus {
                    Focus::DigitalHour => {
                        keyboard_handle(key_code, &mut self.state.time, Duration::hours(1))
                    }
                    Focus::DigitalMinute => {
                        keyboard_handle(key_code, &mut self.state.time, Duration::minutes(1))
                    }
                    Focus::DigitalSecond => {
                        keyboard_handle(key_code, &mut self.state.time, Duration::seconds(1))
                    }
                    _ => {}
                }
            }

            if status == event::Status::Captured {
                self.state.clock_cache.clear()
            }

            status
        } else if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
            self.state.keyboard_modifiers = *modifiers;
            event::Status::Ignored
        } else {
            event::Status::Ignored
        }
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
            .pad(f32::from(PADDING))
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(300)
            .max_height(350);

        // Digital Clock
        let digital_clock_limits = limits;
        let mut digital_clock = digital_clock(self, renderer, digital_clock_limits);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(
            0.0,
            digital_clock.bounds().height
                + cancel_button.bounds().height
                + 2.0 * f32::from(SPACING),
        ));

        // Clock-Canvas
        let mut clock = Row::<(), Renderer>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .layout(renderer, &limits);

        clock.move_to(Point::new(
            clock.bounds().x + f32::from(PADDING),
            clock.bounds().y + f32::from(PADDING),
        ));

        digital_clock.move_to(Point::new(
            digital_clock.bounds().x + f32::from(PADDING),
            digital_clock.bounds().y
                + f32::from(PADDING)
                + f32::from(SPACING)
                + clock.bounds().height,
        ));

        // Buttons
        let cancel_limits = limits
            .clone()
            .max_width(((clock.bounds().width / 2.0) - f32::from(BUTTON_SPACING)).max(0.0) as u32);

        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let submit_limits = limits
            .clone()
            .max_width(((clock.bounds().width / 2.0) - f32::from(BUTTON_SPACING)).max(0.0) as u32);

        let mut submit_button = self.submit_button.layout(renderer, &submit_limits);

        cancel_button.move_to(Point {
            x: cancel_button.bounds().x + f32::from(PADDING),
            y: cancel_button.bounds().y
                + clock.bounds().height
                + f32::from(PADDING)
                + digital_clock.bounds().height
                + 2.0 * f32::from(SPACING),
        });

        submit_button.move_to(Point {
            x: submit_button.bounds().x + clock.bounds().width - submit_button.bounds().width
                + f32::from(PADDING),
            y: submit_button.bounds().y
                + clock.bounds().height
                + f32::from(PADDING)
                + digital_clock.bounds().height
                + 2.0 * f32::from(SPACING),
        });

        let mut node = layout::Node::with_children(
            Size::new(
                clock.bounds().width + (2.0 * f32::from(PADDING)),
                clock.bounds().height
                    + digital_clock.bounds().height
                    + cancel_button.bounds().height
                    + (2.0 * f32::from(PADDING))
                    + 2.0 * f32::from(SPACING),
            ),
            vec![clock, digital_clock, cancel_button, submit_button],
        );

        node.center_and_bounce(position, bounds);

        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        if let event::Status::Captured = self.on_event_keyboard(
            &event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        ) {
            return event::Status::Captured;
        }

        let mut children = layout.children();

        // Clock canvas
        let clock_layout = children
            .next()
            .expect("Native: Layout should have a clock canvas layout");
        let clock_status = self.on_event_clock(
            &event,
            clock_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Digital clock ------------------
        let digital_clock_layout = children
            .next()
            .expect("Native: Layout should have a digital clock parent")
            .children()
            .next()
            .expect("Native: Layout should have a digital clock layout");
        let digital_clock_status = self.on_event_digital_clock(
            &event,
            digital_clock_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Buttons ------------------------
        let cancel_button_layout = children
            .next()
            .expect("Native: Layout should have a cancel button layout for a TimePicker");

        let cancel_status = self.cancel_button.on_event(
            event.clone(),
            cancel_button_layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        );

        let submit_button_layout = children
            .next()
            .expect("Native: Layout should have a submit button layout for a TimePicker");

        let mut fake_messages: Vec<Message> = Vec::new();

        let submit_status = self.submit_button.on_event(
            event,
            submit_button_layout,
            cursor_position,
            renderer,
            clipboard,
            &mut fake_messages,
        );

        if !fake_messages.is_empty() {
            let (hour, period) = if self.state.use_24h {
                (self.state.time.hour(), Period::H24)
            } else {
                let (period, hour) = self.state.time.hour12();
                (hour, if period { Period::Pm } else { Period::Am })
            };

            let time = if self.state.show_seconds {
                Time::Hms {
                    hour,
                    minute: self.state.time.minute(),
                    second: self.state.time.second(),
                    period,
                }
            } else {
                Time::Hm {
                    hour,
                    minute: self.state.time.minute(),
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
                style_sheet: self.style,
                viewport: None,
                focus: self.state.focus,
            },
            self.state,
            &self.cancel_button,
            &self.submit_button,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher, position: Point) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
        self.state.show_seconds.hash(state);
        self.state.use_24h.hash(state);
    }
}

/// Defines the layout of the digital clock of the time picker.
fn digital_clock<'a, Message, Renderer>(
    time_picker: &TimePickerOverlay<'a, Message, Renderer>,
    renderer: &Renderer,
    limits: Limits,
) -> iced_native::layout::Node
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
    let arrow_size = text::Renderer::default_size(renderer);
    let font_size = (1.2 * f32::from(text::Renderer::default_size(renderer))) as u16;

    let mut digital_clock_row = Row::<(), Renderer>::new()
        .align_items(Align::Center)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .spacing(1);

    if !time_picker.state.use_24h {
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
                .push(Text::new(format!("{:02}", time_picker.state.time.hour())).size(font_size))
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
                .push(Text::new(format!("{:02}", time_picker.state.time.hour())).size(font_size))
                .push(
                    // Down Minute arrow
                    Row::new()
                        .width(Length::Units(arrow_size))
                        .height(Length::Units(arrow_size)),
                ),
        );

    if time_picker.state.show_seconds {
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
                    .push(
                        Text::new(format!("{:02}", time_picker.state.time.hour())).size(font_size),
                    )
                    .push(
                        // Down Minute arrow
                        Row::new()
                            .width(Length::Units(arrow_size))
                            .height(Length::Units(arrow_size)),
                    ),
            )
    }

    if !time_picker.state.use_24h {
        digital_clock_row = digital_clock_row.push(
            Column::new()
                .height(Length::Shrink)
                .push(Text::new("AM").size(font_size)),
        );
    }

    Container::new(digital_clock_row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .center_x()
        .center_y()
        .layout(renderer, &limits)
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
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<Self::Defaults, Self::Style, Focus>,
        state: &State,
        cancel_button: &Element<'_, Message, Self>,
        submit_button: &Element<'_, Message, Self>,
    ) -> Self::Output;
}

#[cfg(debug_assertions)]
impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<Self::Defaults, Self::Style, Focus>,
        _state: &State,
        _cancel_button: &Element<'_, Message, Self>,
        _submit_button: &Element<'_, Message, Self>,
    ) -> Self::Output {
    }
}

/// The state of the [`TimePickerOverlay`](TimePickerOverlay).
#[derive(Debug)]
pub struct State {
    /// The selected time of the [`TimePickerOverlay`](TimePickerOverlay).
    pub(crate) time: NaiveTime,
    /// Toggle if the cache needs to be cleared.
    pub(crate) clock_cache_needs_clearance: bool,
    /// The cache of the clock of the [`TimePickerOverlay`](TimePickerOverlay).
    pub(crate) clock_cache: canvas::Cache,
    /// Toggle the use of the 24h clock of the [`TimePickerOverlay`](TimePickerOverlay).
    pub(crate) use_24h: bool,
    /// Toggle the use of the seconds of the [`TimePickerOverlay`](TimePickerOverlay).
    pub(crate) show_seconds: bool,
    /// The dragged clock element of the [`TimePickerOverlay`](TimePickerOverlay).
    pub(crate) clock_dragged: ClockDragged,
    /// The focus of the [`TimePickerOverlay`](TimePickerOverlay).
    pub(crate) focus: Focus,
    /// The previously pressed keyboard modifiers.
    pub(crate) keyboard_modifiers: keyboard::Modifiers,
}

impl Default for State {
    fn default() -> Self {
        Self {
            time: Local::now().naive_local().time(),
            clock_cache_needs_clearance: false,
            clock_cache: canvas::Cache::new(),
            use_24h: false,
            show_seconds: false,
            clock_dragged: ClockDragged::None,
            focus: Focus::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
        }
    }
}

/// The state of the currently dragged watch hand.
#[derive(Copy, Clone, Debug)]
pub enum ClockDragged {
    /// Nothing is dragged.
    None,

    /// The hour hand is dragged.
    Hour,

    /// The minute hand is dragged.
    Minute,

    /// The second hand is dragged.
    Second,
}

/// An enumeration of all focusable elements of the [`TimePickerOverlay`](TimePickerOverlay).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Focus {
    /// Nothing is in focus.
    None,

    /// The overlay itself is in focus.
    Overlay,

    /// The digital hour is in focus.
    DigitalHour,

    /// The digital minute is in focus.
    DigitalMinute,

    /// The digital second is in focus.
    DigitalSecond,

    /// The cancel button is in focus.
    Cancel,

    /// The submit button is in focus.
    Submit,
}

impl Focus {
    /// Gets the next focusable element.
    #[must_use]
    pub const fn next(self, show_seconds: bool) -> Self {
        match self {
            Self::Overlay => Self::DigitalHour,
            Self::DigitalHour => Self::DigitalMinute,
            Self::DigitalMinute => {
                if show_seconds {
                    Self::DigitalSecond
                } else {
                    Self::Cancel
                }
            }
            Self::DigitalSecond => Self::Cancel,
            Self::Cancel => Self::Submit,
            Self::Submit | Self::None => Self::Overlay,
        }
    }

    /// Gets the previous focusable element.
    #[must_use]
    pub const fn previous(self, show_seconds: bool) -> Self {
        match self {
            Self::None => Self::None,
            Self::Overlay => Self::Submit,
            Self::DigitalHour => Self::Overlay,
            Self::DigitalMinute => Self::DigitalHour,
            Self::DigitalSecond => Self::DigitalMinute,
            Self::Cancel => {
                if show_seconds {
                    Self::DigitalSecond
                } else {
                    Self::DigitalMinute
                }
            }
            Self::Submit => Self::Cancel,
        }
    }
}

impl Default for Focus {
    fn default() -> Self {
        Self::None
    }
}
