//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: color_picker*
use iced_native::Layout;
use std::collections::HashMap;

use iced_graphics::{
    backend,
    canvas::{self, LineCap, Path, Stroke},
    Backend, Color, Defaults, Point, Primitive, Rectangle, Renderer, Size, Vector,
};
use iced_native::mouse;

use crate::{
    core::{
        color::{HexString, Hsv},
        renderer::DrawEnvironment,
    },
    native::overlay::color_picker::Focus,
    style::{
        color_picker::{Style, StyleSheet},
        style_state::StyleState,
    },
};

use crate::native::color_picker;
pub use crate::native::color_picker::State;

/// An input element for picking colors.
///
/// This is an alias of an `iced_native` ColorPicker with an `iced_wgpu::Renderer`.
pub type ColorPicker<'a, Message, Backend> =
    color_picker::ColorPicker<'a, Message, Renderer<Backend>>;

impl<B> color_picker::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, Focus>,
        color: &iced_graphics::Color,
        sat_value_canvas_cache: &canvas::Cache,
        hue_canvas_cache: &canvas::Cache,
        //text_input: &iced_native::Element<'_, Message, Self>,
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
            background: style.get(&style_state).unwrap().background,
            border_radius: style.get(&style_state).unwrap().border_radius,
            border_width: style.get(&style_state).unwrap().border_width,
            border_color: style.get(&style_state).unwrap().border_color,
        };

        // ----------- Block 1 ----------------------
        let hsv_color_layout = children.next().unwrap();

        // ----------- RGBA Color ----------------------
        //let hsv_color_layout = block1_children.next().unwrap();
        let (hsv_color, hsv_color_mouse_interaction) = hsv_color(
            hsv_color_layout,
            color,
            sat_value_canvas_cache,
            hue_canvas_cache,
            env.cursor_position,
            &style,
            env.focus,
        );

        // ----------- Block 1 end ------------------

        // ----------- Block 2 ----------------------
        let mut block2_children = children.next().unwrap().children();

        // ----------- RGBA Color ----------------------
        let rgba_color_layout = block2_children.next().unwrap();
        let (rgba_color, rgba_color_mouse_interaction) = rgba_color(
            rgba_color_layout,
            color,
            env.cursor_position,
            env.defaults,
            &style,
            env.focus,
        );

        // ----------- Text input ----------------------
        let text_input_layout = block2_children.next().unwrap();
        let hsv: Hsv = color.clone().into();

        let text_input_style_state = if text_input_layout.bounds().contains(env.cursor_position) {
            StyleState::Hovered
        } else {
            StyleState::Active
        };

        let text_input = Primitive::Group {
            primitives: vec![
                Primitive::Quad {
                    bounds: text_input_layout.bounds(),
                    background: color.clone().into(),
                    border_radius: style
                        .get(&text_input_style_state)
                        .unwrap()
                        .bar_border_radius,
                    border_width: style.get(&text_input_style_state).unwrap().bar_border_width,
                    border_color: style.get(&text_input_style_state).unwrap().bar_border_color,
                },
                Primitive::Text {
                    content: color.to_owned().as_hex_string(),
                    bounds: Rectangle {
                        x: text_input_layout.bounds().center_x(),
                        y: text_input_layout.bounds().center_y(),
                        ..text_input_layout.bounds()
                    },
                    color: Color {
                        a: 1.0,
                        ..Hsv {
                            hue: 0,
                            saturation: 0.0,
                            value: if hsv.value < 0.5 { 1.0 } else { 0.0 },
                        }
                        .into()
                    },
                    size: text_input_layout.bounds().height * 0.7,
                    font: Default::default(),
                    horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
                    vertical_alignment: iced_graphics::VerticalAlignment::Center,
                },
            ],
        };

        // ----------- Buttons -------------------------
        let cancel_button_layout = block2_children.next().unwrap();

        let (cancel_button, cancel_mouse_interaction) = cancel_button.draw(
            self,
            env.defaults,
            cancel_button_layout,
            env.cursor_position,
            &bounds,
        );

        let submit_button_layout = block2_children.next().unwrap();

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
                border_radius: style.get(&StyleState::Focused).unwrap().border_radius,
                border_width: style.get(&StyleState::Focused).unwrap().border_width,
                border_color: style.get(&StyleState::Focused).unwrap().border_color,
            }
        } else {
            Primitive::None
        };

        let submit_button_focus = if env.focus == Focus::Submit {
            Primitive::Quad {
                bounds: submit_button_layout.bounds(),
                background: Color::TRANSPARENT.into(),
                border_radius: style.get(&StyleState::Focused).unwrap().border_radius,
                border_width: style.get(&StyleState::Focused).unwrap().border_width,
                border_color: style.get(&StyleState::Focused).unwrap().border_color,
            }
        } else {
            Primitive::None
        };
        // ----------- Block 2 end ------------------

        (
            Primitive::Group {
                primitives: vec![
                    background,
                    hsv_color,
                    rgba_color,
                    text_input,
                    cancel_button,
                    submit_button,
                    cancel_button_focus,
                    submit_button_focus,
                ],
            },
            mouse_interaction
                .max(hsv_color_mouse_interaction)
                .max(rgba_color_mouse_interaction)
                //.max(text_input_mouse_interaction)
                .max(cancel_mouse_interaction)
                .max(submit_mouse_interaction),
        )
    }
}

