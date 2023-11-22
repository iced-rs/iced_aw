//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: `date_picker`*

use crate::{
    core::{
        date::{Date, IsInMonth},
        overlay::Position,
    },
    date_picker,
    graphics::icons::{
        bootstrap::{icon_to_string, BootstrapIcon},
        BOOTSTRAP_FONT,
    },
    style::{
        date_picker::{Appearance, StyleSheet},
        style_state::StyleState,
    },
};

use chrono::{Datelike, Local, NaiveDate};
use iced_widget::{
    button, container,
    core::{
        self,
        alignment::{Horizontal, Vertical},
        event, keyboard,
        layout::{Limits, Node},
        mouse::{self, Cursor},
        overlay, renderer,
        text::Renderer as _,
        touch,
        widget::tree::Tree,
        Alignment, Clipboard, Color, Element, Event, Layout, Length, Overlay, Padding, Point,
        Rectangle, Renderer as _, Shell, Size, Widget,
    },
    renderer::Renderer,
    text, Button, Column, Container, Row, Text,
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
    cancel_button: Button<'a, Message, Renderer<Theme>>,
    /// The submit button of the [`DatePickerOverlay`].
    submit_button: Button<'a, Message, Renderer<Theme>>,
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
    pub fn overlay(self) -> overlay::Element<'a, Message, Renderer<Theme>> {
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
        cursor: Cursor,
        _messages: &mut Shell<Message>,
        _renderer: &Renderer<Theme>,
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
        _renderer: &Renderer<Theme>,
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
        _renderer: &Renderer<Theme>,
        _clipboard: &mut dyn Clipboard,
    ) -> event::Status {
        if self.state.focus == Focus::None {
            return event::Status::Ignored;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
            let mut status = event::Status::Ignored;

            match key_code {
                keyboard::KeyCode::Tab => {
                    if self.state.keyboard_modifiers.shift() {
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

impl<'a, Message, Theme> Overlay<Message, Renderer<Theme>> for DatePickerOverlay<'a, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + text::StyleSheet + container::StyleSheet,
{
    #[allow(clippy::too_many_lines)]
    fn layout(&self, renderer: &Renderer<Theme>, bounds: Size, position: Point) -> Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .pad(Padding::from(PADDING))
            .width(Length::Fill)
            .height(Length::Fill)
            .max_width(300.0)
            .max_height(300.0);

        // Pre-Buttons TODO: get rid of it
        let cancel_limits = limits;
        let cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let limits = limits.shrink(Size::new(0.0, cancel_button.bounds().height + SPACING));

        // Month/Year
        let font_size = renderer.default_size();

        let month_year = Row::<(), Renderer<Theme>>::new()
            .width(Length::Fill)
            .spacing(SPACING)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                        Container::new(
                            Row::new() // Left Month arrow
                                .width(Length::Shrink)
                                .push(
                                    Text::new(char::from(BootstrapIcon::CaretLeftFill).to_string())
                                        .size(font_size + 1.0)
                                        .font(BOOTSTRAP_FONT),
                                ),
                        )
                        .height(Length::Shrink),
                    )
                    .push(
                        // Month
                        Text::new("October")
                            .width(Length::Fill)
                            .height(Length::Shrink),
                    )
                    .push(
                        // Right Month arrow
                        Container::new(
                            Text::new(char::from(BootstrapIcon::CaretRightFill).to_string())
                                .size(font_size + 1.0)
                                .font(crate::BOOTSTRAP_FONT),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    ),
            )
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                        Container::new(
                            Row::new() // Left Year arrow
                                .width(Length::Shrink)
                                .push(
                                    Text::new(char::from(BootstrapIcon::CaretLeftFill).to_string())
                                        .size(font_size + 1.0)
                                        .font(BOOTSTRAP_FONT),
                                ),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    )
                    .push(
                        // Year
                        Text::new("9999").width(Length::Fill).height(Length::Shrink),
                    )
                    .push(
                        // Right Year arrow
                        Container::new(
                            Row::new()
                                .width(Length::Shrink)
                                .height(Length::Shrink)
                                .push(
                                    Text::new(
                                        char::from(BootstrapIcon::CaretRightFill).to_string(),
                                    )
                                    .size(font_size + 1.0)
                                    .font(BOOTSTRAP_FONT),
                                ),
                        )
                        .height(Length::Shrink)
                        .width(Length::Shrink),
                    ),
            );

        let days = Container::<(), Renderer<Theme>>::new((0..7).fold(
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
                                Container::new(
                                    Row::new()
                                        .width(Length::Shrink)
                                        .height(Length::Shrink)
                                        .push(Text::new("31").size(font_size)),
                                )
                                .width(Length::Fill)
                                .height(Length::Shrink),
                            )
                        },
                    ),
                )
            },
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y();

        let mut col = Column::<(), Renderer<Theme>>::new()
            .spacing(SPACING)
            .align_items(Alignment::Center)
            .push(month_year)
            .push(days)
            .layout(renderer, &limits);

        col.move_to(Point::new(
            col.bounds().x + PADDING,
            col.bounds().y + PADDING,
        ));

        // Buttons
        let cancel_limits =
            limits.max_width(((col.bounds().width / 2.0) - BUTTON_SPACING).max(0.0));

        let mut cancel_button = self.cancel_button.layout(renderer, &cancel_limits);

        let submit_limits =
            limits.max_width(((col.bounds().width / 2.0) - BUTTON_SPACING).max(0.0));

        let mut submit_button = self.submit_button.layout(renderer, &submit_limits);

        cancel_button.move_to(Point {
            x: cancel_button.bounds().x + PADDING,
            y: cancel_button.bounds().y + col.bounds().height + PADDING + SPACING,
        });

        submit_button.move_to(Point {
            x: submit_button.bounds().x + col.bounds().width - submit_button.bounds().width
                + PADDING,
            y: submit_button.bounds().y + col.bounds().height + PADDING + SPACING,
        });

        let mut node = Node::with_children(
            Size::new(
                col.bounds().width + (2.0 * PADDING),
                col.bounds().height + cancel_button.bounds().height + (2.0 * PADDING) + SPACING,
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
        cursor: Cursor,
        renderer: &Renderer<Theme>,
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
        renderer: &Renderer<Theme>,
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
        renderer: &mut Renderer<Theme>,
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
                border_radius: style_sheet[&style_state].border_radius.into(),
                border_width: style_sheet[&style_state].border_width,
                border_color: style_sheet[&style_state].border_color,
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
                    border_radius: style_sheet[&StyleState::Focused].border_radius.into(),
                    border_width: style_sheet[&StyleState::Focused].border_width,
                    border_color: style_sheet[&StyleState::Focused].border_color,
                },
                Color::TRANSPARENT,
            );
        }

        if self.state.focus == Focus::Submit {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: submit_button_layout.bounds(),
                    border_radius: style_sheet[&StyleState::Focused].border_radius.into(),
                    border_width: style_sheet[&StyleState::Focused].border_width,
                    border_color: style_sheet[&StyleState::Focused].border_color,
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
    cancel_button: Element<'a, Message, Renderer<Theme>>,
    /// The submit button of the [`DatePickerOverlay`].
    submit_button: Element<'a, Message, Renderer<Theme>>,
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
impl<'a, Message, Theme> Widget<Message, Renderer<Theme>>
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

    fn width(&self) -> Length {
        unimplemented!("This should never be reached!")
    }

    fn height(&self) -> Length {
        unimplemented!("This should never be reached!")
    }

    fn layout(&self, _renderer: &Renderer<Theme>, _limits: &Limits) -> Node {
        unimplemented!("This should never be reached!")
    }

    fn draw(
        &self,
        _state: &Tree,
        _renderer: &mut Renderer<Theme>,
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
    for Element<'a, Message, Renderer<Theme>>
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
fn month_year<Theme>(
    renderer: &mut Renderer<Theme>,
    layout: Layout<'_>,
    month: &str,
    year: &str,
    cursor: Point,
    //style: &Style,
    style: &HashMap<StyleState, Appearance>,
    focus: Focus,
) where
    Theme: StyleSheet + button::StyleSheet + container::StyleSheet + text::StyleSheet,
{
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
                    border_color: style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .border_color,
                    border_radius: style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .border_radius
                        .into(),
                    border_width: style
                        .get(&style_state)
                        .expect("Style Sheet not found.")
                        .border_width,
                },
                style
                    .get(&style_state)
                    .expect("Style Sheet not found.")
                    .background,
            );
        }

        let mut buffer = [0; 4];

        // Left caret
        renderer.fill_text(core::text::Text {
            content: char::from(BootstrapIcon::CaretLeftFill).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: left_bounds.center_x(),
                y: left_bounds.center_y(),
                ..left_bounds
            },
            size: renderer.default_size() + if left_arrow_hovered { 1.0 } else { 0.0 },
            color: style
                .get(&style_state)
                .expect("Style Sheet not found.")
                .text_color,
            font: BOOTSTRAP_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Advanced,
        });

        // Text
        renderer.fill_text(core::text::Text {
            content: text,
            bounds: Rectangle {
                x: center_bounds.center_x(),
                y: center_bounds.center_y(),
                ..center_bounds
            },
            size: renderer.default_size(),
            color: style
                .get(&style_state)
                .expect("Style Sheet not found.")
                .text_color,
            font: renderer.default_font(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
        });

        // Right caret
        renderer.fill_text(core::text::Text {
            content: char::from(BootstrapIcon::CaretRightFill).encode_utf8(&mut buffer),
            bounds: Rectangle {
                x: right_bounds.center_x(),
                y: right_bounds.center_y(),
                ..right_bounds
            },
            size: renderer.default_size() + if right_arrow_hovered { 1.0 } else { 0.0 },
            color: style
                .get(&style_state)
                .expect("Style Sheet not found.")
                .text_color,
            font: BOOTSTRAP_FONT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Advanced,
        });
    };

    // Draw month
    f(month_layout, month, Focus::Month);

    // Draw year
    f(year_layout, year, Focus::Year);
}

