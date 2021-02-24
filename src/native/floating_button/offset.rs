//! Use a floating button to overlay a button over some content
//!
//! *This API requires the following crate features to be activated: `floating_button`*

use iced_native::Point;

/// The [`Offset`](Offset) for the [`FloatingButton`](super::FloatingButton).
#[derive(Copy, Clone, Debug)]
pub struct Offset {
    /// Offset on the x-axis from the [`Anchor`](super::Anchor)
    pub x: f32,
    /// Offset on the y-axis from the [`Anchor`](super::Anchor)
    pub y: f32,
}

impl From<f32> for Offset {
    fn from(float: f32) -> Self {
        Self { x: float, y: float }
    }
}

impl From<[f32; 2]> for Offset {
    fn from(array: [f32; 2]) -> Self {
        Self {
            x: array[0],
            y: array[1],
        }
    }
}

impl From<Offset> for Point {
    fn from(offset: Offset) -> Self {
        Self::new(offset.x, offset.y)
    }
}

impl From<&Offset> for Point {
    fn from(offset: &Offset) -> Self {
        Self::new(offset.x, offset.y)
    }
}
