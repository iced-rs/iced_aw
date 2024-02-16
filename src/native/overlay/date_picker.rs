//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*

use crate::{
    core::{
        date::{Date, IsInMonth},
        overlay::Position,
    },
    date_picker,
    graphics::icons::{icon_to_string, BootstrapIcon, BOOTSTRAP_FONT},
    style::{
        date_picker::{Appearance, StyleSheet},
        style_state::StyleState,
    },
};

use chrono::{Datelike, Local, NaiveDate};
use iced::{
    self,
    advanced::{
        layout::{Limits, Node},
        overlay,
        renderer,
        widget::tree::Tree,
        Clipboard,
        Overlay,
        Layout,
        Shell,
        Widget,
        Renderer as _,
        text::Renderer as _,
    },
    alignment::{Horizontal, Vertical},
    event, keyboard, mouse::{self, Cursor}, touch,
    widget::{button, container, text, Button, Column, Container, Row, Text,},
    Renderer, // the actual type
    Alignment, Border, Color, Element, Event, Length, Padding, Pixels, Point, Rectangle,
    Shadow, Size,
};
use std::collections::HashMap;

/// The padding around the elements.
const PADDING: f32 = 10.0;
/// The spacing between the elements.
const SPACING: f32 = 15.0;
/// The padding of the day cells.
const DAY_CELL_PADDING: f32 = 7.0;
/// The spacing between the buttons.
const BUTTON_SPACING: f32 = 5.0;

/// The overlay of the [`DatePicker`](crate::native::DatePicker).
#[allow(missing_debug_implementations)]
pub struct DatePickerOverlay<'a, Message, Theme>
where
    Message: Clone,
    Theme: StyleSheet + button::StyleSheet,
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
    style: <Theme as StyleSheet>::Style,
    /// The reference to the tree holding the state of this overlay.
    tree: &'a mut Tree,
}

