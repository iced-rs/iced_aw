//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
use std::collections::HashMap;

use crate::time_picker::{self, Time};
use crate::{
    core::clock::{
        NearestRadius, HOUR_RADIUS_PERCENTAGE, HOUR_RADIUS_PERCENTAGE_NO_SECONDS,
        MINUTE_RADIUS_PERCENTAGE, MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, PERIOD_PERCENTAGE,
        SECOND_RADIUS_PERCENTAGE,
    },
    core::{clock, overlay::Position, time::Period},
    native::IconText,
    style::style_state::StyleState,
    Icon,
};
use chrono::{Duration, Local, NaiveTime, Timelike};
use iced_graphics::{
    widget::canvas::{self, stroke::Style, LineCap, Path, Stroke},
    Backend, Renderer,
};
use iced_native::widget::Tree;
use iced_native::{
    alignment::{Horizontal, Vertical},
    event, keyboard,
    layout::{self, Limits},
    mouse, overlay, renderer,
    text::Renderer as _,
    touch,
    widget::{Button, Column, Container, Row, Text},
    Alignment, Clipboard, Color, Element, Event, Layout, Length, Padding, Point, Rectangle,
    Renderer as _, Shell, Size, Vector, Widget,
};

pub use crate::style::time_picker::{Appearance, StyleSheet};

/// The padding around the elements.
const PADDING: u16 = 10;
/// The spacing between the elements.
const SPACING: u16 = 15;
/// The spacing between the buttons.
const BUTTON_SPACING: u16 = 5;
/// The percentage size of the numbers.
const NUMBER_SIZE_PERCENTAGE: f32 = 0.15;
/// The percentage size of the period.
const PERIOD_SIZE_PERCENTAGE: f32 = 0.2;

/// The overlay of the [`TimePicker`](crate::native::TimePicker).
#[allow(missing_debug_implementations)]
pub struct TimePickerOverlay<'a, Message, B, Theme>
where
    Message: Clone,
    B: Backend,
    Theme: StyleSheet + iced_style::button::StyleSheet,
{
    /// The state of the [`TimePickerOverlay`](TimePickerOverlay).
    state: &'a mut State,
    /// The cancel button of the [`TimePickerOverlay`](TimePickerOverlay).
    cancel_button: Button<'a, Message, Renderer<B, Theme>>,
    /// The submit button of the [`TimePickerOverlay`](TimePickerOverlay).
    submit_button: Button<'a, Message, Renderer<B, Theme>>,
    /// The function that produces a message when the submit button of the [`TimePickerOverlay`] is pressed.
    on_submit: &'a dyn Fn(Time) -> Message,
    /// The position of the [`TimePickerOverlay`](TimePickerOverlay).
    position: Point,
    /// The style of the [`TimePickerOverlay`](TimePickerOverlay).
    style: <Theme as StyleSheet>::Style,
    /// The reference to the tree holding the state of this overlay.
    tree: &'a mut Tree,
}

