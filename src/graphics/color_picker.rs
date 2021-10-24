//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*
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
/// This is an alias of an `iced_native` `ColorPicker` with an `iced_wgpu::Renderer`.
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
            background: style[&style_state].background,
            border_radius: style[&style_state].border_radius,
            border_width: style[&style_state].border_width,
            border_color: style[&style_state].border_color,
        };

        // ----------- Block 1 ----------------------
        let block1_layout = children
            .next()
            .expect("Graphics: Layout should have a 1. block layout");
        let (block1, block1_mouse_interaction) = block1(
            color,
            sat_value_canvas_cache,
            hue_canvas_cache,
            block1_layout,
            env.cursor_position,
            env.focus,
            &style,
        );

        // ----------- Block 2 ----------------------
        let block2_layout = children
            .next()
            .expect("Graphics: Layout should have a 2. block layout");
        let (block2, block2_mouse_interaction) = block2(
            self,
            color,
            cancel_button,
            submit_button,
            &DrawEnvironment {
                defaults: env.defaults,
                layout: block2_layout,
                cursor_position: env.cursor_position,
                style_sheet: &(),
                viewport: Some(&bounds),
                focus: env.focus,
            },
            &style,
        );

        (
            Primitive::Group {
                primitives: vec![background, block1, block2],
            },
            mouse_interaction
                .max(block1_mouse_interaction)
                .max(block2_mouse_interaction),
        )
    }
}

/// Draws the 1. block of the color picker containing the HSV part.
fn block1(
    color: &Color,
    sat_value_canvas_cache: &canvas::Cache,
    hue_canvas_cache: &canvas::Cache,
    layout: Layout<'_>,
    cursor_position: Point,
    focus: Focus,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction) {
    // ----------- Block 1 ----------------------
    let hsv_color_layout = layout;

    // ----------- HSV Color ----------------------
    //let hsv_color_layout = block1_children.next().unwrap();
    let (hsv_color, hsv_color_mouse_interaction) = hsv_color(
        hsv_color_layout,
        color,
        sat_value_canvas_cache,
        hue_canvas_cache,
        cursor_position,
        style,
        focus,
    );

    // ----------- Block 1 end ------------------

    (hsv_color, hsv_color_mouse_interaction)
}

