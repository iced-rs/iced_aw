//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: date_picker*
use std::hash::Hash;

use chrono::{Datelike, Local, NaiveDate};
use iced_native::{
    button, column, container, event, keyboard,
    layout::{self, Limits},
    mouse, overlay, row, text, touch, Align, Button, Clipboard, Column, Container, Element, Event,
    Layout, Length, Point, Row, Size, Text, Widget,
};

use crate::{
    core::{date::Date, renderer::DrawEnvironment},
    graphics::icons::Icon,
    native::{date_picker, icon_text, IconText},
};

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
    state: &'a mut State,
    cancel_button: Element<'a, Message, Renderer>,
    submit_button: Element<'a, Message, Renderer>,
    on_submit: &'a dyn Fn(Date) -> Message,
    position: Point,
    style: &'a <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer> DatePickerOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a
        + self::Renderer
        + button::Renderer
        + column::Renderer
        + container::Renderer
        + icon_text::Renderer
        + row::Renderer
        + text::Renderer,
{
    /// Creates a new [`DatePickerOverlay`](DatePickerOverlay) on the given
    /// position.
    pub fn new(
        state: &'a mut date_picker::State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Date) -> Message,
        position: Point,
        style: &'a <Renderer as self::Renderer>::Style,
        //button_style: impl Clone +  Into<<Renderer as button::Renderer>::Style>, // clone not satisfied
    ) -> Self {
        let date_picker::State {
            overlay_state,
            cancel_button,
            submit_button,
            ..
        } = state;

        DatePickerOverlay {
            state: overlay_state,
            cancel_button: Button::new(cancel_button, IconText::new(Icon::X).width(Length::Fill))
                .width(Length::Fill)
                .on_press(on_cancel.clone())
                //.style(button_style.clone())
                .into(),
            submit_button: Button::new(
                submit_button,
                IconText::new(Icon::Check).width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel) // Sending a fake message
            //.style(button_style)
            .into(),
            on_submit,
            position,
            style,
        }
    }

    /// Turn this [`DatePickerOverlay`](DatePickerOverlay) into an overlay
    /// [`Element`](overlay::Element).
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(self.position, Box::new(self))
    }

    fn year_as_string(&self) -> String {
        crate::core::date::year_as_string(&self.state.date)
    }

    fn month_as_string(&self) -> String {
        crate::core::date::month_as_string(&self.state.date)
    }

    /// The event handling for the month / year bar.
    fn on_event_month_year(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
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
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if month_layout.bounds().contains(cursor_position) {
                    self.state.focus = Focus::Month;
                }

                if left_bounds.contains(cursor_position) {
                    self.state.date = crate::core::date::pred_month(&self.state.date);
                    status = event::Status::Captured;
                } else if right_bounds.contains(cursor_position) {
                    self.state.date = crate::core::date::succ_month(&self.state.date);
                    status = event::Status::Captured;
                }
            }
            _ => {}
        }

        // ----------- Year -----------------------
        let year_layout = children.next().unwrap();
        let mut year_children = year_layout.children();

        let left_bounds = year_children.next().unwrap().bounds();
        let _center_bounds = year_children.next().unwrap().bounds();
        let right_bounds = year_children.next().unwrap().bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if year_layout.bounds().contains(cursor_position) {
                    self.state.focus = Focus::Year;
                }

                if left_bounds.contains(cursor_position) {
                    self.state.date = crate::core::date::pred_year(&self.state.date);
                    status = event::Status::Captured;
                } else if right_bounds.contains(cursor_position) {
                    self.state.date = crate::core::date::succ_year(&self.state.date);
                    status = event::Status::Captured;
                }
            }
            _ => {}
        }

        status
    }

    /// The event handling for the calendar days.
    fn on_event_days(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut children = layout.children();

        let _day_labels_layout = children.next().unwrap();

        let mut status = event::Status::Ignored;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if layout.bounds().contains(cursor_position) {
                    self.state.focus = Focus::Day;
                }

                'outer: for (y, row) in children.enumerate() {
                    for (x, label) in row.children().enumerate() {
                        let bounds = label.bounds();
                        if bounds.contains(cursor_position) {
                            let (day, is_in_month) = crate::core::date::position_to_day(
                                x,
                                y,
                                self.state.date.year(),
                                self.state.date.month(),
                            );

                            // TODO: clean up
                            self.state.date = match is_in_month {
                                -1 => crate::core::date::pred_month(&self.state.date)
                                    .with_day(day as u32)
                                    .unwrap(),
                                0 => self.state.date.with_day(day as u32).unwrap(),
                                1 => crate::core::date::succ_month(&self.state.date)
                                    .with_day(day as u32)
                                    .unwrap(),
                                _ => panic!("Should not happen"),
                            };

                            status = event::Status::Captured;
                            break 'outer;
                        }
                    }
                }
            }
            _ => {}
        }

        status
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
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
            let mut status = event::Status::Ignored;

            match key_code {
                keyboard::KeyCode::Tab => {
                    if self.state.keyboard_modifiers.shift {
                        self.state.focus = self.state.focus.previous();
                    } else {
                        self.state.focus = self.state.focus.next();
                    }
                }
                _ => match self.state.focus {
                    Focus::Overlay => {}
                    Focus::Month => match key_code {
                        keyboard::KeyCode::Left => {
                            self.state.date = crate::core::date::pred_month(&self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Right => {
                            self.state.date = crate::core::date::succ_month(&self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    Focus::Year => match key_code {
                        keyboard::KeyCode::Left => {
                            self.state.date = crate::core::date::pred_year(&self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Right => {
                            self.state.date = crate::core::date::succ_year(&self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    Focus::Day => match key_code {
                        keyboard::KeyCode::Left => {
                            self.state.date = crate::core::date::pred_day(&self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Right => {
                            self.state.date = crate::core::date::succ_day(&self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Up => {
                            self.state.date = crate::core::date::pred_week(&self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Down => {
                            self.state.date = crate::core::date::succ_week(&self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    Focus::Cancel => {}
                    Focus::Submit => {}
                    _ => {}
                },
            }

            status
        } else if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
            self.state.keyboard_modifiers = modifiers;
            event::Status::Ignored
        } else {
            event::Status::Ignored
        }
    }
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer>
    for DatePickerOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a
        + self::Renderer
        + button::Renderer
        + column::Renderer
        + container::Renderer
        + icon_text::Renderer
        + row::Renderer
        + text::Renderer,
{
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .pad(PADDING as f32)
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(300)
            .max_height(300);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(
            0.0,
            cancel_button.bounds().height + SPACING as f32,
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
                            .max_height(font_size),
                    )
                    .push(
                        // Month
                        Text::new("")
                            .width(Length::Fill)
                            .height(Length::Units(font_size as u16)),
                    )
                    .push(
                        // Right Month arrow
                        Row::new()
                            .width(Length::Units(font_size as u16))
                            .height(Length::Fill)
                            .max_height(font_size),
                    ),
            )
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                        Row::new() // Left Year arrow
                            .width(Length::Units(font_size as u16))
                            .height(Length::Fill)
                            .max_height(font_size),
                    )
                    .push(
                        // Year
                        Text::new("")
                            .width(Length::Fill)
                            .height(Length::Units(font_size as u16)),
                    )
                    .push(
                        // Right Year arrow
                        Row::new()
                            .width(Length::Units(font_size as u16))
                            .height(Length::Fill)
                            .max_height(font_size),
                    ),
            );

        let days = Container::<(), Renderer>::new((0..7).into_iter().fold(
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
                                .max_height(font_size),
                        )
                    },
                ))
            },
        ))
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
        let cancel_limits = limits
            .clone()
            .max_width(((col.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32);

        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let submit_limits = limits
            .clone()
            .max_width(((col.bounds().width / 2.0) - BUTTON_SPACING as f32).max(0.0) as u32);

        let mut submit_button = self.submit_button.layout(renderer, &submit_limits);

        cancel_button.move_to(Point {
            x: cancel_button.bounds().x + PADDING as f32,
            y: cancel_button.bounds().y + col.bounds().height + PADDING as f32 + SPACING as f32,
        });

        submit_button.move_to(Point {
            x: submit_button.bounds().x + col.bounds().width - submit_button.bounds().width
                + PADDING as f32,
            y: submit_button.bounds().y + col.bounds().height + PADDING as f32 + SPACING as f32,
        });

        let mut node = layout::Node::with_children(
            Size::new(
                col.bounds().width + (2.0 * PADDING as f32),
                col.bounds().height
                    + cancel_button.bounds().height
                    + (2.0 * PADDING as f32)
                    + SPACING as f32,
            ),
            vec![col, cancel_button, submit_button],
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
            return event::Status::Captured;
        }

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

        if !fake_messages.is_empty() {
            messages.push((self.on_submit)(self.state.date.into()));
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
            DrawEnvironment {
                defaults,
                layout,
                cursor_position,
                style_sheet: &self.style,
                viewport: None,
                focus: self.state.focus,
            },
            &self.state.date,
            &self.year_as_string(),
            &self.month_as_string(),
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
        env: DrawEnvironment<'_, Self::Defaults, Self::Style, Focus>,
        date: &NaiveDate,
        year_str: &str,
        month_str: &str,
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
        _date: &NaiveDate,
        _year_str: &str,
        _month_str: &str,
        _cancel_button: &Element<'_, Message, Self>,
        _submit_button: &Element<'_, Message, Self>,
    ) -> Self::Output {
    }
}

/// The state of the [`DatePickerOverlay`](DatePickerOverlay).
#[derive(Debug)]
pub struct State {
    pub(crate) date: NaiveDate,
    pub(crate) focus: Focus,
    pub(crate) keyboard_modifiers: keyboard::Modifiers,
}

impl Default for State {
    fn default() -> Self {
        Self {
            date: Local::today().naive_local(),
            focus: Focus::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
        }
    }
}

/// An enumeration of all focusable elements of the [`DatePickerOverlay`](DatePickerOverlay).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Focus {
    /// Nothing is in focus.
    None,

    /// The overlay itself is in focus.
    Overlay,

    /// The month area is in focus.
    Month,

    /// The year area is in focus.
    Year,

    /// The day area is in focus.
    Day,

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
            Focus::Overlay => Focus::Month,
            Focus::Month => Focus::Year,
            Focus::Year => Focus::Day,
            Focus::Day => Focus::Cancel,
            Focus::Cancel => Focus::Submit,
            Focus::Submit => Focus::Overlay,
        }
    }

    /// Gets the previous focusable element.
    pub fn previous(self) -> Self {
        match self {
            Focus::None => Focus::None,
            Focus::Overlay => Focus::Submit,
            Focus::Month => Focus::Overlay,
            Focus::Year => Focus::Month,
            Focus::Day => Focus::Year,
            Focus::Cancel => Focus::Day,
            Focus::Submit => Focus::Cancel,
        }
    }
}

impl Default for Focus {
    fn default() -> Self {
        Focus::None
    }
}
