//! Use a time picker as an input element for picking times.
//!
//! *This API requires the following crate features to be activated: `time_picker`*
use crate::{
    core::renderer::DrawEnvironment, native::overlay::time_picker::Focus,
    style::style_state::StyleState,
};
use std::collections::HashMap;

use crate::{
    core::clock::{
        self, NearestRadius, HOUR_RADIUS_PERCENTAGE, HOUR_RADIUS_PERCENTAGE_NO_SECONDS,
        MINUTE_RADIUS_PERCENTAGE, MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, PERIOD_PERCENTAGE,
        SECOND_RADIUS_PERCENTAGE,
    },
    style::time_picker::{Style, StyleSheet},
};
use canvas::{Cache, LineCap, Path, Stroke, Text};
use chrono::{NaiveTime, Timelike};
use iced_graphics::{
    backend, canvas, Backend, Color, HorizontalAlignment, Point, Primitive, Rectangle, Renderer,
    Vector, VerticalAlignment,
};
use iced_native::mouse;

use crate::native::time_picker;
pub use crate::native::time_picker::{Period, State, Time};

use super::icons::{Icon, ICON_FONT};

const NUMBER_SIZE_PERCENTAGE: f32 = 0.15;
const PERIOD_SIZE_PERCENTAGE: f32 = 0.2;

/// An input element for picking times.
///
/// This is an alias of an `iced_native` `TimePicker` with an `iced_wgpu::Renderer`.
pub type TimePicker<'a, Message, Backend> = time_picker::TimePicker<'a, Message, Renderer<Backend>>;

impl<B> time_picker::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<Self::Defaults, Self::Style, Focus>,
        state: &crate::native::overlay::time_picker::State,
        cancel_button: &iced_native::Element<'_, Message, Self>,
        submit_button: &iced_native::Element<'_, Message, Self>,
    ) -> Self::Output {
        let bounds = env.layout.bounds();
        let mut children = env.layout.children();

        let mut style: HashMap<StyleState, Style> = HashMap::new();
        let _ = style.insert(StyleState::Active, env.style_sheet.active());
        let _ = style.insert(StyleState::Selected, env.style_sheet.selected());
        let _ = style.insert(StyleState::Hovered, env.style_sheet.hovered());
        let _ = style.insert(StyleState::Focused, env.style_sheet.focused());

        let mouse_interaction = mouse::Interaction::default();

        let mut style_state = StyleState::Active;
        if env.focus == Focus::Overlay {
            style_state = style_state.max(StyleState::Focused);
        }
        if bounds.contains(env.cursor_position) {
            style_state = style_state.max(StyleState::Hovered);
        }

        let background = Primitive::Quad {
            bounds,
            background: style[&style_state].background,
            border_radius: style[&style_state].border_radius,
            border_width: style[&style_state].border_width,
            border_color: style[&style_state].border_color,
        };

        // ----------- Clock canvas --------------------
        let clock_layout = children
            .next()
            .expect("Graphics: Layout should have a clock canvas layout");
        let (clock, clock_mouse_interaction) = clock(
            clock_layout,
            state.time,
            &state.clock_cache,
            env.cursor_position,
            state.use_24h,
            state.show_seconds,
            &style,
        );

        // ----------- Digital clock ------------------
        let digital_clock_layout = children
            .next()
            .expect("Graphics: Layout should have a digital clock layout");
        let (digital_clock, digital_clock_mouse_interaction) = digital_clock(
            digital_clock_layout,
            state.time,
            env.cursor_position,
            state.use_24h,
            state.show_seconds,
            &style,
            env.focus,
        );

        // ----------- Buttons ------------------------
        let cancel_button_layout = children
            .next()
            .expect("Graphics: Layout should have a cancel button layout for a TimePicker");

        let (cancel_button, cancel_mouse_interaction) = cancel_button.draw(
            self,
            env.defaults,
            cancel_button_layout,
            env.cursor_position,
            &bounds,
        );

        let submit_button_layout = children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a TimePicker");

        let (submit_button, submit_mouse_interaction) = submit_button.draw(
            self,
            env.defaults,
            submit_button_layout,
            env.cursor_position,
            &bounds,
        );

        // Buttons are not focusable right now...
        let cancel_button_focus = if env.focus == Focus::Cancel {
            Primitive::Quad {
                bounds: cancel_button_layout.bounds(),
                background: Color::TRANSPARENT.into(),
                border_radius: style[&StyleState::Focused].border_radius,
                border_width: style[&StyleState::Focused].border_width,
                border_color: style[&StyleState::Focused].border_color,
            }
        } else {
            Primitive::None
        };

        let submit_button_focus = if env.focus == Focus::Submit {
            Primitive::Quad {
                bounds: submit_button_layout.bounds(),
                background: Color::TRANSPARENT.into(),
                border_radius: style[&StyleState::Focused].border_radius,
                border_width: style[&StyleState::Focused].border_width,
                border_color: style[&StyleState::Focused].border_color,
            }
        } else {
            Primitive::None
        };

        (
            Primitive::Group {
                primitives: vec![
                    background,
                    clock,
                    digital_clock,
                    cancel_button,
                    submit_button,
                    cancel_button_focus,
                    submit_button_focus,
                ],
            },
            mouse_interaction
                .max(clock_mouse_interaction)
                .max(digital_clock_mouse_interaction)
                .max(cancel_mouse_interaction)
                .max(submit_mouse_interaction),
        )
    }
}

