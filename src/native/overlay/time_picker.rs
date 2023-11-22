//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*

use crate::{
    core::clock::{
        NearestRadius, HOUR_RADIUS_PERCENTAGE, HOUR_RADIUS_PERCENTAGE_NO_SECONDS,
        MINUTE_RADIUS_PERCENTAGE, MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, PERIOD_PERCENTAGE,
        SECOND_RADIUS_PERCENTAGE,
    },
    core::{clock, overlay::Position, time::Period},
    graphics::icons::{
        bootstrap::{icon_to_string, BootstrapIcon},
        BOOTSTRAP_FONT,
    },
    style::style_state::StyleState,
    time_picker::{self, Time},
};

use chrono::{Duration, Local, NaiveTime, Timelike};
use iced_widget::{
    button,
    canvas::{self, LineCap, Path, Stroke, Style, Text},
    container,
    core::{
        self,
        alignment::{Horizontal, Vertical},
        event, keyboard,
        layout::{Limits, Node},
        mouse::{self, Cursor},
        overlay, renderer,
        text::Renderer as _,
        touch,
        widget::tree::Tree,
        Alignment, Clipboard, Color, Element, Event, Layout, Length, Overlay, Padding, Point,
        Rectangle, Renderer as _, Shell, Size, Vector, Widget,
    },
    graphics::geometry::Renderer as _,
    renderer::Renderer,
    text, Button, Column, Container, Row,
};
use std::collections::HashMap;

pub use crate::style::time_picker::{Appearance, StyleSheet};

/// The padding around the elements.
const PADDING: f32 = 10.0;
/// The spacing between the elements.
const SPACING: f32 = 15.0;
/// The spacing between the buttons.
const BUTTON_SPACING: f32 = 5.0;
/// The percentage size of the numbers.
const NUMBER_SIZE_PERCENTAGE: f32 = 0.15;
/// The percentage size of the period.
const PERIOD_SIZE_PERCENTAGE: f32 = 0.2;

/// The overlay of the [`TimePicker`](crate::native::TimePicker).
#[allow(missing_debug_implementations)]
pub struct TimePickerOverlay<'a, Message, Theme>
where
    Message: Clone,
    Theme: StyleSheet + button::StyleSheet,
{
    /// The state of the [`TimePickerOverlay`].
    state: &'a mut State,
    /// The cancel button of the [`TimePickerOverlay`].
    cancel_button: Button<'a, Message, Renderer<Theme>>,
    /// The submit button of the [`TimePickerOverlay`].
    submit_button: Button<'a, Message, Renderer<Theme>>,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`] is pressed.
    on_submit: &'a dyn Fn(Time) -> Message,
    /// The position of the [`TimePickerOverlay`].
    position: Point,
    /// The style of the [`TimePickerOverlay`].
    style: <Theme as StyleSheet>::Style,
    /// The reference to the tree holding the state of this overlay.
    tree: &'a mut Tree,
}

