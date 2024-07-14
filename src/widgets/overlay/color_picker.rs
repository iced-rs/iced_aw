//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use crate::{
    color_picker,
    core::icons::bootstrap::{icon_to_string, Bootstrap},
    core::{
        color::{HexString, Hsv},
        overlay::Position,
    },
    style::{self, color_picker::Style, style_state::StyleState, Status},
};

use iced::{
    advanced::{
        graphics::geometry::Renderer as _,
        layout::{Limits, Node},
        overlay, renderer,
        text::Renderer as _,
        widget::{self, tree::Tree},
        Clipboard, Layout, Overlay, Renderer as _, Shell, Text, Widget,
    },
    alignment::{Horizontal, Vertical},
    event,
    keyboard,
    mouse::{self, Cursor},
    touch,
    widget::{
        canvas::{self, LineCap, Path, Stroke},
        text, Button, Column, Row,
    },
    Alignment,
    Border,
    Color,
    Element,
    Event,
    Length,
    Padding,
    Point,
    Rectangle,
    Renderer, // the actual type
    Shadow,
    Size,
    Vector,
};
use std::collections::HashMap;

/// The padding around the elements.
const PADDING: f32 = 10.0;
/// The spacing between the element.
const SPACING: f32 = 15.0;
/// The spacing between the buttons.
const BUTTON_SPACING: f32 = 5.0;

/// The step value of the keyboard change of the sat/value color values.
const SAT_VALUE_STEP: f32 = 0.005;
/// The step value of the keyboard change of the hue color value.
const HUE_STEP: i32 = 1;
/// The step value of the keyboard change of the RGBA color values.
const RGBA_STEP: i16 = 1;

/// The overlay of the [`ColorPicker`](crate::widgets::ColorPicker).
#[allow(missing_debug_implementations)]
pub struct ColorPickerOverlay<'a, 'b, Message, Theme>
where
    Message: Clone,
    Theme: style::color_picker::Catalog + iced::widget::button::Catalog,
    'b: 'a,
{
    /// The state of the [`ColorPickerOverlay`].
    state: &'a mut State,
    /// The cancel button of the [`ColorPickerOverlay`].
    cancel_button: Button<'a, Message, Theme, Renderer>,
    /// The submit button of the [`ColorPickerOverlay`].
    submit_button: Button<'a, Message, Theme, Renderer>,
    /// The function that produces a message when the submit button of the [`ColorPickerOverlay`].
    on_submit: &'a dyn Fn(Color) -> Message,
    /// The position of the [`ColorPickerOverlay`].
    position: Point,
    /// The style of the [`ColorPickerOverlay`].
    class: &'a <Theme as style::color_picker::Catalog>::Class<'b>,
    /// The reference to the tree holding the state of this overlay.
    tree: &'a mut Tree,
}

