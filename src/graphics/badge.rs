//! Use a badge for color highlighting important information.
//! 
//! *This API requires the following crate features to be activated: badge*
use iced_graphics::{defaults, Backend, Color, Defaults, Point, Primitive, Renderer};

use crate::native::badge;
pub use crate::style::badge::{Style, StyleSheet};

const BORDER_RADIUS_RATIO: f32 = 34.0 / 15.0;

/// A badge for color highlighting small information.
/// 
/// This is an alias of an `iced_native` Badge with an `iced_wgpu::Renderer`.
pub type Badge<'a, Message, Backend> =
    badge::Badge<'a, Message, Renderer<Backend>>;

impl<B> badge::Renderer for Renderer<B>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        _defaults: &Self::Defaults,
        cursor_position: Point,
        style_sheet: &Self::Style,
        content: &iced_native::Element<'_, Message, Self>,
        layout: iced_native::Layout<'_>,
        viewport: &iced_native::Rectangle,
    ) -> Self::Output {
        let bounds = layout.bounds();
        let mut children = layout.children();
        let is_mouse_over = bounds.contains(cursor_position);
        let style = if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        //println!("height: {}", bounds.height);
        // 34 15
        //  x
        let border_radius = style.border_radius
            .unwrap_or_else(|| (bounds.height as f32 / BORDER_RADIUS_RATIO));
        let background = Primitive::Quad {
            bounds,
            background: style.background,
            border_radius: border_radius as u16, // TODO: will be changed to f32
            border_width: style.border_width as u16, // TODO: same
            border_color: style.border_color.unwrap_or(Color::BLACK),
        };

        let (content, mouse_interaction) = content.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: style.text_color
                }
            },
            children.next().unwrap(),
            cursor_position,
            viewport
        );

        (
            Primitive::Group {
                primitives: vec![background, content],
            },
            mouse_interaction,
        )
    }
}