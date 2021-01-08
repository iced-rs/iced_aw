//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: date_picker*
use std::hash::Hash;

use chrono::{Datelike, NaiveDate};
use iced_native::{
    Align, Button, Clipboard, Column, Container, Element, Event,
    Layout, Length, Point, Row, Size, Text, Widget, button,
    column, container, event, layout::{self, Limits}, mouse, overlay, row, text
};

use crate::{graphics::icons::Icon, native::{IconText, date_picker::State, icon_text}};

const PADDING: u16 = 10;
const SPACING: u16 = 15;
const DAY_CELL_PADDING: u16 = 7;
const BUTTON_SPACING: u16 = 5;

/// The overlay of the [`DatePicker`](crate::native::DatePicker).
#[allow(missing_debug_implementations)]
pub struct DatePickerOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + button::Renderer,
{
    date: &'a mut NaiveDate,
    cancel_button: Element<'a, Message, Renderer>,
    submit_button: Element<'a, Message, Renderer>,
    on_submit: &'a Box<dyn Fn(i32, u32, u32) -> Message>,
    position: Point,
    style: &'a <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer> DatePickerOverlay<'a, Message, Renderer>
where 
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + button::Renderer + column::Renderer
        + container::Renderer + icon_text::Renderer + row::Renderer + text::Renderer,
{
    /// Creates a new [`DatePickerOverlay`](DatePickerOverlay) on the given
    /// position.
    pub fn new(
        state: &'a mut State,
        on_cancel: Message,
        on_submit: &'a Box<dyn Fn(i32, u32, u32) -> Message>,
        position: Point,
        style: &'a <Renderer as self::Renderer>::Style,
        //button_style: impl Clone +  Into<<Renderer as button::Renderer>::Style>, // clone not satisfied
    ) -> Self {
        let State {
            date,
            cancel_button,
            submit_button,
            ..
        } = state;

        DatePickerOverlay {
            date,
            cancel_button: Button::new(
                    cancel_button,
                    IconText::new(Icon::X)
                        .width(Length::Fill)
                )
                .width(Length::Fill)
                .on_press(on_cancel.clone())
                //.style(button_style.clone())
                .into(),
            submit_button: Button::new(
                    submit_button,
                    IconText::new(Icon::Check)
                        .width(Length::Fill)
                )
                .width(Length::Fill)
                .on_press(on_cancel) // Sending a fake message
                //.style(button_style)
                .into(),
            on_submit,
            position,
            style: style.into(),
        }
    }

    /// Turn this [`DatePickerOverlay`](DatePickerOverlay) into an overlay
    /// [`Element`](overlay::Element).
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(
            self.position.clone(),
            Box::new(self)
        )
    }

    fn year_as_string(&self) -> String {
        crate::core::date::year_as_string(self.date)
    }

    fn month_as_string(&self) -> String {
        crate::core::date::month_as_string(&self.date)
    }