impl<'a, 'b, Message, Theme> ColorPickerOverlay<'a, 'b, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced::widget::button::Catalog
        + iced::widget::text::Catalog,
    'b: 'a,
{
    /// Creates a new [`ColorPickerOverlay`] on the given position.
    pub fn new(
        state: &'a mut color_picker::State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Color) -> Message,
        position: Point,
        class: &'a <Theme as style::color_picker::Catalog>::Class<'b>,
        tree: &'a mut Tree,
    ) -> Self {
        //state.color_hex = color_picker::State::color_as_string(state.color);
        let color_picker::State { overlay_state } = state;

        ColorPickerOverlay {
            state: overlay_state,
            cancel_button: Button::new(
                iced::widget::Text::new(icon_to_string(Bootstrap::X))
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
                    .font(crate::BOOTSTRAP_FONT),
            )
            .width(Length::Fill)
            .on_press(on_cancel.clone()),
            submit_button: Button::new(
                iced::widget::Text::new(icon_to_string(Bootstrap::Check))
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
                    .font(crate::BOOTSTRAP_FONT),
            )
            .width(Length::Fill)
            .on_press(on_cancel), // Sending a fake message
            on_submit,
            position,
            class,
            tree,
        }
    }

    /// Turn this [`ColorPickerOverlay`] into an overlay [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(self))
    }

    /// The event handling for the HSV color area.
    fn on_event_hsv_color(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
    ) -> event::Status {
        let mut hsv_color_children = layout.children();

        let hsv_color: Hsv = self.state.color.into();
        let mut color_changed = false;

        let sat_value_bounds = hsv_color_children
            .next()
            .expect("widgets: Layout should have a sat/value layout")
            .bounds();
        let hue_bounds = hsv_color_children
            .next()
            .expect("widgets: Layout should have a hue layout")
            .bounds();

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => match delta {
                mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                    let move_value =
                        |value: u16, y: f32| ((i32::from(value) + y as i32).rem_euclid(360)) as u16;

                    if cursor.is_over(hue_bounds) {
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
                if cursor.is_over(sat_value_bounds) {
                    self.state.color_bar_dragged = ColorBarDragged::SatValue;
                    self.state.focus = Focus::SatValue;
                }
                if cursor.is_over(hue_bounds) {
                    self.state.color_bar_dragged = ColorBarDragged::Hue;
                    self.state.focus = Focus::Hue;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                self.state.color_bar_dragged = ColorBarDragged::None;
            }
            _ => {}
        }

        let calc_percentage_sat =
            |cursor_position: Point| (cursor_position.x.max(0.0) / sat_value_bounds.width).min(1.0);

        let calc_percentage_value = |cursor_position: Point| {
            (cursor_position.y.max(0.0) / sat_value_bounds.height).min(1.0)
        };

        let calc_hue = |cursor_position: Point| {
            ((cursor_position.x.max(0.0) / hue_bounds.width).min(1.0) * 360.0) as u16
        };

        match self.state.color_bar_dragged {
            ColorBarDragged::SatValue => {
                self.state.color = Color {
                    a: self.state.color.a,
                    ..Hsv {
                        saturation: cursor
                            .position_in(sat_value_bounds)
                            .map(calc_percentage_sat)
                            .unwrap_or_default(),
                        value: cursor
                            .position_in(sat_value_bounds)
                            .map(calc_percentage_value)
                            .unwrap_or_default(),
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
                        hue: cursor
                            .position_in(hue_bounds)
                            .map(calc_hue)
                            .unwrap_or_default(),
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
        cursor: Cursor,
    ) -> event::Status {
        let mut rgba_color_children = layout.children();
        let mut color_changed = false;

        let mut red_row_children = rgba_color_children
            .next()
            .expect("widgets: Layout should have a red row layout")
            .children();
        let _ = red_row_children.next();
        let red_bar_bounds = red_row_children
            .next()
            .expect("widgets: Layout should have a red bar layout")
            .bounds();

        let mut green_row_children = rgba_color_children
            .next()
            .expect("widgets: Layout should have a green row layout")
            .children();
        let _ = green_row_children.next();
        let green_bar_bounds = green_row_children
            .next()
            .expect("widgets: Layout should have a green bar layout")
            .bounds();

        let mut blue_row_children = rgba_color_children
            .next()
            .expect("widgets: Layout should have a blue row layout")
            .children();
        let _ = blue_row_children.next();
        let blue_bar_bounds = blue_row_children
            .next()
            .expect("widgets: Layout should have a blue bar layout")
            .bounds();

        let mut alpha_row_children = rgba_color_children
            .next()
            .expect("widgets: Layout should have an alpha row layout")
            .children();
        let _ = alpha_row_children.next();
        let alpha_bar_bounds = alpha_row_children
            .next()
            .expect("widgets: Layout should have an alpha bar layout")
            .bounds();

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => match delta {
                mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                    let move_value =
                        //|value: f32, y: f32| (value * 255.0 + y).clamp(0.0, 255.0) / 255.0;
                        |value: f32, y: f32| value.mul_add(255.0, y).clamp(0.0, 255.0) / 255.0;

                    if cursor.is_over(red_bar_bounds) {
                        self.state.color = Color {
                            r: move_value(self.state.color.r, *y),
                            ..self.state.color
                        };
                        color_changed = true;
                    }
                    if cursor.is_over(green_bar_bounds) {
                        self.state.color = Color {
                            g: move_value(self.state.color.g, *y),
                            ..self.state.color
                        };
                        color_changed = true;
                    }
                    if cursor.is_over(blue_bar_bounds) {
                        self.state.color = Color {
                            b: move_value(self.state.color.b, *y),
                            ..self.state.color
                        };
                        color_changed = true;
                    }
                    if cursor.is_over(alpha_bar_bounds) {
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
                if cursor.is_over(red_bar_bounds) {
                    self.state.color_bar_dragged = ColorBarDragged::Red;
                    self.state.focus = Focus::Red;
                }
                if cursor.is_over(green_bar_bounds) {
                    self.state.color_bar_dragged = ColorBarDragged::Green;
                    self.state.focus = Focus::Green;
                }
                if cursor.is_over(blue_bar_bounds) {
                    self.state.color_bar_dragged = ColorBarDragged::Blue;
                    self.state.focus = Focus::Blue;
                }
                if cursor.is_over(alpha_bar_bounds) {
                    self.state.color_bar_dragged = ColorBarDragged::Alpha;
                    self.state.focus = Focus::Alpha;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                self.state.color_bar_dragged = ColorBarDragged::None;
            }
            _ => {}
        }

        let calc_percentage = |bounds: Rectangle, cursor_position: Point| {
            (cursor_position.x.max(0.0) / bounds.width).min(1.0)
        };

        match self.state.color_bar_dragged {
            ColorBarDragged::Red => {
                self.state.color = Color {
                    r: cursor
                        .position_in(red_bar_bounds)
                        .map(|position| calc_percentage(red_bar_bounds, position))
                        .unwrap_or_default(),
                    ..self.state.color
                };
                color_changed = true;
            }
            ColorBarDragged::Green => {
                self.state.color = Color {
                    g: cursor
                        .position_in(green_bar_bounds)
                        .map(|position| calc_percentage(green_bar_bounds, position))
                        .unwrap_or_default(),
                    ..self.state.color
                };
                color_changed = true;
            }
            ColorBarDragged::Blue => {
                self.state.color = Color {
                    b: cursor
                        .position_in(blue_bar_bounds)
                        .map(|position| calc_percentage(blue_bar_bounds, position))
                        .unwrap_or_default(),
                    ..self.state.color
                };
                color_changed = true;
            }
            ColorBarDragged::Alpha => {
                self.state.color = Color {
                    a: cursor
                        .position_in(alpha_bar_bounds)
                        .map(|position| calc_percentage(alpha_bar_bounds, position))
                        .unwrap_or_default(),
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
    fn on_event_keyboard(&mut self, event: &Event) -> event::Status {
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
            let mut status = event::Status::Ignored;

            if matches!(key, keyboard::Key::Named(keyboard::key::Named::Tab)) {
                if self.state.keyboard_modifiers.shift() {
                    self.state.focus = self.state.focus.previous();
                } else {
                    self.state.focus = self.state.focus.next();
                }
                // TODO: maybe place this better
                self.state.sat_value_canvas_cache.clear();
                self.state.hue_canvas_cache.clear();
            } else {
                let sat_value_handle = |key_code: &keyboard::Key, color: &mut Color| {
                    let mut hsv_color: Hsv = (*color).into();
                    let mut status = event::Status::Ignored;

                    match key_code {
                        keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                            hsv_color.saturation -= SAT_VALUE_STEP;
                            status = event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowRight) => {
                            hsv_color.saturation += SAT_VALUE_STEP;
                            status = event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                            hsv_color.value -= SAT_VALUE_STEP;
                            status = event::Status::Captured;
                        }
                        keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
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

                let hue_handle = |key_code: &keyboard::Key, color: &mut Color| {
                    let mut hsv_color: Hsv = (*color).into();
                    let mut status = event::Status::Ignored;

                    let mut value = i32::from(hsv_color.hue);

                    match key_code {
                        keyboard::Key::Named(
                            keyboard::key::Named::ArrowLeft | keyboard::key::Named::ArrowDown,
                        ) => {
                            value -= HUE_STEP;
                            status = event::Status::Captured;
                        }
                        keyboard::Key::Named(
                            keyboard::key::Named::ArrowRight | keyboard::key::Named::ArrowUp,
                        ) => {
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

                let rgba_bar_handle = |key_code: &keyboard::Key, value: &mut f32| {
                    let mut byte_value = (*value * 255.0) as i16;
                    let mut status = event::Status::Captured;

                    match key_code {
                        keyboard::Key::Named(
                            keyboard::key::Named::ArrowLeft | keyboard::key::Named::ArrowDown,
                        ) => {
                            byte_value -= RGBA_STEP;
                            status = event::Status::Captured;
                        }
                        keyboard::Key::Named(
                            keyboard::key::Named::ArrowRight | keyboard::key::Named::ArrowUp,
                        ) => {
                            byte_value += RGBA_STEP;
                            status = event::Status::Captured;
                        }
                        _ => {}
                    }
                    *value = f32::from(byte_value.clamp(0, 255)) / 255.0;

                    status
                };

                match self.state.focus {
                    Focus::SatValue => status = sat_value_handle(key, &mut self.state.color),
                    Focus::Hue => status = hue_handle(key, &mut self.state.color),
                    Focus::Red => status = rgba_bar_handle(key, &mut self.state.color.r),
                    Focus::Green => status = rgba_bar_handle(key, &mut self.state.color.g),
                    Focus::Blue => status = rgba_bar_handle(key, &mut self.state.color.b),
                    Focus::Alpha => status = rgba_bar_handle(key, &mut self.state.color.a),
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

impl<'a, 'b, Message, Theme> Overlay<Message, Theme, Renderer>
    for ColorPickerOverlay<'a, 'b, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced::widget::button::Catalog
        + iced::widget::text::Catalog,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        let (max_width, max_height) = if bounds.width > bounds.height {
            (600.0, 300.0)
        } else {
            (300.0, 600.0)
        };

        let limits = Limits::new(Size::ZERO, bounds)
            .shrink(Padding::from(PADDING))
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(max_width)
            .max_height(max_height);

        let divider = if bounds.width > bounds.height {
            Row::<(), Theme, Renderer>::new()
                .spacing(SPACING)
                .push(Row::new().width(Length::Fill).height(Length::Fill))
                .push(Row::new().width(Length::Fill).height(Length::Fill))
                .layout(self.tree, renderer, &limits)
        } else {
            Column::<(), Theme, Renderer>::new()
                .spacing(SPACING)
                .push(Row::new().width(Length::Fill).height(Length::Fill))
                .push(Row::new().width(Length::Fill).height(Length::Fill))
                .layout(self.tree, renderer, &limits)
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
        let block1_node = block1_layout(self, renderer, block1_bounds);

        // ----------- Block 2 ----------------------
        let block2_node = block2_layout(self, renderer, block2_bounds);

        let (width, height) = if bounds.width > bounds.height {
            (
                block1_node.size().width + block2_node.size().width + SPACING, // + (2.0 * PADDING as f32),
                block2_node.size().height,
            )
        } else {
            (
                block2_node.size().width,
                block1_node.size().height + block2_node.size().height + SPACING,
            )
        };

        let mut node =
            Node::with_children(Size::new(width, height), vec![block1_node, block2_node]);

        node.center_and_bounce(self.position, bounds);
        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        if event::Status::Captured == self.on_event_keyboard(&event) {
            self.state.sat_value_canvas_cache.clear();
            self.state.hue_canvas_cache.clear();
            return event::Status::Captured;
        }

        let mut children = layout.children();

        let status = event::Status::Ignored;

        // ----------- Block 1 ----------------------
        let block1_layout = children
            .next()
            .expect("widgets: Layout should have a 1. block layout");
        let hsv_color_status = self.on_event_hsv_color(&event, block1_layout, cursor);
        // ----------- Block 1 end ------------------

        // ----------- Block 2 ----------------------
        let mut block2_children = children
            .next()
            .expect("widgets: Layout should have a 2. block layout")
            .children();

        // ----------- RGB Color -----------------------
        let rgba_color_layout = block2_children
            .next()
            .expect("widgets: Layout should have a RGBA color layout");
        let rgba_color_status = self.on_event_rgba_color(&event, rgba_color_layout, cursor);

        let mut fake_messages: Vec<Message> = Vec::new();

        // ----------- Text input ----------------------
        let _text_input_layout = block2_children
            .next()
            .expect("widgets: Layout should have a hex text layout");

        // ----------- Buttons -------------------------
        let cancel_button_layout = block2_children
            .next()
            .expect("widgets: Layout should have a cancel button layout for a ColorPicker");
        let cancel_button_status = self.cancel_button.on_event(
            &mut self.tree.children[0],
            event.clone(),
            cancel_button_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );

        let submit_button_layout = block2_children
            .next()
            .expect("widgets: Layout should have a submit button layout for a ColorPicker");
        let submit_button_status = self.submit_button.on_event(
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
            shell.publish((self.on_submit)(self.state.color));
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

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();

        let mouse_interaction = mouse::Interaction::default();

        // Block 1
        let block1_layout = children
            .next()
            .expect("Graphics: Layout should have a 1. block layout");
        let mut block1_mouse_interaction = mouse::Interaction::default();
        // HSV color
        let mut hsv_color_children = block1_layout.children();
        let sat_value_layout = hsv_color_children
            .next()
            .expect("Graphics: Layout should have a sat/value layout");
        if cursor.is_over(sat_value_layout.bounds()) {
            block1_mouse_interaction = block1_mouse_interaction.max(mouse::Interaction::Pointer);
        }
        let hue_layout = hsv_color_children
            .next()
            .expect("Graphics: Layout should have a hue layout");
        if cursor.is_over(hue_layout.bounds()) {
            block1_mouse_interaction = block1_mouse_interaction.max(mouse::Interaction::Pointer);
        }

        // Block 2
        let block2_layout = children
            .next()
            .expect("Graphics: Layout should have a 2. block layout");
        let mut block2_mouse_interaction = mouse::Interaction::default();
        let mut block2_children = block2_layout.children();
        // RGBA color
        let rgba_color_layout = block2_children
            .next()
            .expect("Graphics: Layout should have a RGBA color layout");
        let mut rgba_color_children = rgba_color_layout.children();

        let f = |layout: Layout<'_>, cursor: Cursor| {
            let mut children = layout.children();

            let _label_layout = children.next();
            let bar_layout = children
                .next()
                .expect("Graphics: Layout should have a bar layout");

            if cursor.is_over(bar_layout.bounds()) {
                mouse::Interaction::ResizingHorizontally
            } else {
                mouse::Interaction::default()
            }
        };
        let red_row_layout = rgba_color_children
            .next()
            .expect("Graphics: Layout should have a red row layout");
        block2_mouse_interaction = block2_mouse_interaction.max(f(red_row_layout, cursor));
        let green_row_layout = rgba_color_children
            .next()
            .expect("Graphics: Layout should have a green row layout");
        block2_mouse_interaction = block2_mouse_interaction.max(f(green_row_layout, cursor));
        let blue_row_layout = rgba_color_children
            .next()
            .expect("Graphics: Layout should have a blue row layout");
        block2_mouse_interaction = block2_mouse_interaction.max(f(blue_row_layout, cursor));
        let alpha_row_layout = rgba_color_children
            .next()
            .expect("Graphics: Layout should have an alpha row layout");
        block2_mouse_interaction = block2_mouse_interaction.max(f(alpha_row_layout, cursor));

        let _hex_text_layout = block2_children.next();

        // Buttons
        let cancel_button_layout = block2_children
            .next()
            .expect("Graphics: Layout should have a cancel button layout for a ColorPicker");
        let cancel_mouse_interaction = self.cancel_button.mouse_interaction(
            &self.tree.children[1],
            cancel_button_layout,
            cursor,
            viewport,
            renderer,
        );

        let submit_button_layout = block2_children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a ColorPicker");
        let submit_mouse_interaction = self.submit_button.mouse_interaction(
            &self.tree.children[1],
            submit_button_layout,
            cursor,
            viewport,
            renderer,
        );

        mouse_interaction
            .max(block1_mouse_interaction)
            .max(block2_mouse_interaction)
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
            style::color_picker::Catalog::style(theme, self.class, Status::Active),
        );
        let _ = style_sheet.insert(
            StyleState::Selected,
            style::color_picker::Catalog::style(theme, self.class, Status::Selected),
        );
        let _ = style_sheet.insert(
            StyleState::Hovered,
            style::color_picker::Catalog::style(theme, self.class, Status::Hovered),
        );
        let _ = style_sheet.insert(
            StyleState::Focused,
            style::color_picker::Catalog::style(theme, self.class, Status::Focused),
        );

        let mut style_state = StyleState::Active;
        if self.state.focus == Focus::Overlay {
            style_state = style_state.max(StyleState::Focused);
        }
        if cursor.is_over(bounds) {
            style_state = style_state.max(StyleState::Hovered);
        }

        if (bounds.width > 0.) && (bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style_sheet[&style_state].border_radius.into(),
                        width: style_sheet[&style_state].border_width,
                        color: style_sheet[&style_state].border_color,
                    },
                    shadow: Shadow::default(),
                },
                style_sheet[&style_state].background,
            );
        }

        // ----------- Block 1 ----------------------
        let block1_layout = children
            .next()
            .expect("Graphics: Layout should have a 1. block layout");
        block1(renderer, self, block1_layout, cursor, &style_sheet);

        // ----------- Block 2 ----------------------
        let block2_layout = children
            .next()
            .expect("Graphics: Layout should have a 2. block layout");
        block2(
            renderer,
            self,
            block2_layout,
            cursor,
            theme,
            style,
            &bounds,
            &style_sheet,
        );
    }
}

/// Defines the layout of the 1. block of the color picker containing the HSV part.
fn block1_layout<'a, Message, Theme>(
    color_picker: &mut ColorPickerOverlay<'_, '_, Message, Theme>,
    renderer: &Renderer,
    bounds: Rectangle,
) -> Node
where
    Message: 'static + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced::widget::button::Catalog
        + iced::widget::text::Catalog,
{
    let block1_limits = Limits::new(Size::ZERO, bounds.size())
        .width(Length::Fill)
        .height(Length::Fill);

    let block1_node = Column::<(), Theme, Renderer>::new()
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
        .layout(color_picker.tree, renderer, &block1_limits);

    block1_node.move_to(Point::new(bounds.x + PADDING, bounds.y + PADDING))
}

/// Defines the layout of the 2. block of the color picker containing the RGBA part, Hex and buttons.
fn block2_layout<'a, Message, Theme>(
    color_picker: &mut ColorPickerOverlay<'_, '_, Message, Theme>,
    renderer: &Renderer,
    bounds: Rectangle,
) -> Node
where
    Message: 'static + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced::widget::button::Catalog
        + iced::widget::text::Catalog,
{
    let block2_limits = Limits::new(Size::ZERO, bounds.size())
        .width(Length::Fill)
        .height(Length::Fill);

    // Pre-Buttons TODO: get rid of it
    let cancel_limits = block2_limits;
    let cancel_button = color_picker.cancel_button.layout(
        &mut color_picker.tree.children[0],
        renderer,
        &cancel_limits,
    );

    let hex_text_limits = block2_limits;

    let mut hex_text_layout = Row::<Message, Theme, Renderer>::new()
        .width(Length::Fill)
        .height(Length::Fixed(renderer.default_size().0 + 2.0 * PADDING))
        .layout(color_picker.tree, renderer, &hex_text_limits);

    let block2_limits = block2_limits.shrink(Size::new(
        0.0,
        cancel_button.bounds().height + hex_text_layout.bounds().height + 2.0 * SPACING,
    ));

    // RGBA Colors
    let mut rgba_colors: Column<'_, Message, Theme, Renderer> =
        Column::<Message, Theme, Renderer>::new();

    for _ in 0..4 {
        rgba_colors = rgba_colors.push(
            Row::new()
                .align_y(Alignment::Center)
                .spacing(SPACING)
                .padding(PADDING)
                .height(Length::Fill)
                .push(
                    widget::Text::new("X:")
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center),
                )
                .push(
                    Row::new()
                        .width(Length::FillPortion(5))
                        .height(Length::Fill),
                )
                .push(
                    widget::Text::new("XXX")
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center),
                ),
        );
    }
    let element: Element<Message, Theme, Renderer> = Element::new(rgba_colors);
    let rgba_tree = if let Some(child_tree) = color_picker.tree.children.get_mut(2) {
        child_tree.diff(element.as_widget());
        child_tree
    } else {
        let child_tree = Tree::new(element.as_widget());
        color_picker.tree.children.insert(2, child_tree);
        &mut color_picker.tree.children[2]
    };

    let mut rgba_colors = element
        .as_widget()
        .layout(rgba_tree, renderer, &block2_limits);

    let rgba_bounds = rgba_colors.bounds();
    rgba_colors = rgba_colors.move_to(Point::new(rgba_bounds.x + PADDING, rgba_bounds.y + PADDING));
    let rgba_bounds = rgba_colors.bounds();

    // Hex text
    let hex_bounds = hex_text_layout.bounds();
    hex_text_layout = hex_text_layout.move_to(Point::new(
        hex_bounds.x + PADDING,
        hex_bounds.y + rgba_bounds.height + PADDING + SPACING,
    ));
    let hex_bounds = hex_text_layout.bounds();

    // Buttons
    let cancel_limits =
        block2_limits.max_width(((rgba_bounds.width / 2.0) - BUTTON_SPACING).max(0.0));

    let mut cancel_button = color_picker.cancel_button.layout(
        &mut color_picker.tree.children[0],
        renderer,
        &cancel_limits,
    );

    let submit_limits =
        block2_limits.max_width(((rgba_bounds.width / 2.0) - BUTTON_SPACING).max(0.0));

    let mut submit_button = color_picker.submit_button.layout(
        &mut color_picker.tree.children[1],
        renderer,
        &submit_limits,
    );

    let cancel_bounds = cancel_button.bounds();
    cancel_button = cancel_button.move_to(Point::new(
        cancel_bounds.x + PADDING,
        cancel_bounds.y + rgba_bounds.height + hex_bounds.height + PADDING + 2.0 * SPACING,
    ));
    let cancel_bounds = cancel_button.bounds();

    let submit_bounds = submit_button.bounds();
    submit_button = submit_button.move_to(Point::new(
        submit_bounds.x + rgba_colors.bounds().width - submit_bounds.width + PADDING,
        submit_bounds.y + rgba_bounds.height + hex_bounds.height + PADDING + 2.0 * SPACING,
    ));

    Node::with_children(
        Size::new(
            rgba_bounds.width + (2.0 * PADDING),
            rgba_bounds.height
                + hex_bounds.height
                + cancel_bounds.height
                + (2.0 * PADDING)
                + (2.0 * SPACING),
        ),
        vec![rgba_colors, hex_text_layout, cancel_button, submit_button],
    )
    .move_to(Point::new(bounds.x, bounds.y))
}

/// Draws the 1. block of the color picker containing the HSV part.
fn block1<Message, Theme>(
    renderer: &mut Renderer,
    color_picker: &ColorPickerOverlay<'_, '_, Message, Theme>,
    layout: Layout<'_>,
    cursor: Cursor,
    style_sheet: &HashMap<StyleState, Style>,
) where
    Message: Clone,
    Theme:
        style::color_picker::Catalog + iced::widget::button::Catalog + iced::widget::text::Catalog,
{
    // ----------- Block 1 ----------------------
    let hsv_color_layout = layout;

    // ----------- HSV Color ----------------------
    //let hsv_color_layout = block1_children.next().unwrap();
    hsv_color(
        renderer,
        color_picker,
        hsv_color_layout,
        cursor,
        style_sheet,
    );

    // ----------- Block 1 end ------------------
}

/// Draws the 2. block of the color picker containing the RGBA part, Hex and buttons.
#[allow(clippy::too_many_arguments)]
fn block2<Message, Theme>(
    renderer: &mut Renderer,
    color_picker: &ColorPickerOverlay<'_, '_, Message, Theme>,
    layout: Layout<'_>,
    cursor: Cursor,
    theme: &Theme,
    style: &renderer::Style,
    viewport: &Rectangle,
    style_sheet: &HashMap<StyleState, Style>,
) where
    Message: Clone,
    Theme:
        style::color_picker::Catalog + iced::widget::button::Catalog + iced::widget::text::Catalog,
{
    // ----------- Block 2 ----------------------
    let mut block2_children = layout.children();

    // ----------- RGBA Color ----------------------
    let rgba_color_layout = block2_children
        .next()
        .expect("Graphics: Layout should have a RGBA color layout");
    rgba_color(
        renderer,
        rgba_color_layout,
        &color_picker.state.color,
        cursor,
        style,
        style_sheet,
        color_picker.state.focus,
    );

    // ----------- Hex text ----------------------
    let hex_text_layout = block2_children
        .next()
        .expect("Graphics: Layout should have a hex text layout");
    hex_text(
        renderer,
        hex_text_layout,
        &color_picker.state.color,
        cursor,
        style,
        style_sheet,
        color_picker.state.focus,
    );

    // ----------- Buttons -------------------------
    let cancel_button_layout = block2_children
        .next()
        .expect("Graphics: Layout should have a cancel button layout for a ColorPicker");

    color_picker.cancel_button.draw(
        &color_picker.tree.children[0],
        renderer,
        theme,
        style,
        cancel_button_layout,
        cursor,
        viewport,
    );

    let submit_button_layout = block2_children
        .next()
        .expect("Graphics: Layout should have a submit button layout for a ColorPicker");

    color_picker.submit_button.draw(
        &color_picker.tree.children[1],
        renderer,
        theme,
        style,
        submit_button_layout,
        cursor,
        viewport,
    );

    // Buttons are not focusable right now...
    if color_picker.state.focus == Focus::Cancel {
        let bounds = cancel_button_layout.bounds();
        if (bounds.width > 0.) && (bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style_sheet[&StyleState::Focused].border_radius.into(),
                        width: style_sheet[&StyleState::Focused].border_width,
                        color: style_sheet[&StyleState::Focused].border_color,
                    },
                    shadow: Shadow::default(),
                },
                Color::TRANSPARENT,
            );
        }
    }

    if color_picker.state.focus == Focus::Submit {
        let bounds = submit_button_layout.bounds();
        if (bounds.width > 0.) && (bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style_sheet[&StyleState::Focused].border_radius.into(),
                        width: style_sheet[&StyleState::Focused].border_width,
                        color: style_sheet[&StyleState::Focused].border_color,
                    },
                    shadow: Shadow::default(),
                },
                Color::TRANSPARENT,
            );
        }
    }
    // ----------- Block 2 end ------------------
}

/// Draws the HSV color area.
#[allow(clippy::too_many_lines)]
fn hsv_color<Message, Theme>(
    renderer: &mut Renderer,
    color_picker: &ColorPickerOverlay<'_, '_, Message, Theme>,
    layout: Layout<'_>,
    cursor: Cursor,
    style_sheet: &HashMap<StyleState, Style>,
) where
    Message: Clone,
    Theme:
        style::color_picker::Catalog + iced::widget::button::Catalog + iced::widget::text::Catalog,
{
    let mut hsv_color_children = layout.children();
    let hsv_color: Hsv = color_picker.state.color.into();

    let sat_value_layout = hsv_color_children
        .next()
        .expect("Graphics: Layout should have a sat/value layout");
    let mut sat_value_style_state = StyleState::Active;
    if color_picker.state.focus == Focus::SatValue {
        sat_value_style_state = sat_value_style_state.max(StyleState::Focused);
    }
    if cursor.is_over(sat_value_layout.bounds()) {
        sat_value_style_state = sat_value_style_state.max(StyleState::Hovered);
    }

    let geometry = color_picker.state.sat_value_canvas_cache.draw(
        renderer,
        sat_value_layout.bounds().size(),
        |frame| {
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
                style: canvas::Style::Solid(
                    Hsv {
                        hue: 0,
                        saturation: 0.0,
                        value: 1.0 - hsv_color.value,
                    }
                    .into(),
                ),
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
                style: canvas::Style::Solid(
                    style_sheet
                        .get(&sat_value_style_state)
                        .expect("Style Sheet not found.")
                        .bar_border_color,
                ),
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
        },
    );

    let translation = Vector::new(sat_value_layout.bounds().x, sat_value_layout.bounds().y);
    renderer.with_translation(translation, |renderer| {
        renderer.draw_geometry(geometry);
    });

    let hue_layout = hsv_color_children
        .next()
        .expect("Graphics: Layout should have a hue layout");
    let mut hue_style_state = StyleState::Active;
    if color_picker.state.focus == Focus::Hue {
        hue_style_state = hue_style_state.max(StyleState::Focused);
    }
    if cursor.is_over(hue_layout.bounds()) {
        hue_style_state = hue_style_state.max(StyleState::Hovered);
    }

    let geometry =
        color_picker
            .state
            .hue_canvas_cache
            .draw(renderer, hue_layout.bounds().size(), |frame| {
                let column_count = frame.width() as u16;

                for column in 0..column_count {
                    let hue = (f32::from(column) * 360.0 / frame.width()) as u16;

                    let hsv_color = Hsv::from_hsv(hue, 1.0, 1.0);
                    let stroke = Stroke {
                        style: canvas::Style::Solid(hsv_color.into()),
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
                    style: canvas::Style::Solid(Color::BLACK),
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
                    style: canvas::Style::Solid(
                        style_sheet
                            .get(&hue_style_state)
                            .expect("Style Sheet not found.")
                            .bar_border_color,
                    ),
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
            });

    let translation = Vector::new(hue_layout.bounds().x, hue_layout.bounds().y);
    renderer.with_translation(translation, |renderer| {
        renderer.draw_geometry(geometry);
    });
}

/// Draws the RGBA color area.
#[allow(clippy::too_many_lines)]
fn rgba_color(
    renderer: &mut Renderer,
    layout: Layout<'_>,
    color: &Color,
    cursor: Cursor,
    style: &renderer::Style,
    style_sheet: &HashMap<StyleState, Style>,
    focus: Focus,
) {
    let mut rgba_color_children = layout.children();

    let f = |renderer: &mut Renderer,
             layout: Layout,
             label: &str,
             color: Color,
             value: f32,
             cursor: Cursor,
             target: Focus| {
        let mut children = layout.children();

        let label_layout = children
            .next()
            .expect("Graphics: Layout should have a label layout");
        let bar_layout = children
            .next()
            .expect("Graphics: Layout should have a bar layout");
        let value_layout = children
            .next()
            .expect("Graphics: Layout should have a value layout");

        // Label
        renderer.fill_text(
            Text {
                content: label.to_owned(),
                bounds: Size::new(label_layout.bounds().width, label_layout.bounds().height),
                size: renderer.default_size(),
                font: crate::BOOTSTRAP_FONT,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Advanced,
            },
            Point::new(
                label_layout.bounds().center_x(),
                label_layout.bounds().center_y(),
            ),
            style.text_color,
            label_layout.bounds(),
        );

        let bar_bounds = bar_layout.bounds();

        let bar_style_state = if cursor.is_over(bar_bounds) {
            StyleState::Hovered
        } else {
            StyleState::Active
        };

        // Bar background
        let background_bounds = Rectangle {
            x: bar_bounds.x,
            y: bar_bounds.y,
            width: bar_bounds.width * value,
            height: bar_bounds.height,
        };
        if (background_bounds.width > 0.) && (background_bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: background_bounds,
                    border: Border {
                        radius: style_sheet
                            .get(&bar_style_state)
                            .expect("Style Sheet not found.")
                            .bar_border_radius
                            .into(),
                        width: style_sheet
                            .get(&bar_style_state)
                            .expect("Style Sheet not found.")
                            .bar_border_width,
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
                },
                color,
            );
        }

        // Bar
        if (bar_bounds.width > 0.) && (bar_bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: bar_bounds,
                    border: Border {
                        radius: style_sheet
                            .get(&bar_style_state)
                            .expect("Style Sheet not found.")
                            .bar_border_radius
                            .into(),
                        width: style_sheet
                            .get(&bar_style_state)
                            .expect("Style Sheet not found.")
                            .bar_border_width,
                        color: style_sheet
                            .get(&bar_style_state)
                            .expect("Style Sheet not found.")
                            .bar_border_color,
                    },
                    shadow: Shadow::default(),
                },
                Color::TRANSPARENT,
            );
        }

        // Value
        renderer.fill_text(
            Text {
                content: format!("{}", (255.0 * value) as u8),
                bounds: Size::new(value_layout.bounds().width, value_layout.bounds().height),
                size: renderer.default_size(),
                font: renderer.default_font(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: iced::widget::text::LineHeight::Relative(1.3),
                shaping: iced::widget::text::Shaping::Advanced,
            },
            Point::new(
                value_layout.bounds().center_x(),
                value_layout.bounds().center_y(),
            ),
            style.text_color,
            value_layout.bounds(),
        );

        let bounds = layout.bounds();
        if (focus == target) && (bounds.width > 0.) && (bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style_sheet
                            .get(&StyleState::Focused)
                            .expect("Style Sheet not found.")
                            .border_radius
                            .into(),
                        width: style_sheet
                            .get(&StyleState::Focused)
                            .expect("Style Sheet not found.")
                            .border_width,
                        color: style_sheet
                            .get(&StyleState::Focused)
                            .expect("Style Sheet not found.")
                            .border_color,
                    },
                    shadow: Shadow::default(),
                },
                Color::TRANSPARENT,
            );
        }
    };

    // Red
    let red_row_layout = rgba_color_children
        .next()
        .expect("Graphics: Layout should have a red row layout");

    f(
        renderer,
        red_row_layout,
        "R:",
        Color::from_rgb(color.r, 0.0, 0.0),
        color.r,
        cursor,
        Focus::Red,
    );

    // Green
    let green_row_layout = rgba_color_children
        .next()
        .expect("Graphics: Layout should have a green row layout");

    f(
        renderer,
        green_row_layout,
        "G:",
        Color::from_rgb(0.0, color.g, 0.0),
        color.g,
        cursor,
        Focus::Green,
    );

    // Blue
    let blue_row_layout = rgba_color_children
        .next()
        .expect("Graphics: Layout should have a blue row layout");

    f(
        renderer,
        blue_row_layout,
        "B:",
        Color::from_rgb(0.0, 0.0, color.b),
        color.b,
        cursor,
        Focus::Blue,
    );

    // Alpha
    let alpha_row_layout = rgba_color_children
        .next()
        .expect("Graphics: Layout should have an alpha row layout");

    f(
        renderer,
        alpha_row_layout,
        "A:",
        Color::from_rgba(0.0, 0.0, 0.0, color.a),
        color.a,
        cursor,
        Focus::Alpha,
    );
}