/// Draws the days
fn days<Theme>(
    renderer: &mut Renderer<Theme>,
    layout: Layout<'_>,
    date: chrono::NaiveDate,
    cursor: Point,
    //style: &Style,
    style: &HashMap<StyleState, Appearance>,
    focus: Focus,
) where
    Theme: StyleSheet + button::StyleSheet + container::StyleSheet + text::StyleSheet,
{
    let mut children = layout.children();

    let day_labels_layout = children
        .next()
        .expect("Graphics: Layout should have a day labels layout");
    day_labels(renderer, day_labels_layout, style, focus);

    day_table(renderer, &mut children, date, cursor, style, focus);
}

/// Draws the day labels
fn day_labels<Theme>(
    renderer: &mut Renderer<Theme>,
    layout: Layout<'_>,
    style: &HashMap<StyleState, Appearance>,
    _focus: Focus,
) where
    Theme: StyleSheet + button::StyleSheet + container::StyleSheet + text::StyleSheet,
{
    for (i, label) in layout.children().enumerate() {
        let bounds = label.bounds();

        renderer.fill_text(core::text::Text {
            content: &crate::core::date::WEEKDAY_LABELS[i],
            bounds: Rectangle {
                x: bounds.center_x(),
                y: bounds.center_y(),
                ..bounds
            },
            size: renderer.default_size(),
            color: style
                .get(&StyleState::Active)
                .expect("Style Sheet not found.")
                .text_color,
            font: renderer.default_font(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: text::LineHeight::Relative(1.3),
            shaping: text::Shaping::Basic,
        });
    }
}

/// Draws the day table
fn day_table<Theme>(
    renderer: &mut Renderer<Theme>,
    children: &mut dyn Iterator<Item = Layout<'_>>,
    date: chrono::NaiveDate,
    cursor: Point,
    style: &HashMap<StyleState, Appearance>,
    focus: Focus,
) where
    Theme: StyleSheet + button::StyleSheet + container::StyleSheet + text::StyleSheet,
{
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
                    border_radius: (bounds.height / 2.0).into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
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
                        border_radius: style
                            .get(&StyleState::Focused)
                            .expect("Style Sheet not found.")
                            .border_radius
                            .into(),
                        border_width: style
                            .get(&StyleState::Focused)
                            .expect("Style Sheet not found.")
                            .border_width,
                        border_color: style
                            .get(&StyleState::Focused)
                            .expect("Style Sheet not found.")
                            .border_color,
                    },
                    Color::TRANSPARENT,
                );
            }

            renderer.fill_text(core::text::Text {
                content: &format!("{number:02}"), // Todo: is there some way of static format as this has a fixed size?
                bounds: Rectangle {
                    x: bounds.center_x(),
                    y: bounds.center_y(),
                    ..bounds
                },
                size: renderer.default_size(),
                color: if is_in_month == IsInMonth::Same {
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
                font: renderer.default_font(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: text::LineHeight::Relative(1.3),
                shaping: text::Shaping::Basic,
            });
        }
    }
}
