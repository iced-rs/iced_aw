//! Use a grid as an input element for creating grids.
//!
//! *This API requires the following crate features to be activated: `grid`*
use iced_graphics::Renderer;

use crate::native::grid;

/// A container that distributes its contents in a grid.
///
/// This is an alias of an `iced_native` `Grid` with a default `iced_wgpu::Renderer`.
pub type Grid<'a, Message, Backend> = grid::Grid<'a, Message, Renderer<Backend>>;

/*
impl<B> grid::Renderer for Renderer<B>
where
    B: Backend,
{
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        elements: &[Element<'_, Message, Self>],
    ) -> Self::Output {
        let mut mouse_cursor = mouse::Interaction::default();

        (
            Primitive::Group {
                primitives: {
                    elements
                        .iter()
                        .zip(layout.children())
                        .map(|(element, layout)| {
                            let (primitive, new_mouse_cursor) =
                                element.draw(self, defaults, layout, cursor_position, viewport);

                            if new_mouse_cursor > mouse_cursor {
                                mouse_cursor = new_mouse_cursor;
                            }

                            primitive
                        })
                        .collect()
                },
            },
            mouse_cursor,
        )
    }
}
*/
