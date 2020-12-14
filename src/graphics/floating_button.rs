//! TODO

use iced_graphics::{Backend, Point, Rectangle, Renderer, backend};

use crate::native::floating_button;
//pub use crate::style::floating_button::{Style, StyleSheet};
pub use floating_button::{Anchor, Offset};

/// TODO
pub type FloatingButton<'a, B, Message, Backend> =
    floating_button::FloatingButton<'a, B, Message, Renderer<Backend>>;

impl<B> floating_button::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    //type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: Point,
        //style_sheet: &Self::Style,
        layout: iced_native::Layout<'_>,
        floating: &iced_native::Element<'_, Message, Self>,
        viewport: &Rectangle,
    ) -> Self::Output {
        floating.draw(self, &defaults, layout, cursor_position, viewport)
    }
}