impl<'a, Message, Theme> TimePickerOverlay<'a, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    /// Creates a new [`TimePickerOverlay`] on the given position.
    pub fn new(
        state: &'a mut time_picker::State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Time) -> Message,
        position: Point,
        style: <Theme as StyleSheet>::Style,
        tree: &'a mut Tree,
    ) -> Self {
        let time_picker::State { overlay_state } = state;

        TimePickerOverlay {
            state: overlay_state,
            cancel_button: Button::new(
                text::Text::new(icon_to_string(BootstrapIcon::X))
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel.clone()),
            submit_button: Button::new(
                text::Text::new(icon_to_string(BootstrapIcon::Check))
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel), // Sending a fake message
            on_submit,
            position,
            style,
            tree,
        }
    }

    /// Turn this [`TimePickerOverlay`] into an overlay [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer<Theme>> {
        overlay::Element::new(self.position, Box::new(self))
    }

    /// The event handling for the clock.
    #[allow(clippy::too_many_lines)]
    fn on_event_clock(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _shell: &mut Shell<Message>,
        _renderer: &Renderer<Theme>,
        _clipboard: &mut dyn Clipboard,
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
        _shell: &mut Shell<Message>,
        _renderer: &Renderer<Theme>,
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
    fn on_event_keyboard(
        &mut self,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _shell: &mut Shell<Message>,
        _renderer: &Renderer<Theme>,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
            let mut status = event::Status::Ignored;

            if matches!(key_code, keyboard::KeyCode::Tab) {
                if self.state.keyboard_modifiers.shift() {
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
                        keyboard_handle(key_code, &mut self.state.time, Duration::hours(1));
                    }
                    Focus::DigitalMinute => {
                        keyboard_handle(key_code, &mut self.state.time, Duration::minutes(1));
                    }
                    Focus::DigitalSecond => {
                        keyboard_handle(key_code, &mut self.state.time, Duration::seconds(1));
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

impl<'a, Message, Theme> Overlay<Message, Renderer<Theme>> for TimePickerOverlay<'a, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    fn layout(&self, renderer: &Renderer<Theme>, bounds: Size, position: Point) -> Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .pad(Padding::from(PADDING))
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(300.0)
            .max_height(350.0);

        // Digital Clock
        let digital_clock_limits = limits;
        let mut digital_clock = digital_clock(self, renderer, digital_clock_limits);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(
            0.0,
            digital_clock.bounds().height + cancel_button.bounds().height + 2.0 * SPACING,
        ));

        // Clock-Canvas
        let mut clock = Row::<(), Renderer<Theme>>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .layout(renderer, &limits);

        clock.move_to(Point::new(
            clock.bounds().x + PADDING,
            clock.bounds().y + PADDING,
        ));

        digital_clock.move_to(Point::new(
            digital_clock.bounds().x + PADDING,
            digital_clock.bounds().y + PADDING + SPACING + clock.bounds().height,
        ));

        // Buttons
        let cancel_limits =
            limits.max_width(((clock.bounds().width / 2.0) - BUTTON_SPACING).max(0.0));

        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let submit_limits =
            limits.max_width(((clock.bounds().width / 2.0) - BUTTON_SPACING).max(0.0));

        let mut submit_button = self.submit_button.layout(renderer, &submit_limits);

        cancel_button.move_to(Point {
            x: cancel_button.bounds().x + PADDING,
            y: cancel_button.bounds().y
                + clock.bounds().height
                + PADDING
                + digital_clock.bounds().height
                + 2.0 * SPACING,
        });

        submit_button.move_to(Point {
            x: submit_button.bounds().x + clock.bounds().width - submit_button.bounds().width
                + PADDING,
            y: submit_button.bounds().y
                + clock.bounds().height
                + PADDING
                + digital_clock.bounds().height
                + 2.0 * SPACING,
        });

        let mut node = Node::with_children(
            Size::new(
                clock.bounds().width + (2.0 * PADDING),
                clock.bounds().height
                    + digital_clock.bounds().height
                    + cancel_button.bounds().height
                    + (2.0 * PADDING)
                    + 2.0 * SPACING,
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
        cursor: Cursor,
        renderer: &Renderer<Theme>,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        if event::Status::Captured
            == self.on_event_keyboard(&event, layout, cursor, shell, renderer, clipboard)
        {
            return event::Status::Captured;
        }

        let mut children = layout.children();

        // Clock canvas
        let clock_layout = children
            .next()
            .expect("Native: Layout should have a clock canvas layout");
        let clock_status =
            self.on_event_clock(&event, clock_layout, cursor, shell, renderer, clipboard);

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
            cursor,
            shell,
            renderer,
            clipboard,
        );

        // ----------- Buttons ------------------------
        let cancel_button_layout = children
            .next()
            .expect("Native: Layout should have a cancel button layout for a TimePicker");

        let cancel_status = self.cancel_button.on_event(
            &mut self.tree.children[0],
            event.clone(),
            cancel_button_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );

        let submit_button_layout = children
            .next()
            .expect("Native: Layout should have a submit button layout for a TimePicker");

        let mut fake_messages: Vec<Message> = Vec::new();

        let submit_status = self.submit_button.on_event(
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
        }

        clock_status
            .merge(digital_clock_status)
            .merge(cancel_status)
            .merge(submit_status)
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer<Theme>,
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
            viewport,
            renderer,
        );

        let submit_button_layout = children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a TimePicker");

        let submit_mouse_interaction = self.submit_button.mouse_interaction(
            &self.tree.children[1],
            submit_button_layout,
            cursor,
            viewport,
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
        renderer: &mut Renderer<Theme>,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let mut style_sheet: HashMap<StyleState, Appearance> = HashMap::new();
        let _ = style_sheet.insert(StyleState::Active, StyleSheet::active(theme, &self.style));
        let _ = style_sheet.insert(
            StyleState::Selected,
            StyleSheet::selected(theme, &self.style),
        );
        let _ = style_sheet.insert(StyleState::Hovered, StyleSheet::hovered(theme, &self.style));
        let _ = style_sheet.insert(StyleState::Focused, StyleSheet::focused(theme, &self.style));

        let mut style_state = StyleState::Active;
        if self.state.focus == Focus::Overlay {
            style_state = style_state.max(StyleState::Focused);
        }
        if cursor.is_over(bounds) {
            style_state = style_state.max(StyleState::Hovered);
        }

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: style_sheet[&style_state].border_radius.into(),
                border_width: style_sheet[&style_state].border_width,
                border_color: style_sheet[&style_state].border_color,
            },
            style_sheet[&style_state].background,
        );

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
        if self.state.focus == Focus::Cancel {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: cancel_button_layout.bounds(),
                    border_radius: style_sheet[&StyleState::Focused].border_radius.into(),
                    border_width: style_sheet[&StyleState::Focused].border_width,
                    border_color: style_sheet[&StyleState::Focused].border_color,
                },
                Color::TRANSPARENT,
            );
        }

        if self.state.focus == Focus::Submit {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: submit_button_layout.bounds(),
                    border_radius: style_sheet[&StyleState::Focused].border_radius.into(),
                    border_width: style_sheet[&StyleState::Focused].border_width,
                    border_color: style_sheet[&StyleState::Focused].border_color,
                },
                Color::TRANSPARENT,
            );
        }
    }
}

