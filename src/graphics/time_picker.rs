//! TODO

use crate::{core::clock::{self, HOUR_RADIUS_PERCENTAGE, HOUR_RADIUS_PERCENTAGE_NO_SECONDS, MINUTE_RADIUS_PERCENTAGE, MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, NearestRadius, PERIOD_PERCENTAGE, SECOND_RADIUS_PERCENTAGE}, style::time_picker::{Style, StyleSheet}};
use canvas::{Cache, LineCap, Path, Stroke, Text};
use chrono::{NaiveTime, Timelike};
use iced_graphics::{Backend, HorizontalAlignment, Point, Primitive, Rectangle, Renderer, Vector, VerticalAlignment, backend, canvas};
use iced_native::mouse;

use crate::native::time_picker;
pub use crate::native::time_picker::{State, Time, Period};

use super::icons::{ICON_FONT, Icon};

const NUMBER_SIZE_PERCENTAGE: f32 = 0.15;
const PERIOD_SIZE_PERCENTAGE: f32 = 0.2;

/// TODO
pub type TimePicker<'a, Message, Backend> = 
    time_picker::TimePicker<'a, Message, Renderer<Backend>>;

impl<B> time_picker::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: iced_graphics::Point,
        style_sheet: &Self::Style,
        time: &chrono::NaiveTime,
        clock_cache: &canvas::Cache,
        cancel_button: &iced_native::Element<'_, Message, Self>,
        submit_button: &iced_native::Element<'_, Message, Self>,
        use_24h: bool,
        show_seconds: bool,
        layout: iced_native::Layout<'_>,
    ) -> Self::Output {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let style = style_sheet.active();

        let mouse_interaction = mouse::Interaction::default();
        
        let background = Primitive::Quad {
            bounds,
            background: style.background, // TODO
            border_radius: style.border_radius as u16, // TODO: will change in the future
            border_width: style.border_width as u16, // TODO: same
            border_color: style.border_color,
        };

        // ----------- Clock canvas --------------------
        let clock_layout = children.next().unwrap();
        let (clock, clock_mouse_interaction) = clock(
            clock_layout,
            time,
            clock_cache,
            cursor_position,
            use_24h,
            show_seconds,
            &style,
        );       

        // ----------- Digital clock ------------------
        let digital_clock_layout = children.next().unwrap();
        let (digital_clock, digital_clock_mouse_interaction) = digital_clock(
            digital_clock_layout,
            time,
            cursor_position,
            use_24h,
            show_seconds,
            &style
        );

        // ----------- Buttons ------------------------
        let cancel_button_layout = children.next().unwrap();
        
        let (cancel_button, cancel_mouse_interaction) = cancel_button.draw(
            self,
            defaults,
            cancel_button_layout,
            cursor_position,
            &bounds,
        );

        let submit_button_layout = children.next().unwrap();

        let (submit_button, submit_mouse_interaction) = submit_button.draw(
            self,
            defaults,
            submit_button_layout,
            cursor_position,
            &bounds,
        );

        (
            Primitive::Group {
                primitives: vec![background, clock, digital_clock, cancel_button, submit_button],
            },
            mouse_interaction
                .max(clock_mouse_interaction)
                .max(digital_clock_mouse_interaction)
                .max(cancel_mouse_interaction)
                .max(submit_mouse_interaction)
        )
    }
}

