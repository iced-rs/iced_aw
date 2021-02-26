//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*
use std::hash::Hash;

use iced_graphics::canvas;
use iced_native::{
    button, column, event, keyboard,
    layout::{self, Limits},
    mouse, overlay, row, text, text_input, touch, Align, Button, Clipboard, Color, Column, Element,
    Event, Layout, Length, Point, Rectangle, Row, Size, Text, Widget,
};

use crate::{
    core::{color::Hsv, overlay::Position, renderer::DrawEnvironment},
    graphics::icons::Icon,
    native::{color_picker, icon_text, IconText},
};

/// The padding around the elements.
const PADDING: u16 = 10;
/// The spacing between the element.
const SPACING: u16 = 15;
/// The spacing between the buttons.
const BUTTON_SPACING: u16 = 5;

/// The step value of the keyboard change of the sat/value color values.
const SAT_VALUE_STEP: f32 = 0.005;
/// The step value of the keyboard change of the hue color value.
const HUE_STEP: i32 = 1;
/// The step value of the keyboard change of the RGBA color values.
const RGBA_STEP: i16 = 1;

/// The overlay of the [`ColorPicker`](crate::native::ColorPicker).
#[allow(missing_debug_implementations)]
pub struct ColorPickerOverlay<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    /// The state of the [`ColorPickerOverlay`](ColorPickerOverlay).
    state: &'a mut State,
    /// The cancel button of the [`ColorPickerOverlay`](ColorPickerOverlay).
    cancel_button: Element<'a, Message, Renderer>,
    /// The submit button of the [`ColorPickerOverlay`](ColorPickerOverlay).
    submit_button: Element<'a, Message, Renderer>,
    /// The function that produces a message when the submit button of the [`ColorPickerOverlay`](ColorPickerOverlay).
    on_submit: &'a dyn Fn(Color) -> Message,
    /// The position of the [`ColorPickerOverlay`](ColorPickerOverlay).
    position: Point,
    /// The style of the [`ColorPickerOverlay`](ColorPickerOverlay).
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
            overlay_state,
            cancel_button,
            submit_button,
            ..
        } = state;

        ColorPickerOverlay {
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

    /// Turn this [`ColorPickerOverlay`](ColorPickerOverlay) into an overlay
    /// [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(self.position, Box::new(self))
    }

    /// The event handling for the HSV color area.
    fn on_event_hsv_color(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut hsv_color_children = layout.children();

        let hsv_color: Hsv = self.state.color.to_owned().into();
        let mut color_changed = false;

        let sat_value_bounds = hsv_color_children
            .next()
            .expect("Native: Layout should have a sat/value layout")
            .bounds();
        let hue_bounds = hsv_color_children
            .next()
            .expect("Native: Layout should have a hue layout")
            .bounds();

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => match delta {
                mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                    let move_value =
                        |value: u16, y: f32| ((i32::from(value) + y as i32).rem_euclid(360)) as u16;

                    if hue_bounds.contains(cursor_position) {
                        self.state.color = Color {
                            a: self.state.color.a,
                            ..Hsv {
                                hue: move_value(hsv_color.hue, *y),
                                ..hsv_color
                            }
                            .into()
                        };
                        color_changed = true;
                    }
                }
            },
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if sat_value_bounds.contains(cursor_position) {
                    self.state.color_bar_dragged = ColorBarDragged::SatValue;
                    self.state.focus = Focus::SatValue;
                }
                if hue_bounds.contains(cursor_position) {
                    self.state.color_bar_dragged = ColorBarDragged::Hue;
                    self.state.focus = Focus::Hue;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                self.state.color_bar_dragged = ColorBarDragged::None;
            }
            _ => {}
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

        match self.state.color_bar_dragged {
            ColorBarDragged::SatValue => {
                self.state.color = Color {
                    a: self.state.color.a,
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
                self.state.color = Color {
                    a: self.state.color.a,
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
    #[allow(clippy::too_many_lines)]
    fn on_event_rgba_color(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut rgba_color_children = layout.children();
        let mut color_changed = false;

        let mut red_row_children = rgba_color_children
            .next()
            .expect("Native: Layout should have a red row layout")
            .children();
        let _ = red_row_children.next();
        let red_bar_bounds = red_row_children
            .next()
            .expect("Native: Layout should have a red bar layout")
            .bounds();

        let mut green_row_children = rgba_color_children
            .next()
            .expect("Native: Layout should have a green row layout")
            .children();
        let _ = green_row_children.next();
        let green_bar_bounds = green_row_children
            .next()
            .expect("Native: Layout should have a green bar layout")
            .bounds();

        let mut blue_row_children = rgba_color_children
            .next()
            .expect("Native: Layout should have a blue row layout")
            .children();
        let _ = blue_row_children.next();
        let blue_bar_bounds = blue_row_children
            .next()
            .expect("Native: Layout should have a blue bar layout")
            .bounds();

        let mut alpha_row_children = rgba_color_children
            .next()
            .expect("Native: Layout should have an alpha row layout")
            .children();
        let _ = alpha_row_children.next();
        let alpha_bar_bounds = alpha_row_children
            .next()
            .expect("Native: Layout should have an alpha bar layout")
            .bounds();

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => match delta {
                mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                    let move_value =
                        //|value: f32, y: f32| (value * 255.0 + y).clamp(0.0, 255.0) / 255.0;
                        |value: f32, y: f32| value.mul_add(255.0, y).clamp(0.0, 255.0) / 255.0;

                    if red_bar_bounds.contains(cursor_position) {
                        self.state.color = Color {
                            r: move_value(self.state.color.r, *y),
                            ..self.state.color
                        };
                        color_changed = true;
                    }
                    if green_bar_bounds.contains(cursor_position) {
                        self.state.color = Color {
                            g: move_value(self.state.color.g, *y),
                            ..self.state.color
                        };
                        color_changed = true;
                    }
                    if blue_bar_bounds.contains(cursor_position) {
                        self.state.color = Color {
                            b: move_value(self.state.color.b, *y),
                            ..self.state.color
                        };
                        color_changed = true;
                    }
                    if alpha_bar_bounds.contains(cursor_position) {
                        self.state.color = Color {
                            a: move_value(self.state.color.a, *y),
                            ..self.state.color
                        };
                        color_changed = true;
                    }
                }
            },
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if red_bar_bounds.contains(cursor_position) {
                    self.state.color_bar_dragged = ColorBarDragged::Red;
                    self.state.focus = Focus::Red;
                }
                if green_bar_bounds.contains(cursor_position) {
                    self.state.color_bar_dragged = ColorBarDragged::Green;
                    self.state.focus = Focus::Green;
                }
                if blue_bar_bounds.contains(cursor_position) {
                    self.state.color_bar_dragged = ColorBarDragged::Blue;
                    self.state.focus = Focus::Blue;
                }
                if alpha_bar_bounds.contains(cursor_position) {
                    self.state.color_bar_dragged = ColorBarDragged::Alpha;
                    self.state.focus = Focus::Alpha;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                self.state.color_bar_dragged = ColorBarDragged::None;
            }
            _ => {}
        }

        let calc_percantage = |bounds: &Rectangle, cursor_position: &Point| {
            ((cursor_position.x - bounds.x).max(0.0) / bounds.width).min(1.0)
        };

        match self.state.color_bar_dragged {
            ColorBarDragged::Red => {
                self.state.color = Color {
                    r: calc_percantage(&red_bar_bounds, &cursor_position),
                    ..self.state.color
                };
                color_changed = true;
            }
            ColorBarDragged::Green => {
                self.state.color = Color {
                    g: calc_percantage(&green_bar_bounds, &cursor_position),
                    ..self.state.color
                };
                color_changed = true;
            }
            ColorBarDragged::Blue => {
                self.state.color = Color {
                    b: calc_percantage(&blue_bar_bounds, &cursor_position),
                    ..self.state.color
                };
                color_changed = true;
            }
            ColorBarDragged::Alpha => {
                self.state.color = Color {
                    a: calc_percantage(&alpha_bar_bounds, &cursor_position),
                    ..self.state.color
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

    /// The even handling for the keyboard input.
    fn on_event_keyboard(
        &mut self,
        event: &Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
            let mut status = event::Status::Ignored;

            if let keyboard::KeyCode::Tab = key_code {
                if self.state.keyboard_modifiers.shift {
                    self.state.focus = self.state.focus.previous();
                } else {
                    self.state.focus = self.state.focus.next();
                }
                // TODO: maybe place this better
                self.state.sat_value_canvas_cache.clear();
                self.state.hue_canvas_cache.clear();
            } else {
                let sat_value_handle = |key_code: &keyboard::KeyCode, color: &mut Color| {
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

                    hsv_color.saturation = hsv_color.saturation.clamp(0.0, 1.0);
                    hsv_color.value = hsv_color.value.clamp(0.0, 1.0);

                    *color = Color {
                        a: color.a,
                        ..hsv_color.into()
                    };
                    status
                };

                let hue_handle = |key_code: &keyboard::KeyCode, color: &mut Color| {
                    let mut hsv_color: Hsv = color.to_owned().into();
                    let mut status = event::Status::Ignored;

                    let mut value = i32::from(hsv_color.hue);

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

                let rgba_bar_handle = |key_code: &keyboard::KeyCode, value: &mut f32| {
                    let mut byte_value = (*value * 255.0) as i16;
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
                    *value = f32::from(byte_value.clamp(0, 255)) / 255.0;

                    status
                };

                match self.state.focus {
                    Focus::SatValue => status = sat_value_handle(key_code, &mut self.state.color),
                    Focus::Hue => status = hue_handle(key_code, &mut self.state.color),
                    Focus::Red => status = rgba_bar_handle(key_code, &mut self.state.color.r),
                    Focus::Green => status = rgba_bar_handle(key_code, &mut self.state.color.g),
                    Focus::Blue => status = rgba_bar_handle(key_code, &mut self.state.color.b),
                    Focus::Alpha => status = rgba_bar_handle(key_code, &mut self.state.color.a),
                    _ => {}
                }
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
            .pad(f32::from(PADDING))
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

        let block1_bounds = divider_children
            .next()
            .expect("Divider should have a first child")
            .bounds();
        let block2_bounds = divider_children
            .next()
            .expect("Divider should have a second child")
            .bounds();

        // ----------- Block 1 ----------------------
        let block1_node = block1_layout(self, renderer, block1_bounds, position);

        // ----------- Block 2 ----------------------
        let block2_node = block2_layout(self, renderer, block2_bounds, position);

        let (width, height) = if bounds.width > bounds.height {
            (
                block1_node.size().width + block2_node.size().width + f32::from(SPACING), // + (2.0 * PADDING as f32),
                block2_node.size().height,
            )
        } else {
            (
                block2_node.size().width,
                block1_node.size().height + block2_node.size().height + f32::from(SPACING),
            )
        };

        let mut node =
            layout::Node::with_children(Size::new(width, height), vec![block1_node, block2_node]);

        node.center_and_bounce(position, bounds);

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
            &event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        ) {
            self.state.sat_value_canvas_cache.clear();
            self.state.hue_canvas_cache.clear();
            return event::Status::Captured;
        }

        let mut children = layout.children();

        let status = event::Status::Ignored;

        // ----------- Block 1 ----------------------
        let block1_layout = children
            .next()
            .expect("Native: Layout should have a 1. block layout");
        let hsv_color_status = self.on_event_hsv_color(
            &event,
            block1_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );
        // ----------- Block 1 end ------------------

        // ----------- Block 2 ----------------------
        let mut block2_children = children
            .next()
            .expect("Native: Layout should have a 2. block layout")
            .children();

        // ----------- RGB Color -----------------------
        let rgba_color_layout = block2_children
            .next()
            .expect("Native: Layout should have a RGBA color layout");
        let rgba_color_status = self.on_event_rgba_color(
            &event,
            rgba_color_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        let mut fake_messages: Vec<Message> = Vec::new();

        // ----------- Text input ----------------------
        let _text_input_layout = block2_children
            .next()
            .expect("Native: Layout should have a hex text layout");

        // ----------- Buttons -------------------------
        let cancel_button_layout = block2_children
            .next()
            .expect("Native: Layout should have a cancel button layout for a ColorPicker");
        let cancel_button_status = self.cancel_button.on_event(
            event.clone(),
            cancel_button_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        let submit_button_layout = block2_children
            .next()
            .expect("Native: Layout should have a submit button layout for a ColorPicker");
        let submit_button_status = self.submit_button.on_event(
            event,
            submit_button_layout,
            cursor_position,
            &mut fake_messages,
            renderer,
            clipboard,
        );

        if !fake_messages.is_empty() {
            messages.push((self.on_submit)(self.state.color));
        }
        // ----------- Block 2 end ------------------

        if hsv_color_status == event::Status::Captured
            || rgba_color_status == event::Status::Captured
        {
            self.state.sat_value_canvas_cache.clear();
            self.state.hue_canvas_cache.clear();
        }

        status
            .merge(hsv_color_status)
            .merge(rgba_color_status)
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
                style_sheet: self.style,
                viewport: None,
                focus: self.state.focus,
            },
            &self.state.color,
            &self.state.sat_value_canvas_cache,
            &self.state.hue_canvas_cache,
            //&self.text_input,
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
    }
}

/// Defines the layout of the 1. block of the color picker containing the HSV part.
fn block1_layout<'a, Message, Renderer>(
    _color_picker: &ColorPickerOverlay<'a, Message, Renderer>,
    renderer: &Renderer,
    bounds: iced_graphics::Rectangle,
    _position: Point,
) -> iced_native::layout::Node
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
    let block1_limits = Limits::new(Size::ZERO, bounds.size())
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
        bounds.x + f32::from(PADDING),
        bounds.y + f32::from(PADDING),
    ));

    block1_node
}

/// Defines the layout of the 2. block of the color picker containing the RGBA part, Hex and buttons.
fn block2_layout<'a, Message, Renderer>(
    color_picker: &ColorPickerOverlay<'a, Message, Renderer>,
    renderer: &Renderer,
    bounds: iced_graphics::Rectangle,
    _position: Point,
) -> iced_native::layout::Node
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
    let block2_limits = Limits::new(Size::ZERO, bounds.size())
        .width(Length::Fill)
        .height(Length::Fill);

    // Pre-Buttons TODO: get rid of it
    let cancel_limits = block2_limits;
    let cancel_button = color_picker.cancel_button.layout(renderer, &cancel_limits);

    let hex_text_limits = block2_limits;
    let mut hex_text = Row::<(), Renderer>::new()
        .width(Length::Fill)
        .height(Length::Units(
            text::Renderer::default_size(renderer) + 2 * PADDING,
        ))
        .layout(renderer, &hex_text_limits);

    let block2_limits = block2_limits.shrink(Size::new(
        0.0,
        cancel_button.bounds().height + hex_text.bounds().height + 2.0 * f32::from(SPACING),
    ));

    // RGBA Colors
    let mut rgba_colors = Column::<(), Renderer>::new();

    for _ in 0..4 {
        rgba_colors = rgba_colors.push(
            Row::new()
                .align_items(Align::Center)
                .spacing(SPACING)
                .padding(PADDING)
                .height(Length::Fill)
                .push(
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
                    Text::new("XXX")
                        .horizontal_alignment(iced_graphics::HorizontalAlignment::Center)
                        .vertical_alignment(iced_graphics::VerticalAlignment::Center),
                ),
        );
    }

    let mut rgba_colors = rgba_colors.layout(renderer, &block2_limits);

    rgba_colors.move_to(Point::new(
        rgba_colors.bounds().x + f32::from(PADDING),
        rgba_colors.bounds().y + f32::from(PADDING),
    ));

    // Hex text
    hex_text.move_to(Point::new(
        hex_text.bounds().x + f32::from(PADDING),
        hex_text.bounds().y + rgba_colors.bounds().height + f32::from(PADDING) + f32::from(SPACING),
    ));

    // Buttons
    let cancel_limits = block2_limits.clone().max_width(
        ((rgba_colors.bounds().width / 2.0) - f32::from(BUTTON_SPACING)).max(0.0) as u32,
    );

    let mut cancel_button = color_picker.cancel_button.layout(renderer, &cancel_limits);

    let submit_limits = block2_limits.clone().max_width(
        ((rgba_colors.bounds().width / 2.0) - f32::from(BUTTON_SPACING)).max(0.0) as u32,
    );

    let mut submit_button = color_picker.submit_button.layout(renderer, &submit_limits);

    cancel_button.move_to(Point::new(
        cancel_button.bounds().x + f32::from(PADDING),
        cancel_button.bounds().y
            + rgba_colors.bounds().height
            + hex_text.bounds().height
            + f32::from(PADDING)
            + 2.0 * f32::from(SPACING),
    ));

    submit_button.move_to(Point::new(
        submit_button.bounds().x + rgba_colors.bounds().width - submit_button.bounds().width
            + f32::from(PADDING),
        submit_button.bounds().y
            + rgba_colors.bounds().height
            + hex_text.bounds().height
            + f32::from(PADDING)
            + 2.0 * f32::from(SPACING),
    ));

    let mut block2_node = layout::Node::with_children(
        Size::new(
            rgba_colors.bounds().width + (2.0 * f32::from(PADDING)),
            rgba_colors.bounds().height
                + hex_text.bounds().height
                + cancel_button.bounds().height
                + (2.0 * f32::from(PADDING))
                + (2.0 * f32::from(SPACING)),
        ),
        vec![rgba_colors, hex_text, cancel_button, submit_button],
    );
    block2_node.move_to(Point::new(bounds.x, bounds.y));

    block2_node
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

/// The state of the [`ColorPickerOverlay`](ColorPickerOverlay).
#[derive(Debug)]
pub struct State {
    /// The selected color of the [`ColorPickerOverlay`](ColorPickerOverlay).
    pub(crate) color: Color,
    /// The cache of the sat/value canvas of the [`ColorPickerOverlay`](ColorPickerOverlay).
    pub(crate) sat_value_canvas_cache: canvas::Cache,
    /// The cache of the hue canvas of the [`ColorPickerOverlay`](ColorPickerOverlay).
    pub(crate) hue_canvas_cache: canvas::Cache,
    /// The dragged color bar of the [`ColorPickerOverlay`](ColorPickerOverlay).
    pub(crate) color_bar_dragged: ColorBarDragged,
    /// the focus of the [`ColorPickerOverlay`](ColorPickerOverlay).
    pub(crate) focus: Focus,
    /// The previously pressed keyboard modifiers.
    pub(crate) keyboard_modifiers: keyboard::Modifiers,
}

impl Default for State {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(0.5, 0.25, 0.25),
            sat_value_canvas_cache: canvas::Cache::default(),
            hue_canvas_cache: canvas::Cache::default(),
            color_bar_dragged: ColorBarDragged::None,
            focus: Focus::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
        }
    }
}

/// The state of the currently dragged area.
#[derive(Copy, Clone, Debug)]
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
        Self::None
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
    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Overlay => Self::SatValue,
            Self::SatValue => Self::Hue,
            Self::Hue => Self::Red,
            Self::Red => Self::Green,
            Self::Green => Self::Blue,
            Self::Blue => Self::Alpha,
            Self::Alpha => Self::Cancel,
            Self::Cancel => Self::Submit,
            Self::Submit | Self::None => Self::Overlay,
        }
    }

    /// Gets the previous focusable element.
    #[must_use]
    pub const fn previous(self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Overlay => Self::Submit,
            Self::SatValue => Self::Overlay,
            Self::Hue => Self::SatValue,
            Self::Red => Self::Hue,
            Self::Green => Self::Red,
            Self::Blue => Self::Green,
            Self::Alpha => Self::Blue,
            Self::Cancel => Self::Alpha,
            Self::Submit => Self::Cancel,
        }
    }
}

impl Default for Focus {
    fn default() -> Self {
        Self::None
    }
}
