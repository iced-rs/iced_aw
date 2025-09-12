//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*

use crate::iced_aw_font::advanced_text::{cancel, left_open, ok, right_open};
use crate::{
    core::{
        date::{Date, IsInMonth},
        overlay::Position,
    },
    date_picker,
    style::{date_picker::Style, style_state::StyleState, Status},
};
use chrono::{Datelike, Local, NaiveDate};
use iced_core::{
    alignment::{Horizontal, Vertical},
    event, keyboard,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer,
    text::Renderer as _,
    touch,
    widget::tree::Tree,
    Alignment, Border, Clipboard, Color, Element, Event, Layout, Length, Overlay, Padding, Pixels,
    Point, Rectangle, Renderer as _, Shadow, Shell, Size, Widget,
};
use iced_widget::{
    text::{self, Wrapping},
    Button, Column, Container, Renderer, Row, Text,
};
use std::collections::HashMap;

/// The padding around the elements.
const PADDING: Padding = Padding::new(10.0);
/// The spacing between the elements.
const SPACING: Pixels = Pixels(15.0);
/// The padding of the day cells.
const DAY_CELL_PADDING: Padding = Padding::new(7.0);
/// The spacing between the buttons.
const BUTTON_SPACING: Pixels = Pixels(5.0);

/// The overlay of the [`DatePicker`](crate::widget::DatePicker).
#[allow(missing_debug_implementations)]
pub struct DatePickerOverlay<'a, 'b, Message, Theme>
where
    Message: Clone,
    Theme: crate::style::date_picker::Catalog + iced_widget::button::Catalog,
    'b: 'a,
{
    /// The state of the [`DatePickerOverlay`].
    state: &'a mut State,
    /// The cancel button of the [`DatePickerOverlay`].
    cancel_button: Button<'a, Message, Theme, Renderer>,
    /// The submit button of the [`DatePickerOverlay`].
    submit_button: Button<'a, Message, Theme, Renderer>,
    /// The function that produces a message when the submit button of the [`DatePickerOverlay`] is pressed.
    on_submit: &'a dyn Fn(Date) -> Message,
    /// The position of the [`DatePickerOverlay`].
    position: Point,
    /// The style of the [`DatePickerOverlay`].
    class: &'a <Theme as crate::style::date_picker::Catalog>::Class<'b>,
    /// The reference to the tree holding the state of this overlay.
    tree: &'a mut Tree,
    /// The font size of text and icons in the [`DatePickerOverlay`]
    font_size: Pixels,
    viewport: Rectangle,
}