fn clock(
    layout: iced_native::Layout<'_>,
    time: &NaiveTime,
    clock_cache: &Cache,
    cursor_position: Point,
    use_24h: bool,
    show_seconds: bool,
    style: &Style,
) -> (Primitive, mouse::Interaction) {
    let clock_mouse_interaction = if layout.bounds().contains(cursor_position) {
        mouse::Interaction::Pointer
    } else {
        mouse::Interaction::default()
    };
    
    let clock = clock_cache.draw(
        layout.bounds().size(),
        |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) * 0.5;
            let period = if time.hour12().0 { clock::Period::PM } else { clock::Period::AM };

            let number_size = radius * NUMBER_SIZE_PERCENTAGE;
            let period_size = radius * PERIOD_SIZE_PERCENTAGE;

            let period_radius = radius * PERIOD_PERCENTAGE;
            /*let hour_radius = radius * HOUR_RADIUS_PERCENTAGE;
            let minute_radius = radius * MINUTE_RADIUS_PERCENTAGE;
            let second_radius = radius * SECOND_RADIUS_PERCENTAGE;*/

            let (hour_radius, minute_radius, second_radius) = if show_seconds {
                (radius * HOUR_RADIUS_PERCENTAGE, radius * MINUTE_RADIUS_PERCENTAGE, radius * SECOND_RADIUS_PERCENTAGE)
            } else {
                (radius * HOUR_RADIUS_PERCENTAGE_NO_SECONDS, radius * MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, f32::MAX)
            };

            let internal_cursor_position = cursor_position - Vector::new(layout.bounds().x, layout.bounds().y);

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
            } else { NearestRadius::None };

            /*frame.fill_rectangle(
                Point::ORIGIN,
                Size::new(frame.width(), frame.height()),
                Color::WHITE,
            );*/

            let hour_points = crate::core::clock::circle_points(hour_radius, center, 12);
            let minute_points = crate::core::clock::circle_points(minute_radius, center, 60);
            let second_points = crate::core::clock::circle_points(second_radius, center, 60);

            let hand_stroke = Stroke {
                width: style.clock_hand_width,
                color: style.clock_hand_color,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            match nearest_radius {
                NearestRadius::Period => {
                    frame.fill(&Path::circle(center.clone(), period_size), style.clock_number_background_hovered);
                },
                NearestRadius::Hour => {
                    let nearest_point = hour_points[crate::core::clock::nearest_point(&hour_points, internal_cursor_position)];

                    frame.fill(&Path::circle(nearest_point, 5.0), style.clock_number_background_hovered);
                },
                NearestRadius::Minute => {
                    let nearest_point = minute_points[crate::core::clock::nearest_point(&minute_points, internal_cursor_position)];
                    
                    frame.fill(&Path::circle(nearest_point, 5.0), style.clock_number_background_hovered);
                },
                NearestRadius::Second => {
                    let nearest_point = second_points[crate::core::clock::nearest_point(&second_points, internal_cursor_position)];
                    
                    frame.fill(&Path::circle(nearest_point, 5.0), style.clock_number_background_hovered);
                },
                _ => {}
            }

            let period_text = Text {
                content: format!("{}", period),
                position: center.clone(),
                color: style.clock_number_color,
                size: period_size,
                font: Default::default(),
                horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                vertical_alignment: iced_graphics::VerticalAlignment::Center,
            };
            frame.fill_text(period_text);

            hour_points.iter()
                .enumerate()
                .for_each(|(i, p)| {
                    //let selected = time.hour() % 12 == i as u32;
                    let (pm, selected) = {
                        let (pm, _) = time.hour12();
                        let hour = time.hour();
                        (pm, hour % 12 == i as u32)
                    };

                    // TODO: 24/12 Hour
                    if selected {
                        frame.fill(&Path::circle(p.clone(), number_size * 0.8), style.clock_number_background_selected);
                        frame.stroke(&Path::line(center.clone(), p.clone()), hand_stroke);
                    }

                    let text = Text {
                        //content: format!("{}", if i == 0 { 12 } else { i }),
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
                        position: p.clone(),
                        color: if selected {
                            style.clock_number_color_selected
                        } else {
                            style.clock_number_color
                        },
                        size: number_size,
                        font: Default::default(),
                        horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                        vertical_alignment: iced_graphics::VerticalAlignment::Center,
                    };

                    frame.fill_text(text);
                });


            minute_points.iter()
                .enumerate()
                .for_each(|(i, p)| {
                    let selected = time.minute() == i as u32;

                    if selected {
                        frame.fill(&Path::circle(p.clone(), number_size*0.5), style.clock_number_background_selected);
                        frame.stroke(&Path::line(center.clone(), p.clone()), hand_stroke);
                    }

                    if i % 5 == 0 {
                        let text = Text {
                            content: format!("{:02}", i),
                            position: p.clone(),
                            color: if selected {
                                style.clock_number_color_selected
                            } else {
                                style.clock_number_color
                            },
                            size: number_size,
                            font: Default::default(),
                            horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                            vertical_alignment: iced_graphics::VerticalAlignment::Center,
                        };

                        frame.fill_text(text);
                    } else {
                        let circle = Path::circle(p.clone(), number_size*0.1);
                        frame.fill(&circle, style.clock_dots_color);
                    }
                });

            if show_seconds {
                second_points.iter()
                    .enumerate()
                    .for_each(|(i, p)| {
                        let selected = time.second() == i as u32;

                        if selected {
                            frame.fill(&Path::circle(p.clone(), number_size*0.5), style.clock_number_background_selected);
                            frame.stroke(&Path::line(center.clone(), p.clone()), hand_stroke);
                        }

                        if i % 10 == 0 {
                            let text = Text {
                                content: format!("{:02}", i),
                                position: p.clone(),
                                color: if selected {
                                    style.clock_number_color_selected
                                } else {
                                    style.clock_number_color
                                },
                                size: number_size,
                                font: Default::default(),
                                horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                                vertical_alignment: iced_graphics::VerticalAlignment::Center,
                            };

                            frame.fill_text(text);
                        } else {
                            let circle = Path::circle(p.clone(), number_size*0.1);
                            frame.fill(&circle, style.clock_dots_color);
                        }
                    })
            }
        }
    ).into_primitive();
    let clock = Primitive::Translate {
        translation: Vector::new(layout.bounds().x, layout.bounds().y),
        content: Box::new(clock),
    };

    (clock, clock_mouse_interaction)
}