/// Draws the HSV color area.
fn hsv_color(
    layout: Layout<'_>,
    color: &Color,
    sat_value_canvas_cache: &canvas::Cache,
    hue_canvas_cache: &canvas::Cache,
    cursor_position: Point,
    style: &HashMap<StyleState, Style>,
    focus: Focus,
) -> (Primitive, mouse::Interaction) {
    let mut hsv_color_children = layout.children();
    let hsv_color: Hsv = color.clone().into();

    let mouse_interaction = mouse::Interaction::default();

    let sat_value_layout = hsv_color_children.next().unwrap();
    let (mut sat_value_style_state, mut sat_value_mouse_interaction) =
        (StyleState::Active, mouse::Interaction::default());
    if focus == Focus::SatValue {
        sat_value_style_state = sat_value_style_state.max(StyleState::Focused)
    }
    if sat_value_layout.bounds().contains(cursor_position) {
        sat_value_style_state = sat_value_style_state.max(StyleState::Hovered);
        sat_value_mouse_interaction = sat_value_mouse_interaction.max(mouse::Interaction::Pointer);
    }

    let sat_value = sat_value_canvas_cache
        .draw(sat_value_layout.bounds().size(), |frame| {
            let column_count = frame.width() as u16;
            let row_count = frame.height() as u16;

            for column in 0..column_count {
                for row in 0..row_count {
                    let saturation = column as f32 / frame.width();
                    let value = row as f32 / frame.height();

                    frame.fill_rectangle(
                        Point::new(column as f32, row as f32),
                        Size::new(1.0, 1.0),
                        Color::from(Hsv::from_hsv(hsv_color.hue, saturation, value)),
                    );
                }
            }

            let stroke = Stroke {
                color: Hsv {
                    hue: 0,
                    saturation: 0.0,
                    value: 1.0 - hsv_color.value,
                }
                .into(),
                width: 3.0,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            let saturation = hsv_color.saturation * frame.width();
            let value = hsv_color.value * frame.height();

            frame.stroke(
                &Path::line(
                    Point::new(saturation, 0.0),
                    Point::new(saturation, frame.height()),
                ),
                stroke,
            );

            frame.stroke(
                &Path::line(Point::new(0.0, value), Point::new(frame.width(), value)),
                stroke,
            );

            let stroke = Stroke {
                color: style.get(&sat_value_style_state).unwrap().bar_border_color,
                width: 2.0,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            frame.stroke(
                &Path::rectangle(
                    Point::new(0.0, 0.0),
                    Size::new(frame.size().width - 0.0, frame.size().height - 0.0),
                ),
                stroke,
            );
        })
        .into_primitive();
    let sat_value = Primitive::Translate {
        translation: Vector::new(sat_value_layout.bounds().x, sat_value_layout.bounds().y),
        content: Box::new(sat_value),
    };

    let hue_layout = hsv_color_children.next().unwrap();
    let (mut hue_style_state, mut hue_mouse_interaction) =
        (StyleState::Active, mouse::Interaction::default());
    if focus == Focus::Hue {
        hue_style_state = hue_style_state.max(StyleState::Focused);
    }
    if hue_layout.bounds().contains(cursor_position) {
        hue_style_state = hue_style_state.max(StyleState::Hovered);
        hue_mouse_interaction = hue_mouse_interaction.max(mouse::Interaction::Pointer);
    }

    let hue = hue_canvas_cache
        .draw(hue_layout.bounds().size(), |frame| {
            let column_count = frame.width() as u16;

            for column in 0..column_count {
                let hue = (column as f32 * 360.0 / frame.width()) as u16;

                let hsv_color = Hsv::from_hsv(hue, 1.0, 1.0);
                let stroke = Stroke {
                    color: hsv_color.into(),
                    width: 1.0,
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                };

                frame.stroke(
                    &Path::line(
                        Point::new(column as f32, 0.0),
                        Point::new(column as f32, frame.height()),
                    ),
                    stroke,
                );
            }

            let stroke = Stroke {
                color: Color::BLACK,
                width: 3.0,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            let column = hsv_color.hue as f32 * frame.width() / 360.0;

            frame.stroke(
                &Path::line(Point::new(column, 0.0), Point::new(column, frame.height())),
                stroke,
            );

            let stroke = Stroke {
                color: style.get(&hue_style_state).unwrap().bar_border_color,
                width: 2.0,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            frame.stroke(
                &Path::rectangle(
                    Point::new(0.0, 0.0),
                    Size::new(frame.size().width, frame.size().height),
                ),
                stroke,
            );
        })
        .into_primitive();
    let hue = Primitive::Translate {
        translation: Vector::new(hue_layout.bounds().x, hue_layout.bounds().y),
        content: Box::new(hue),
    };

    (
        Primitive::Group {
            primitives: vec![sat_value, hue],
        },
        mouse_interaction
            .max(sat_value_mouse_interaction)
            .max(hue_mouse_interaction),
    )
}

/// Draws the RGBA color area.
fn rgba_color(
    layout: Layout<'_>,
    color: &Color,
    cursor_position: Point,
    defaults: &Defaults,
    style: &HashMap<StyleState, Style>,
    focus: Focus,
) -> (Primitive, mouse::Interaction) {
    let mut rgba_color_children = layout.children();

    let mouse_interaction = mouse::Interaction::default();

    let f = |layout: Layout,
             label: &str,
             color: Color,
             value: f32,
             cursor_position: Point,
             target: Focus|
     -> (Primitive, mouse::Interaction) {
        let mut children = layout.children();

        let mouse_interaction = mouse::Interaction::default();

        let label_layout = children.next().unwrap();
        let bar_layout = children.next().unwrap();
        let value_layout = children.next().unwrap();

        let label = Primitive::Text {
            content: label.to_owned(),
            bounds: Rectangle {
                x: label_layout.bounds().center_x(),
                y: label_layout.bounds().center_y(),
                ..label_layout.bounds()
            },
            color: defaults.text.color,
            size: label_layout.bounds().height,
            font: Default::default(),
            horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
            vertical_alignment: iced_graphics::VerticalAlignment::Center,
        };

        let bounds = bar_layout.bounds();

        let (bar_style_state, bar_mouse_interaction) =
            if bar_layout.bounds().contains(cursor_position) {
                (
                    StyleState::Hovered,
                    mouse::Interaction::ResizingHorizontally,
                )
            } else {
                (StyleState::Active, mouse::Interaction::default())
            };

        let bar_background = Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width * value,
                height: bounds.height,
            },
            background: color.into(),
            border_radius: style.get(&bar_style_state).unwrap().bar_border_radius,
            border_width: style.get(&bar_style_state).unwrap().bar_border_width,
            border_color: Color::TRANSPARENT,
        };

        let bar = Primitive::Quad {
            bounds,
            background: Color::TRANSPARENT.into(),
            border_radius: style.get(&bar_style_state).unwrap().bar_border_radius,
            border_width: style.get(&bar_style_state).unwrap().bar_border_width,
            border_color: style.get(&bar_style_state).unwrap().bar_border_color,
        };

        let value = Primitive::Text {
            content: format!("{}", (255.0 * value) as u8),
            bounds: Rectangle {
                x: value_layout.bounds().center_x(),
                y: value_layout.bounds().center_y(),
                ..value_layout.bounds()
            },
            color: defaults.text.color,
            size: value_layout.bounds().height,
            font: Default::default(),
            horizontal_alignment: iced_graphics::HorizontalAlignment::Center,
            vertical_alignment: iced_graphics::VerticalAlignment::Center,
        };

        let focus = if focus == target {
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Color::TRANSPARENT.into(),
                border_radius: style.get(&StyleState::Focused).unwrap().border_radius,
                border_width: style.get(&StyleState::Focused).unwrap().border_width,
                border_color: style.get(&StyleState::Focused).unwrap().border_color,
            }
        } else {
            Primitive::None
        };

        (
            Primitive::Group {
                primitives: vec![label, bar_background, bar, focus, value],
            },
            mouse_interaction.max(bar_mouse_interaction),
        )
    };

    // Red
    let red_row_layout = rgba_color_children.next().unwrap();

    let (red, red_mouse_interaction) = f(
        red_row_layout,
        "R:",
        Color::from_rgb(color.r, 0.0, 0.0),
        color.r,
        cursor_position,
        Focus::Red,
    );

    // Green
    let green_row_layout = rgba_color_children.next().unwrap();

    let (green, green_mouse_interaction) = f(
        green_row_layout,
        "G:",
        Color::from_rgb(0.0, color.g, 0.0),
        color.g,
        cursor_position,
        Focus::Green,
    );

    // Blue
    let blue_row_layout = rgba_color_children.next().unwrap();

    let (blue, blue_mouse_interaction) = f(
        blue_row_layout,
        "B:",
        Color::from_rgb(0.0, 0.0, color.b),
        color.b,
        cursor_position,
        Focus::Blue,
    );

    // Alpha
    let alpha_row_layout = rgba_color_children.next().unwrap();

    let (alpha, alpha_mouse_interaction) = f(
        alpha_row_layout,
        "A:",
        Color::from_rgba(0.0, 0.0, 0.0, color.a),
        color.a,
        cursor_position,
        Focus::Alpha,
    );

    (
        Primitive::Group {
            primitives: vec![red, green, blue, alpha],
        },
        mouse_interaction
            .max(red_mouse_interaction)
            .max(green_mouse_interaction)
            .max(blue_mouse_interaction)
            .max(alpha_mouse_interaction),
    )
}
