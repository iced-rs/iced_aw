//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*
use std::hash::Hash;

use chrono::{Datelike, Local, NaiveDate};
use iced_native::{
    button, column, container, event, keyboard,
    layout::{self, Limits},
    mouse, overlay, row, text, touch, Align, Button, Clipboard, Column, Container, Element, Event,
    Layout, Length, Padding, Point, Row, Size, Text, Widget,
};

use crate::{
    core::{
        date::{Date, IsInMonth},
        overlay::Position,
        renderer::DrawEnvironment,
    },
    graphics::icons::Icon,
    native::{date_picker, icon_text, IconText},
};

/// The padding around the elements.
const PADDING: u16 = 10;
/// The spacing between the elements.
const SPACING: u16 = 15;
/// The padding of the day cells.
const DAY_CELL_PADDING: u16 = 7;
/// The spacing between the buttons.
const BUTTON_SPACING: u16 = 5;

/// The overlay of the [`DatePicker`](crate::native::DatePicker).
#[allow(missing_debug_implementations)]
pub struct DatePickerOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + button::Renderer,
{
    /// The state of the [`DatePickerOverlay`](DatePickerOverlay).
    state: &'a mut State,
    /// The cancel button of the [`DatePickerOverlay`](DatePickerOverlay).
    cancel_button: Element<'a, Message, Renderer>,
    /// The submit button of the [`DatePickerOverlay`](DatePickerOverlay).
    submit_button: Element<'a, Message, Renderer>,
    /// The function that produces a message when the submit button of the [`DatePickerOverlay`](DatePickerOverlay) is pressed.
    on_submit: &'a dyn Fn(Date) -> Message,
    /// The position of the [`DatePickerOverlay`](DatePickerOverlay).
    position: Point,
    /// The style of teh [`DatePickerOverlay`](DatePickerOverlay).
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
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(self.position, Box::new(self))
    }

    /// String representation of the current year.
    fn year_as_string(&self) -> String {
        crate::core::date::year_as_string(self.state.date)
    }

    /// String representation of the current month.
    fn month_as_string(&self) -> String {
        crate::core::date::month_as_string(self.state.date)
    }

    /// The event handling for the month / year bar.
    fn on_event_month_year(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
        let mut children = layout.children();

        let mut status = event::Status::Ignored;

        // ----------- Month ----------------------
        let month_layout = children
            .next()
            .expect("Native: Layout should have a month layout");
        let mut month_children = month_layout.children();

        let left_bounds = month_children
            .next()
            .expect("Native: Layout should have a left month arrow layout")
            .bounds();
        let _center_bounds = month_children
            .next()
            .expect("Native: Layout should have a center month layout")
            .bounds();
        let right_bounds = month_children
            .next()
            .expect("Native: Layout should have a right month arrow layout")
            .bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if month_layout.bounds().contains(cursor_position) {
                    self.state.focus = Focus::Month;
                }

                if left_bounds.contains(cursor_position) {
                    self.state.date = crate::core::date::pred_month(self.state.date);
                    status = event::Status::Captured;
                } else if right_bounds.contains(cursor_position) {
                    self.state.date = crate::core::date::succ_month(self.state.date);
                    status = event::Status::Captured;
                }
            }
            _ => {}
        }

        // ----------- Year -----------------------
        let year_layout = children
            .next()
            .expect("Native: Layout should have a year layout");
        let mut year_children = year_layout.children();

        let left_bounds = year_children
            .next()
            .expect("Native: Layout should have a left year arrow layout")
            .bounds();
        let _center_bounds = year_children
            .next()
            .expect("Native: Layout should have a center year layout")
            .bounds();
        let right_bounds = year_children
            .next()
            .expect("Native: Layout should have a right year arrow layout")
            .bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if year_layout.bounds().contains(cursor_position) {
                    self.state.focus = Focus::Year;
                }

                if left_bounds.contains(cursor_position) {
                    self.state.date = crate::core::date::pred_year(self.state.date);
                    status = event::Status::Captured;
                } else if right_bounds.contains(cursor_position) {
                    self.state.date = crate::core::date::succ_year(self.state.date);
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
        event: &Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
        let mut children = layout.children();

        let _day_labels_layout = children
            .next()
            .expect("Native: Layout should have a day label layout");

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

                            self.state.date = match is_in_month {
                                IsInMonth::Previous => {
                                    crate::core::date::pred_month(self.state.date)
                                        .with_day(day as u32)
                                        .expect("Previous month with day should be valid")
                                }
                                IsInMonth::Same => self
                                    .state
                                    .date
                                    .with_day(day as u32)
                                    .expect("Same month with day should be valid"),
                                IsInMonth::Next => crate::core::date::succ_month(self.state.date)
                                    .with_day(day as u32)
                                    .expect("Succeeding month with day should be valid"),
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

    /// The event handling for the keyboard input.
    fn on_event_keyboard(
        &mut self,
        event: &Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
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
                    Focus::Month => match key_code {
                        keyboard::KeyCode::Left => {
                            self.state.date = crate::core::date::pred_month(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Right => {
                            self.state.date = crate::core::date::succ_month(self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    Focus::Year => match key_code {
                        keyboard::KeyCode::Left => {
                            self.state.date = crate::core::date::pred_year(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Right => {
                            self.state.date = crate::core::date::succ_year(self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    Focus::Day => match key_code {
                        keyboard::KeyCode::Left => {
                            self.state.date = crate::core::date::pred_day(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Right => {
                            self.state.date = crate::core::date::succ_day(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Up => {
                            self.state.date = crate::core::date::pred_week(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::KeyCode::Down => {
                            self.state.date = crate::core::date::succ_week(self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    _ => {}
                },
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
    #[allow(clippy::too_many_lines)]
    fn layout(
        &self,
        renderer: &Renderer,
        bounds: iced_graphics::Size,
        position: Point,
    ) -> iced_native::layout::Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .pad(Padding::from(PADDING))
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(300)
            .max_height(300);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(
            0.0,
            cancel_button.bounds().height + f32::from(SPACING),
        ));

        // Month/Year
        let font_size = u32::from(text::Renderer::default_size(renderer));

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
            col.bounds().x + f32::from(PADDING),
            col.bounds().y + f32::from(PADDING),
        ));

        // Buttons
        let cancel_limits = limits
            .max_width(((col.bounds().width / 2.0) - f32::from(BUTTON_SPACING)).max(0.0) as u32);

        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let submit_limits = limits
            .max_width(((col.bounds().width / 2.0) - f32::from(BUTTON_SPACING)).max(0.0) as u32);

        let mut submit_button = self.submit_button.layout(renderer, &submit_limits);

        cancel_button.move_to(Point {
            x: cancel_button.bounds().x + f32::from(PADDING),
            y: cancel_button.bounds().y
                + col.bounds().height
                + f32::from(PADDING)
                + f32::from(SPACING),
        });

        submit_button.move_to(Point {
            x: submit_button.bounds().x + col.bounds().width - submit_button.bounds().width
                + f32::from(PADDING),
            y: submit_button.bounds().y
                + col.bounds().height
                + f32::from(PADDING)
                + f32::from(SPACING),
        });

        let mut node = layout::Node::with_children(
            Size::new(
                col.bounds().width + (2.0 * f32::from(PADDING)),
                col.bounds().height
                    + cancel_button.bounds().height
                    + (2.0 * f32::from(PADDING))
                    + f32::from(SPACING),
            ),
            vec![col, cancel_button, submit_button],
        );

        node.center_and_bounce(position, bounds);

        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        if let event::Status::Captured = self.on_event_keyboard(
            &event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        ) {
            return event::Status::Captured;
        }

        let mut children = layout.children();

        let mut date_children = children
            .next()
            .expect("Native: Layout should have date children")
            .children();

        // ----------- Year/Month----------------------
        let month_year_layout = date_children
            .next()
            .expect("Native: Layout should have a month/year layout");
        let month_year_status = self.on_event_month_year(
            &event,
            month_year_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Days ----------------------
        let days_layout = date_children
            .next()
            .expect("Native: Layout should have a days table parent")
            .children()
            .next()
            .expect("Native: Layout should have a days table layout");
        let days_status = self.on_event_days(
            &event,
            days_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        // ----------- Buttons ------------------------
        let cancel_button_layout = children
            .next()
            .expect("Native: Layout should have a cancel button layout for a DatePicker");

        let cancel_status = self.cancel_button.on_event(
            event.clone(),
            cancel_button_layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        );

        let submit_button_layout = children
            .next()
            .expect("Native: Layout should have a submit button layout for a DatePicker");

        let mut fake_messages: Vec<Message> = Vec::new();

        let submit_status = self.submit_button.on_event(
            event,
            submit_button_layout,
            cursor_position,
            renderer,
            clipboard,
            &mut fake_messages,
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
                style_sheet: self.style,
                viewport: None,
                focus: self.state.focus,
            },
            self.state.date,
            &self.year_as_string(),
            &self.month_as_string(),
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
        date: NaiveDate,
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
        _date: NaiveDate,
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
    /// The selected date of the [`DatePickerOverlay`](DatePickerOverlay).
    pub(crate) date: NaiveDate,
    /// The focus of the [`DatePickerOverlay`](DatePickerOverlay).
    pub(crate) focus: Focus,
    /// The previously pressed keyboard modifiers.
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
    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Overlay => Self::Month,
            Self::Month => Self::Year,
            Self::Year => Self::Day,
            Self::Day => Self::Cancel,
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
            Self::Month => Self::Overlay,
            Self::Year => Self::Month,
            Self::Day => Self::Year,
            Self::Cancel => Self::Day,
            Self::Submit => Self::Cancel,
        }
    }
}

impl Default for Focus {
    fn default() -> Self {
        Self::None
    }
}
