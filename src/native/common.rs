//! Common types for reuse.
//!

use iced_widget::core::{Padding, Rectangle};

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
    pub fn get_bounds(&self, outer_bounds: Rectangle) -> Rectangle {
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
