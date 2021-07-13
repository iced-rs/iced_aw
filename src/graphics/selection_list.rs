//! Display a list of selectable values.
use iced_graphics::{backend, Backend, Color, Point, Primitive, Renderer};
use iced_native::{mouse, HorizontalAlignment, Rectangle, VerticalAlignment};

use crate::native::overlay::list;
pub use crate::native::selection_list::{self, State};
pub use crate::style::selection_list::{Style, StyleSheet};

/// A widget allowing the selection of a single value from a list of options.
pub type SelectionList<'a, T, Message, Backend> =
    selection_list::SelectionList<'a, T, Message, Renderer<Backend>>;

impl<B> selection_list::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    fn decorate(
        &mut self,
        bounds: Rectangle,
        _cursor_position: Point,
        style: &Style,
        (primitives, mouse_cursor): Self::Output,
    ) -> Self::Output {
        (
            Primitive::Group {
                primitives: vec![
                    Primitive::Quad {
                        bounds,
                        background: style.background,
                        border_color: style.border_color,
                        border_width: style.border_width,
                        border_radius: 0.0,
                    },
                    primitives,
                ],
            },
            mouse_cursor,
        )
    }

    fn draw(&mut self) -> Self::Output {
        (Primitive::None, mouse::Interaction::default())
    }
}

impl<B> list::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    fn draw<T: ToString>(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        viewport: &Rectangle,
        options: &[T],
        hovered_option: Option<usize>,
        font: Self::Font,
        style: &Style,
    ) -> Self::Output {
        use std::f32;
        let is_mouse_over = bounds.contains(cursor_position);
        let option_height = style.text_size + (style.padding * 2);

        let mut primitives = Vec::new();

        let offset = viewport.y - bounds.y;
        let start = (offset / f32::from(option_height)) as usize;
        let end = ((offset + viewport.height) / f32::from(option_height)).ceil() as usize;

        let visible_options = &options[start..end.min(options.len())];

        for (i, option) in visible_options.iter().enumerate() {
            let i = start + i;
            let is_selected = hovered_option == Some(i);

            let bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + f32::from(option_height * i as u16),
                width: bounds.width,
                height: f32::from(style.text_size + (style.padding * 2)),
            };

            if is_selected {
                primitives.push(Primitive::Quad {
                    bounds,
                    background: style.selected_background,
                    border_color: Color::TRANSPARENT,
                    border_width: 0.0,
                    border_radius: 0.0,
                });
            }

            primitives.push(Primitive::Text {
                content: option.to_string(),
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.center_y(),
                    width: f32::INFINITY,
                    ..bounds
                },
                size: f32::from(style.text_size),
                font,
                color: if is_selected {
                    style.selected_text_color
                } else {
                    style.text_color
                },
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Center,
            });
        }

        (
            Primitive::Group { primitives },
            if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}
