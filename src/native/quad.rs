//! A dummy widget that draws a quad
//!
//! *This API requires the following crate features to be activated: `quad`*

use crate::native::InnerBounds;
use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::Tree,
        Layout, Widget,
    },
    mouse::Cursor,
    Border, Color, Element, Length, Rectangle, Shadow, Size,
};

/// A dummy widget that draws a quad
#[allow(missing_debug_implementations)]
pub struct Quad {
    /// Width of the quad
    pub width: Length,
    /// Height of the quad
    pub height: Length,
    /// Color of the quad
    pub color: Color,
    /// Background color of the quad
    pub background: Option<Color>,
    /// Methods for creating inner bounds
    pub inner_bounds: InnerBounds,
    /// Border radius of the Quad
    pub border_radius: [f32; 4],
    /// Border width of the quad
    pub border_width: f32,
    /// Border color of the quad
    pub border_color: Color,
}
impl Default for Quad {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            color: Color::from([0.5; 3]),
            background: None,
            inner_bounds: InnerBounds::Ratio(0.5, 0.5),
            border_radius: [0.0, 0.0, 0.0, 0.0],
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Quad
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        Node::new(limits.max())
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        if let Some(b) = self.background {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        radius: self.border_radius.into(),
                        width: self.border_width,
                        color: self.border_color,
                    },
                    shadow: Shadow::default(),
                },
                b,
            );
        }
        renderer.fill_quad(
            renderer::Quad {
                bounds: self.inner_bounds.get_bounds(layout.bounds()),
                border: Border {
                    radius: self.border_radius.into(),
                    width: self.border_width,
                    color: self.border_color,
                },
                shadow: Shadow::default(),
            },
            self.color,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Quad> for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Theme: 'a,
{
    fn from(value: Quad) -> Self {
        Self::new(value)
    }
}