fn digital_clock(
    layout: iced_native::Layout<'_>,
    time: &NaiveTime,
    cursor_position: Point,
    use_24h: bool,
    show_seconds: bool,
    style: &Style,
) -> (Primitive, mouse::Interaction) {
    let mut children = layout.children().next().unwrap().children();

    let f = |layout: iced_native::Layout<'_>, text: String| {
        let mut children = layout.children();

        let up_bounds = children.next().unwrap().bounds();
        let center_bounds = children.next().unwrap().bounds();
        let down_bounds = children.next().unwrap().bounds();

        let mut mouse_interaction = mouse::Interaction::default();

        let up_arrow_hovered = up_bounds.contains(cursor_position);
        let down_arrow_hovered = down_bounds.contains(cursor_position);

        if up_arrow_hovered || down_arrow_hovered {
            mouse_interaction = mouse_interaction.max(mouse::Interaction::Pointer);
        }

        let primitive = Primitive::Group {
            primitives: vec![
                Primitive::Text {
                    content: Icon::CaretUpFill.into(),
                    bounds: Rectangle {
                        x: up_bounds.center_x(),
                        y: up_bounds.center_y(),
                        .. up_bounds
                    },
                    color: style.text_color,
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
                        .. center_bounds
                    },
                    color: style.text_color,
                    size: center_bounds.height,
                    font: Default::default(),
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },

                Primitive::Text {
                    content: Icon::CaretDownFill.into(),
                    bounds: Rectangle {
                        x: down_bounds.center_x(),
                        y: down_bounds.center_y(),
                        .. down_bounds
                    },
                    color: style.text_color,
                    size: down_bounds.height + if down_arrow_hovered { 5.0 } else { 0.0 },
                    font: ICON_FONT,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                }
            ]
        };

        (primitive, mouse_interaction)
    };

    if !use_24h {
        // Placeholder
        let _ = children.next();
    }

    // ---
    // ---
    
    let mouse_interaction = mouse::Interaction::default();
    
    let hour_layout = children.next().unwrap();
    let (hour, hour_mouse_interaction) = f(
        hour_layout,
        format!(
            "{:02}",
            if use_24h { time.hour() } else { time.hour12().1 }
        )
    );
    
    let hour_minute_seperator = children.next().unwrap();
    let hour_minute_seperator = Primitive::Text {
        content: ":".to_owned(),
        bounds: Rectangle {
            x: hour_minute_seperator.bounds().center_x(),
            y: hour_minute_seperator.bounds().center_y(),
            .. hour_minute_seperator.bounds()
        },
        color: style.text_color,
        size: hour_minute_seperator.bounds().height,
        font: Default::default(),
        horizontal_alignment: HorizontalAlignment::Center,
        vertical_alignment: VerticalAlignment::Center,
    };
    
    let minute_layout = children.next().unwrap();
    let (minute, minute_mouse_interaction) = f(minute_layout, format!("{:02}", time.minute()));
    
    /*let minute_second_seperator = children.next().unwrap();
    let minute_second_seperator = Primitive::Text {
        content: ":".to_owned(),
        bounds: Rectangle {
            x: minute_second_seperator.bounds().center_x(),
            y: minute_second_seperator.bounds().center_y(),
            .. minute_second_seperator.bounds()
        },
        color: style.text_color,
        size: minute_second_seperator.bounds().height,
        font: Default::default(),
        horizontal_alignment: HorizontalAlignment::Center,
        vertical_alignment: VerticalAlignment::Center,
    };
    
    let second_layout = children.next().unwrap();
    let (second, second_mouse_interaction) = f(second_layout, format!("{:02}", time.second()));*/

    let (minute_second_seperator, second, second_mouse_interaction) = if show_seconds {
        let minute_second_seperator = children.next().unwrap();
        let minute_second_seperator = Primitive::Text {
            content: ":".to_owned(),
            bounds: Rectangle {
                x: minute_second_seperator.bounds().center_x(),
                y: minute_second_seperator.bounds().center_y(),
                .. minute_second_seperator.bounds()
            },
            color: style.text_color,
            size: minute_second_seperator.bounds().height,
            font: Default::default(),
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        };

        let second_layout = children.next().unwrap();
        let (second, second_mouse_interaction) = f(second_layout, format!("{:02}", time.second()));

        (minute_second_seperator, second, second_mouse_interaction)
    } else {
        (Primitive::None, Primitive::None, mouse::Interaction::default())
    };
    
    let period = if !use_24h {
        let period = children.next().unwrap();
        Primitive::Text {
            content: format!("{}", if time.hour12().0 {
                "PM"
            } else {
                "AM"
            }),
            bounds: Rectangle {
                x: period.bounds().center_x(),
                y: period.bounds().center_y(),
                .. period.bounds()
            },
            color: style.text_color,
            size: period.bounds().height,
            font: Default::default(),
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        }
    } else {
        Primitive::None
    };    

    (
        Primitive::Group {
            primitives: vec![hour, hour_minute_seperator, minute, minute_second_seperator, second, period],
        },
        mouse_interaction
            .max(hour_mouse_interaction)
            .max(minute_mouse_interaction)
            .max(second_mouse_interaction)
    )
}