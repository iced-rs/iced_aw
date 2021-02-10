//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: color_picker*
use std::hash::Hash;

use iced_graphics::canvas;
use iced_native::{
    button, column, event, keyboard,
    layout::{self, Limits},
    mouse, overlay, row, text, text_input, Align, Button, Clipboard, Color, Column, Element, Event,
    Layout, Length, Point, Rectangle, Row, Size, Text, Widget,
};

use crate::{
    core::{color::Hsv, renderer::DrawEnvironment},
    graphics::icons::Icon,
    native::{color_picker, icon_text, IconText},
};

const PADDING: u16 = 10;
const SPACING: u16 = 15;
const BUTTON_SPACING: u16 = 5;

const SAT_VALUE_STEP: f32 = 0.005;
const HUE_STEP: i32 = 1;
const RGBA_STEP: u16 = 1;

/// The overlay of the [`ColorPicker`](crate::native::ColorPicker).
#[allow(missing_debug_implementations)]
pub struct ColorPickerOverlay<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    color: &'a mut Color,
    //color_hex: &'a mut String,
    sat_value_canvas_cache: &'a mut canvas::Cache,
    hue_canvas_cache: &'a mut canvas::Cache,
    //text_input: Element<'a, Message, Renderer>,
    cancel_button: Element<'a, Message, Renderer>,
    submit_button: Element<'a, Message, Renderer>,
    on_submit: &'a dyn Fn(Color) -> Message,
    color_bar_dragged: &'a mut ColorBarDragged,
    focus: &'a mut Focus,
    keyboard_modifiers: &'a mut keyboard::Modifiers,
    position: Point,
    style: &'a <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer> ColorPickerOverlay<'a, Message, Renderer>
