//! Helper functions for overlays

use iced_widget::core::{layout, Point, Size};

/// Trait containing functions for positioning of nodes.
pub trait Position {
    /// Centers this node around the given position. If the node is over the
    /// specified bounds it's bouncing back to be fully visible on screen.
    fn center_and_bounce(self, position: Point, bounds: Size) -> Self;
}

impl Position for layout::Node {
    fn center_and_bounce(self, position: Point, bounds: Size) -> Self {
        let size = self.size();
        let new_self = self.move_to(Point::new(
            (position.x - size.width / 2.0).max(0.0),
            (position.y - size.height / 2.0).max(0.0),
        ));

        let new_self_bounds = new_self.bounds();

        new_self.move_to(Point::new(
            if new_self_bounds.x + new_self_bounds.width > bounds.width {
                (new_self_bounds.x - (new_self_bounds.width - (bounds.width - new_self_bounds.x)))
                    .max(0.0)
            } else {
                new_self_bounds.x
            },
            if new_self_bounds.y + new_self_bounds.height > bounds.height {
                (new_self_bounds.y - (new_self_bounds.height - (bounds.height - new_self_bounds.y)))
                    .max(0.0)
            } else {
                new_self_bounds.y
            },
        ))
    }
}
