//! Use a badge for color highlighting important information.
//! 
//! *This API requires the following crate features to be activated: badge*

use iced_graphics::{Backend, Color, Primitive, Renderer};

use crate::native::modal;
pub use crate::native::modal::State;
pub use crate::style::modal::{Style, StyleSheet};

/// A modal content as an overlay.
/// 
/// This is an alias of an `iced_native` Modal with an `iced_wgpu::Renderer`.
pub type Modal<'a, State, Content, Message, Backend> =
    modal::Modal<'a, State, Content, Message, Renderer<Backend>>;

impl<B> modal::Renderer for Renderer<B>
where 
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: iced_graphics::Point,
        style_sheet: &Self::Style,
        modal: &iced_native::Element<'_, Message, Self>,
        layout: iced_native::Layout<'_>,
    ) -> Self::Output {
        let bounds = layout.bounds();

        let style = style_sheet.active();

        let background = Primitive::Quad {
            bounds,
            background: style.background,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };

        let (modal, mouse_interaction) = modal.draw(
            self,
            defaults,
            layout,
            cursor_position,
            &bounds
        );

        (
            Primitive::Group {
                primitives: vec![background, modal],
            },
            mouse_interaction,
        )
    }
}