where
    Message: 'static + Clone,
    Renderer: 'a
        + self::Renderer
        + column::Renderer
        + button::Renderer
        + icon_text::Renderer
        + row::Renderer
        + text::Renderer
        + text_input::Renderer,
{
    /// Creates a new [`ColorPickerOverlay`](ColorPickerOverlay) on the given
    /// position.
    pub fn new(
        state: &'a mut color_picker::State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Color) -> Message,
        position: Point,
        style: &'a <Renderer as self::Renderer>::Style,
    ) -> Self {
        //state.color_hex = color_picker::State::color_as_string(state.color);
        let color_picker::State {
            color,
            //color_hex,
            sat_value_canvas_cache,
            hue_canvas_cache,
            cancel_button,
            submit_button,
            color_bar_dragged,
            focus,
            keyboard_modifiers,
            ..
        } = state;

        ColorPickerOverlay {
            color,
            sat_value_canvas_cache,
            hue_canvas_cache,
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
            color_bar_dragged,
            focus,
            keyboard_modifiers,
            position,
            style,
        }
    }

    /// Turn this [`ColorPickerOverlay`](ColorPickerOverlay) into an overlay
    /// [`Element`](overlay::Element).
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(self.position, Box::new(self))
    }

    /// The event handling for the HSV color area.
    fn on_event_hsv_color(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut hsv_color_children = layout.children();

        let hsv_color: Hsv = self.color.to_owned().into();
        let mut color_changed = false;

        let sat_value_bounds = hsv_color_children.next().unwrap().bounds();
        let hue_bounds = hsv_color_children.next().unwrap().bounds();

        if let Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
            match delta {
                mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                    let move_value =
                        |value: u16, y: f32| ((value as i32 + y as i32).rem_euclid(360)) as u16;

                    println!(
                        "hue: {}, moved: {}",
                        hsv_color.hue,
                        move_value(hsv_color.hue, y)
                    );

                    if hue_bounds.contains(cursor_position) {
                        *self.color = Color {
                            a: self.color.a,
                            ..Hsv {
                                hue: move_value(hsv_color.hue, y),
                                ..hsv_color
                            }
                            .into()
                        };
                        color_changed = true;
                    }
                }
            }
        } else if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if sat_value_bounds.contains(cursor_position) {
                *self.color_bar_dragged = ColorBarDragged::SatValue;
                *self.focus = Focus::SatValue;
            }
            if hue_bounds.contains(cursor_position) {
                *self.color_bar_dragged = ColorBarDragged::Hue;
                *self.focus = Focus::Hue;
            }
        } else if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = event {
            *self.color_bar_dragged = ColorBarDragged::None;
        }

        let calc_percentage_sat = |bounds: &Rectangle, cursor_position: &Point| {
            ((cursor_position.x - bounds.x).max(0.0) / bounds.width).min(1.0)
        };

        let calc_percentage_value = |bounds: &Rectangle, cursor_position: &Point| {
            ((cursor_position.y - bounds.y).max(0.0) / bounds.height).min(1.0)
        };

        let calc_hue = |bounds: &Rectangle, cursor_position: &Point| {
            (((cursor_position.x - bounds.x).max(0.0) / bounds.width).min(1.0) * 360.0) as u16
        };

        match self.color_bar_dragged {
            ColorBarDragged::SatValue => {
                *self.color = Color {
                    a: self.color.a,
                    ..Hsv {
                        saturation: calc_percentage_sat(&sat_value_bounds, &cursor_position),
                        value: calc_percentage_value(&sat_value_bounds, &cursor_position),
                        ..hsv_color
                    }
                    .into()
                };
                color_changed = true;
            }
            ColorBarDragged::Hue => {
                *self.color = Color {
                    a: self.color.a,
                    ..Hsv {
                        hue: calc_hue(&hue_bounds, &cursor_position),
                        ..hsv_color
                    }
                    .into()
                };
                color_changed = true;
            }
            _ => {}
        }

        if color_changed {
            event::Status::Captured
        } else {
            event::Status::Ignored
        }
    }

    /// The event handling for the RGBA color area.
    fn on_event_rgba_color(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut rgba_color_children = layout.children();
        let mut color_changed = false;

        let mut red_row_children = rgba_color_children.next().unwrap().children();
        let _ = red_row_children.next();
        let red_bar_bounds = red_row_children.next().unwrap().bounds();

        let mut green_row_children = rgba_color_children.next().unwrap().children();
        let _ = green_row_children.next();
        let green_bar_bounds = green_row_children.next().unwrap().bounds();

        let mut blue_row_children = rgba_color_children.next().unwrap().children();
        let _ = blue_row_children.next();
        let blue_bar_bounds = blue_row_children.next().unwrap().bounds();

        let mut alpha_row_children = rgba_color_children.next().unwrap().children();
        let _ = alpha_row_children.next();
        let alpha_bar_bounds = alpha_row_children.next().unwrap().bounds();

        if let Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
            match delta {
                mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                    let move_value =
                        |value: f32, y: f32| (value * 255.0 + y).min(255.0).max(0.0) / 255.0;

                    if red_bar_bounds.contains(cursor_position) {
                        *self.color = Color {
                            r: move_value(self.color.r, y),
                            ..*self.color
                        };
                        color_changed = true;
                    }
                    if green_bar_bounds.contains(cursor_position) {
                        *self.color = Color {
                            g: move_value(self.color.g, y),
                            ..*self.color
                        };
                        color_changed = true;
                    }
                    if blue_bar_bounds.contains(cursor_position) {
                        *self.color = Color {
                            b: move_value(self.color.b, y),
                            ..*self.color
                        };
                        color_changed = true;
                    }
                    if alpha_bar_bounds.contains(cursor_position) {
                        *self.color = Color {
                            a: move_value(self.color.a, y),
                            ..*self.color
                        };
                        color_changed = true;
                    }
                }
            }
        } else if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if red_bar_bounds.contains(cursor_position) {
                *self.color_bar_dragged = ColorBarDragged::Red;
                *self.focus = Focus::Red;
            }
            if green_bar_bounds.contains(cursor_position) {
                *self.color_bar_dragged = ColorBarDragged::Green;
                *self.focus = Focus::Green;
            }
            if blue_bar_bounds.contains(cursor_position) {
                *self.color_bar_dragged = ColorBarDragged::Blue;
                *self.focus = Focus::Blue;
            }
            if alpha_bar_bounds.contains(cursor_position) {
                *self.color_bar_dragged = ColorBarDragged::Alpha;
                *self.focus = Focus::Alpha;
            }
        } else if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = event {
            *self.color_bar_dragged = ColorBarDragged::None;
        }

        let calc_percantage = |bounds: &Rectangle, cursor_position: &Point| {
            ((cursor_position.x - bounds.x).max(0.0) / bounds.width).min(1.0)
        };

        match self.color_bar_dragged {
            ColorBarDragged::Red => {
                *self.color = Color {
                    r: calc_percantage(&red_bar_bounds, &cursor_position),
                    ..*self.color
                };
                color_changed = true;
            }
            ColorBarDragged::Green => {
                *self.color = Color {
                    g: calc_percantage(&green_bar_bounds, &cursor_position),
                    ..*self.color
                };
                color_changed = true;
            }
            ColorBarDragged::Blue => {
                *self.color = Color {
                    b: calc_percantage(&blue_bar_bounds, &cursor_position),
                    ..*self.color
                };
                color_changed = true;
            }
            ColorBarDragged::Alpha => {
                *self.color = Color {
                    a: calc_percantage(&alpha_bar_bounds, &cursor_position),
                    ..*self.color
                };
                color_changed = true;
            }
            _ => {}
        }

        if color_changed {
            event::Status::Captured
        } else {
            event::Status::Ignored
        }
    }

    fn on_event_keyboard(
        &mut self,
        event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        // TODO: clean this up a bit
        if *self.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
            let mut status = event::Status::Ignored;

            match key_code {
                keyboard::KeyCode::Tab => {
                    if self.keyboard_modifiers.shift {
                        *self.focus = self.focus.previous();
                    } else {
                        *self.focus = self.focus.next();
                    }
                    // TODO: maybe place this better
                    self.sat_value_canvas_cache.clear();
                    self.hue_canvas_cache.clear();
                }
                _ => {
                    let sat_value_handle = |key_code: keyboard::KeyCode, color: &mut Color| {
                        let mut hsv_color: Hsv = color.to_owned().into();
                        let mut status = event::Status::Ignored;

                        match key_code {
                            keyboard::KeyCode::Left => {
                                hsv_color.saturation -= SAT_VALUE_STEP;
                                status = event::Status::Captured;
                            }
                            keyboard::KeyCode::Right => {
                                hsv_color.saturation += SAT_VALUE_STEP;
                                status = event::Status::Captured;
                            }
                            keyboard::KeyCode::Up => {
                                hsv_color.value -= SAT_VALUE_STEP;
                                status = event::Status::Captured;
                            }
                            keyboard::KeyCode::Down => {
                                hsv_color.value += SAT_VALUE_STEP;
                                status = event::Status::Captured;
                            }
                            _ => {}
                        }

                        hsv_color.saturation = hsv_color.saturation.min(1.0).max(0.0);
                        hsv_color.value = hsv_color.value.min(1.0).max(0.0);

                        *color = Color {
                            a: color.a,
                            ..hsv_color.into()
                        };
                        status
                    };

                    let hue_handle = |key_code: keyboard::KeyCode, color: &mut Color| {
                        let mut hsv_color: Hsv = color.to_owned().into();
                        let mut status = event::Status::Ignored;

                        let mut value = hsv_color.hue as i32;

                        match key_code {
                            keyboard::KeyCode::Left | keyboard::KeyCode::Down => {
                                value -= HUE_STEP;
                                status = event::Status::Captured;
                            }
                            keyboard::KeyCode::Right | keyboard::KeyCode::Up => {
                                value += HUE_STEP;
                                status = event::Status::Captured;
                            }
                            _ => {}
                        }

                        hsv_color.hue = value.rem_euclid(360) as u16;

                        *color = Color {
                            a: color.a,
                            ..hsv_color.into()
                        };

                        status
                    };

                    let rgba_bar_handle = |key_code: keyboard::KeyCode, value: &mut f32| {
                        let mut byte_value = (*value * 255.0) as u16;
                        let mut status = event::Status::Captured;

                        match key_code {
                            keyboard::KeyCode::Left | keyboard::KeyCode::Down => {
                                byte_value -= RGBA_STEP;
                                status = event::Status::Captured;
                            }
                            keyboard::KeyCode::Right | keyboard::KeyCode::Up => {
                                byte_value += RGBA_STEP;
                                status = event::Status::Captured;
                            }
                            _ => {}
                        }
                        *value = byte_value.min(255).max(0) as f32 / 255.0;

                        status
                    };

                    match self.focus {
                        Focus::Overlay => {}
                        Focus::SatValue => status = sat_value_handle(key_code, &mut self.color),
                        Focus::Hue => status = hue_handle(key_code, &mut self.color),
                        Focus::Red => status = rgba_bar_handle(key_code, &mut self.color.r),
                        Focus::Green => status = rgba_bar_handle(key_code, &mut self.color.g),
                        Focus::Blue => status = rgba_bar_handle(key_code, &mut self.color.b),
                        Focus::Alpha => status = rgba_bar_handle(key_code, &mut self.color.a),
                        Focus::Cancel => {}
                        Focus::Submit => {}
                        _ => {}
                    }
                }
            }

            status
        } else if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
            *self.keyboard_modifiers = modifiers;
            event::Status::Ignored
        } else {
            event::Status::Ignored
        }
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for ColorPickerOverlay<'a, Message, Renderer>
where
    Message: 'static + Clone,
    Renderer: 'a
        + self::Renderer
        + column::Renderer
        + button::Renderer
        + icon_text::Renderer
        + row::Renderer
        + text::Renderer
        + text_input::Renderer,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let (max_width, max_height) = if bounds.width > bounds.height {
            (600, 300)
        } else {
            (300, 600)
        };

        let limits = Limits::new(Size::ZERO, bounds)
            .pad(PADDING as f32)
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(max_width)
            .max_height(max_height);

        let divider = if bounds.width > bounds.height {
            Row::<(), Renderer>::new()
                .spacing(SPACING)
                .push(Row::new().width(Length::Fill).height(Length::Fill))
                .push(Row::new().width(Length::Fill).height(Length::Fill))
                .layout(renderer, &limits)
        } else {
            Column::<(), Renderer>::new()
                .spacing(SPACING)
                .push(Row::new().width(Length::Fill).height(Length::Fill))
                .push(Row::new().width(Length::Fill).height(Length::Fill))
                .layout(renderer, &limits)
        };

        let mut divider_children = divider.children().iter();

        let block1_bounds = divider_children.next().unwrap().bounds();
        let block2_bounds = divider_children.next().unwrap().bounds();

        // ----------- Block 1 ----------------------
        let block1_limits = Limits::new(Size::ZERO, block1_bounds.size())
            .width(Length::Fill)
            .height(Length::Fill);

        let mut block1_node = Column::<(), Renderer>::new()
            .spacing(PADDING)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .height(Length::FillPortion(7)),
            )
            .push(
                Row::new()
                    .width(Length::Fill)
                    .height(Length::FillPortion(1)),
            )
            .layout(renderer, &block1_limits);

        block1_node.move_to(Point::new(
            block1_bounds.x + PADDING as f32,
            block1_bounds.y + PADDING as f32,
        ));
        // ----------- Block 1 end ------------------

        // ----------- Block 2 ----------------------
        let block2_limits = Limits::new(Size::ZERO, block2_bounds.size())
            //.pad(PADDING as f32)
            .width(Length::Fill)
            .height(Length::Fill);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = block2_limits;
        let cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        // Text input
        //let text_input_limits = block2_limits;
        //let mut text_input = self.text_input.layout(renderer, &text_input_limits);

        let text_input_limits = block2_limits;
        let mut text_input = Row::<(), Renderer>::new()
            .width(Length::Fill)
            .height(Length::Units(
                text::Renderer::default_size(renderer) + 2 * PADDING,
            ))
            .layout(renderer, &text_input_limits);

        let block2_limits = block2_limits.shrink(Size::new(
            0.0,
            cancel_button.bounds().height + text_input.bounds().height + 2.0 * SPACING as f32,
        ));

        // RGBA Colors
        let mut rgba_colors = Column::<(), Renderer>::new(); //.spacing(SPACING);

        for _ in 0..4 {
            rgba_colors = rgba_colors.push(
                Row::new()
                    .align_items(Align::Center)
                    .spacing(SPACING)
                    .padding(PADDING)
                    .height(Length::Fill)
                    .push(
                        //Row::new().width(Length::FillPortion(1)).height(Length::Fill)
                        Text::new("X:")
                            .horizontal_alignment(iced_graphics::HorizontalAlignment::Center)
                            .vertical_alignment(iced_graphics::VerticalAlignment::Center),
                    )
                    .push(
                        Row::new()
                            .width(Length::FillPortion(5))
                            .height(Length::Fill),
                    )
                    .push(
                        //Row::new().width(Length::FillPortion(1)).height(Length::Fill)
                        Text::new("XXX")
                            .horizontal_alignment(iced_graphics::HorizontalAlignment::Center)
                            .vertical_alignment(iced_graphics::VerticalAlignment::Center),
                    ),
            );
        }

        let mut rgba_colors = rgba_colors.layout(renderer, &block2_limits);

        rgba_colors.move_to(Point::new(
            rgba_colors.bounds().x + PADDING as f32,
            rgba_colors.bounds().y + PADDING as f32,
        ));

        // Text input
        text_input.move_to(Point::new(
            text_input.bounds().x + PADDING as f32,
            text_input.bounds().y + rgba_colors.bounds().height + PADDING as f32 + SPACING as f32,
        ));

        // Buttons
        let cancel_limits = block2_limits.clone().max_width(
            ((rgba_colors.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32,
        );

        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let submit_limits = block2_limits.clone().max_width(
            ((rgba_colors.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32,
        );

        let mut submit_button = self.submit_button.layout(renderer, &submit_limits);

        cancel_button.move_to(Point::new(
            cancel_button.bounds().x + PADDING as f32,
            cancel_button.bounds().y
                + rgba_colors.bounds().height
                + text_input.bounds().height
                + PADDING as f32
                + 2.0 * SPACING as f32,
        ));

        submit_button.move_to(Point::new(
            submit_button.bounds().x + rgba_colors.bounds().width - submit_button.bounds().width
                + PADDING as f32,
            submit_button.bounds().y
                + rgba_colors.bounds().height
                + text_input.bounds().height
                + PADDING as f32
                + 2.0 * SPACING as f32,
        ));

        let mut block2_node = layout::Node::with_children(
            Size::new(
                rgba_colors.bounds().width + (2.0 * PADDING as f32),
                rgba_colors.bounds().height
                    + text_input.bounds().height
                    + cancel_button.bounds().height
                    + (2.0 * PADDING as f32)
                    + (2.0 * SPACING as f32),
            ),
            vec![rgba_colors, text_input, cancel_button, submit_button],
        );
        block2_node.move_to(Point::new(block2_bounds.x, block2_bounds.y));
        // ----------- Block 2 end ------------------

        let (width, height) = if bounds.width > bounds.height {
            (
                block1_node.size().width + block2_node.size().width + SPACING as f32, // + (2.0 * PADDING as f32),
                block2_node.size().height,
            )
        } else {
            (
                block2_node.size().width,
                block1_node.size().height + block2_node.size().height + SPACING as f32,
            )
        };

        let mut node =
            layout::Node::with_children(Size::new(width, height), vec![block1_node, block2_node]);

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
        if let event::Status::Captured = self.on_event_keyboard(
            event.clone(),
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        ) {
            self.sat_value_canvas_cache.clear();
            self.hue_canvas_cache.clear();
            return event::Status::Captured;
        }

        let mut children = layout.children();

        let status = event::Status::Ignored;

        // ----------- Block 1 ----------------------
        let block1_layout = children.next().unwrap();
        let hsv_color_status = self.on_event_hsv_color(
            event.clone(),
            block1_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Block 1 end ------------------

        // ----------- Block 2 ----------------------
        let mut block2_children = children.next().unwrap().children();

        // ----------- RGB Color -----------------------
        let rgba_color_layout = block2_children.next().unwrap();
        let rgba_color_status = self.on_event_rgba_color(
            event.clone(),
            rgba_color_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        let mut fake_messages: Vec<Message> = Vec::new();

        // ----------- Text input ----------------------
        let _text_input_layout = block2_children.next().unwrap();

        // ----------- Buttons -------------------------
        let cancel_button_layout = block2_children.next().unwrap();
        let cancel_button_status = self.cancel_button.on_event(
            event.clone(),
            cancel_button_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        let submit_button_layout = block2_children.next().unwrap();
        let submit_button_status = self.submit_button.on_event(
            event,
            submit_button_layout,
            cursor_position,
            &mut fake_messages,
            renderer,
            clipboard,
        );

        if !fake_messages.is_empty() {
            messages.push((self.on_submit)(*self.color));
        }
        // ----------- Block 2 end ------------------
        if hsv_color_status == event::Status::Captured
            || rgba_color_status == event::Status::Captured
        {
            self.sat_value_canvas_cache.clear();
            self.hue_canvas_cache.clear();
        }

        status
            .merge(hsv_color_status)
            .merge(rgba_color_status)
            //.merge(text_input_status)
            .merge(cancel_button_status)
            .merge(submit_button_status)
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
                focus: *self.focus,
            },
            &self.color,
            &self.sat_value_canvas_cache,
            &self.hue_canvas_cache,
            //&self.text_input,
            &self.cancel_button,
            &self.submit_button,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher, position: Point) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
    }
}

/// The renderer of a [`ColorPickerOverlay`](ColorPickerOverlay).
///
/// Your renderer will need to implement this trait before being
/// able to use a [`ColorPicker`](crate::native::ColorPicker) in your user
/// interface.
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`ColorPickerOverlay`](ColorPickerOverlay)
    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, Focus>,
        color: &Color,
        sat_value_canvas_cache: &canvas::Cache,
        hue_canvas_cache: &canvas::Cache,
        //text_input: &Element<'_, Message, Self>,
        cancel_button: &Element<'_, Message, Self>,
        submit_button: &Element<'_, Message, Self>,
    ) -> Self::Output;
}

