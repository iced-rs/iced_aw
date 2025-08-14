//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*

use crate::iced_aw_font::advanced_text::{cancel, down_open, ok, up_open};
use crate::{
    core::clock::{
        NearestRadius, HOUR_RADIUS_PERCENTAGE, HOUR_RADIUS_PERCENTAGE_NO_SECONDS,
        MINUTE_RADIUS_PERCENTAGE, MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, PERIOD_PERCENTAGE,
        SECOND_RADIUS_PERCENTAGE,
    },
    core::{clock, overlay::Position, time::Period},
    style::{
        style_state::StyleState,
        time_picker::{Catalog, Style},
        Status,
    },
    time_picker::{self, Time},
};
use chrono::{Duration, Local, NaiveTime, Timelike};
use iced::{
    advanced::{
        graphics::geometry::Renderer as _,
        layout::{Limits, Node},
        overlay, renderer,
        text::Renderer as _,
        widget::tree::Tree,
        Clipboard, Layout, Overlay, Renderer as _, Shell, Text, Widget,
    },
    alignment::{Horizontal, Vertical},
    event,
    keyboard,
    mouse::{self, Cursor},
    touch,
    widget::{
        button,
        canvas::{self, LineCap, Path, Stroke, Text as CanvasText},
        container,
        text::{self, Wrapping},
        Button, Column, Container, Row,
    },
    Alignment,
    Border,
    Color,
    Element,
    Event,
    Length,
    Padding,
    Pixels,
    Point,
    Rectangle,
    Renderer, // the actual type
    Size,
    Vector,
};
use std::collections::HashMap;

/// The padding around the elements.
const PADDING: Padding = Padding::new(10.0);
/// The spacing between the elements.
const SPACING: Pixels = Pixels(15.0);
/// The spacing between the buttons.
const BUTTON_SPACING: Pixels = Pixels(5.0);
/// The percentage size of the numbers.
const NUMBER_SIZE_PERCENTAGE: f32 = 0.15;
/// The percentage size of the period.
const PERIOD_SIZE_PERCENTAGE: f32 = 0.2;

/// The overlay of the [`TimePicker`](crate::widget::TimePicker).
#[allow(missing_debug_implementations)]
pub struct TimePickerOverlay<'a, 'b, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + button::Catalog,
    'b: 'a,
{
    /// The state of the [`TimePickerOverlay`].
    state: &'a mut State,
    /// The cancel button of the [`TimePickerOverlay`].
    cancel_button: Button<'a, Message, Theme, Renderer>,
    /// The submit button of the [`TimePickerOverlay`].
    submit_button: Button<'a, Message, Theme, Renderer>,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`] is pressed.
    on_submit: &'a dyn Fn(Time) -> Message,
    /// The position of the [`TimePickerOverlay`].
    position: Point,
    /// The style of the [`TimePickerOverlay`].
    class: &'a <Theme as Catalog>::Class<'b>,
    /// The reference to the tree holding the state of this overlay.
    tree: &'a mut Tree,
    viewport: Rectangle,
}