/// Draws the hex text representation of the color.
fn hex_text(
    renderer: &mut Renderer,
    layout: Layout<'_>,
    color: &Color,
    cursor: Cursor,
    _style: &renderer::Style,
    style_sheet: &HashMap<StyleState, Style>,
    _focus: Focus,
) {
    let hsv: Hsv = (*color).into();

    let hex_text_style_state = if cursor.is_over(layout.bounds()) {
        StyleState::Hovered
    } else {
        StyleState::Active
    };

    let bounds = layout.bounds();
    if (bounds.width > 0.) && (bounds.height > 0.) {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: style_sheet[&hex_text_style_state].bar_border_radius.into(),
                    width: style_sheet[&hex_text_style_state].bar_border_width,
                    color: style_sheet[&hex_text_style_state].bar_border_color,
                },
                shadow: Shadow::default(),
            },
            *color,
        );
    }

    renderer.fill_text(
        Text {
            content: color.as_hex_string(),
            bounds: Size::new(bounds.width, bounds.height),
            size: renderer.default_size(),
            font: renderer.default_font(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
        },
        Point::new(bounds.center_x(), bounds.center_y()),
        Color {
            a: 1.0,
            ..Hsv {
                hue: 0,
                saturation: 0.0,
                value: if hsv.value < 0.5 { 1.0 } else { 0.0 },
            }
            .into()
        },
        bounds,
    );
}