/// Draws the analog clock.
fn clock(
    layout: iced_native::Layout<'_>,
    time: NaiveTime,
    clock_cache: &Cache,
    cursor_position: Point,
    use_24h: bool,
    show_seconds: bool,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction) {
    let mut clock_style_state = StyleState::Active;
    let mut clock_mouse_interaction = mouse::Interaction::default();
    if layout.bounds().contains(cursor_position) {
        clock_style_state = clock_style_state.max(StyleState::Hovered);
        clock_mouse_interaction = clock_mouse_interaction.max(mouse::Interaction::Pointer);
    }

    let clock = clock_cache
        .draw(layout.bounds().size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) * 0.5;
            let period = if time.hour12().0 {
                clock::Period::PM
            } else {
                clock::Period::AM
            };

            let number_size = radius * NUMBER_SIZE_PERCENTAGE;
            let period_size = radius * PERIOD_SIZE_PERCENTAGE;

            let period_radius = radius * PERIOD_PERCENTAGE;

            let (hour_radius, minute_radius, second_radius) = if show_seconds {
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
                    &if show_seconds {
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
                width: style.get(&clock_style_state).unwrap().clock_hand_width,
                color: style.get(&clock_style_state).unwrap().clock_hand_color,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            match nearest_radius {
                NearestRadius::Period => {
                    frame.fill(
                        &Path::circle(center, period_size),
                        style
                            .get(&StyleState::Hovered)
                            .unwrap()
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
                            .unwrap()
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
                            .unwrap()
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
                            .unwrap()
                            .clock_number_background,
                    );
                }
                NearestRadius::None => {}
            }

            let period_text = Text {
                content: format!("{}", period),
                position: center,
                color: style.get(&clock_style_state).unwrap().clock_number_color,
                size: period_size,
                font: iced_graphics::Font::default(),
                horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                vertical_alignment: iced_graphics::VerticalAlignment::Center,
            };
            frame.fill_text(period_text);

            hour_points.iter().enumerate().for_each(|(i, p)| {
                let (pm, selected) = {
                    let (pm, _) = time.hour12();
                    let hour = time.hour();
                    (pm, hour % 12 == i as u32)
                };

                let mut style_state = StyleState::Active;
                if selected {
                    frame.fill(
                        &Path::circle(*p, number_size * 0.8),
                        style
                            .get(&StyleState::Selected)
                            .unwrap()
                            .clock_number_background,
                    );
                    frame.stroke(&Path::line(center, *p), hand_stroke);
                    style_state = style_state.max(StyleState::Selected);
                }

                let text = Text {
                    content: format!(
                        "{}",
                        if pm && use_24h {
                            i + 12
                        } else if !use_24h && i == 0 {
                            12
                        } else {
                            i
                        }
                    ),
                    position: *p,
                    color: style.get(&style_state).unwrap().clock_number_color,
                    size: number_size,
                    font: iced_graphics::Font::default(),
                    horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                    vertical_alignment: iced_graphics::VerticalAlignment::Center,
                };

                frame.fill_text(text);
            });

            minute_points.iter().enumerate().for_each(|(i, p)| {
                let selected = time.minute() == i as u32;

                let mut style_state = StyleState::Active;
                if selected {
                    frame.fill(
                        &Path::circle(*p, number_size * 0.5),
                        style
                            .get(&StyleState::Selected)
                            .unwrap()
                            .clock_number_background,
                    );
                    frame.stroke(&Path::line(center, *p), hand_stroke);
                    style_state = style_state.max(StyleState::Selected)
                }

                if i % 5 == 0 {
                    let text = Text {
                        content: format!("{:02}", i),
                        position: *p,
                        color: style.get(&style_state).unwrap().clock_number_color,
                        size: number_size,
                        font: iced_graphics::Font::default(),
                        horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                        vertical_alignment: iced_graphics::VerticalAlignment::Center,
                    };

                    frame.fill_text(text);
                } else {
                    let circle = Path::circle(*p, number_size * 0.1);
                    frame.fill(
                        &circle,
                        style.get(&StyleState::Active).unwrap().clock_dots_color,
                    );
                }
            });

            if show_seconds {
                second_points.iter().enumerate().for_each(|(i, p)| {
                    let selected = time.second() == i as u32;

                    let mut style_state = StyleState::Active;
                    if selected {
                        frame.fill(
                            &Path::circle(*p, number_size * 0.5),
                            style
                                .get(&StyleState::Selected)
                                .unwrap()
                                .clock_number_background,
                        );
                        frame.stroke(&Path::line(center, *p), hand_stroke);
                        style_state = style_state.max(StyleState::Selected);
                    }

                    if i % 10 == 0 {
                        let text = Text {
                            content: format!("{:02}", i),
                            position: *p,
                            color: style.get(&style_state).unwrap().clock_number_color,
                            size: number_size,
                            font: iced_graphics::Font::default(),
                            horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                            vertical_alignment: iced_graphics::VerticalAlignment::Center,
                        };

                        frame.fill_text(text);
                    } else {
                        let circle = Path::circle(*p, number_size * 0.1);
                        frame.fill(
                            &circle,
                            style.get(&StyleState::Active).unwrap().clock_dots_color,
                        );
                    }
                })
            }
        })
        .into_primitive();
    let clock = Primitive::Translate {
        translation: Vector::new(layout.bounds().x, layout.bounds().y),
        content: Box::new(clock),
    };

    (clock, clock_mouse_interaction)
}