impl<'a, 'b, Message, Theme> TimePickerOverlay<'a, 'b, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a + Catalog + button::Catalog + text::Catalog + container::Catalog,
    'b: 'a,
{
    /// Creates a new [`TimePickerOverlay`] on the given position.
    pub fn new(
        state: &'a mut time_picker::State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Time) -> Message,
        position: Point,
        class: &'a <Theme as Catalog>::Class<'b>,
        tree: &'a mut Tree,
        viewport: Rectangle,
    ) -> Self {
        let time_picker::State { overlay_state } = state;
        let (cancel_content, cancel_font, _cancel_shaping) = cancel();
        let (submit_content, submit_font, _submit_shaping) = ok();

        TimePickerOverlay {
            state: overlay_state,
            cancel_button: Button::new(
                text::Text::new(cancel_content)
                    .font(cancel_font)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel.clone()),
            submit_button: Button::new(
                text::Text::new(submit_content)
                    .font(submit_font)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel), // Sending a fake message
            on_submit,
            position,
            class,
            tree,
            viewport,
        }
    }

    /// Turn this [`TimePickerOverlay`] into an overlay [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(self))
    }

    /// The event handling for the clock.
    #[allow(clippy::too_many_lines)]
    fn on_event_clock(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
    ) -> event::Status {
        if cursor.is_over(layout.bounds()) {
            self.state.clock_cache_needs_clearance = true;
            self.state.clock_cache.clear();
        } else if self.state.clock_cache_needs_clearance {
            self.state.clock_cache.clear();
            self.state.clock_cache_needs_clearance = false;
        }

        let clock_bounds = layout.bounds();
        if cursor.is_over(clock_bounds) {
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
                cursor.position().unwrap_or_default(),
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
                | Event::Touch(
                    touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. },
                ) => {
                    self.state.clock_dragged = ClockDragged::None;
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            };

            let clock_dragged_status = match self.state.clock_dragged {
                ClockDragged::Hour => {
                    let hour_points = crate::core::clock::circle_points(hour_radius, center, 12);
                    let nearest_point = crate::core::clock::nearest_point(
                        &hour_points,
                        cursor.position().unwrap_or_default(),
                    );

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
                    let nearest_point = crate::core::clock::nearest_point(
                        &minute_points,
                        cursor.position().unwrap_or_default(),
                    );

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
                    let nearest_point = crate::core::clock::nearest_point(
                        &second_points,
                        cursor.position().unwrap_or_default(),
                    );

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
                | Event::Touch(
                    touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. },
                ) => {
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
        cursor: Cursor,
    ) -> event::Status {
        let mut digital_clock_children = layout.children();

        if !self.state.use_24h {
            // Placeholder
            let _ = digital_clock_children.next();
        }

        let hour_layout = digital_clock_children
            .next()
            .expect("widget: Layout should have a hour layout");
        let mut hour_children = hour_layout.children();

        let hour_up_arrow = hour_children
            .next()
            .expect("widget: Layout should have an up arrow for hours");
        let _ = hour_children.next();
        let hour_down_arrow = hour_children
            .next()
            .expect("widget: Layout should have a down arrow for hours");

        let _ = digital_clock_children.next();

        let minute_layout = digital_clock_children
            .next()
            .expect("widget: Layout should have a minute layout");
        let mut minute_children = minute_layout.children();

        let minute_up_arrow = minute_children
            .next()
            .expect("widget: Layout should have an up arrow for minutes");
        let _ = minute_children.next();
        let minute_down_arrow = minute_children
            .next()
            .expect("widget: Layout should have a down arrow for minutes");

        let calculate_time = |time: &mut NaiveTime,
                              up_arrow: Layout<'_>,
                              down_arrow: Layout<'_>,
                              duration: Duration| {
            if cursor.is_over(up_arrow.bounds()) {
                *time += duration;
                event::Status::Captured
            } else if cursor.is_over(down_arrow.bounds()) {
                *time -= duration;
                event::Status::Captured
            } else {
                event::Status::Ignored
            }
        };

        let digital_clock_status = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(hour_layout.bounds()) {
                    self.state.focus = Focus::DigitalHour;

                    calculate_time(
                        &mut self.state.time,
                        hour_up_arrow,
                        hour_down_arrow,
                        Duration::hours(1),
                    )
                } else if cursor.is_over(minute_layout.bounds()) {
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
                .expect("widget: Layout should have a second layout");
            let mut second_children = second_layout.children();

            let second_up_arrow = second_children
                .next()
                .expect("widget: Layout should have an up arrow for seconds");
            let _ = second_children.next();
            let second_down_arrow = second_children
                .next()
                .expect("widget: Layout should have a down arrow for seconds");

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if cursor.is_over(second_layout.bounds()) {
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
            self.state.clock_cache.clear();
        }

        digital_clock_status
    }

    /// The event handling for the keyboard input.
    fn on_event_keyboard(&mut self, event: &Event) -> event::Status {
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
            let mut status = event::Status::Ignored;

            if matches!(key, keyboard::Key::Named(keyboard::key::Named::Tab)) {
                if self.state.keyboard_modifiers.shift() {
                    self.state.focus = self.state.focus.previous(self.state.show_seconds);
                } else {
                    self.state.focus = self.state.focus.next(self.state.show_seconds);
                }
            } else {
                let mut keyboard_handle =
                    |key_code: &keyboard::Key, time: &mut NaiveTime, duration: Duration| {
                        match key_code {
                            keyboard::Key::Named(
                                keyboard::key::Named::ArrowLeft | keyboard::key::Named::ArrowDown,
                            ) => {
                                *time -= duration;
                                status = event::Status::Captured;
                            }
                            keyboard::Key::Named(
                                keyboard::key::Named::ArrowRight | keyboard::key::Named::ArrowUp,
                            ) => {
                                *time += duration;
                                status = event::Status::Captured;
                            }
                            _ => {}
                        }
                    };

                match self.state.focus {
                    Focus::DigitalHour => {
                        keyboard_handle(key, &mut self.state.time, Duration::hours(1));
                    }
                    Focus::DigitalMinute => {
                        keyboard_handle(key, &mut self.state.time, Duration::minutes(1));
                    }
                    Focus::DigitalSecond => {
                        keyboard_handle(key, &mut self.state.time, Duration::seconds(1));
                    }
                    _ => {}
                }
            }

            if status == event::Status::Captured {
                self.state.clock_cache.clear();
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

impl<'a, 'b, Message, Theme> Overlay<Message, Theme, Renderer>
    for TimePickerOverlay<'a, 'b, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a + Catalog + button::Catalog + text::Catalog + container::Catalog,
    'b: 'a,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .shrink(PADDING)
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(300.0)
            .max_height(350.0);

        // Digital Clock
        let digital_clock_limits = limits;
        let mut digital_clock = digital_clock(self, renderer, digital_clock_limits);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button =
            self.cancel_button
                .layout(&mut self.tree.children[0], renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(
            0.0,
            digital_clock.bounds().height + cancel_button.bounds().height + 2.0 * SPACING.0,
        ));

        // Clock-Canvas
        let mut clock = Row::<(), Renderer>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .layout(self.tree, renderer, &limits);

        let clock_bounds = clock.bounds();
        clock = clock.move_to(Point::new(
            clock_bounds.x + PADDING.left,
            clock_bounds.y + PADDING.top,
        ));

        let digital_bounds = digital_clock.bounds();
        digital_clock = digital_clock.move_to(Point::new(
            digital_bounds.x + PADDING.left,
            digital_bounds.y + PADDING.top + SPACING.0 + clock.bounds().height,
        ));

        // Buttons
        let cancel_limits =
            limits.max_width(((clock.bounds().width / 2.0) - BUTTON_SPACING.0).max(0.0));

        let mut cancel_button =
            self.cancel_button
                .layout(&mut self.tree.children[0], renderer, &cancel_limits);

        let submit_limits =
            limits.max_width(((clock.bounds().width / 2.0) - BUTTON_SPACING.0).max(0.0));

        let mut submit_button =
            self.submit_button
                .layout(&mut self.tree.children[1], renderer, &submit_limits);

        let cancel_bounds = cancel_button.bounds();
        cancel_button = cancel_button.move_to(Point {
            x: cancel_bounds.x + PADDING.left,
            y: cancel_bounds.y
                + clock.bounds().height
                + PADDING.top
                + digital_clock.bounds().height
                + 2.0 * SPACING.0,
        });

        let submit_bounds = submit_button.bounds();
        submit_button = submit_button.move_to(Point {
            x: submit_bounds.x + clock.bounds().width - submit_bounds.width + PADDING.left,
            y: submit_bounds.y
                + clock.bounds().height
                + PADDING.top
                + digital_clock.bounds().height
                + 2.0 * SPACING.0,
        });

        let mut node = Node::with_children(
            Size::new(
                clock.bounds().width + PADDING.horizontal(),
                clock.bounds().height
                    + digital_clock.bounds().height
                    + cancel_button.bounds().height
                    + PADDING.vertical()
                    + 2.0 * SPACING.0,
            ),
            vec![clock, digital_clock, cancel_button, submit_button],
        );

        node.center_and_bounce(self.position, bounds);
        node
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) {
        let mut status = self.on_event_keyboard(event);
        let mut children = layout.children();

        // Clock canvas
        let clock_layout = children
            .next()
            .expect("widget: Layout should have a clock canvas layout");
        let clock_status = self.on_event_clock(event, clock_layout, cursor);

        // ----------- Digital clock ------------------
        let digital_clock_layout = children
            .next()
            .expect("widget: Layout should have a digital clock parent")
            .children()
            .next()
            .expect("widget: Layout should have a digital clock layout");
        let digital_clock_status = self.on_event_digital_clock(event, digital_clock_layout, cursor);

        if digital_clock_status == event::Status::Captured
            || clock_status == event::Status::Captured
        {
            status = event::Status::Captured;
        }

        // ----------- Buttons --------------
        let cancel_button_layout = children
            .next()
            .expect("widget: Layout should have a cancel button layout for a TimePicker");

        let mut fake_messages: Vec<Message> = Vec::new();

        self.cancel_button.update(
            &mut self.tree.children[0],
            event,
            cancel_button_layout,
            cursor,
            renderer,
            clipboard,
            &mut Shell::new(&mut fake_messages),
            &layout.bounds(),
        );

        while let Some(message) = fake_messages.pop() {
            shell.publish(message);
            status = event::Status::Captured;
        }

        let submit_button_layout = children
            .next()
            .expect("widget: Layout should have a submit button layout for a TimePicker");

        self.submit_button.update(
            &mut self.tree.children[1],
            event,
            submit_button_layout,
            cursor,
            renderer,
            clipboard,
            &mut Shell::new(&mut fake_messages),
            &layout.bounds(),
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

            shell.publish((self.on_submit)(time));
            status = event::Status::Captured;
        }

        if status == event::Status::Captured {
            shell.capture_event();
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let mouse_interaction = mouse::Interaction::default();

        // Clock canvas
        let clock_layout = children
            .next()
            .expect("Graphics: Layout should have a clock canvas layout");
        let clock_mouse_interaction = if cursor.is_over(clock_layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        };

        // Digital clock
        let digital_clock_layout = children
            .next()
            .expect("Graphics: Layout should have a digital clock layout");
        //let digital_clock_mouse_interaction = mouse::Interaction::default();
        let mut digital_clock_children = digital_clock_layout
            .children()
            .next()
            .expect("Graphics: Layout should have digital clock children")
            .children();

        let f = |layout: Layout<'_>| {
            let mut children = layout.children();

            let up_bounds = children
                .next()
                .expect("Graphics: Layout should have a up arrow bounds")
                .bounds();
            let _center_bounds = children.next();
            let down_bounds = children
                .next()
                .expect("Graphics: Layout should have a down arrow bounds")
                .bounds();

            let mut mouse_interaction = mouse::Interaction::default();

            let up_arrow_hovered = cursor.is_over(up_bounds);
            let down_arrow_hovered = cursor.is_over(down_bounds);

            if up_arrow_hovered || down_arrow_hovered {
                mouse_interaction = mouse_interaction.max(mouse::Interaction::Pointer);
            }

            mouse_interaction
        };

        if !self.state.use_24h {
            // Placeholder
            let _ = digital_clock_children.next();
        }

        let hour_layout = digital_clock_children
            .next()
            .expect("Graphics: Layout should have a hour layout");
        let hour_mouse_interaction = f(hour_layout);

        let _hour_minute_separator = digital_clock_children.next();

        let minute_layout = digital_clock_children
            .next()
            .expect("Graphics: Layout should have a minute layout");
        let minute_mouse_interaction = f(minute_layout);

        let second_mouse_interaction = if self.state.show_seconds {
            let _minute_second_separator = digital_clock_children.next();

            let second_layout = digital_clock_children
                .next()
                .expect("Graphics: Layout should have a second layout");
            f(second_layout)
        } else {
            mouse::Interaction::default()
        };

        // Buttons
        let cancel_button_layout = children
            .next()
            .expect("Graphics: Layout should have a cancel button layout for a TimePicker");

        let cancel_mouse_interaction = self.cancel_button.mouse_interaction(
            &self.tree.children[0],
            cancel_button_layout,
            cursor,
            &self.viewport,
            renderer,
        );

        let submit_button_layout = children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a TimePicker");

        let submit_mouse_interaction = self.submit_button.mouse_interaction(
            &self.tree.children[1],
            submit_button_layout,
            cursor,
            &self.viewport,
            renderer,
        );

        mouse_interaction
            .max(clock_mouse_interaction)
            .max(hour_mouse_interaction)
            .max(minute_mouse_interaction)
            .max(second_mouse_interaction)
            .max(cancel_mouse_interaction)
            .max(submit_mouse_interaction)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let mut style_sheet: HashMap<StyleState, Style> = HashMap::new();
        let _ = style_sheet.insert(
            StyleState::Active,
            Catalog::style(theme, self.class, Status::Active),
        );
        let _ = style_sheet.insert(
            StyleState::Selected,
            Catalog::style(theme, self.class, Status::Selected),
        );
        let _ = style_sheet.insert(
            StyleState::Hovered,
            Catalog::style(theme, self.class, Status::Hovered),
        );
        let _ = style_sheet.insert(
            StyleState::Focused,
            Catalog::style(theme, self.class, Status::Focused),
        );

        let mut style_state = StyleState::Active;
        if self.state.focus == Focus::Overlay {
            style_state = style_state.max(StyleState::Focused);
        }
        if cursor.is_over(bounds) {
            style_state = style_state.max(StyleState::Hovered);
        }

        // Background
        if (bounds.width > 0.) && (bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style_sheet[&style_state].border_radius.into(),
                        width: style_sheet[&style_state].border_width,
                        color: style_sheet[&style_state].border_color,
                    },
                    ..renderer::Quad::default()
                },
                style_sheet[&style_state].background,
            );
        }

        // ----------- Clock canvas --------------------
        let clock_layout = children
            .next()
            .expect("Graphics: Layout should have a clock canvas layout");
        draw_clock(renderer, self, clock_layout, cursor, &style_sheet);

        // ----------- Digital clock ------------------
        let digital_clock_layout = children
            .next()
            .expect("Graphics: Layout should have a digital clock layout");
        draw_digital_clock(renderer, self, digital_clock_layout, cursor, &style_sheet);

        // ----------- Buttons ------------------------
        let cancel_button_layout = children
            .next()
            .expect("Graphics: Layout should have a cancel button layout for a TimePicker");

        self.cancel_button.draw(
            &self.tree.children[0],
            renderer,
            theme,
            style,
            cancel_button_layout,
            cursor,
            &bounds,
        );

        let submit_button_layout = children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a TimePicker");

        self.submit_button.draw(
            &self.tree.children[1],
            renderer,
            theme,
            style,
            submit_button_layout,
            cursor,
            &bounds,
        );

        // Buttons are not focusable right now...
        let cancel_button_bounds = cancel_button_layout.bounds();
        if (self.state.focus == Focus::Cancel)
            && (cancel_button_bounds.width > 0.)
            && (cancel_button_bounds.height > 0.)
        {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: cancel_button_bounds,
                    border: Border {
                        radius: style_sheet[&StyleState::Focused].border_radius.into(),
                        width: style_sheet[&StyleState::Focused].border_width,
                        color: style_sheet[&StyleState::Focused].border_color,
                    },
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }

        let submit_button_bounds = submit_button_layout.bounds();
        if (self.state.focus == Focus::Submit)
            && (submit_button_bounds.width > 0.)
            && (submit_button_bounds.height > 0.)
        {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: submit_button_bounds,
                    border: Border {
                        radius: style_sheet[&StyleState::Focused].border_radius.into(),
                        width: style_sheet[&StyleState::Focused].border_width,
                        color: style_sheet[&StyleState::Focused].border_color,
                    },
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }
    }
}