/// Defines the layout of the digital clock of the time picker.
fn digital_clock<Message, Theme>(
    time_picker: &TimePickerOverlay<'_, Message, Theme>,
    renderer: &Renderer<Theme>,
    limits: Limits,
) -> Node
where
    Message: 'static + Clone,
    Theme: StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    let arrow_size = renderer.default_size();
    let font_size = 1.2 * renderer.default_size();

    let mut digital_clock_row = Row::<(), Renderer<Theme>>::new()
        .align_items(Alignment::Center)
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
                .align_items(Alignment::Center)
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
                .align_items(Alignment::Center)
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
                    .align_items(Alignment::Center)
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

    Container::new(digital_clock_row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .center_x()
        .center_y()
        .layout(renderer, &limits)
}

/// Draws the analog clock.
#[allow(clippy::too_many_lines)]
fn draw_clock<Message, Theme>(
    renderer: &mut Renderer<Theme>,
    time_picker: &TimePickerOverlay<'_, Message, Theme>,
    layout: Layout<'_>,
    cursor: Cursor,
    style: &HashMap<StyleState, Appearance>,
) where
    Message: 'static + Clone,
    Theme: StyleSheet + button::StyleSheet + text::StyleSheet,
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
                style: Style::Solid(
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

            let period_text = Text {
                content: format!("{period}"),
                position: center,
                color: style
                    .get(&clock_style_state)
                    .expect("Style Sheet not found.")
                    .clock_number_color,
                size: period_size,
                font: renderer.default_font(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
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
                    frame.stroke(&Path::line(center, *p), hand_stroke.clone());
                    frame.fill(
                        &Path::circle(*p, number_size * 0.8),
                        style
                            .get(&StyleState::Selected)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                    style_state = style_state.max(StyleState::Selected);
                }

                let text = Text {
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
                    size: number_size,
                    font: renderer.default_font(),
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    shaping: text::Shaping::Basic,
                    line_height: text::LineHeight::Relative(1.3),
                };

                frame.fill_text(text);
            });

            minute_points.iter().enumerate().for_each(|(i, p)| {
                let selected = time_picker.state.time.minute() == i as u32;

                let mut style_state = StyleState::Active;
                if selected {
                    frame.stroke(&Path::line(center, *p), hand_stroke.clone());
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
                    let text = Text {
                        content: format!("{i:02}"),
                        position: *p,
                        color: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .clock_number_color,
                        size: number_size,
                        font: renderer.default_font(),
                        horizontal_alignment: Horizontal::Center,
                        vertical_alignment: Vertical::Center,
                        shaping: text::Shaping::Basic,
                        line_height: text::LineHeight::Relative(1.3),
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
                        frame.stroke(&Path::line(center, *p), hand_stroke.clone());
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
                        let text = Text {
                            content: format!("{i:02}"),
                            position: *p,
                            color: style
                                .get(&style_state)
                                .expect("Style Sheet not found.")
                                .clock_number_color,
                            size: number_size,
                            font: renderer.default_font(),
                            horizontal_alignment: Horizontal::Center,
                            vertical_alignment: Vertical::Center,
                            shaping: text::Shaping::Basic,
                            line_height: text::LineHeight::Relative(1.3),
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
        renderer.draw(vec![geometry]);
    });
}

/// Draws the digital clock.
#[allow(clippy::too_many_lines)]
fn draw_digital_clock<Message, Theme>(
    renderer: &mut Renderer<Theme>,
    time_picker: &TimePickerOverlay<'_, Message, Theme>,
    layout: Layout<'_>,
    cursor: Cursor,
    style: &HashMap<StyleState, Appearance>,
) where
    Message: 'static + Clone,
    Theme: StyleSheet + button::StyleSheet + text::StyleSheet,
{
    //println!("layout: {:#?}", layout);
    let mut children = layout
        .children()
        .next()
        .expect("Graphics: Layout should have digital clock children")
        .children();

    let f = |renderer: &mut Renderer<Theme>, layout: Layout<'_>, text: String, target: Focus| {
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
                    border_color: style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .border_color,
                    border_radius: style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .border_radius
                        .into(),
                    border_width: style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .border_width,
                },
                style
                    .get(&style_state)
                    .expect("Style Sheet not found.")
                    .background,
            );
        }

        let mut buffer = [0; 4];

        // Caret up
        renderer.fill_text(core::Text {
            content: char::from(BootstrapIcon::CaretUpFill).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: up_bounds.center_x(),
                y: up_bounds.center_y(),
                ..up_bounds
            },
            size: renderer.default_size() + if up_arrow_hovered { 1.0 } else { 0.0 },
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: crate::graphics::icons::BOOTSTRAP_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
        });

        // Text
        renderer.fill_text(core::Text {
            content: &text,
            bounds: Rectangle {
                x: center_bounds.center_x(),
                y: center_bounds.center_y(),
                ..center_bounds
            },
            size: renderer.default_size(),
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: renderer.default_font(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
        });

        // Down caret
        renderer.fill_text(core::Text {
            content: char::from(BootstrapIcon::CaretDownFill).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: down_bounds.center_x(),
                y: down_bounds.center_y(),
                ..down_bounds
            },
            size: renderer.default_size() + if down_arrow_hovered { 1.0 } else { 0.0 },
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: crate::graphics::icons::BOOTSTRAP_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
        });
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
    renderer.fill_text(core::Text {
        content: ":",
        bounds: Rectangle {
            x: hour_minute_separator.bounds().center_x(),
            y: hour_minute_separator.bounds().center_y(),
            ..hour_minute_separator.bounds()
        },
        size: renderer.default_size(),
        color: style[&StyleState::Active].text_color,
        font: renderer.default_font(),
        horizontal_alignment: Horizontal::Center,
        vertical_alignment: Vertical::Center,
        line_height: text::LineHeight::Relative(1.3),
        shaping: text::Shaping::Basic,
    });

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
        renderer.fill_text(core::Text {
            content: ":",
            bounds: Rectangle {
                x: minute_second_separator.bounds().center_x(),
                y: minute_second_separator.bounds().center_y(),
                ..minute_second_separator.bounds()
            },
            size: renderer.default_size(),
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: renderer.default_font(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
        });

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
        renderer.fill_text(core::Text {
            content: if time_picker.state.time.hour12().0 {
                "PM"
            } else {
                "AM"
            },
            bounds: Rectangle {
                x: period.bounds().center_x(),
                y: period.bounds().center_y(),
                ..period.bounds()
            },
            size: renderer.default_size(),
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: renderer.default_font(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
        });
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
    pub fn new(time: Time) -> Self {
        Self {
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
    Theme: StyleSheet + button::StyleSheet,
{
    /// The cancel button of the [`TimePickerOverlay`].
    cancel_button: Element<'a, Message, Renderer<Theme>>,
    /// The submit button of the [`TimePickerOverlay`].
    submit_button: Element<'a, Message, Renderer<Theme>>,
}

impl<'a, Message, Theme> Default for TimePickerOverlayButtons<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet,
{
    fn default() -> Self {
        Self {
            cancel_button: Button::new(
                text::Text::new(icon_to_string(BootstrapIcon::X))
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .into(),
            submit_button: Button::new(
                text::Text::new(icon_to_string(BootstrapIcon::Check))
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .into(),
        }
    }
}

#[allow(clippy::unimplemented)]
impl<'a, Message, Theme> Widget<Message, Renderer<Theme>>
    for TimePickerOverlayButtons<'a, Message, Theme>
where
    Message: Clone,
    Theme: StyleSheet + button::StyleSheet,
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

    fn width(&self) -> Length {
        unimplemented!("This should never be reached!")
    }

    fn height(&self) -> Length {
        unimplemented!("This should never be reached!")
    }

    fn layout(&self, _renderer: &Renderer<Theme>, _limits: &Limits) -> Node {
        unimplemented!("This should never be reached!")
    }

    fn draw(
        &self,
        _state: &Tree,
        _renderer: &mut Renderer<Theme>,
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
    for Element<'a, Message, Renderer<Theme>>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet,
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
