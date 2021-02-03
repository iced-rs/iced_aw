//! Use a date picker as an input element for picking dates.
//!
//! *This API requires the following crate features to be activated: date_picker*
use std::collections::HashMap;

use crate::style::date_picker::StyleSheet;
use crate::{
    core::renderer::DrawEnvironment,
    style::{date_picker::Style, style_state::StyleState},
};

use chrono::{self, Datelike};
use iced_graphics::{
    backend, Backend, Color, HorizontalAlignment, Primitive, Rectangle, Renderer, VerticalAlignment,
};
use iced_native::{mouse, Element};

use crate::native::date_picker;
pub use crate::native::date_picker::State;

use super::icons::{Icon, ICON_FONT};

/// An input element for picking dates.
///
/// This is an alias of an `iced_native` DatePicker with an `iced_wgpu::Renderer`.
pub type DatePicker<'a, Message, Backend> = date_picker::DatePicker<'a, Message, Renderer<Backend>>;

impl<B> date_picker::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<'_, Self::Defaults, Self::Style>,
        date: &chrono::NaiveDate,
        year_str: &str,
        month_str: &str,
        cancel_button: &Element<'_, Message, Self>,
        submit_button: &Element<'_, Message, Self>,
    ) -> Self::Output {
        let bounds = env.layout.bounds();
        let mut children = env.layout.children();
        let mut date_children = children.next().unwrap().children();

        let mut style: HashMap<StyleState, Style> = HashMap::new();
        let _ = style.insert(StyleState::Active, env.style_sheet.active());
        let _ = style.insert(StyleState::Selected, env.style_sheet.selected());
        let _ = style.insert(StyleState::Hovered, env.style_sheet.hovered());

        let mouse_interaction = mouse::Interaction::default();

        let mut style_state = StyleState::Active;
        if bounds.contains(env.cursor_position) {
            style_state = style_state.max(StyleState::Hovered);
        }

        let background = Primitive::Quad {
            bounds,
            background: style.get(&style_state).unwrap().background, // TODO
            border_radius: style.get(&style_state).unwrap().border_radius as u16, // TODO: will change in the future
            border_width: style.get(&style_state).unwrap().border_width as u16,   // TODO: same
            border_color: style.get(&style_state).unwrap().border_color,
        };

        // ----------- Year/Month----------------------
        let month_year_layout = date_children.next().unwrap();

        let (month_year, month_year_mouse_interaction) = month_year(
            month_year_layout,
            month_str,
            year_str,
            env.cursor_position,
            &style,
        );

        // ----------- Days ---------------------------
        let days_layout = date_children.next().unwrap().children().next().unwrap();

        let (days, days_mouse_interaction) = days(days_layout, date, env.cursor_position, &style);

        // ----------- Buttons ------------------------
        let cancel_button_layout = children.next().unwrap();

        let (cancel_button, cancel_mouse_interaction) = cancel_button.draw(
            self,
            env.defaults,
            cancel_button_layout,
            env.cursor_position,
            &bounds,
        );

        let submit_button_layout = children.next().unwrap();

        let (submit_button, submit_mouse_interaction) = submit_button.draw(
            self,
            env.defaults,
            submit_button_layout,
            env.cursor_position,
            &bounds,
        );

        (
            Primitive::Group {
                primitives: vec![background, month_year, days, cancel_button, submit_button],
            },
            mouse_interaction
                .max(month_year_mouse_interaction)
                .max(days_mouse_interaction)
                .max(cancel_mouse_interaction)
                .max(submit_mouse_interaction),
        )
    }
}