/// Defines the layout of the digital clock of the time picker.
fn digital_clock<Message, Theme>(
    time_picker: &mut TimePickerOverlay<'_, '_, Message, Theme>,
    renderer: &Renderer,
    limits: Limits,
) -> Node
where
    Message: 'static + Clone,
    Theme: Catalog + button::Catalog + text::Catalog + container::Catalog,
{
    let arrow_size = renderer.default_size().0;
    let font_size = 1.2 * renderer.default_size().0;

    let mut digital_clock_row = Row::<Message, Theme, Renderer>::new()
        .align_y(Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .spacing(1);

    if !time_picker.state.use_24h {
        digital_clock_row = digital_clock_row.push(
            Column::new() // Just a placeholder
                .height(Length::Shrink)
                .push(text::Text::new("AM").size(font_size)),
        );
    }

    digital_clock_row = digital_clock_row
        .push(
            // Hour
            Column::new()
                .align_x(Alignment::Center)
                .height(Length::Shrink)
                .push(
                    // Up Hour arrow
                    Row::new()
                        .width(Length::Fixed(arrow_size))
                        .height(Length::Fixed(arrow_size)),
                )
                .push(
                    text::Text::new(format!("{:02}", time_picker.state.time.hour()))
                        .size(font_size),
                )
                .push(
                    // Down Hour arrow
                    Row::new()
                        .width(Length::Fixed(arrow_size))
                        .height(Length::Fixed(arrow_size)),
                ),
        )
        .push(
            Column::new()
                .height(Length::Shrink)
                .push(text::Text::new(":").size(font_size)),
        )
        .push(
            Column::new()
                .align_x(Alignment::Center)
                .height(Length::Shrink)
                .push(
                    // Up Minute arrow
                    Row::new()
                        .width(Length::Fixed(arrow_size))
                        .height(Length::Fixed(arrow_size)),
                )
                .push(
                    text::Text::new(format!("{:02}", time_picker.state.time.hour()))
                        .size(font_size),
                )
                .push(
                    // Down Minute arrow
                    Row::new()
                        .width(Length::Fixed(arrow_size))
                        .height(Length::Fixed(arrow_size)),
                ),
        );

    if time_picker.state.show_seconds {
        digital_clock_row = digital_clock_row
            .push(
                Column::new()
                    .height(Length::Shrink)
                    .push(text::Text::new(":").size(font_size)),
            )
            .push(
                Column::new()
                    .align_x(Alignment::Center)
                    .height(Length::Shrink)
                    .push(
                        // Up Minute arrow
                        Row::new()
                            .width(Length::Fixed(arrow_size))
                            .height(Length::Fixed(arrow_size)),
                    )
                    .push(
                        text::Text::new(format!("{:02}", time_picker.state.time.hour()))
                            .size(font_size),
                    )
                    .push(
                        // Down Minute arrow
                        Row::new()
                            .width(Length::Fixed(arrow_size))
                            .height(Length::Fixed(arrow_size)),
                    ),
            );
    }

    if !time_picker.state.use_24h {
        digital_clock_row = digital_clock_row.push(
            Column::new()
                .height(Length::Shrink)
                .push(text::Text::new("AM").size(font_size)),
        );
    }

    let container = Container::new(digital_clock_row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .center_x(Length::Fill)
        .center_y(Length::Shrink);

    let element: Element<Message, Theme, Renderer> = Element::new(container);
    let container_tree = if let Some(child_tree) = time_picker.tree.children.get_mut(2) {
        child_tree.diff(element.as_widget());
        child_tree
    } else {
        let child_tree = Tree::new(element.as_widget());
        time_picker.tree.children.insert(2, child_tree);
        &mut time_picker.tree.children[2]
    };

    element
        .as_widget()
        .layout(container_tree, renderer, &limits)
}

/// Draws the analog clock.
#[allow(clippy::too_many_lines)]
fn draw_clock<Message, Theme>(
    renderer: &mut Renderer,
    time_picker: &TimePickerOverlay<'_, '_, Message, Theme>,
    layout: Layout<'_>,
    cursor: Cursor,
    style: &HashMap<StyleState, Style>,
) where
    Message: 'static + Clone,
    Theme: Catalog + button::Catalog + text::Catalog,
{
    let mut clock_style_state = StyleState::Active;
    if cursor.is_over(layout.bounds()) {
        clock_style_state = clock_style_state.max(StyleState::Hovered);
    }

    let geometry = time_picker
        .state
        .clock_cache
        .draw(renderer, layout.bounds().size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) * 0.5;
            let period = if time_picker.state.time.hour12().0 {
                clock::Period::PM
            } else {
                clock::Period::AM
            };

            let number_size = radius * NUMBER_SIZE_PERCENTAGE;
            let period_size = radius * PERIOD_SIZE_PERCENTAGE;

            let period_radius = radius * PERIOD_PERCENTAGE;

            let (hour_radius, minute_radius, second_radius) = if time_picker.state.show_seconds {
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

            let internal_cursor = cursor.position().unwrap_or_default()
                - Vector::new(layout.bounds().x, layout.bounds().y);

            let nearest_radius = if cursor.is_over(layout.bounds()) {
                crate::core::clock::nearest_radius(
                    &if time_picker.state.show_seconds {
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
                    internal_cursor,
                    center,
                )
            } else {
                NearestRadius::None
            };

            let hour_points = crate::core::clock::circle_points(hour_radius, center, 12);
            let minute_points = crate::core::clock::circle_points(minute_radius, center, 60);
            let second_points = crate::core::clock::circle_points(second_radius, center, 60);

            let hand_stroke = Stroke {
                style: canvas::Style::Solid(
                    style
                        .get(&clock_style_state)
                        .expect("Style Sheet not found.")
                        .clock_hand_color,
                ),
                width: style
                    .get(&clock_style_state)
                    .expect("Style Sheet not found.")
                    .clock_hand_width,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            match nearest_radius {
                NearestRadius::Period => {
                    frame.fill(
                        &Path::circle(center, period_size),
                        style
                            .get(&StyleState::Hovered)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                }
                NearestRadius::Hour => {
                    let nearest_point = hour_points
                        [crate::core::clock::nearest_point(&hour_points, internal_cursor)];

                    frame.fill(
                        &Path::circle(nearest_point, 5.0),
                        style
                            .get(&StyleState::Hovered)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                }
                NearestRadius::Minute => {
                    let nearest_point = minute_points
                        [crate::core::clock::nearest_point(&minute_points, internal_cursor)];

                    frame.fill(
                        &Path::circle(nearest_point, 5.0),
                        style
                            .get(&StyleState::Hovered)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                }
                NearestRadius::Second => {
                    let nearest_point = second_points
                        [crate::core::clock::nearest_point(&second_points, internal_cursor)];

                    frame.fill(
                        &Path::circle(nearest_point, 5.0),
                        style
                            .get(&StyleState::Hovered)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                }
                NearestRadius::None => {}
            }

            let period_text = CanvasText {
                content: format!("{period}"),
                position: center,
                color: style
                    .get(&clock_style_state)
                    .expect("Style Sheet not found.")
                    .clock_number_color,
                size: Pixels(period_size),
                font: renderer.default_font(),
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
                max_width: f32::INFINITY,
            };
            frame.fill_text(period_text);

            hour_points.iter().enumerate().for_each(|(i, p)| {
                let (pm, selected) = {
                    let (pm, _) = time_picker.state.time.hour12();
                    let hour = time_picker.state.time.hour();
                    (pm, hour % 12 == i as u32)
                };

                let mut style_state = StyleState::Active;
                if selected {
                    frame.stroke(&Path::line(center, *p), hand_stroke);
                    frame.fill(
                        &Path::circle(*p, number_size * 0.8),
                        style
                            .get(&StyleState::Selected)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                    style_state = style_state.max(StyleState::Selected);
                }

                let text = CanvasText {
                    content: format!(
                        "{}",
                        if pm && time_picker.state.use_24h {
                            i + 12
                        } else if !time_picker.state.use_24h && i == 0 {
                            12
                        } else {
                            i
                        }
                    ),
                    position: *p,
                    color: style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .clock_number_color,
                    size: Pixels(number_size),
                    font: renderer.default_font(),
                    align_x: text::Alignment::Center,
                    align_y: Vertical::Center,
                    shaping: text::Shaping::Basic,
                    line_height: text::LineHeight::Relative(1.3),
                    max_width: f32::INFINITY,
                };

                frame.fill_text(text);
            });

            minute_points.iter().enumerate().for_each(|(i, p)| {
                let selected = time_picker.state.time.minute() == i as u32;

                let mut style_state = StyleState::Active;
                if selected {
                    frame.stroke(&Path::line(center, *p), hand_stroke);
                    frame.fill(
                        &Path::circle(*p, number_size * 0.6),
                        style
                            .get(&StyleState::Selected)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                    style_state = style_state.max(StyleState::Selected);
                }

                if i % 5 == 0 {
                    let text = CanvasText {
                        content: format!("{i:02}"),
                        position: *p,
                        color: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .clock_number_color,
                        size: Pixels(number_size),
                        font: renderer.default_font(),
                        align_x: text::Alignment::Center,
                        align_y: Vertical::Center,
                        shaping: text::Shaping::Basic,
                        line_height: text::LineHeight::Relative(1.3),
                        max_width: f32::INFINITY,
                    };

                    frame.fill_text(text);
                } else {
                    let circle = Path::circle(*p, number_size * 0.1);
                    frame.fill(
                        &circle,
                        style
                            .get(&StyleState::Active)
                            .expect("Style Sheet not found.")
                            .clock_dots_color,
                    );
                }
            });

            if time_picker.state.show_seconds {
                second_points.iter().enumerate().for_each(|(i, p)| {
                    let selected = time_picker.state.time.second() == i as u32;

                    let mut style_state = StyleState::Active;
                    if selected {
                        frame.stroke(&Path::line(center, *p), hand_stroke);
                        frame.fill(
                            &Path::circle(*p, number_size * 0.6),
                            style
                                .get(&StyleState::Selected)
                                .expect("Style Sheet not found.")
                                .clock_number_background,
                        );
                        style_state = style_state.max(StyleState::Selected);
                    }

                    if i % 10 == 0 {
                        let text = CanvasText {
                            content: format!("{i:02}"),
                            position: *p,
                            color: style
                                .get(&style_state)
                                .expect("Style Sheet not found.")
                                .clock_number_color,
                            size: Pixels(number_size),
                            font: renderer.default_font(),
                            align_x: text::Alignment::Center,
                            align_y: Vertical::Center,
                            shaping: text::Shaping::Basic,
                            line_height: text::LineHeight::Relative(1.3),
                            max_width: f32::INFINITY,
                        };

                        frame.fill_text(text);
                    } else {
                        let circle = Path::circle(*p, number_size * 0.1);
                        frame.fill(
                            &circle,
                            style
                                .get(&StyleState::Active)
                                .expect("Style Sheet not found.")
                                .clock_dots_color,
                        );
                    }
                });
            }
        });

    let translation = Vector::new(layout.bounds().x, layout.bounds().y);
    renderer.with_translation(translation, |renderer| {
        renderer.draw_geometry(geometry);
    });
}

/// Draws the digital clock.
#[allow(clippy::too_many_lines)]
fn draw_digital_clock<Message, Theme>(
    renderer: &mut Renderer,
    time_picker: &TimePickerOverlay<'_, '_, Message, Theme>,
    layout: Layout<'_>,
    cursor: Cursor,
    style: &HashMap<StyleState, Style>,
) where
    Message: 'static + Clone,
    Theme: Catalog + button::Catalog + text::Catalog,
{
    //println!("layout: {:#?}", layout);
    let mut children = layout
        .children()
        .next()
        .expect("Graphics: Layout should have digital clock children")
        .children();

    let f = |renderer: &mut Renderer, layout: Layout<'_>, text: String, target: Focus| {
        let style_state = if time_picker.state.focus == target {
            StyleState::Focused
        } else {
            StyleState::Active
        };

        let mut children = layout.children();

        let up_bounds = children
            .next()
            .expect("Graphics: Layout should have a up arrow bounds")
            .bounds();
        let center_bounds = children
            .next()
            .expect("Graphics: Layout should have a center bounds")
            .bounds();
        let down_bounds = children
            .next()
            .expect("Graphics: Layout should have a down arrow bounds")
            .bounds();

        let up_arrow_hovered = cursor.is_over(up_bounds);
        let down_arrow_hovered = cursor.is_over(down_bounds);

        // Background
        if style_state == StyleState::Focused {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        radius: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .border_radius
                            .into(),
                        width: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .border_width,
                        color: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .border_color,
                    },
                    ..renderer::Quad::default()
                },
                style
                    .get(&style_state)
                    .expect("Style Sheet not found.")
                    .background,
            );
        }

        let (up_content, up_font, _up_shaping) = up_open();
        let (down_content, down_font, _down_shaping) = down_open();
        // Caret up
        renderer.fill_text(
            Text {
                content: up_content,
                bounds: Size::new(up_bounds.width, up_bounds.height),
                size: Pixels(renderer.default_size().0 + if up_arrow_hovered { 1.0 } else { 0.0 }),
                font: up_font,
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
                wrapping: Wrapping::default(),
            },
            Point::new(up_bounds.center_x(), up_bounds.center_y()),
            style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            up_bounds,
        );

        // Text
        renderer.fill_text(
            Text {
                content: text,
                bounds: Size::new(center_bounds.width, center_bounds.height),
                size: renderer.default_size(),
                font: renderer.default_font(),
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
                wrapping: Wrapping::default(),
            },
            Point::new(center_bounds.center_x(), center_bounds.center_y()),
            style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            center_bounds,
        );

        // Down caret
        renderer.fill_text(
            Text {
                content: down_content,
                bounds: Size::new(down_bounds.width, down_bounds.height),
                size: Pixels(
                    renderer.default_size().0 + if down_arrow_hovered { 1.0 } else { 0.0 },
                ),
                font: down_font,
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
                wrapping: Wrapping::default(),
            },
            Point::new(down_bounds.center_x(), down_bounds.center_y()),
            style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            down_bounds,
        );
    };

    if !time_picker.state.use_24h {
        // Placeholder
        let _ = children.next();
    }

    // Draw hours
    let hour_layout = children
        .next()
        .expect("Graphics: Layout should have a hour layout");
    f(
        renderer,
        hour_layout,
        format!(
            "{:02}",
            if time_picker.state.use_24h {
                time_picker.state.time.hour()
            } else {
                time_picker.state.time.hour12().1
            }
        ),
        Focus::DigitalHour,
    );

    // Draw separator between hours and minutes
    let hour_minute_separator = children
        .next()
        .expect("Graphics: Layout should have a hour/minute separator layout");

    renderer.fill_text(
        Text {
            content: ":".to_owned(),
            bounds: Size::new(
                hour_minute_separator.bounds().width,
                hour_minute_separator.bounds().height,
            ),
            size: renderer.default_size(),
            font: renderer.default_font(),
            align_x: text::Alignment::Center,
            align_y: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
            wrapping: Wrapping::default(),
        },
        Point::new(
            hour_minute_separator.bounds().center_x(),
            hour_minute_separator.bounds().center_y(),
        ),
        style[&StyleState::Active].text_color,
        hour_minute_separator.bounds(),
    );

    // Draw minutes
    let minute_layout = children
        .next()
        .expect("Graphics: Layout should have a minute layout");
    f(
        renderer,
        minute_layout,
        format!("{:02}", time_picker.state.time.minute()),
        Focus::DigitalMinute,
    );

    if time_picker.state.show_seconds {
        // Draw separator between minutes and seconds
        let minute_second_separator = children
            .next()
            .expect("Graphics: Layout should have a minute/second separator layout");
        renderer.fill_text(
            Text {
                content: ":".to_owned(),
                bounds: Size::new(
                    minute_second_separator.bounds().width,
                    minute_second_separator.bounds().height,
                ),
                size: renderer.default_size(),
                font: renderer.default_font(),
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
                wrapping: Wrapping::default(),
            },
            Point::new(
                minute_second_separator.bounds().center_x(),
                minute_second_separator.bounds().center_y(),
            ),
            style[&StyleState::Active].text_color,
            minute_second_separator.bounds(),
        );

        // Draw seconds
        let second_layout = children
            .next()
            .expect("Graphics: Layout should have a second layout");
        f(
            renderer,
            second_layout,
            format!("{:02}", time_picker.state.time.second()),
            Focus::DigitalSecond,
        );
    }

    // Draw period
    if !time_picker.state.use_24h {
        let period = children
            .next()
            .expect("Graphics: Layout should have a period layout");
        renderer.fill_text(
            Text {
                content: if time_picker.state.time.hour12().0 {
                    "PM".to_owned()
                } else {
                    "AM".to_owned()
                },
                bounds: Size::new(period.bounds().width, period.bounds().height),
                size: renderer.default_size(),
                font: renderer.default_font(),
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
                wrapping: Wrapping::default(),
            },
            Point::new(period.bounds().center_x(), period.bounds().center_y()),
            style[&StyleState::Active].text_color,
            period.bounds(),
        );
    }
}

/// The state of the [`TimePickerOverlay`].
#[derive(Debug)]
pub struct State {
    /// The selected time of the [`TimePickerOverlay`].
    pub(crate) time: NaiveTime,
    /// Toggle if the cache needs to be cleared.
    pub(crate) clock_cache_needs_clearance: bool,
    /// The cache of the clock of the [`TimePickerOverlay`].
    pub(crate) clock_cache: canvas::Cache,
    /// Toggle the use of the 24h clock of the [`TimePickerOverlay`].
    pub(crate) use_24h: bool,
    /// Toggle the use of the seconds of the [`TimePickerOverlay`].
    pub(crate) show_seconds: bool,
    /// The dragged clock element of the [`TimePickerOverlay`].
    pub(crate) clock_dragged: ClockDragged,
    /// The focus of the [`TimePickerOverlay`].
    pub(crate) focus: Focus,
    /// The previously pressed keyboard modifiers.
    pub(crate) keyboard_modifiers: keyboard::Modifiers,
}

impl State {
    /// Creates a new State with the given time.
    #[must_use]
    pub fn new(time: Time, use_24h: bool, show_seconds: bool) -> Self {
        Self {
            use_24h,
            show_seconds,
            time: time.into(),
            ..Self::default()
        }
    }
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

/// Just a workaround to pass the button states from the tree to the overlay
#[allow(missing_debug_implementations)]
pub struct TimePickerOverlayButtons<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + button::Catalog,
{
    /// The cancel button of the [`TimePickerOverlay`].
    cancel_button: Element<'a, Message, Theme, Renderer>,
    /// The submit button of the [`TimePickerOverlay`].
    submit_button: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme> Default for TimePickerOverlayButtons<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog + button::Catalog + text::Catalog,
{
    fn default() -> Self {
        let (cancel_content, cancel_font, _cancel_shaping) = cancel();
        let (submit_content, submit_font, _submit_shaping) = ok();

        Self {
            cancel_button: Button::new(
                text::Text::new(cancel_content)
                    .font(cancel_font)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .into(),
            submit_button: Button::new(
                text::Text::new(submit_content)
                    .font(submit_font)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .into(),
        }
    }
}

#[allow(clippy::unimplemented)]
impl<Message, Theme> Widget<Message, Theme, Renderer>
    for TimePickerOverlayButtons<'_, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + button::Catalog,
{
    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.cancel_button),
            Tree::new(&self.submit_button),
        ]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.cancel_button, &self.submit_button]);
    }

    fn size(&self) -> Size<Length> {
        unimplemented!("This should never be reached!")
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, _limits: &Limits) -> Node {
        unimplemented!("This should never be reached!")
    }

    fn draw(
        &self,
        _state: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        unimplemented!("This should never be reached!")
    }
}

impl<'a, Message, Theme> From<TimePickerOverlayButtons<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog + button::Catalog,
{
    fn from(overlay: TimePickerOverlayButtons<'a, Message, Theme>) -> Self {
        Self::new(overlay)
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

/// An enumeration of all focusable elements of the [`TimePickerOverlay`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