/// The state of the [`ColorPickerOverlay`].
#[derive(Debug)]
pub struct State {
    /// The selected color of the [`ColorPickerOverlay`].
    pub(crate) color: Color,
    /// The cache of the sat/value canvas of the [`ColorPickerOverlay`].
    pub(crate) sat_value_canvas_cache: canvas::Cache,
    /// The cache of the hue canvas of the [`ColorPickerOverlay`].
    pub(crate) hue_canvas_cache: canvas::Cache,
    /// The dragged color bar of the [`ColorPickerOverlay`].
    pub(crate) color_bar_dragged: ColorBarDragged,
    /// the focus of the [`ColorPickerOverlay`].
    pub(crate) focus: Focus,
    /// The previously pressed keyboard modifiers.
    pub(crate) keyboard_modifiers: keyboard::Modifiers,
}

impl State {
    /// Creates a new State with the given color.
    #[must_use]
    pub fn new(color: Color) -> Self {
        Self {
            color,
            ..Self::default()
        }
    }
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

/// Just a workaround to pass the button states from the tree to the overlay
#[allow(missing_debug_implementations)]
pub struct ColorPickerOverlayButtons<'a, Message, Theme>
where
    Message: Clone,
    Theme: style::color_picker::Catalog + iced::widget::button::Catalog,
{
    /// The cancel button of the [`ColorPickerOverlay`].
    cancel_button: Element<'a, Message, Theme, Renderer>,
    /// The submit button of the [`ColorPickerOverlay`].
    submit_button: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme> Default for ColorPickerOverlayButtons<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced::widget::button::Catalog
        + iced::widget::text::Catalog,
{
    fn default() -> Self {
        Self {
            cancel_button: Button::new(
                widget::Text::new(icon_to_string(Bootstrap::X)).font(crate::BOOTSTRAP_FONT),
            )
            .into(),
            submit_button: Button::new(
                widget::Text::new(icon_to_string(Bootstrap::Check)).font(crate::BOOTSTRAP_FONT),
            )
            .into(),
        }
    }
}

#[allow(clippy::unimplemented)]
impl<'a, Message, Theme> Widget<Message, Theme, Renderer>
    for ColorPickerOverlayButtons<'a, Message, Theme>
where
    Message: Clone,
    Theme:
        style::color_picker::Catalog + iced::widget::button::Catalog + iced::widget::text::Catalog,
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

impl<'a, Message, Theme> From<ColorPickerOverlayButtons<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a
        + style::color_picker::Catalog
        + iced::widget::button::Catalog
        + iced::widget::text::Catalog,
{
    fn from(overlay: ColorPickerOverlayButtons<'a, Message, Theme>) -> Self {
        Self::new(overlay)
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

/// An enumeration of all focusable element of the [`ColorPickerOverlay`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