/// Draws the digital clock.
fn digital_clock(
    layout: iced_native::Layout<'_>,
    time: NaiveTime,
    cursor_position: Point,
    use_24h: bool,
    show_seconds: bool,
    style: &HashMap<StyleState, Style>,
    focus: Focus,
) -> (Primitive, mouse::Interaction) {
    //println!("layout: {:#?}", layout);
    let mut children = layout
        .children()
        .next()
        .expect("Graphics: Layout should have digital clock children")
        .children();

    let f = |layout: iced_native::Layout<'_>, text: String, target: Focus| {
        let style_state = if focus == target {
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

        let mut mouse_interaction = mouse::Interaction::default();

        let up_arrow_hovered = up_bounds.contains(cursor_position);
        let down_arrow_hovered = down_bounds.contains(cursor_position);

        if up_arrow_hovered || down_arrow_hovered {
            mouse_interaction = mouse_interaction.max(mouse::Interaction::Pointer);
        }

        let primitive = Primitive::Group {
            primitives: vec![
                if style_state == StyleState::Focused {
                    Primitive::Quad {
                        bounds: layout.bounds(),
                        background: style.get(&style_state).unwrap().background,
                        border_color: style.get(&style_state).unwrap().border_color,
                        border_radius: style.get(&style_state).unwrap().border_radius,
                        border_width: style.get(&style_state).unwrap().border_width,
                    }
                } else {
                    Primitive::None
                },
                Primitive::Text {
                    content: Icon::CaretUpFill.into(),
                    bounds: Rectangle {
                        x: up_bounds.center_x(),
                        y: up_bounds.center_y(),
                        ..up_bounds
                    },
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: up_bounds.height + if up_arrow_hovered { 5.0 } else { 0.0 },
                    font: ICON_FONT,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },
                Primitive::Text {
                    content: text,
                    bounds: Rectangle {
                        x: center_bounds.center_x(),
                        y: center_bounds.center_y(),
                        ..center_bounds
                    },
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: center_bounds.height,
                    font: iced_graphics::Font::default(),
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },
                Primitive::Text {
                    content: Icon::CaretDownFill.into(),
                    bounds: Rectangle {
                        x: down_bounds.center_x(),
                        y: down_bounds.center_y(),
                        ..down_bounds
                    },
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: down_bounds.height + if down_arrow_hovered { 5.0 } else { 0.0 },
                    font: ICON_FONT,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },
            ],
        };

        (primitive, mouse_interaction)
    };

    if !use_24h {
        // Placeholder
        let _ = children.next();
    }

    let mouse_interaction = mouse::Interaction::default();

    let hour_layout = children
        .next()
        .expect("Graphics: Layout should have a hour layout");
    let (hour, hour_mouse_interaction) = f(
        hour_layout,
        format!(
            "{:02}",
            if use_24h {
                time.hour()
            } else {
                time.hour12().1
            }
        ),
        Focus::DigitalHour,
    );

    let hour_minute_separator = children
        .next()
        .expect("Graphics: Layout should have a hour/minute separator layout");
    let hour_minute_separator = Primitive::Text {
        content: ":".to_owned(),
        bounds: Rectangle {
            x: hour_minute_separator.bounds().center_x(),
            y: hour_minute_separator.bounds().center_y(),
            ..hour_minute_separator.bounds()
        },
        color: style[&StyleState::Active].text_color,
        size: hour_minute_separator.bounds().height,
        font: iced_graphics::Font::default(),
        horizontal_alignment: HorizontalAlignment::Center,
        vertical_alignment: VerticalAlignment::Center,
    };

    let minute_layout = children
        .next()
        .expect("Graphics: Layout should have a minute layout");
    let (minute, minute_mouse_interaction) = f(
        minute_layout,
        format!("{:02}", time.minute()),
        Focus::DigitalMinute,
    );

    let (minute_second_separator, second, second_mouse_interaction) = if show_seconds {
        let minute_second_separator = children
            .next()
            .expect("Graphics: Layout should have a minute/second separator layout");
        let minute_second_separator = Primitive::Text {
            content: ":".to_owned(),
            bounds: Rectangle {
                x: minute_second_separator.bounds().center_x(),
                y: minute_second_separator.bounds().center_y(),
                ..minute_second_separator.bounds()
            },
            color: style.get(&StyleState::Active).unwrap().text_color,
            size: minute_second_separator.bounds().height,
            font: iced_graphics::Font::default(),
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        };

        let second_layout = children
            .next()
            .expect("Graphics: Layout should have a second layout");
        let (second, second_mouse_interaction) = f(
            second_layout,
            format!("{:02}", time.second()),
            Focus::DigitalSecond,
        );

        (minute_second_separator, second, second_mouse_interaction)
    } else {
        (
            Primitive::None,
            Primitive::None,
            mouse::Interaction::default(),
        )
    };

    let period = if use_24h {
        Primitive::None
    } else {
        let period = children
            .next()
            .expect("Graphics: Layout should have a period layout");
        Primitive::Text {
            content: if time.hour12().0 { "PM" } else { "AM" }.to_owned(),
            bounds: Rectangle {
                x: period.bounds().center_x(),
                y: period.bounds().center_y(),
                ..period.bounds()
            },
            color: style.get(&StyleState::Active).unwrap().text_color,
            size: period.bounds().height,
            font: iced_graphics::Font::default(),
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        }
    };

    (
        Primitive::Group {
            primitives: vec![
                hour,
                hour_minute_separator,
                minute,
                minute_second_separator,
                second,
                period,
            ],
        },
        mouse_interaction
            .max(hour_mouse_interaction)
            .max(minute_mouse_interaction)
            .max(second_mouse_interaction),
    )
}
