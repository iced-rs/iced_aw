//! Use a split to split the available space in two parts to display two different elements.
//!
//! *This API requires the following crate features to be activated: split*
use iced_graphics::{Backend, Color, Primitive, Renderer, Vector};
use iced_native::mouse;

use crate::native::split;
pub use crate::native::split::{Axis, State};
pub use crate::style::split::{Style, StyleSheet};

/// A split can divide the available space by half to display two different elements.
/// It can split horizontally or vertically.
///
/// This is an alias of an `iced_native` Split with an `iced_wgpu::Renderer`.
pub type Split<'a, Message, Backend> = split::Split<'a, Message, Renderer<Backend>>;

impl<B> split::Renderer for Renderer<B>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    #[allow(clippy::too_many_lines)]
    fn draw<Message>(
        &mut self,
        env: crate::core::renderer::DrawEnvironment<Self::Defaults, Self::Style, ()>,
        first: &iced_native::Element<'_, Message, Self>,
        second: &iced_native::Element<'_, Message, Self>,
        dragging: bool,
        axis: Axis,
    ) -> Self::Output {
        let mut children = env.layout.children();

        let background = Primitive::Quad {
            bounds: env.layout.bounds(),
            background: env
                .style_sheet
                .active()
                .background
                .unwrap_or_else(|| Color::TRANSPARENT.into()),
            border_radius: 0.0,
            border_width: env.style_sheet.active().border_width,
            border_color: env.style_sheet.active().border_color,
        };

        let first_layout = children
            .next()
            .expect("Graphics: Layout should have a first layout");
        let (first, first_mouse_interaction) = first.draw(
            self,
            env.defaults,
            first_layout,
            env.cursor_position,
            env.viewport
                .expect("A viewport should exist for a SplitPane"),
        );

        let first_background = Primitive::Quad {
            bounds: first_layout.bounds(),
            background: if first_layout.bounds().contains(env.cursor_position) {
                env.style_sheet.hovered().first_background
            } else {
                env.style_sheet.active().first_background
            }
            .unwrap_or_else(|| Color::TRANSPARENT.into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };

        let divider_layout = children
            .next()
            .expect("Graphics: Layout should have a divider layout");
        let divider_mouse_interaction = if divider_layout.bounds().contains(env.cursor_position) {
            match axis {
                Axis::Horizontal => mouse::Interaction::ResizingVertically,
                Axis::Vertical => mouse::Interaction::ResizingHorizontally,
            }
        } else {
            mouse::Interaction::default()
        };

        let divider_style = if dragging {
            env.style_sheet.dragged()
        } else if divider_layout.bounds().contains(env.cursor_position) {
            env.style_sheet.hovered()
        } else {
            env.style_sheet.active()
        };
        let divider_background = Primitive::Quad {
            bounds: divider_layout.bounds(),
            background: divider_style.divider_background,
            border_radius: 0.0,
            border_width: divider_style.divider_border_width,
            border_color: divider_style.divider_border_color,
        };

        let second_layout = children
            .next()
            .expect("Graphics: Layout should have a second layout");
        let (second, second_mouse_interaction) = second.draw(
            self,
            env.defaults,
            second_layout,
            env.cursor_position,
            env.viewport
                .expect("A viewport should exist for a SplitPane"),
        );

        let second_background = Primitive::Quad {
            bounds: second_layout.bounds(),
            background: if second_layout.bounds().contains(env.cursor_position) {
                env.style_sheet.hovered().second_background
            } else {
                env.style_sheet.active().second_background
            }
            .unwrap_or_else(|| Color::TRANSPARENT.into()),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };

        (
            Primitive::Group {
                primitives: vec![
                    background,
                    first_background,
                    second_background,
                    Primitive::Clip {
                        bounds: first_layout.bounds(),
                        offset: Vector::new(0, 0),
                        content: Box::new(first),
                    },
                    Primitive::Clip {
                        bounds: second_layout.bounds(),
                        offset: Vector::new(0, 0),
                        content: Box::new(second),
                    },
                    divider_background,
                ],
            },
            first_mouse_interaction
                .max(second_mouse_interaction)
                .max(divider_mouse_interaction),
        )
    }
}