/// Draws the month/year row
fn month_year(
    layout: iced_native::Layout<'_>,
    month: &str,
    year: &str,
    cursor_position: iced_graphics::Point,
    //style: &Style,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction) {
    let mut children = layout.children();

    let month_layout = children.next().unwrap();
    let year_layout = children.next().unwrap();

    let f = |layout: iced_native::Layout<'_>, text: &str| {
        let mut children = layout.children();

        let left_bounds = children.next().unwrap().bounds();
        let center_bounds = children.next().unwrap().bounds();
        let right_bounds = children.next().unwrap().bounds();

        let mut mouse_interaction = mouse::Interaction::default();

        let left_arrow_hovered = left_bounds.contains(cursor_position);
        let right_arrow_hovered = right_bounds.contains(cursor_position);

        if left_arrow_hovered || right_arrow_hovered {
            mouse_interaction = mouse_interaction.max(mouse::Interaction::Pointer);
        }

        let primitive = Primitive::Group {
            primitives: vec![
                Primitive::Text {
                    content: Icon::CaretLeftFill.into(),
                    bounds: Rectangle {
                        x: left_bounds.center_x(),
                        y: left_bounds.center_y(),
                        ..left_bounds
                    },
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: left_bounds.height + if left_arrow_hovered { 5.0 } else { 0.0 },
                    font: ICON_FONT,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },
                Primitive::Text {
                    content: text.to_owned(),
                    bounds: Rectangle {
                        x: center_bounds.center_x(),
                        y: center_bounds.center_y(),
                        ..center_bounds
                    },
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: center_bounds.height,
                    font: Default::default(),
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },
                Primitive::Text {
                    content: Icon::CaretRightFill.into(),
                    bounds: Rectangle {
                        x: right_bounds.center_x(),
                        y: right_bounds.center_y(),
                        ..right_bounds
                    },
                    color: style.get(&StyleState::Active).unwrap().text_color,
                    size: right_bounds.height + if right_arrow_hovered { 5.0 } else { 0.0 },
                    font: ICON_FONT,
                    horizontal_alignment: HorizontalAlignment::Center,
                    vertical_alignment: VerticalAlignment::Center,
                },
            ],
        };

        (primitive, mouse_interaction)
    };

    let mouse_interaction = mouse::Interaction::default();

    let (month, month_mouse_interaction) = f(month_layout, month);

    let (year, year_mouse_interaction) = f(year_layout, year);

    (
        Primitive::Group {
            primitives: vec![month, year],
        },
        mouse_interaction
            .max(month_mouse_interaction)
            .max(year_mouse_interaction),
    )
}

/// Draws the days
fn days(
    layout: iced_native::Layout<'_>,
    date: &chrono::NaiveDate,
    cursor_position: iced_graphics::Point,
    //style: &Style,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction) {
    let mut children = layout.children();

    let day_labels_layout = children.next().unwrap();
    let labels = day_labels(day_labels_layout, style);

    let (table, table_mouse_interaction) = day_table(&mut children, date, cursor_position, style);

    (
        Primitive::Group {
            primitives: vec![labels, table],
        },
        table_mouse_interaction,
    )
}

/// Draws the day labels
fn day_labels(layout: iced_native::Layout<'_>, style: &HashMap<StyleState, Style>) -> Primitive {
    let mut labels: Vec<Primitive> = Vec::new();

    for (i, label) in layout.children().enumerate() {
        let bounds = label.bounds();

        labels.push(Primitive::Text {
            content: crate::core::date::WEEKDAY_LABELS[i].to_string(),
            bounds: Rectangle {
                x: bounds.center_x(),
                y: bounds.center_y(),
                ..bounds
            },
            color: style.get(&StyleState::Active).unwrap().text_color,
            size: bounds.height + 5.0,
            font: Default::default(),
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        })
    }

    Primitive::Group { primitives: labels }
}

/// Draws the day table
fn day_table(
    children: &mut dyn Iterator<Item = iced_native::Layout<'_>>,
    date: &chrono::NaiveDate,
    cursor_position: iced_graphics::Point,
    style: &HashMap<StyleState, Style>,
) -> (Primitive, mouse::Interaction) {
    let mut primitives: Vec<Primitive> = Vec::new();

    let mut mouse_interaction = mouse::Interaction::default();

    for (y, row) in children.enumerate() {
        for (x, label) in row.children().enumerate() {
            let bounds = label.bounds();
            let (number, is_in_month) =
                crate::core::date::position_to_day(x, y, date.year(), date.month());

            let mouse_over = bounds.contains(cursor_position);
            if mouse_over {
                mouse_interaction = mouse_interaction.max(mouse::Interaction::Pointer);
            }

            let selected = date.day() == number as u32 && is_in_month == 0;

            let mut style_state = StyleState::Active;
            if selected {
                style_state = style_state.max(StyleState::Selected);
            }
            if mouse_over {
                style_state = style_state.max(StyleState::Hovered);
            }

            primitives.push(Primitive::Quad {
                bounds,
                background: style.get(&style_state).unwrap().day_background,
                border_radius: bounds.height as u16 / 2,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            });

            primitives.push(Primitive::Text {
                content: format!("{:02}", number),
                bounds: Rectangle {
                    x: bounds.center_x(),
                    y: bounds.center_y(),
                    ..bounds
                },
                color: if is_in_month == 0 {
                    style.get(&style_state).unwrap().text_color
                } else {
                    style.get(&style_state).unwrap().text_attenuated_color
                },
                size: if bounds.width < bounds.height {
                    bounds.width
                } else {
                    bounds.height
                },
                font: Default::default(),
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_alignment: VerticalAlignment::Center,
            })
        }
    }

    (Primitive::Group { primitives }, mouse_interaction)
}