impl<'a, 'b, Message, Theme> DatePickerOverlay<'a, 'b, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a
        + crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog
        + iced_widget::container::Catalog,
    'b: 'a,
{
    #[allow(clippy::too_many_arguments)]
    /// Creates a new [`DatePickerOverlay`] on the given position.
    pub fn new(
        state: &'a mut date_picker::State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Date) -> Message,
        position: Point,
        class: &'a <Theme as crate::style::date_picker::Catalog>::Class<'b>,
        tree: &'a mut Tree,
        //button_style: impl Clone +  Into<<Renderer as button::Renderer>::Style>, // clone not satisfied
        font_size: Pixels,
        viewport: Rectangle,
    ) -> Self {
        let date_picker::State { overlay_state } = state;

        let (cancel_content, cancel_font, _cancel_shaping) = cancel();
        let (submit_content, submit_font, _submit_shaping) = ok();
        DatePickerOverlay {
            state: overlay_state,
            cancel_button: Button::new(
                text::Text::new(cancel_content)
                    .font(cancel_font)
                    .size(font_size)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel.clone()),
            submit_button: Button::new(
                text::Text::new(submit_content)
                    .font(submit_font)
                    .size(font_size)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel), // Sending a fake message
            on_submit,
            position,
            class,
            tree,
            font_size,
            viewport,
        }
    }

    /// Turn this [`DatePickerOverlay`] into an overlay [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(self))
    }

    /// String representation of the current month and year.
    fn date_as_string(&self) -> String {
        crate::core::date::date_as_string(self.state.date)
    }

    /// The event handling for the month / year bar.
    fn on_event_month_year(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        shell: &mut Shell<Message>,
    ) {
        let mut children = layout.children();

        // ----------- Month ----------------------
        let month_layout = children
            .next()
            .expect("widget: Layout should have a month layout");
        let mut month_children = month_layout.children();

        let left_bounds = month_children
            .next()
            .expect("widget: Layout should have a left month arrow layout")
            .bounds();
        let _center_bounds = month_children
            .next()
            .expect("widget: Layout should have a center month layout")
            .bounds();
        let right_bounds = month_children
            .next()
            .expect("widget: Layout should have a right month arrow layout")
            .bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(month_layout.bounds()) {
                    self.state.focus = Focus::Month;
                    shell.request_redraw();
                }

                if cursor.is_over(left_bounds) {
                    self.state.date = crate::core::date::pred_month(self.state.date);
                    shell.capture_event();
                    shell.request_redraw();
                } else if cursor.is_over(right_bounds) {
                    self.state.date = crate::core::date::succ_month(self.state.date);
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if month_layout.bounds().contains(*position)
                    || left_bounds.contains(*position)
                    || right_bounds.contains(*position)
                {
                    self.state.day_mouse_over = None;
                    shell.request_redraw();
                }
            }
            _ => {}
        }

        // ----------- Year -----------------------
        let year_layout = children
            .next()
            .expect("widget: Layout should have a year layout");
        let mut year_children = year_layout.children();

        let left_bounds = year_children
            .next()
            .expect("widget: Layout should have a left year arrow layout")
            .bounds();
        let _center_bounds = year_children
            .next()
            .expect("widget: Layout should have a center year layout")
            .bounds();
        let right_bounds = year_children
            .next()
            .expect("widget: Layout should have a right year arrow layout")
            .bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(year_layout.bounds()) {
                    self.state.focus = Focus::Year;
                    shell.request_redraw();
                }

                if cursor.is_over(left_bounds) {
                    self.state.date = crate::core::date::pred_year(self.state.date);
                    shell.capture_event();
                    shell.request_redraw();
                } else if cursor.is_over(right_bounds) {
                    self.state.date = crate::core::date::succ_year(self.state.date);
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if year_layout.bounds().contains(*position)
                    || left_bounds.contains(*position)
                    || right_bounds.contains(*position)
                {
                    self.state.day_mouse_over = None;
                    shell.request_redraw();
                }
            }
            _ => {}
        }
    }

    /// The event handling for the calendar days.
    fn on_event_days(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        shell: &mut Shell<Message>,
    ) {
        let mut children = layout.children();

        let _day_labels_layout = children
            .next()
            .expect("widget: Layout should have a day label layout");

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    self.state.focus = Focus::Day;
                    shell.request_redraw();
                }

                'outer: for (y, row) in children.enumerate() {
                    for (x, label) in row.children().enumerate() {
                        let bounds = label.bounds();
                        if cursor.is_over(bounds) {
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

                            shell.capture_event();
                            shell.request_redraw();
                            break 'outer;
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if cursor.is_over(layout.bounds()) {
                    self.state.focus = Focus::Day;
                    shell.request_redraw();
                }

                self.state.day_mouse_over = None;

                'outer: for (y, row) in children.enumerate() {
                    for (x, label) in row.children().enumerate() {
                        let bounds = label.bounds();

                        if bounds.contains(*position) {
                            self.state.day_mouse_over = Some((y, x));
                            shell.capture_event();
                            shell.request_redraw();
                            break 'outer;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// The event handling for the keyboard input.
    fn on_event_keyboard(&mut self, event: &Event) -> event::Status {
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
            let mut status = event::Status::Ignored;

            match key.as_ref() {
                keyboard::Key::Named(keyboard::key::Named::Tab) => {
                    if self.state.keyboard_modifiers.shift() {
                        self.state.focus = self.state.focus.previous();
                    } else {
                        self.state.focus = self.state.focus.next();
                    }
                }
                keyboard::Key::Named(k) => match self.state.focus {
                    Focus::Month => match k {
                        keyboard::key::Named::ArrowLeft => {
                            self.state.date = crate::core::date::pred_month(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::key::Named::ArrowRight => {
                            self.state.date = crate::core::date::succ_month(self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    Focus::Year => match k {
                        keyboard::key::Named::ArrowLeft => {
                            self.state.date = crate::core::date::pred_year(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::key::Named::ArrowRight => {
                            self.state.date = crate::core::date::succ_year(self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    Focus::Day => match k {
                        keyboard::key::Named::ArrowLeft => {
                            self.state.date = crate::core::date::pred_day(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::key::Named::ArrowRight => {
                            self.state.date = crate::core::date::succ_day(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::key::Named::ArrowUp => {
                            self.state.date = crate::core::date::pred_week(self.state.date);
                            status = event::Status::Captured;
                        }
                        keyboard::key::Named::ArrowDown => {
                            self.state.date = crate::core::date::succ_week(self.state.date);
                            status = event::Status::Captured;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
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
    for DatePickerOverlay<'a, 'b, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a
        + crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog
        + iced_widget::container::Catalog,
    'b: 'a,
{
    #[allow(clippy::too_many_lines)]
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .shrink(PADDING)
            .width(Length::Shrink)
            .height(Length::Shrink);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button =
            self.cancel_button
                .layout(&mut self.tree.children[0], renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(0.0, cancel_button.bounds().height + SPACING.0));

        // Month/Year
        let font_size = self.font_size;

        let (left_content, left_font, _left_shaping) = left_open();
        let (right_content, right_font, _right_shaping) = right_open();

        let month_year = Row::<Message, Theme, Renderer>::new()
            .width(Length::Shrink)
            .spacing(SPACING)
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .spacing(SPACING)
                    .align_y(Alignment::Center)
                    .push(
                        // Left Month arrow
                        Container::new(
                            Text::new(&left_content)
                                .size(font_size.0 + 1.0)
                                .font(left_font),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    )
                    .push(
                        // Month
                        Text::new("September").size(font_size).width(Length::Shrink),
                    )
                    .push(
                        // Right Month arrow
                        Container::new(
                            Text::new(&right_content)
                                .size(font_size.0 + 1.0)
                                .font(right_font),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    ),
            )
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .spacing(SPACING)
                    .align_y(Alignment::Center)
                    .push(
                        // Left Year arrow
                        Container::new(
                            Text::new(&left_content)
                                .size(font_size.0 + 1.0)
                                .font(left_font),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    )
                    .push(
                        // Year
                        Text::new("9999").size(font_size).width(Length::Shrink),
                    )
                    .push(
                        // Right Year arrow
                        Container::new(
                            Text::new(&right_content)
                                .size(font_size.0 + 1.0)
                                .font(right_font),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    ),
            );

        let days = Container::<Message, Theme, Renderer>::new((0..7).fold(
            Column::new().width(Length::Shrink).height(Length::Shrink),
            |column, _y| {
                column.push(
                    (0..7).fold(
                        Row::new()
                            .height(Length::Shrink)
                            .width(Length::Shrink)
                            .spacing(SPACING),
                        |row, _x| {
                            row.push(
                                Container::new(Row::new().push(Text::new("31").size(font_size)))
                                    .width(Length::Shrink)
                                    .height(Length::Shrink)
                                    .padding(DAY_CELL_PADDING),
                            )
                        },
                    ),
                )
            },
        ))
        .width(Length::Shrink)
        .height(Length::Shrink)
        .center_y(Length::Shrink);

        let col = Column::<Message, Theme, Renderer>::new()
            .spacing(SPACING)
            .align_x(Alignment::Center)
            .push(month_year)
            .push(days);

        let mut element: Element<Message, Theme, Renderer> = Element::new(col);
        let col_tree = if let Some(child_tree) = self.tree.children.get_mut(2) {
            child_tree.diff(element.as_widget_mut());
            child_tree
        } else {
            let child_tree = Tree::new(element.as_widget());
            self.tree.children.insert(2, child_tree);
            &mut self.tree.children[2]
        };

        let mut col = element.as_widget_mut().layout(col_tree, renderer, &limits);
        let col_bounds = col.bounds();
        col = col.move_to(Point::new(
            col_bounds.x + PADDING.left,
            col_bounds.y + PADDING.top,
        ));

        // Buttons
        let cancel_limits =
            limits.max_width(((col.bounds().width / 2.0) - BUTTON_SPACING.0).max(0.0));

        let mut cancel_button =
            self.cancel_button
                .layout(&mut self.tree.children[0], renderer, &cancel_limits);

        let submit_limits =
            limits.max_width(((col.bounds().width / 2.0) - BUTTON_SPACING.0).max(0.0));

        let mut submit_button =
            self.submit_button
                .layout(&mut self.tree.children[1], renderer, &submit_limits);

        let cancel_bounds = cancel_button.bounds();
        cancel_button = cancel_button.move_to(Point {
            x: cancel_bounds.x + PADDING.left,
            y: cancel_bounds.y + col.bounds().height + PADDING.top + SPACING.0,
        });

        let submit_bounds = submit_button.bounds();
        submit_button = submit_button.move_to(Point {
            x: submit_bounds.x + col.bounds().width - submit_bounds.width + PADDING.left,
            y: submit_bounds.y + col.bounds().height + PADDING.top + SPACING.0,
        });

        let mut node = Node::with_children(
            Size::new(
                col.bounds().width + PADDING.horizontal(),
                col.bounds().height
                    + cancel_button.bounds().height
                    + PADDING.vertical()
                    + SPACING.0,
            ),
            vec![col, cancel_button, submit_button],
        );
        node.center_and_bounce(self.position, bounds);
        node
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) {
        if event::Status::Captured == self.on_event_keyboard(event) {
            shell.capture_event();
            shell.request_redraw();
            return;
        }

        let mut children = layout.children();

        let mut date_children = children
            .next()
            .expect("widget: Layout should have date children")
            .children();

        // ----------- Year/Month----------------------
        let month_year_layout = date_children
            .next()
            .expect("widget: Layout should have a month/year layout");
        self.on_event_month_year(event, month_year_layout, cursor, shell);

        // ----------- Days ----------------------
        let days_layout = date_children
            .next()
            .expect("widget: Layout should have a days table parent")
            .children()
            .next()
            .expect("widget: Layout should have a days table layout");
        self.on_event_days(event, days_layout, cursor, shell);

        // ----------- Buttons ------------------------
        let cancel_button_layout = children
            .next()
            .expect("widget: Layout should have a cancel button layout for a DatePicker");

        if cursor.is_over(cancel_button_layout.bounds()) {
            self.state.day_mouse_over = None;
        }

        self.cancel_button.update(
            &mut self.tree.children[0],
            event,
            cancel_button_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );

        let submit_button_layout = children
            .next()
            .expect("widget: Layout should have a submit button layout for a DatePicker");

        if cursor.is_over(submit_button_layout.bounds()) {
            self.state.day_mouse_over = None;
        }

        let mut fake_messages: Vec<Message> = Vec::new();

        self.submit_button.update(
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
            shell.publish((self.on_submit)(self.state.date.into()));
            shell.capture_event();
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mouse_interaction = mouse::Interaction::default();

        let mut children = layout.children();
        let mut date_children = children
            .next()
            .expect("Graphics: Layout should have a date layout")
            .children();

        // Month and year mouse interaction
        let month_year_layout = date_children
            .next()
            .expect("Graphics: Layout should have a month/year layout");
        let mut month_year_children = month_year_layout.children();
        let month_layout = month_year_children
            .next()
            .expect("Graphics: Layout should have a month layout");
        let year_layout = month_year_children
            .next()
            .expect("Graphics: Layout should have a year layout");

        let f = |layout: Layout<'_>| {
            let mut children = layout.children();

            let left_bounds = children
                .next()
                .expect("Graphics: Layout should have a left arrow layout")
                .bounds();
            let _center = children.next();
            let right_bounds = children
                .next()
                .expect("Graphics: Layout should have a right arrow layout")
                .bounds();

            let left_arrow_hovered = cursor.is_over(left_bounds);
            let right_arrow_hovered = cursor.is_over(right_bounds);

            if left_arrow_hovered || right_arrow_hovered {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            }
        };

        let month_mouse_interaction = f(month_layout);
        let year_mouse_interaction = f(year_layout);

        // Days
        let days_layout = date_children
            .next()
            .expect("Graphics: Layout should have a days layout parent")
            .children()
            .next()
            .expect("Graphics: Layout should have a days layout");
        let mut days_children = days_layout.children();
        let _day_labels_layout = days_children.next();

        let mut table_mouse_interaction = mouse::Interaction::default();

        for row in days_children {
            for label in row.children() {
                let bounds = label.bounds();

                let mouse_over = cursor.is_over(bounds);
                if mouse_over {
                    table_mouse_interaction =
                        table_mouse_interaction.max(mouse::Interaction::Pointer);
                }
            }
        }

        // Buttons
        let cancel_button_layout = children
            .next()
            .expect("Graphics: Layout should have a cancel button layout for a DatePicker");

        let cancel_button_mouse_interaction = self.cancel_button.mouse_interaction(
            &self.tree.children[0],
            cancel_button_layout,
            cursor,
            &self.viewport,
            renderer,
        );

        let submit_button_layout = children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a DatePicker");

        let submit_button_mouse_interaction = self.submit_button.mouse_interaction(
            &self.tree.children[1],
            submit_button_layout,
            cursor,
            &self.viewport,
            renderer,
        );

        mouse_interaction
            .max(month_mouse_interaction)
            .max(year_mouse_interaction)
            .max(table_mouse_interaction)
            .max(cancel_button_mouse_interaction)
            .max(submit_button_mouse_interaction)
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
        let mut date_children = children
            .next()
            .expect("Graphics: Layout should have a date layout")
            .children();
        let cursor_position = cursor.position().unwrap_or_default();
        let mut style_sheet: HashMap<StyleState, Style> = HashMap::new();
        let _ = style_sheet.insert(
            StyleState::Active,
            crate::style::date_picker::Catalog::style(theme, self.class, Status::Active),
        );
        let _ = style_sheet.insert(
            StyleState::Selected,
            crate::style::date_picker::Catalog::style(theme, self.class, Status::Selected),
        );
        let _ = style_sheet.insert(
            StyleState::Hovered,
            crate::style::date_picker::Catalog::style(theme, self.class, Status::Hovered),
        );
        let _ = style_sheet.insert(
            StyleState::Focused,
            crate::style::date_picker::Catalog::style(theme, self.class, Status::Focused),
        );

        let style_state = if self.state.focus == Focus::Overlay {
            StyleState::Focused
        } else if cursor.is_over(bounds) {
            StyleState::Hovered
        } else {
            StyleState::Active
        };

        // Background
        if (bounds.width > 0.) && (bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style_sheet[&style_state].border_radius.into(),
                        width: style_sheet[&style_state].border_width,
                        color: style_sheet[&style_state].border_color,
                    },
                    ..renderer::Quad::default()
                },
                style_sheet[&style_state].background,
            );
        }

        // ----------- Year/Month----------------------
        let month_year_layout = date_children
            .next()
            .expect("Graphics: Layout should have a month/year layout");

        month_year(
            renderer,
            month_year_layout,
            &self.date_as_string(),
            cursor_position,
            &style_sheet,
            self.state.focus,
            self.font_size,
        );

        // ----------- Days ---------------------------
        let days_layout = date_children
            .next()
            .expect("Graphics: Layout should have a days layout parent")
            .children()
            .next()
            .expect("Graphics: Layout should have a days layout");

        days(
            renderer,
            days_layout,
            self.state.date,
            &style_sheet,
            self.state.focus,
            self.font_size,
            self.state.day_mouse_over,
        );

        // ----------- Buttons ------------------------
        let cancel_button_layout = children
            .next()
            .expect("Graphics: Layout should have a cancel button layout for a DatePicker");

        self.cancel_button.draw(
            &self.tree.children[0],
            renderer,
            theme,
            style,
            cancel_button_layout,
            cursor,
            &bounds,
        );

        let submit_button_layout = children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a DatePicker");

        self.submit_button.draw(
            &self.tree.children[1],
            renderer,
            theme,
            style,
            submit_button_layout,
            cursor,
            &bounds,
        );

        let border = Border {
            radius: style_sheet[&StyleState::Focused].border_radius.into(),
            width: style_sheet[&StyleState::Focused].border_width,
            color: style_sheet[&StyleState::Focused].border_color,
        };

        // Buttons are not focusable right now...
        if self.state.focus == Focus::Cancel {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: cancel_button_layout.bounds(),
                    border,
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }

        if self.state.focus == Focus::Submit {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: submit_button_layout.bounds(),
                    border,
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }
    }
}

/// The state of the [`DatePickerOverlay`].
#[derive(Debug)]
pub struct State {
    /// The selected date of the [`DatePickerOverlay`].
    pub(crate) date: NaiveDate,
    /// The focus of the [`DatePickerOverlay`].
    pub(crate) focus: Focus,
    /// The previously pressed keyboard modifiers.
    pub(crate) keyboard_modifiers: keyboard::Modifiers,
    /// ID of the day the mouse is over.
    pub(crate) day_mouse_over: Option<(usize, usize)>,
}

impl State {
    /// Creates a new State with the given date.
    #[must_use]
    pub fn new(date: NaiveDate) -> Self {
        Self {
            date,
            ..Self::default()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            date: Local::now().naive_local().date(),
            focus: Focus::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
            day_mouse_over: None,
        }
    }
}

/// Just a workaround to pass the button states from the tree to the overlay
#[allow(missing_debug_implementations)]
pub struct DatePickerOverlayButtons<'a, Message, Theme>
where
    Message: Clone,
    Theme: crate::style::date_picker::Catalog + iced_widget::button::Catalog,
{
    /// The cancel button of the [`DatePickerOverlay`].
    cancel_button: Element<'a, Message, Theme, Renderer>,
    /// The submit button of the [`DatePickerOverlay`].
    submit_button: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme> Default for DatePickerOverlayButtons<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::text::Catalog
        + iced_widget::container::Catalog,
{
    fn default() -> Self {
        let (cancel_content, cancel_font, _cancel_shaping) = cancel();
        let (submit_content, submit_font, _submit_shaping) = ok();

        Self {
            cancel_button: Button::new(
                text::Text::new(cancel_content)
                    .font(cancel_font)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .into(),
            submit_button: Button::new(
                text::Text::new(submit_content)
                    .font(submit_font)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .into(),
        }
    }
}

#[allow(clippy::unimplemented)]
impl<Message, Theme> Widget<Message, Theme, Renderer>
    for DatePickerOverlayButtons<'_, Message, Theme>
where
    Message: Clone,
    Theme: crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::container::Catalog,
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

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, _limits: &Limits) -> Node {
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

impl<'a, Message, Theme> From<DatePickerOverlayButtons<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a
        + crate::style::date_picker::Catalog
        + iced_widget::button::Catalog
        + iced_widget::container::Catalog,
{
    fn from(overlay: DatePickerOverlayButtons<'a, Message, Theme>) -> Self {
        Self::new(overlay)
    }
}

/// An enumeration of all focusable elements of the [`DatePickerOverlay`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

/// Draws the month/year row
fn month_year(
    renderer: &mut Renderer,
    layout: Layout<'_>,
    date: &str,
    cursor: Point,
    //style: &Style,
    style: &HashMap<StyleState, Style>,
    focus: Focus,
    font_size: Pixels,
) {
    let mut children = layout.children();

    let month_layout = children
        .next()
        .expect("Graphics: Layout should have a month layout");
    let year_layout = children
        .next()
        .expect("Graphics: Layout should have a year layout");

    let mut f = |layout: Layout<'_>, text: &str, target: Focus, font_size: Pixels| {
        let style_state = if focus == target {
            StyleState::Focused
        } else {
            StyleState::Active
        };

        let mut children = layout.children();

        let left_bounds = children
            .next()
            .expect("Graphics: Layout should have a left arrow layout")
            .bounds();
        let center_bounds = children
            .next()
            .expect("Graphics: Layout should have a center layout")
            .bounds();
        let right_bounds = children
            .next()
            .expect("Graphics: Layout should have a right arrow layout")
            .bounds();

        let left_arrow_hovered = left_bounds.contains(cursor);
        let right_arrow_hovered = right_bounds.contains(cursor);

        let bounds = layout.bounds();
        if (style_state == StyleState::Focused) && (bounds.width > 0.) && (bounds.height > 0.) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .border_radius
                            .into(),
                        width: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .border_width,
                        color: style
                            .get(&style_state)
                            .expect("Style Sheet not found.")
                            .border_color,
                    },
                    shadow: Shadow::default(),
                    snap: false,
                },
                style
                    .get(&style_state)
                    .expect("Style Sheet not found.")
                    .background,
            );
        }

        let (left_content, left_font, _left_shaping) = left_open();
        let (right_content, right_font, _right_shaping) = right_open();

        // Left caret
        renderer.fill_text(
            iced_core::Text {
                content: left_content,
                bounds: Size::new(left_bounds.width, left_bounds.height),
                size: Pixels(font_size.0 + if left_arrow_hovered { 1.0 } else { 0.0 }),
                font: left_font,
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Advanced,
                wrapping: Wrapping::default(),
            },
            Point::new(left_bounds.center_x(), left_bounds.center_y()),
            style
                .get(&style_state)
                .expect("Style Sheet not found.")
                .text_color,
            left_bounds,
        );

        // Text
        renderer.fill_text(
            iced_core::Text {
                content: text.to_owned(),
                bounds: Size::new(center_bounds.width, center_bounds.height),
                size: font_size,
                font: renderer.default_font(),
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
                wrapping: Wrapping::default(),
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
            },
            Point::new(center_bounds.center_x(), center_bounds.center_y()),
            style
                .get(&style_state)
                .expect("Style Sheet not found.")
                .text_color,
            center_bounds,
        );

        // Right caret
        renderer.fill_text(
            iced_core::Text {
                content: right_content,
                bounds: Size::new(right_bounds.width, right_bounds.height),
                size: Pixels(font_size.0 + if right_arrow_hovered { 1.0 } else { 0.0 }),
                font: right_font,
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Advanced,
                wrapping: Wrapping::default(),
            },
            Point::new(right_bounds.center_x(), right_bounds.center_y()),
            style
                .get(&style_state)
                .expect("Style Sheet not found.")
                .text_color,
            right_bounds,
        );
    };

    let (year, month) = date.split_once(' ').expect("Date must contain space");

    // Draw month
    f(month_layout, month, Focus::Month, font_size);

    // Draw year
    f(year_layout, year, Focus::Year, font_size);
}

/// Draws the days
#[allow(clippy::too_many_arguments)]
fn days(
    renderer: &mut Renderer,
    layout: Layout<'_>,
    date: chrono::NaiveDate,
    style: &HashMap<StyleState, Style>,
    focus: Focus,
    font_size: Pixels,
    mouse_over: Option<(usize, usize)>,
) {
    let mut children = layout.children();

    let day_labels_layout = children
        .next()
        .expect("Graphics: Layout should have a day labels layout");
    day_labels(renderer, day_labels_layout, style, focus, font_size);

    day_table(
        renderer,
        &mut children,
        date,
        style,
        focus,
        font_size,
        mouse_over,
    );
}

/// Draws the day labels
fn day_labels(
    renderer: &mut Renderer,
    layout: Layout<'_>,
    style: &HashMap<StyleState, Style>,
    _focus: Focus,
    font_size: Pixels,
) {
    for (i, label) in layout.children().enumerate() {
        let bounds = label.bounds();

        renderer.fill_text(
            iced_core::Text {
                content: crate::core::date::WEEKDAY_LABELS[i].clone(),
                bounds: Size::new(bounds.width, bounds.height),
                size: font_size,
                font: renderer.default_font(),
                align_x: text::Alignment::Center,
                align_y: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
                wrapping: Wrapping::default(),
            },
            Point::new(bounds.center_x(), bounds.center_y()),
            style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            bounds,
        );
    }
}

#[allow(clippy::too_many_arguments)]
/// Draws the day table
fn day_table(
    renderer: &mut Renderer,
    children: &mut dyn Iterator<Item = Layout<'_>>,
    date: chrono::NaiveDate,
    style: &HashMap<StyleState, Style>,
    focus: Focus,
    font_size: Pixels,
    day_mouse_over: Option<(usize, usize)>,
) {
    for (y, row) in children.enumerate() {
        for (x, label) in row.children().enumerate() {
            let bounds = label.bounds();
            let (number, is_in_month) =
                crate::core::date::position_to_day(x, y, date.year(), date.month());

            let mouse_over = if let Some((day_y, day_x)) = day_mouse_over {
                day_y == y && day_x == x
            } else {
                false
            };

            let selected = date.day() == number as u32 && is_in_month == IsInMonth::Same;

            let style_state = if selected {
                StyleState::Selected
            } else if mouse_over {
                StyleState::Hovered
            } else {
                StyleState::Active
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: (bounds.height / 2.0).into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
                    snap: false,
                },
                style
                    .get(&style_state)
                    .expect("Style Sheet not found.")
                    .day_background,
            );

            if focus == Focus::Day && selected {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border: Border {
                            radius: style
                                .get(&StyleState::Focused)
                                .expect("Style Sheet not found.")
                                .border_radius
                                .into(),
                            width: style
                                .get(&StyleState::Focused)
                                .expect("Style Sheet not found.")
                                .border_width,
                            color: style
                                .get(&StyleState::Focused)
                                .expect("Style Sheet not found.")
                                .border_color,
                        },
                        shadow: Shadow::default(),
                        snap: false,
                    },
                    Color::TRANSPARENT,
                );
            }

            renderer.fill_text(
                iced_core::Text {
                    content: format!("{number:02}"), // Todo: is there some way of static format as this has a fixed size?
                    bounds: Size::new(bounds.width, bounds.height),
                    size: font_size,
                    font: renderer.default_font(),
                    align_x: text::Alignment::Center,
                    align_y: Vertical::Center,
                    line_height: text::LineHeight::Relative(1.3),
                    shaping: text::Shaping::Basic,
                    wrapping: Wrapping::default(),
                },
                Point::new(bounds.center_x(), bounds.center_y()),
                if is_in_month == IsInMonth::Same {
                    style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .text_color
                } else {
                    style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .text_attenuated_color
                },
                bounds,
            );
        }
    }
}
