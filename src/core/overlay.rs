//! Helper functions for overlays
use iced_native::{Point, Size};

/// Trait containing functions for positioning of nodes.
pub trait Position {
    /// Centers this node around the given position. If the node is over the
    /// specified bounds it's bouncing back to be fully visible on screen.
    fn center_and_bounce(&mut self, position: Point, bounds: Size);
}

impl Position for iced_native::layout::Node {
    fn center_and_bounce(&mut self, position: Point, bounds: Size) {
        self.move_to(Point::new(
            (position.x - self.size().width / 2.0).max(0.0),
            (position.y - self.size().height / 2.0).max(0.0),
        ));

        self.move_to(Point::new(
            if self.bounds().x + self.bounds().width > bounds.width {
                (self.bounds().x - (self.bounds().width - (bounds.width - self.bounds().x)))
                    .max(0.0)
            } else {
                self.bounds().x
            },
            if self.bounds().y + self.bounds().height > bounds.height {
                (self.bounds().y - (self.bounds().height - (bounds.height - self.bounds().y)))
                    .max(0.0)
            } else {
                self.bounds().y
            },
        ));
    }
}