impl<'a, Message, B, Theme> TimePickerOverlay<'a, Message, B, Theme>
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: 'a
        + StyleSheet
        + iced_style::button::StyleSheet
        + iced_style::text::StyleSheet
        + iced_style::container::StyleSheet,
{
    /// Creates a new [`TimePickerOverlay`](TimePickerOverlay) on the given
    /// position.
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
            cancel_button: Button::new(IconText::new(Icon::X).width(Length::Fill))
                .width(Length::Fill)
                .on_press(on_cancel.clone()),
            submit_button: Button::new(IconText::new(Icon::Check).width(Length::Fill))
                .width(Length::Fill)
                .on_press(on_cancel), // Sending a fake message
            on_submit,
            position,
            style,
            tree,
        }
    }

    /// Turn this [`TimePickerOverlay`](TimePickerOverlay) into an overlay
    /// [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer<B, Theme>> {
        overlay::Element::new(self.position, Box::new(self))
    }

    /// The event handling for the clock.
    #[allow(clippy::too_many_lines)]
    fn on_event_clock(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _shell: &mut Shell<Message>,
        _renderer: &Renderer<B, Theme>,
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
        cursor_position: Point,
        _shell: &mut Shell<Message>,
        _renderer: &Renderer<B, Theme>,
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
            self.state.clock_cache.clear();
        }

        digital_clock_status
    }

    /// The event handling for the keyboard input.
    fn on_event_keyboard(
        &mut self,
        event: &Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _shell: &mut Shell<Message>,
        _renderer: &Renderer<B, Theme>,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
            let mut status = event::Status::Ignored;

            if let keyboard::KeyCode::Tab = key_code {
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

impl<'a, Message, B, Theme> iced_native::Overlay<Message, Renderer<B, Theme>>
    for TimePickerOverlay<'a, Message, B, Theme>
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: 'a
        + StyleSheet
        + iced_style::button::StyleSheet
        + iced_style::text::StyleSheet
        + iced_style::container::StyleSheet,
{
    fn layout(
        &self,
        renderer: &Renderer<B, Theme>,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .pad(Padding::from(PADDING))
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
        let mut clock = Row::<(), Renderer<B, Theme>>::new()
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
            .max_width(((clock.bounds().width / 2.0) - f32::from(BUTTON_SPACING)).max(0.0) as u32);

        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let submit_limits = limits
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
        renderer: &Renderer<B, Theme>,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        if event::Status::Captured
            == self.on_event_keyboard(&event, layout, cursor_position, shell, renderer, clipboard)
        {
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
            shell,
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
            cursor_position,
            renderer,
            clipboard,
            shell,
        );

        let submit_button_layout = children
            .next()
            .expect("Native: Layout should have a submit button layout for a TimePicker");

        let mut fake_messages: Vec<Message> = Vec::new();

        let submit_status = self.submit_button.on_event(
            &mut self.tree.children[1],
            event,
            submit_button_layout,
            cursor_position,
            renderer,
            clipboard,
            &mut Shell::new(&mut fake_messages),
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
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer<B, Theme>,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let mouse_interaction = mouse::Interaction::default();

        // Clock canvas
        let clock_layout = children
            .next()
            .expect("Graphics: Layout should have a clock canvas layout");
        let clock_mouse_interaction = if clock_layout.bounds().contains(cursor_position) {
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

        let f = |layout: iced_native::Layout<'_>| {
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

            let up_arrow_hovered = up_bounds.contains(cursor_position);
            let down_arrow_hovered = down_bounds.contains(cursor_position);

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
            cursor_position,
            viewport,
            renderer,
        );

        let submit_button_layout = children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a TimePicker");

        let submit_mouse_interaction = self.submit_button.mouse_interaction(
            &self.tree.children[1],
            submit_button_layout,
            cursor_position,
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
        renderer: &mut Renderer<B, Theme>,
        theme: &<Renderer<B, Theme> as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let mut style_sheet: HashMap<StyleState, Appearance> = HashMap::new();
        let _ = style_sheet.insert(StyleState::Active, StyleSheet::active(theme, self.style));
        let _ = style_sheet.insert(
            StyleState::Selected,
            StyleSheet::selected(theme, self.style),
        );
        let _ = style_sheet.insert(StyleState::Hovered, StyleSheet::hovered(theme, self.style));
        let _ = style_sheet.insert(StyleState::Focused, StyleSheet::focused(theme, self.style));

        let mut style_state = StyleState::Active;
        if self.state.focus == Focus::Overlay {
            style_state = style_state.max(StyleState::Focused);
        }
        if bounds.contains(cursor_position) {
            style_state = style_state.max(StyleState::Hovered);
        }

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: style_sheet[&style_state].border_radius,
                border_width: style_sheet[&style_state].border_width,
                border_color: style_sheet[&style_state].border_color,
            },
            style_sheet[&style_state].background,
        );

        // ----------- Clock canvas --------------------
        let clock_layout = children
            .next()
            .expect("Graphics: Layout should have a clock canvas layout");
        draw_clock(renderer, self, clock_layout, cursor_position, &style_sheet);

        // ----------- Digital clock ------------------
        let digital_clock_layout = children
            .next()
            .expect("Graphics: Layout should have a digital clock layout");
        draw_digital_clock(
            renderer,
            self,
            digital_clock_layout,
            cursor_position,
            &style_sheet,
        );

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
            cursor_position,
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
            cursor_position,
            &bounds,
        );

        // Buttons are not focusable right now...
        if self.state.focus == Focus::Cancel {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: cancel_button_layout.bounds(),
                    border_radius: style_sheet[&StyleState::Focused].border_radius,
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
                    border_radius: style_sheet[&StyleState::Focused].border_radius,
                    border_width: style_sheet[&StyleState::Focused].border_width,
                    border_color: style_sheet[&StyleState::Focused].border_color,
                },
                Color::TRANSPARENT,
            );
        }
    }
}