impl<'a, Message, Theme> DatePickerOverlay<'a, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    /// Creates a new [`DatePickerOverlay`] on the given position.
    pub fn new(
        state: &'a mut date_picker::State,
        on_cancel: Message,
        on_submit: &'a dyn Fn(Date) -> Message,
        position: Point,
        style: <Theme as StyleSheet>::Style,
        tree: &'a mut Tree,
        //button_style: impl Clone +  Into<<Renderer as button::Renderer>::Style>, // clone not satisfied
    ) -> Self {
        let date_picker::State { overlay_state } = state;

        DatePickerOverlay {
            state: overlay_state,
            cancel_button: Button::new(
                text::Text::new(icon_to_string(BootstrapIcon::X))
                    .font(crate::BOOTSTRAP_FONT)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel.clone()),
            submit_button: Button::new(
                text::Text::new(icon_to_string(BootstrapIcon::Check))
                    .font(crate::BOOTSTRAP_FONT)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .on_press(on_cancel), // Sending a fake message
            on_submit,
            position,
            style,
            tree,
        }
    }

    /// Turn this [`DatePickerOverlay`] into an overlay [`Element`](overlay::Element).
    #[must_use]
    pub fn overlay(self) -> overlay::Element<'a, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(self))
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
        cursor: Cursor,
        _messages: &mut Shell<Message>,
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
                if cursor.is_over(month_layout.bounds()) {
                    self.state.focus = Focus::Month;
                }

                if cursor.is_over(left_bounds) {
                    self.state.date = crate::core::date::pred_month(self.state.date);
                    status = event::Status::Captured;
                } else if cursor.is_over(right_bounds) {
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
                if cursor.is_over(year_layout.bounds()) {
                    self.state.focus = Focus::Year;
                }

                if cursor.is_over(left_bounds) {
                    self.state.date = crate::core::date::pred_year(self.state.date);
                    status = event::Status::Captured;
                } else if cursor.is_over(right_bounds) {
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
        cursor: Cursor,
        _messages: &mut Shell<Message>,
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
                if cursor.is_over(layout.bounds()) {
                    self.state.focus = Focus::Day;
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
        _cursor: Cursor,
        _messages: &mut Shell<Message>,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
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

impl<'a, Message, Theme> Overlay<Message, Theme, Renderer> for DatePickerOverlay<'a, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    #[allow(clippy::too_many_lines)]
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .shrink(Padding::from(PADDING))
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(300.0)
            .max_height(300.0);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button =
            self.cancel_button
                .layout(&mut self.tree.children[0], renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(0.0, cancel_button.bounds().height + SPACING));

        // Month/Year
        let font_size = renderer.default_size();

        let month_year = Row::<Message, Theme, Renderer>::new()
            .width(Length::Fill)
            .spacing(SPACING)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(Container::new(
                        Row::new().push(
                            Text::new(icon_to_string(BootstrapIcon::CaretLeftFill))
                                .size(font_size.0 + 1.0)
                                .font(crate::BOOTSTRAP_FONT),
                        ),
                    ))
                    .push(
                        // Month
                        Text::new("October").width(Length::Fill),
                    )
                    .push(
                        // Right Month arrow
                        Container::new(
                            Text::new(icon_to_string(BootstrapIcon::CaretRightFill))
                                .size(font_size.0 + 1.0)
                                .font(crate::BOOTSTRAP_FONT),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    ),
            )
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(Container::new(
                        Row::new().push(
                            Text::new(icon_to_string(BootstrapIcon::CaretLeftFill))
                                .size(font_size.0 + 1.0)
                                .font(BOOTSTRAP_FONT),
                        ),
                    ))
                    .push(
                        // Year
                        Text::new("9999").width(Length::Fill),
                    )
                    .push(
                        // Right Year arrow
                        Container::new(
                            Row::new().push(
                                Text::new(icon_to_string(BootstrapIcon::CaretRightFill))
                                    .size(font_size.0 + 1.0)
                                    .font(BOOTSTRAP_FONT),
                            ),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    ),
            );

        let days = Container::<Message, Theme, Renderer>::new((0..7).fold(
            Column::new().width(Length::Fill).height(Length::Fill),
            |column, _y| {
                column.push(
                    (0..7).fold(
                        Row::new()
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .padding(DAY_CELL_PADDING),
                        |row, _x| {
                            row.push(
                                Container::new(Row::new().push(Text::new("31").size(font_size)))
                                    .width(Length::Fill)
                                    .height(Length::Fill),
                            )
                        },
                    ),
                )
            },
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y();

        let col = Column::<Message, Theme, Renderer>::new()
            .spacing(SPACING)
            .align_items(Alignment::Center)
            .push(month_year)
            .push(days);

        let element: Element<Message, Theme, Renderer> = Element::new(col);
        let col_tree = if let Some(child_tree) = self.tree.children.get_mut(2) {
            child_tree.diff(element.as_widget());
            child_tree
        } else {
            let child_tree = Tree::new(element.as_widget());
            self.tree.children.insert(2, child_tree);
            &mut self.tree.children[2]
        };

        let mut col = element.as_widget().layout(col_tree, renderer, &limits);
        let col_bounds = col.bounds();
        col = col.move_to(Point::new(col_bounds.x + PADDING, col_bounds.y + PADDING));

        // Buttons
        let cancel_limits =
            limits.max_width(((col.bounds().width / 2.0) - BUTTON_SPACING).max(0.0));

        let mut cancel_button =
            self.cancel_button
                .layout(&mut self.tree.children[0], renderer, &cancel_limits);

        let submit_limits =
            limits.max_width(((col.bounds().width / 2.0) - BUTTON_SPACING).max(0.0));

        let mut submit_button =
            self.submit_button
                .layout(&mut self.tree.children[1], renderer, &submit_limits);

        let cancel_bounds = cancel_button.bounds();
        cancel_button = cancel_button.move_to(Point {
            x: cancel_bounds.x + PADDING,
            y: cancel_bounds.y + col.bounds().height + PADDING + SPACING,
        });

        let submit_bounds = submit_button.bounds();
        submit_button = submit_button.move_to(Point {
            x: submit_bounds.x + col.bounds().width - submit_bounds.width + PADDING,
            y: submit_bounds.y + col.bounds().height + PADDING + SPACING,
        });

        let mut node = Node::with_children(
            Size::new(
                col.bounds().width + (2.0 * PADDING),
                col.bounds().height + cancel_button.bounds().height + (2.0 * PADDING) + SPACING,
            ),
            vec![col, cancel_button, submit_button],
        );
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
        if event::Status::Captured
            == self.on_event_keyboard(&event, layout, cursor, shell, renderer, clipboard)
        {
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
            cursor,
            shell,
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
        let days_status =
            self.on_event_days(&event, days_layout, cursor, shell, renderer, clipboard);

        // ----------- Buttons ------------------------
        let cancel_button_layout = children
            .next()
            .expect("Native: Layout should have a cancel button layout for a DatePicker");

        let cancel_status = self.cancel_button.on_event(
            &mut self.tree.children[0],
            event.clone(),
            cancel_button_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );

        let submit_button_layout = children
            .next()
            .expect("Native: Layout should have a submit button layout for a DatePicker");

        let mut fake_messages: Vec<Message> = Vec::new();

        let submit_status = self.submit_button.on_event(
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
        }

        month_year_status
            .merge(days_status)
            .merge(cancel_status)
            .merge(submit_status)
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
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

            let mut mouse_interaction = mouse::Interaction::default();

            let left_arrow_hovered = cursor.is_over(left_bounds);
            let right_arrow_hovered = cursor.is_over(right_bounds);

            if left_arrow_hovered || right_arrow_hovered {
                mouse_interaction = mouse_interaction.max(mouse::Interaction::Pointer);
            }

            mouse_interaction
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
            viewport,
            renderer,
        );

        let submit_button_layout = children
            .next()
            .expect("Graphics: Layout should have a submit button layout for a DatePicker");

        let submit_button_mouse_interaction = self.submit_button.mouse_interaction(
            &self.tree.children[1],
            submit_button_layout,
            cursor,
            viewport,
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

        let mut style_sheet: HashMap<StyleState, Appearance> = HashMap::new();
        let _ = style_sheet.insert(StyleState::Active, StyleSheet::active(theme, &self.style));
        let _ = style_sheet.insert(
            StyleState::Selected,
            StyleSheet::selected(theme, &self.style),
        );
        let _ = style_sheet.insert(StyleState::Hovered, StyleSheet::hovered(theme, &self.style));
        let _ = style_sheet.insert(StyleState::Focused, StyleSheet::focused(theme, &self.style));

        let mut style_state = StyleState::Active;
        if self.state.focus == Focus::Overlay {
            style_state = style_state.max(StyleState::Focused);
        }
        if cursor.is_over(bounds) {
            style_state = style_state.max(StyleState::Hovered);
        }

        // Background
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

        // ----------- Year/Month----------------------
        let month_year_layout = date_children
            .next()
            .expect("Graphics: Layout should have a month/year layout");

        month_year(
            renderer,
            month_year_layout,
            &self.month_as_string(),
            &self.year_as_string(),
            cursor.position().unwrap_or_default(),
            &style_sheet,
            self.state.focus,
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
            cursor.position().unwrap_or_default(),
            &style_sheet,
            self.state.focus,
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

        // Buttons are not focusable right now...
        if self.state.focus == Focus::Cancel {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: cancel_button_layout.bounds(),
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

        if self.state.focus == Focus::Submit {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: submit_button_layout.bounds(),
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
        }
    }
}

/// Just a workaround to pass the button states from the tree to the overlay
#[allow(missing_debug_implementations)]
pub struct DatePickerOverlayButtons<'a, Message, Theme>
where
    Message: Clone,
    Theme: StyleSheet + button::StyleSheet,
{
    /// The cancel button of the [`DatePickerOverlay`].
    cancel_button: Element<'a, Message, Theme, Renderer>,
    /// The submit button of the [`DatePickerOverlay`].
    submit_button: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme> Default for DatePickerOverlayButtons<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    fn default() -> Self {
        Self {
            cancel_button: Button::new(
                text::Text::new(icon_to_string(BootstrapIcon::X))
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .into(),
            submit_button: Button::new(
                text::Text::new(icon_to_string(BootstrapIcon::Check))
                    .font(BOOTSTRAP_FONT)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .into(),
        }
    }
}

#[allow(clippy::unimplemented)]
impl<'a, Message, Theme> Widget<Message, Theme, Renderer>
    for DatePickerOverlayButtons<'a, Message, Theme>
where
    Message: Clone,
    Theme: StyleSheet + button::StyleSheet + container::StyleSheet,
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

impl<'a, Message, Theme> From<DatePickerOverlayButtons<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + container::StyleSheet,
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
    month: &str,
    year: &str,
    cursor: Point,
    //style: &Style,
    style: &HashMap<StyleState, Appearance>,
    focus: Focus,
) {
    let mut children = layout.children();

    let month_layout = children
        .next()
        .expect("Graphics: Layout should have a month layout");
    let year_layout = children
        .next()
        .expect("Graphics: Layout should have a year layout");

    let mut f = |layout: Layout<'_>, text: &str, target: Focus| {
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

        if style_state == StyleState::Focused {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
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
                },
                style
                    .get(&style_state)
                    .expect("Style Sheet not found.")
                    .background,
            );
        }

        // Left caret

        renderer.fill_text(
            iced::advanced::Text {
                content: &icon_to_string(BootstrapIcon::CaretLeftFill),
                bounds: Size::new(left_bounds.width, left_bounds.height),
                size: Pixels(
                    renderer.default_size().0 + if left_arrow_hovered { 1.0 } else { 0.0 },
                ),
                font: BOOTSTRAP_FONT,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Advanced,
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
            iced::advanced::Text {
                content: text,
                bounds: Size::new(center_bounds.width, center_bounds.height),
                size: renderer.default_size(),
                font: renderer.default_font(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
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
            iced::advanced::Text {
                content: &icon_to_string(BootstrapIcon::CaretRightFill),
                bounds: Size::new(right_bounds.width, right_bounds.height),
                size: Pixels(
                    renderer.default_size().0 + if right_arrow_hovered { 1.0 } else { 0.0 },
                ),
                font: BOOTSTRAP_FONT,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Advanced,
            },
            Point::new(right_bounds.center_x(), right_bounds.center_y()),
            style
                .get(&style_state)
                .expect("Style Sheet not found.")
                .text_color,
            right_bounds,
        );
    };

    // Draw month
    f(month_layout, month, Focus::Month);

    // Draw year
    f(year_layout, year, Focus::Year);
}

/// Draws the days
fn days(
    renderer: &mut Renderer,
    layout: Layout<'_>,
    date: chrono::NaiveDate,
    cursor: Point,
    //style: &Style,
    style: &HashMap<StyleState, Appearance>,
    focus: Focus,
) {
    let mut children = layout.children();

    let day_labels_layout = children
        .next()
        .expect("Graphics: Layout should have a day labels layout");
    day_labels(renderer, day_labels_layout, style, focus);

    day_table(renderer, &mut children, date, cursor, style, focus);
}

/// Draws the day labels
fn day_labels(
    renderer: &mut Renderer,
    layout: Layout<'_>,
    style: &HashMap<StyleState, Appearance>,
    _focus: Focus,
) {
    for (i, label) in layout.children().enumerate() {
        let bounds = label.bounds();

        renderer.fill_text(
            iced::advanced::Text {
                content: &crate::core::date::WEEKDAY_LABELS[i],
                bounds: Size::new(bounds.width, bounds.height),
                size: renderer.default_size(),
                font: renderer.default_font(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
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

/// Draws the day table
fn day_table(
    renderer: &mut Renderer,
    children: &mut dyn Iterator<Item = Layout<'_>>,
    date: chrono::NaiveDate,
    cursor: Point,
    style: &HashMap<StyleState, Appearance>,
    focus: Focus,
) {
    for (y, row) in children.enumerate() {
        for (x, label) in row.children().enumerate() {
            let bounds = label.bounds();
            let (number, is_in_month) =
                crate::core::date::position_to_day(x, y, date.year(), date.month());

            let mouse_over = bounds.contains(cursor);

            let selected = date.day() == number as u32 && is_in_month == IsInMonth::Same;

            let mut style_state = StyleState::Active;
            if selected {
                style_state = style_state.max(StyleState::Selected);
            }
            if mouse_over {
                style_state = style_state.max(StyleState::Hovered);
            }

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: (bounds.height / 2.0).into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: Shadow::default(),
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
                    },
                    Color::TRANSPARENT,
                );
            }

            renderer.fill_text(
                iced::advanced::Text {
                    content: &format!("{number:02}"), // Todo: is there some way of static format as this has a fixed size?
                    bounds: Size::new(bounds.width, bounds.height),
                    size: renderer.default_size(),
                    font: renderer.default_font(),
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    line_height: text::LineHeight::Relative(1.3),
                    shaping: text::Shaping::Basic,
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