/// Draws the 2. block of the color picker containing the RGBA part, Hex and buttons.
fn block2<Message, B>(
    renderer: &mut Renderer<B>,
    color: &Color,
    cancel_button: &iced_native::Element<'_, Message, Renderer<B>>,
    submit_button: &iced_native::Element<'_, Message, Renderer<B>>,
    env: &DrawEnvironment<'_, Defaults, (), Focus>,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction)
where
    B: Backend + backend::Text,
{
    // ----------- Block 2 ----------------------
    let mut block2_children = env.layout.children();

    // ----------- RGBA Color ----------------------
    let rgba_color_layout = block2_children
        .next()
        .expect("Graphics: Layout should have a RGBA color layout");
    let (rgba_color, rgba_color_mouse_interaction) = rgba_color(
        rgba_color_layout,
        color,
        env.cursor_position,
        env.defaults,
        style,
        env.focus,
    );

    // ----------- Hex text ----------------------
    let hex_text_layout = block2_children
        .next()
        .expect("Graphics: Layout should have a hex text layout");
    let hex_text = hex_text(
        hex_text_layout,
        color,
        env.cursor_position,
        env.defaults,
        style,
        env.focus,
    );

    // ----------- Buttons -------------------------
    let cancel_button_layout = block2_children
        .next()
        .expect("Graphics: Layout should have a cancel button layout for a ColorPicker");

    let (cancel_button, cancel_mouse_interaction) = cancel_button.draw(
        renderer,
        env.defaults,
        cancel_button_layout,
        env.cursor_position,
        env.viewport
            .expect("Should have a viewport for ColorPicker"),
    );

    let submit_button_layout = block2_children
        .next()
        .expect("Graphics: Layout should have a submit button layout for a ColorPicker");

    let (submit_button, submit_mouse_interaction) = submit_button.draw(
        renderer,
        env.defaults,
        submit_button_layout,
        env.cursor_position,
        env.viewport
            .expect("Should have a viewport for ColorPicker"),
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
    // ----------- Block 2 end ------------------

    (
        Primitive::Group {
            primitives: vec![
                rgba_color,
                hex_text,
                cancel_button,
                submit_button,
                cancel_button_focus,
                submit_button_focus,
            ],
        },
        rgba_color_mouse_interaction
            .max(cancel_mouse_interaction)
            .max(submit_mouse_interaction),
    )
}

/// Draws the HSV color area.
#[allow(clippy::too_many_lines)]
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
    let hsv_color: Hsv = (*color).into();

    let mouse_interaction = mouse::Interaction::default();

    let sat_value_layout = hsv_color_children
        .next()
        .expect("Graphics: Layout should have a sat/value layout");
    let (mut sat_value_style_state, mut sat_value_mouse_interaction) =
        (StyleState::Active, mouse::Interaction::default());
    if focus == Focus::SatValue {
        sat_value_style_state = sat_value_style_state.max(StyleState::Focused);
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
                    let saturation = f32::from(column) / frame.width();
                    let value = f32::from(row) / frame.height();

                    frame.fill_rectangle(
                        Point::new(f32::from(column), f32::from(row)),
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

    let hue_layout = hsv_color_children
        .next()
        .expect("Graphics: Layout should have a hue layout");
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
                let hue = (f32::from(column) * 360.0 / frame.width()) as u16;

                let hsv_color = Hsv::from_hsv(hue, 1.0, 1.0);
                let stroke = Stroke {
                    color: hsv_color.into(),
                    width: 1.0,
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                };

                frame.stroke(
                    &Path::line(
                        Point::new(f32::from(column), 0.0),
                        Point::new(f32::from(column), frame.height()),
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

            let column = f32::from(hsv_color.hue) * frame.width() / 360.0;

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
#[allow(clippy::too_many_lines)]
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

        let label_layout = children
            .next()
            .expect("Graphics: Layout should have a label layout");
        let bar_layout = children
            .next()
            .expect("Graphics: Layout should have a bar layout");
        let value_layout = children
            .next()
            .expect("Graphics: Layout should have a value layout");

        let label = Primitive::Text {
            content: label.to_owned(),
            bounds: Rectangle {
                x: label_layout.bounds().center_x(),
                y: label_layout.bounds().center_y(),
                ..label_layout.bounds()
            },
            color: defaults.text.color,
            size: label_layout.bounds().height,
            font: iced_graphics::Font::default(),
            horizontal_alignment: iced_graphics::alignment::Horizontal::Center,
            vertical_alignment: iced_graphics::alignment::Vertical::Center,
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
            font: iced_graphics::Font::default(),
            horizontal_alignment: iced_graphics::alignment::Horizontal::Center,
            vertical_alignment: iced_graphics::alignment::Vertical::Center,
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
    let red_row_layout = rgba_color_children
        .next()
        .expect("Graphics: Layout should have a red row layout");

    let (red, red_mouse_interaction) = f(
        red_row_layout,
        "R:",
        Color::from_rgb(color.r, 0.0, 0.0),
        color.r,
        cursor_position,
        Focus::Red,
    );

    // Green
    let green_row_layout = rgba_color_children
        .next()
        .expect("Graphics: Layout should have a green row layout");

    let (green, green_mouse_interaction) = f(
        green_row_layout,
        "G:",
        Color::from_rgb(0.0, color.g, 0.0),
        color.g,
        cursor_position,
        Focus::Green,
    );

    // Blue
    let blue_row_layout = rgba_color_children
        .next()
        .expect("Graphics: Layout should have a blue row layout");

    let (blue, blue_mouse_interaction) = f(
        blue_row_layout,
        "B:",
        Color::from_rgb(0.0, 0.0, color.b),
        color.b,
        cursor_position,
        Focus::Blue,
    );

    // Alpha
    let alpha_row_layout = rgba_color_children
        .next()
        .expect("Graphics: Layout should have an alpha row layout");

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

/// Draws the hex text representation of the color.
fn hex_text(
    layout: Layout<'_>,
    color: &Color,
    cursor_position: Point,
    _defaults: &Defaults,
    style: &HashMap<StyleState, Style>,
    _focus: Focus,
) -> Primitive {
    let hsv: Hsv = (*color).into();

    let hex_text_style_state = if layout.bounds().contains(cursor_position) {
        StyleState::Hovered
    } else {
        StyleState::Active
    };

    Primitive::Group {
        primitives: vec![
            Primitive::Quad {
                bounds: layout.bounds(),
                background: (*color).into(),
                border_radius: style[&hex_text_style_state].bar_border_radius,
                border_width: style[&hex_text_style_state].bar_border_width,
                border_color: style[&hex_text_style_state].bar_border_color,
            },
            Primitive::Text {
                content: color.clone().as_hex_string(),
                bounds: Rectangle {
                    x: layout.bounds().center_x(),
                    y: layout.bounds().center_y(),
                    ..layout.bounds()
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
                size: layout.bounds().height * 0.7,
                font: iced_graphics::Font::default(),
                horizontal_alignment: iced_graphics::alignment::Horizontal::Center,
                vertical_alignment: iced_graphics::alignment::Vertical::Center,
            },
        ],
    }
}
