//! Use a color picker as an input element for picking colors.
//! 
//! *This API requires the following crate features to be activated: color_picker*
use std::hash::Hash;

use button::State;
use event::Status;
use iced_graphics::canvas;
use iced_native::{Align, Button, Clipboard, Color, Column, Element, Event, Layout, Length, Point, Rectangle, Row, Size, Text, TextInput, Widget, button, column, event, layout::{self, Limits}, mouse, overlay, row, text, text_input};

use crate::{core::{color::Hsv, renderer::DrawEnvironment}, graphics::icons::Icon, native::{IconText, color_picker, icon_text}};

const PADDING: u16 = 10;
const SPACING: u16 = 15;
const BUTTON_SPACING: u16 = 5;

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
    color_bar_focussed: &'a mut ColorBarFocussed,
    on_submit: &'a dyn Fn(Color) -> Message,
    position: Point,
    style: &'a <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer> ColorPickerOverlay<'a, Message, Renderer>
where 
    Message: 'static + Clone,
    Renderer: 'a + self::Renderer + column::Renderer + button::Renderer + icon_text::Renderer
        + row::Renderer + text::Renderer + text_input::Renderer,
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
            text_input,
            cancel_button,
            submit_button,
            color_bar_focussed,
            ..
        } = state;

        let fake = on_cancel.clone();
        let foo = color.clone();

        ColorPickerOverlay {
            color,
            sat_value_canvas_cache,
            hue_canvas_cache,
            /*text_input: TextInput::new(
                text_input,
                "HEX Color",
                &color_picker::State::color_as_string(&foo),
                move |_s| fake.clone()
            )
            .padding(PADDING as u16)
            .into(),*/
            cancel_button: Button::new(cancel_button, IconText::new(Icon::X).width(Length::Fill))
                .width(Length::Fill)
                .on_press(on_cancel.clone())
                .into(),
            submit_button: Button::new(submit_button, IconText::new(Icon::Check).width(Length::Fill))
                .width(Length::Fill)
                .on_press(on_cancel) // Sending a fake message
                .into(),
            color_bar_focussed,
            on_submit,
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

        if let Event::Mouse(mouse::Event::WheelScrolled {delta}) = event {
            match delta {
                mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                    let move_value = |value: u16, y: f32| {
                        ((value as i32 + y as i32).rem_euclid(360)) as u16
                    };

                    println!("hue: {}, moved: {}", hsv_color.hue, move_value(hsv_color.hue, y));

                    if hue_bounds.contains(cursor_position) {
                        *self.color = Color {
                            a: self.color.a,
                            ..Hsv {
                                hue: move_value(hsv_color.hue, y),
                                ..hsv_color
                            }.into()
                        };
                        color_changed = true;
                        color_changed = true;
                    }
                }
            }
        } else if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if sat_value_bounds.contains(cursor_position) {
                *self.color_bar_focussed = ColorBarFocussed::SatValue;
            }
            if hue_bounds.contains(cursor_position) {
                *self.color_bar_focussed = ColorBarFocussed::Hue;
            }
        } else if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = event {
            *self.color_bar_focussed = ColorBarFocussed::None;
        }

        let calc_percentage_sat = |bounds: &Rectangle, cursor_position: &Point| {
            let min_x = bounds.x;
            let max = bounds.width;

            let value = (cursor_position.x - min_x).max(0.0);
            let value = (value / max).min(1.0);

            value
        };

        let calc_percentage_value = |bounds: &Rectangle, cursor_position: &Point| {
            let min_y = bounds.y;
            let max = bounds.height;

            let value = (cursor_position.y - min_y).max(0.0);
            let value = (value / max).min(1.0);

            value
        };

        let calc_hue = |bounds: &Rectangle, cursor_position: &Point| {
            let min_x = bounds.x;
            let max = bounds.width;

            let value = (cursor_position.x - min_x).max(0.0);
            let value = (value / max).min(1.0);

            (value * 360.0) as u16
        };

        match self.color_bar_focussed {
            ColorBarFocussed::SatValue => {
                *self.color = Color {
                    a: self.color.a,
                    ..Hsv {
                        saturation: calc_percentage_sat(&sat_value_bounds, &cursor_position),
                        value: calc_percentage_value(&sat_value_bounds, &cursor_position),
                        ..hsv_color
                    }.into()
                };
                color_changed = true;
            },
            ColorBarFocussed::Hue => {
                *self.color = Color {
                    a: self.color.a,
                    ..Hsv {
                        hue: calc_hue(&hue_bounds, &cursor_position),
                        ..hsv_color
                    }.into()
                };
                color_changed = true;
            },
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

        if let Event::Mouse(mouse::Event::WheelScrolled {delta}) = event {
            match delta {
                mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                    let move_value = |value: f32, y: f32| {
                        (value * 255.0 + y).min(255.0).max(0.0) / 255.0
                    };

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
                *self.color_bar_focussed = ColorBarFocussed::Red;
            }
            if green_bar_bounds.contains(cursor_position) {
                *self.color_bar_focussed = ColorBarFocussed::Green;
            }
            if blue_bar_bounds.contains(cursor_position) {
                *self.color_bar_focussed = ColorBarFocussed::Blue;
            }
            if alpha_bar_bounds.contains(cursor_position) {
                *self.color_bar_focussed = ColorBarFocussed::Alpha;
            }
        } else if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = event {
            *self.color_bar_focussed = ColorBarFocussed::None;
        }

        let calc_percantage = |bounds: &Rectangle, cursor_position: &Point| {
            let min_x = bounds.x;
            let max = bounds.width;

            let value = (cursor_position.x - min_x).max(0.0);
            let value = (value / max).min(1.0);

            value
        };

        match self.color_bar_focussed {
            ColorBarFocussed::Red => {
                *self.color = Color {
                    r: calc_percantage(&red_bar_bounds, &cursor_position),
                    .. *self.color
                };
                color_changed = true;
            },
            ColorBarFocussed::Green => {
                *self.color = Color {
                    g: calc_percantage(&green_bar_bounds, &cursor_position),
                    .. *self.color
                };
                color_changed = true;
            },
            ColorBarFocussed::Blue => {
                *self.color = Color {
                    b: calc_percantage(&blue_bar_bounds, &cursor_position),
                    .. *self.color
                };
                color_changed = true;
            },
            ColorBarFocussed::Alpha => {
                *self.color = Color {
                    a: calc_percantage(&alpha_bar_bounds, &cursor_position),
                    .. *self.color
                };
                color_changed = true;
            },
            _ => {}
        }

        if color_changed {
            event::Status::Captured
        } else {
            event::Status::Ignored
        }
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for ColorPickerOverlay<'a, Message, Renderer>
where
    Message: 'static + Clone,
    Renderer: 'a + self::Renderer + column::Renderer + button::Renderer + icon_text::Renderer + row::Renderer + text::Renderer + text_input::Renderer,
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
            //.pad(PADDING as f32)
            .width(Length::Fill)
            .height(Length::Fill);

        /*let mut block1_node = Row::<(), Renderer>::new().width(Length::Fill).height(Length::Fill)
            .layout(renderer, &block1_limits);*/

        let mut block1_node = Column::<(), Renderer>::new()
            //.padding(PADDING)
            //.spacing(SPACING)
            .spacing(PADDING)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .height(Length::FillPortion(7))
            )
            .push(
                Row::new()
                    .width(Length::Fill)
                    .height(Length::FillPortion(1))
            )
            .layout(renderer, &block1_limits);

        block1_node.move_to(Point::new(block1_bounds.x + PADDING as f32, block1_bounds.y + PADDING as f32));
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
            .height(Length::Units(text::Renderer::default_size(renderer) + 2 * PADDING))
            .layout(renderer, &text_input_limits);

        let block2_limits = block2_limits.shrink(Size::new(
            0.0,
            cancel_button.bounds().height + text_input.bounds().height + 2.0 * SPACING as f32,
        ));

        // RGBA Colors
        let mut rgba_colors = Column::<(), Renderer>::new();//.spacing(SPACING);
            
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
                            .vertical_alignment(iced_graphics::VerticalAlignment::Center)
                    )
                    .push(
                        Row::new().width(Length::FillPortion(5)).height(Length::Fill)
                    )
                    .push(
                        //Row::new().width(Length::FillPortion(1)).height(Length::Fill)
                        Text::new("XXX")
                            .horizontal_alignment(iced_graphics::HorizontalAlignment::Center)
                            .vertical_alignment(iced_graphics::VerticalAlignment::Center)
                    )
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
        let cancel_limits = block2_limits
            .clone()
            .max_width(((rgba_colors.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32);
            
        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);
            
        let submit_limits = block2_limits
            .clone()
            .max_width(((rgba_colors.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32);

        let mut submit_button = self.submit_button.layout(renderer, &submit_limits);

        cancel_button.move_to(Point::new(
            cancel_button.bounds().x + PADDING as f32,
            cancel_button.bounds().y + rgba_colors.bounds().height + text_input.bounds().height
                + PADDING as f32 + 2.0 * SPACING as f32,
        ));

        submit_button.move_to(Point::new(
            submit_button.bounds().x + rgba_colors.bounds().width - submit_button.bounds().width
                + PADDING as f32,
            submit_button.bounds().y + rgba_colors.bounds().height + text_input.bounds().height
                + PADDING as f32 + 2.0 * SPACING as f32,  
        ));

        let mut block2_node = layout::Node::with_children(
            Size::new(
                rgba_colors.bounds().width + (2.0 * PADDING as f32),
                rgba_colors.bounds().height + text_input.bounds().height
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

        let mut node = layout::Node::with_children(
            Size::new(
                width,
                height,
            ),
            vec![block1_node, block2_node],
        );

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
        clipboard: Option<&dyn Clipboard>
    ) -> event::Status {
        let bounds = layout.bounds();
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
        let text_input_layout = block2_children.next().unwrap();
        /*let text_input_status = self.text_input.on_event(
            event.clone(),
            text_input_layout,
            cursor_position,
            &mut fake_messages,
            renderer,
            clipboard,
        );*/

        fake_messages.clear();

        // ----------- Buttons -------------------------
        let cancel_button_layout = block2_children.next().unwrap();
        let cancel_button_status = self.cancel_button.on_event(
            event.clone(),
            cancel_button_layout,
            cursor_position,
            messages,
            renderer,
            clipboard
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
            messages.push((self.on_submit)(
                self.color.clone()
            ));
        }
        // ----------- Block 2 end ------------------

        /*if color_changed {
            self.color_hex = self.color_as_string();
        }*/

        if  hsv_color_status == event::Status::Captured
            || rgba_color_status == event::Status::Captured {
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
        env: DrawEnvironment<'_, Self::Defaults, Self::Style>,
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
        _env: DrawEnvironment<'_, Self::Defaults, Self::Style>,
        _color: &Color,
        _sat_value_canvas_cache: &canvas::Cache,
        _hue_canvas_cache: &canvas::Cache,
        //_text_input: &Element<'_, Message, Self>,
        _cancel_button: &Element<'_, Message, Self>,
        _submit_button: &Element<'_, Message, Self>,
    ) -> Self::Output {
    }
}

/// The state of the currently focussed area.
#[derive(Debug)]
pub enum ColorBarFocussed {
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