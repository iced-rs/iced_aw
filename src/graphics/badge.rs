//! Use a badge for color highlighting important information.
//!
//! *This API requires the following crate features to be activated: badge*
use iced_graphics::{Backend, Color, Primitive, Renderer};

pub use crate::style::badge::{Style, StyleSheet};
use crate::{core::renderer::DrawEnvironment, native::badge};

/// The ratio of the border radius.
const BORDER_RADIUS_RATIO: f32 = 34.0 / 15.0;

/// A badge for color highlighting small information.
///
/// This is an alias of an `iced_native` Badge with an `iced_wgpu::Renderer`.
pub type Badge<'a, Message, Backend> = badge::Badge<'a, Message, Renderer<Backend>>;

/*
impl<B> badge::Renderer for Renderer<B>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        env: DrawEnvironment<Self::Defaults, Self::Style, ()>,
        content: &iced_native::Element<'_, Message, Self>,
    ) -> Self::Output {
        let bounds = env.layout.bounds();
        let mut children = env.layout.children();
        let is_mouse_over = bounds.contains(env.cursor_position);
        let style = if is_mouse_over {
            env.style_sheet.hovered()
        } else {
            env.style_sheet.active()
        };

        //println!("height: {}", bounds.height);
        // 34 15
        //  x
        let border_radius = style
            .border_radius
            .unwrap_or_else(|| (bounds.height as f32 / BORDER_RADIUS_RATIO));
        let background = Primitive::Quad {
            bounds,
            background: style.background,
            border_radius,
            border_width: style.border_width,
            border_color: style.border_color.unwrap_or(Color::BLACK),
        };

        let (content, mouse_interaction) = content.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: style.text_color,
                },
            },
            children
                .next()
                .expect("Graphics: Layout should have a children layout for Badge"),
            env.cursor_position,
            env.viewport.expect("A viewport should exist for Badge"),
        );

        (
            Primitive::Group {
                primitives: vec![background, content],
            },
            mouse_interaction,
        )
    }
}
*/
