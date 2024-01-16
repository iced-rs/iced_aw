//! Helper functions for overlays

use iced_widget::core::{layout, Point, Size};

/// Trait containing functions for positioning of nodes.
pub trait Position {
    /// Centers this node around the given position. If the node is over the
    /// specified bounds it's bouncing back to be fully visible on screen.
    fn center_and_bounce(self, position: Point, bounds: Size) -> Self;
}

impl Position for layout::Node {
    fn center_and_bounce(self, position: Point, _bounds: Size) -> Self {
        let size = self.size();
        let mut new_self = self.move_to(Point::new(
            (position.x - size.width / 2.0).max(0.0),
            (position.y - size.height / 2.0).max(0.0),
        ));

        let bounds = new_self.bounds();

        new_self = new_self.move_to(Point::new(
            if bounds.x + bounds.width > bounds.width {
                (bounds.x - (bounds.width - (bounds.width - bounds.x))).max(0.0)
            } else {
                bounds.x
            },
            if bounds.y + bounds.height > bounds.height {
                (bounds.y - (bounds.height - (bounds.height - bounds.y))).max(0.0)
            } else {
                bounds.y
            },
        ));

        new_self
    }
}