#[cfg(debug_assertions)]
impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _env: DrawEnvironment<'_, Self::Defaults, Self::Style, Focus>,
        _color: &Color,
        _sat_value_canvas_cache: &canvas::Cache,
        _hue_canvas_cache: &canvas::Cache,
        //_text_input: &Element<'_, Message, Self>,
        _cancel_button: &Element<'_, Message, Self>,
        _submit_button: &Element<'_, Message, Self>,
    ) -> Self::Output {
    }
}

/// The state of the currently dragged area.
#[derive(Debug)]
pub enum ColorBarDragged {
    /// No area is focussed.
    None,

    /// The saturation/value area is focussed.
    SatValue,

    /// The hue area is focussed.
    Hue,

    /// The red area is focussed.
    Red,

    /// The green area is focussed.
    Green,

    /// The blue area is focussed.
    Blue,

    /// The alpha area is focussed.
    Alpha,
}

impl Default for ColorBarDragged {
    fn default() -> Self {
        ColorBarDragged::None
    }
}

/// An enumeration of all focusable element of the [`ColorPickerOverlay`](ColorPickerOverlay).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Focus {
    /// Nothing is in focus.
    None,

    /// The overlay itself is in focus.
    Overlay,

    /// The saturation and value area is in focus.
    SatValue,

    /// The hue bar is in focus.
    Hue,

    /// The red bar is in focus.
    Red,

    /// The green bar is in focus.
    Green,

    /// The blue bar is in focus.
    Blue,

    /// The alpha bar is in focus.
    Alpha,

    /// The cancel button is in focus.
    Cancel,

    /// The submit button is in focus.
    Submit,
}

impl Focus {
    /// Gets the next focusable element.
    pub fn next(self) -> Self {
        match self {
            Focus::None => Focus::Overlay,
            Focus::Overlay => Focus::SatValue,
            Focus::SatValue => Focus::Hue,
            Focus::Hue => Focus::Red,
            Focus::Red => Focus::Green,
            Focus::Green => Focus::Blue,
            Focus::Blue => Focus::Alpha,
            Focus::Alpha => Focus::Cancel,
            Focus::Cancel => Focus::Submit,
            Focus::Submit => Focus::Overlay,
        }
    }

    /// Gets the previous focusable element.
    pub fn previous(self) -> Self {
        match self {
            Focus::None => Focus::None,
            Focus::Overlay => Focus::Submit,
            Focus::SatValue => Focus::Overlay,
            Focus::Hue => Focus::SatValue,
            Focus::Red => Focus::Hue,
            Focus::Green => Focus::Red,
            Focus::Blue => Focus::Green,
            Focus::Alpha => Focus::Blue,
            Focus::Cancel => Focus::Alpha,
            Focus::Submit => Focus::Cancel,
        }
    }
}

impl Default for Focus {
    fn default() -> Self {
        Focus::None
    }
}
