//! A dummy widget that draws a quad
//!
//! *This API requires the following crate features to be activated: `quad`*

use iced_widget::core::{
    layout::{Limits, Node},
    mouse::Cursor,
    renderer,
    widget::Tree,
    Color, Element, Layout, Length, Padding, Rectangle, Widget,
};

/// Methods for creating inner bounds
#[allow(missing_debug_implementations)]
pub enum InnerBounds {
    /// Create inner bounds ratio to the outer bounds
    Ratio(f32, f32),
    /// Create inner bounds by padding the outer bounds
    Padding(Padding),
    /// Create square inner bounds
    Square(f32),
    /// Create inner bounds with a custom function
    Custom(Box<dyn Fn(Rectangle) -> Rectangle>),
}
impl InnerBounds {
    /// Gets the inner bounds of the Set type.
    fn get_bounds(&self, outer_bounds: Rectangle) -> Rectangle {
        use InnerBounds::{Custom, Padding, Ratio, Square};
        match self {
            Ratio(w, h) => {
                let width = w * outer_bounds.width;
                let height = h * outer_bounds.height;
                let x = outer_bounds.x + (outer_bounds.width - width) * 0.5;
                let y = outer_bounds.y + (outer_bounds.height - height) * 0.5;
                Rectangle {
                    x,
                    y,
                    width,
                    height,
                }
            }
            Padding(p) => {
                let x = outer_bounds.x + p.left;
                let y = outer_bounds.y + p.top;
                let width = outer_bounds.width - p.horizontal();
                let height = outer_bounds.width - p.vertical();
                Rectangle {
                    x,
                    y,
                    width,
                    height,
                }
            }
            Square(l) => {
                let width = *l;
                let height = *l;
                let x = outer_bounds.x + (outer_bounds.width - width) * 0.5;
                let y = outer_bounds.y + (outer_bounds.height - height) * 0.5;
                Rectangle {
                    x,
                    y,
                    width,
                    height,
                }
            }
            Custom(f) => f(outer_bounds),
        }
    }
}

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

impl<Message, Renderer> Widget<Message, Renderer> for Quad
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);
        Node::new(limits.max())
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &<Renderer as renderer::Renderer>::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        if let Some(b) = self.background {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: self.border_radius.into(),
                    border_width: self.border_width,
                    border_color: self.border_color,
                },
                b,
            );
        }
        renderer.fill_quad(
            renderer::Quad {
                bounds: self.inner_bounds.get_bounds(layout.bounds()),
                border_radius: self.border_radius.into(),
                border_width: self.border_width,
                border_color: self.border_color,
            },
            self.color,
        );
    }
}
impl<Message, Renderer> From<Quad> for Element<'_, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(value: Quad) -> Self {
        Self::new(value)
    }
}