/// Defines the layout of the digital clock of the time picker.
fn digital_clock<'a, Message, B, Theme>(
    time_picker: &TimePickerOverlay<'a, Message, B, Theme>,
    renderer: &Renderer<B, Theme>,
    limits: Limits,
) -> iced_native::layout::Node
where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: StyleSheet
        + iced_style::button::StyleSheet
        + iced_style::text::StyleSheet
        + iced_style::container::StyleSheet,
{
    let arrow_size = renderer.default_size();
    let font_size = (1.2 * f32::from(renderer.default_size())) as u16;

    let mut digital_clock_row = Row::<(), Renderer<B, Theme>>::new()
        .align_items(Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .spacing(1);

    if !time_picker.state.use_24h {
        digital_clock_row = digital_clock_row.push(
            Column::new() // Just a placeholder
                .height(Length::Shrink)
                .push(Text::new("AM").size(font_size)),
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
                .align_items(Alignment::Center)
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
                    .align_items(Alignment::Center)
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
            );
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

/// Draws the analog clock.
#[allow(clippy::too_many_lines)]
fn draw_clock<'a, Message, B, Theme>(
    renderer: &mut Renderer<B, Theme>,
    time_picker: &TimePickerOverlay<'a, Message, B, Theme>,
    layout: iced_native::Layout<'_>,
    cursor_position: Point,
    style: &HashMap<StyleState, Appearance>,
) where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: StyleSheet + iced_style::button::StyleSheet + iced_style::text::StyleSheet,
{
    let mut clock_style_state = StyleState::Active;
    if layout.bounds().contains(cursor_position) {
        clock_style_state = clock_style_state.max(StyleState::Hovered);
    }

    let geometry = time_picker
        .state
        .clock_cache
        .draw(layout.bounds().size(), |frame| {
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

            let internal_cursor_position =
                cursor_position - Vector::new(layout.bounds().x, layout.bounds().y);

            let nearest_radius = if layout.bounds().contains(cursor_position) {
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
                    internal_cursor_position,
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
                        [crate::core::clock::nearest_point(&hour_points, internal_cursor_position)];

                    frame.fill(
                        &Path::circle(nearest_point, 5.0),
                        style
                            .get(&StyleState::Hovered)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                }
                NearestRadius::Minute => {
                    let nearest_point = minute_points[crate::core::clock::nearest_point(
                        &minute_points,
                        internal_cursor_position,
                    )];

                    frame.fill(
                        &Path::circle(nearest_point, 5.0),
                        style
                            .get(&StyleState::Hovered)
                            .expect("Style Sheet not found.")
                            .clock_number_background,
                    );
                }
                NearestRadius::Second => {
                    let nearest_point = second_points[crate::core::clock::nearest_point(
                        &second_points,
                        internal_cursor_position,
                    )];

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

            let period_text = canvas::Text {
                content: format!("{period}"),
                position: center,
                color: style
                    .get(&clock_style_state)
                    .expect("Style Sheet not found.")
                    .clock_number_color,
                size: period_size,
                font: iced_graphics::Font::default(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
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

                let text = canvas::Text {
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
                    font: iced_graphics::Font::default(),
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
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
                    let text = canvas::Text {
                        content: format!("{i:02}"),
                        position: *p,
                        color: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .clock_number_color,
                        size: number_size,
                        font: iced_graphics::Font::default(),
                        horizontal_alignment: Horizontal::Center,
                        vertical_alignment: Vertical::Center,
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
                        let text = canvas::Text {
                            content: format!("{i:02}"),
                            position: *p,
                            color: style
                                .get(&style_state)
                                .expect("Style Sheet not found.")
                                .clock_number_color,
                            size: number_size,
                            font: iced_graphics::Font::default(),
                            horizontal_alignment: Horizontal::Center,
                            vertical_alignment: Vertical::Center,
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

    // TODO: find out how to render a canvas
    let translation = Vector::new(layout.bounds().x, layout.bounds().y);
    renderer.with_translation(translation, |renderer| {
        renderer.draw_primitive(geometry.into_primitive());
    });
}

/// Draws the digital clock.
#[allow(clippy::too_many_lines)]
fn draw_digital_clock<'a, Message, B, Theme>(
    renderer: &mut Renderer<B, Theme>,
    time_picker: &TimePickerOverlay<'a, Message, B, Theme>,
    layout: iced_native::Layout<'_>,
    cursor_position: Point,
    style: &HashMap<StyleState, Appearance>,
) where
    Message: 'static + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: StyleSheet + iced_style::button::StyleSheet + iced_style::text::StyleSheet,
{
    //println!("layout: {:#?}", layout);
    let mut children = layout
        .children()
        .next()
        .expect("Graphics: Layout should have digital clock children")
        .children();

    let f = |renderer: &mut Renderer<B, Theme>,
             layout: iced_native::Layout<'_>,
             text: String,
             target: Focus| {
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

        let up_arrow_hovered = up_bounds.contains(cursor_position);
        let down_arrow_hovered = down_bounds.contains(cursor_position);

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
                        .border_radius,
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
        renderer.fill_text(iced_native::text::Text {
            content: char::from(Icon::CaretUpFill).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: up_bounds.center_x(),
                y: up_bounds.center_y(),
                ..up_bounds
            },
            size: up_bounds.height + if up_arrow_hovered { 5.0 } else { 0.0 },
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: crate::graphics::icons::ICON_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        });

        // Text
        renderer.fill_text(iced_native::text::Text {
            content: &text,
            bounds: Rectangle {
                x: center_bounds.center_x(),
                y: center_bounds.center_y(),
                ..center_bounds
            },
            size: center_bounds.height,
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: iced_graphics::Font::default(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        });

        // Down caret
        renderer.fill_text(iced_native::text::Text {
            content: char::from(Icon::CaretDownFill).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: down_bounds.center_x(),
                y: down_bounds.center_y(),
                ..down_bounds
            },
            size: down_bounds.height + if down_arrow_hovered { 5.0 } else { 0.0 },
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: crate::graphics::icons::ICON_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
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
    renderer.fill_text(iced_native::text::Text {
        content: ":",
        bounds: Rectangle {
            x: hour_minute_separator.bounds().center_x(),
            y: hour_minute_separator.bounds().center_y(),
            ..hour_minute_separator.bounds()
        },
        size: hour_minute_separator.bounds().height,
        color: style[&StyleState::Active].text_color,
        font: iced_graphics::Font::default(),
        horizontal_alignment: Horizontal::Center,
        vertical_alignment: Vertical::Center,
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
        renderer.fill_text(iced_native::text::Text {
            content: ":",
            bounds: Rectangle {
                x: minute_second_separator.bounds().center_x(),
                y: minute_second_separator.bounds().center_y(),
                ..minute_second_separator.bounds()
            },
            size: minute_second_separator.bounds().height,
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: iced_graphics::Font::default(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
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
        renderer.fill_text(iced_native::text::Text {
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
            size: period.bounds().height,
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: iced_graphics::Font::default(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        });
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
pub struct TimePickerOverlayButtons<'a, Message, B, Theme>
where
    Message: Clone,
    B: Backend,
    Theme: StyleSheet + iced_style::button::StyleSheet,
{
    /// The cancel button of the [`TimePickerOverlay`](TimePickerOverlay).
    cancel_button: Element<'a, Message, Renderer<B, Theme>>,
    /// The submit button of the [`TimePickerOverlay`](TimePickerOverlay).
    submit_button: Element<'a, Message, Renderer<B, Theme>>,
}

impl<'a, Message, B, Theme> Default for TimePickerOverlayButtons<'a, Message, B, Theme>
where
    Message: 'a + Clone,
    B: 'a + Backend + iced_graphics::backend::Text,
    Theme: 'a + StyleSheet + iced_style::button::StyleSheet + iced_style::text::StyleSheet,
{
    fn default() -> Self {
        Self {
            cancel_button: Button::new(IconText::new(Icon::X)).into(),
            submit_button: Button::new(IconText::new(Icon::Check)).into(),
        }
    }
}

#[allow(clippy::unimplemented)]
impl<'a, Message, B, Theme> iced_native::Widget<Message, Renderer<B, Theme>>
    for TimePickerOverlayButtons<'a, Message, B, Theme>
where
    Message: Clone,
    B: Backend,
    Theme: StyleSheet + iced_style::button::StyleSheet,
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

    fn layout(
        &self,
        _renderer: &Renderer<B, Theme>,
        _limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        unimplemented!("This should never be reached!")
    }

    fn draw(
        &self,
        _state: &Tree,
        _renderer: &mut Renderer<B, Theme>,
        _theme: &<Renderer<B, Theme> as iced_native::Renderer>::Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        unimplemented!("This should never be reached!")
    }
}

impl<'a, Message, B, Theme> From<TimePickerOverlayButtons<'a, Message, B, Theme>>
    for Element<'a, Message, Renderer<B, Theme>>
where
    Message: 'a + Clone,
    B: 'a + Backend,
    Theme: 'a + StyleSheet + iced_style::button::StyleSheet,
{
    fn from(overlay: TimePickerOverlayButtons<'a, Message, B, Theme>) -> Self {
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

/// An enumeration of all focusable elements of the [`TimePickerOverlay`](TimePickerOverlay).
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
