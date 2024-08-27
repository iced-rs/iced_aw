//! A dummy widget that draws a quad
//!
//! *This API requires the following crate features to be activated: `quad`*

use crate::widget::InnerBounds;
use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::Tree,
        Layout, Widget,
    },
    mouse::Cursor,
    Background, Border, Color, Element, Length, Rectangle, Shadow, Size,
};

/// A dummy widget that draws a quad
#[allow(missing_debug_implementations)]
pub struct Quad {
    /// Width of the quad
    pub width: Length,
    /// Height of the quad
    pub height: Length,

    /// Methods for creating inner bounds
    pub inner_bounds: InnerBounds,

    /// Color of the quad
    pub quad_color: Background,
    /// Border of the quad
    pub quad_border: Border,
    /// Shadow of the quad
    pub quad_shadow: Shadow,

    /// Background color of the quad
    pub bg_color: Option<Background>,
    /// Border of the background
    pub bg_border: Border,
    /// Shadow of the background
    pub bg_shadow: Shadow,
}
impl Default for Quad {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            inner_bounds: InnerBounds::Ratio(0.5, 0.5),

            quad_color: Color::from([0.5; 3]).into(),
            quad_border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            quad_shadow: Shadow::default(),

            bg_color: None,
            bg_border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            bg_shadow: Shadow::default(),
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
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.intersects(viewport) {
            if let Some(b) = self.bg_color {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border: self.bg_border,
                        shadow: self.bg_shadow,
                    },
                    b,
                );
            }
            renderer.fill_quad(
                renderer::Quad {
                    bounds: self.inner_bounds.get_bounds(bounds),
                    border: self.quad_border,
                    shadow: self.quad_shadow,
                },
                self.quad_color,
            );
        }
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