    /// The event handling for the month / year bar.
    fn on_event_month_year (
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>
    ) -> event::Status {
        let mut children = layout.children();

        let mut status = event::Status::Ignored;

        // ----------- Month ----------------------
        let month_layout = children.next().unwrap();
        let mut month_children = month_layout.children();

        let left_bounds = month_children.next().unwrap().bounds();
        let _center_bounds = month_children.next().unwrap().bounds();
        let right_bounds = month_children.next().unwrap().bounds();
        
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if left_bounds.contains(cursor_position) {
                    *self.date = crate::core::date::pred_month(self.date);
                    status = event::Status::Captured;
                } else if right_bounds.contains(cursor_position) {
                    *self.date = crate::core::date::succ_month(self.date);
                    status = event::Status::Captured;
                }

            },
            _ => {},
        }

        // ----------- Year -----------------------
        let year_layout = children.next().unwrap();
        let mut year_children = year_layout.children();

        let left_bounds = year_children.next().unwrap().bounds();
        let _center_bounds = year_children.next().unwrap().bounds();
        let right_bounds = year_children.next().unwrap().bounds();
        
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if left_bounds.contains(cursor_position) {
                    *self.date = crate::core::date::pred_year(self.date);
                    status = event::Status::Captured;
                } else if right_bounds.contains(cursor_position) {
                    *self.date = crate::core::date::succ_year(self.date);
                    status = event::Status::Captured;
                }

            },
            _ => {},
        }
        
        status
    }

    /// The event handling for the calendar days.
    fn on_event_days (
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>
    ) -> event::Status {
        let mut children = layout.children();

        let _day_labels_layout = children.next().unwrap();

        let mut status = event::Status::Ignored;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                'outer: for (y, row) in children.enumerate() {
                    for (x, label) in row.children().enumerate() {
                        let bounds = label.bounds();
                        if bounds.contains(cursor_position) {
                            let (day, is_in_month) = crate::core::date::position_to_day(
                                x,
                                y,
                                self.date.year(),
                                self.date.month()
                            );

                            // TODO: clean up
                            *self.date = match is_in_month {
                                -1 => crate::core::date::pred_month(self.date).with_day(day as u32).unwrap(),
                                0 => self.date.with_day(day as u32).unwrap(),
                                1 => crate::core::date::succ_month(self.date).with_day(day as u32).unwrap(),
                                _ => panic!("Should not happen")
                            };

                            status = event::Status::Captured;
                            break 'outer;
                        }
                    }
                }
            },
            _ => {}
        }

        status
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for DatePickerOverlay<'a, Message, Renderer>
where 
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + button::Renderer + column::Renderer
        + container::Renderer + icon_text::Renderer + row::Renderer + text::Renderer,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(
            Size::ZERO,
            bounds,
        )
        .pad(PADDING as f32)
        .width(Length::Fill)
        .height(Length::Fill)
        .max_width(300)
        .max_height(300);
        
        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits.clone();
        let cancel_button = self.cancel_button
            .layout(renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(
            0.0,
            cancel_button.bounds().height + SPACING as f32
        ));
        
        // Month/Year
        let font_size = text::Renderer::default_size(renderer) as u32;

        let month_year = Row::<(), Renderer>::new()
            .width(Length::Fill)
            .spacing(SPACING)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                        Row::new() // Left Month arrow
                            .width(Length::Units(font_size as u16))
                            .height(Length::Fill)
                            .max_height(font_size)
                    )
                    .push( // Month
                        Text::new("")
                            .width(Length::Fill)
                            .height(Length::Units(font_size as u16))
                    )
                    .push( // Right Month arrow
                        Row::new()
                            .width(Length::Units(font_size as u16))
                            .height(Length::Fill)
                            .max_height(font_size)
                    )
            )
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                        Row::new() // Left Year arrow
                            .width(Length::Units(font_size as u16))
                            .height(Length::Fill)
                            .max_height(font_size)
                    )
                    .push( // Year
                        Text::new("")
                            .width(Length::Fill)
                            .height(Length::Units(font_size as u16))
                    )
                    .push( // Right Year arrow
                        Row::new()
                            .width(Length::Units(font_size as u16))
                            .height(Length::Fill)
                            .max_height(font_size)
                    )
            );

        let days = Container::<(), Renderer>::new(
            (0..7).into_iter().fold(
                Column::new().height(Length::Fill),
                |column, _y| {
                    column.push((0..7).into_iter().fold(
                        Row::new().height(Length::Fill).padding(DAY_CELL_PADDING),
                        |row, _x| {
                            row.push(
                                Row::new()
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .max_width(font_size)
                                    .max_height(font_size)
                            )
                        }
                    ))
                }
            )
        )
        .height(Length::Fill)
        .center_y();

        let mut col = Column::<(), Renderer>::new()
            .spacing(SPACING)
            .align_items(Align::Center)
            .push(month_year)
            .push(days)
            .layout(renderer, &limits);
        
        col.move_to(Point::new(
            col.bounds().x + PADDING as f32,
            col.bounds().y + PADDING as f32,
        ));        
        

        // Buttons
        let cancel_limits = limits.clone()
            .max_width(((col.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32);

        let mut cancel_button = self.cancel_button
            .layout(renderer, &cancel_limits);
        
        let submit_limits = limits.clone()
            .max_width(((col.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32);

        let mut submit_button = self.submit_button
            .layout(renderer, &submit_limits);

        cancel_button.move_to(Point {
            x: cancel_button.bounds().x + PADDING as f32,
            y: cancel_button.bounds().y + col.bounds().height + PADDING as f32
                + SPACING as f32,
        });

        submit_button.move_to(Point {
            x: submit_button.bounds().x + col.bounds().width - submit_button.bounds().width
                + PADDING as f32,
            y: submit_button.bounds().y + col.bounds().height + PADDING as f32
                + SPACING as f32,
        });

        let mut node = layout::Node::with_children(
            Size::new(
                col.bounds().width + (2.0 * PADDING as f32),
                col.bounds().height + cancel_button.bounds().height + (2.0 * PADDING as f32)
                    + SPACING as f32,
            ),
            vec![col, cancel_button, submit_button],
        );

        node.move_to(Point::new(
            (position.x - node.size().width/2.0).max(0.0),
            (position.y - node.size().height/2.0).max(0.0),
        ));

        node.move_to(Point::new(
            if node.bounds().x + node.bounds().width > bounds.width {
                (node.bounds().x - (node.bounds().width - (bounds.width - node.bounds().x))).max(0.0)
            } else { node.bounds().x },
            //node.bounds().x,
            if node.bounds().y + node.bounds().height > bounds.height {
                (node.bounds().y - (node.bounds().height - (bounds.height - node.bounds().y))).max(0.0)
            } else { node.bounds().y },
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
        let mut children = layout.children();

        let mut date_children = children.next().unwrap().children();

        // ----------- Year/Month----------------------
        let month_year_layout = date_children.next().unwrap();
        let month_year_status = self.on_event_month_year(
            event.clone(),
            month_year_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Days ----------------------
        let days_layout = date_children.next().unwrap().children().next().unwrap();
        let days_status = self.on_event_days(
            event.clone(),
            days_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Buttons ------------------------
        let cancel_button_layout = children.next().unwrap();
        
        let cancel_status = self.cancel_button.on_event(
            event.clone(),
            cancel_button_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        let submit_button_layout = children.next().unwrap();

        let mut fake_messages: Vec<Message> = Vec::new();

        let submit_status = self.submit_button.on_event(
            event,
            submit_button_layout,
            cursor_position,
            //messages,
            &mut fake_messages,
            renderer,
            clipboard,
        );


        if fake_messages.len() > 0 {
            messages.push(
                (self.on_submit)(
                    self.date.year(),
                    self.date.month(),
                    self.date.day()
                )
            );
        }

        month_year_status
            .merge(days_status)
            .merge(cancel_status)
            .merge(submit_status)
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
            defaults,
            cursor_position,
            &self.style,
            &self.date,
            &self.year_as_string(),
            &self.month_as_string(),
            &self.cancel_button,
            &self.submit_button,
            layout,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher, position: Point) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        (position.x as u32).hash(state);
        (position.y as u32).hash(state);
    }
}


/// The renderer of a [`DatePickerOverlay`](DatePickerOverlay).
///
/// Your renderer will need to implement this trait before being
/// able to use a [`DatePicker`](crate::native::DatePicker) in your user
/// interface.
pub trait Renderer: iced_native::Renderer {

    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`DatePickerOverlay`](DatePickerOverlay).
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: Point,
        style_sheet: &Self::Style,
        date: &NaiveDate,
        year_str: &str,
        month_str: &str,
        cancel_button: &Element<'_, Message, Self>,
        submit_button: &Element<'_, Message, Self>,
        layout: Layout<'_>,
    ) -> Self::Output;
}

#[cfg(debug_assertions)]
impl Renderer for iced_native::renderer::Null {
    type Style = ();

    fn draw<Message>(
        &mut self,
        _defaults: &Self::Defaults,
        _cursor_position: Point,
        _style_sheet: &Self::Style,
        _date: &NaiveDate,
        _year_str: &str,
        _month_str: &str,
        _cancel_button: &Element<'_, Message, Self>,
        _submit_button: &Element<'_, Message, Self>,
        _layout: Layout<'_>,
    ) -> Self::Output {}
}