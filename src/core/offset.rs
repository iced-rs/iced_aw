//! Offset struct

use iced_core::Point;

/// Represents an offset in a two-dimensional space.
#[derive(Copy, Clone, Debug)]
pub struct Offset {
    /// Offset on the x-axis
    pub x: f32,
    /// Offset on the y-axis
    pub y: f32,
}

impl Offset {
    /// Construct a new [`Offset`]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_offset() {
        let offset = Offset::new(10.0, 20.0);
        assert_eq!(offset.x, 10.0);
        assert_eq!(offset.y, 20.0);
    }

    #[test]
    fn new_handles_negative_values() {
        let offset = Offset::new(-5.0, -15.0);
        assert_eq!(offset.x, -5.0);
        assert_eq!(offset.y, -15.0);
    }

    #[test]
    fn new_handles_zero() {
        let offset = Offset::new(0.0, 0.0);
        assert_eq!(offset.x, 0.0);
        assert_eq!(offset.y, 0.0);
    }

    #[test]
    fn from_f32_creates_offset_with_same_values() {
        let offset = Offset::from(5.0);
        assert_eq!(offset.x, 5.0);
        assert_eq!(offset.y, 5.0);
    }

    #[test]
    fn from_f32_handles_negative() {
        let offset = Offset::from(-3.5);
        assert_eq!(offset.x, -3.5);
        assert_eq!(offset.y, -3.5);
    }

    #[test]
    fn from_array_creates_offset() {
        let offset = Offset::from([12.0, 34.0]);
        assert_eq!(offset.x, 12.0);
        assert_eq!(offset.y, 34.0);
    }

    #[test]
    fn from_array_handles_different_signs() {
        let offset = Offset::from([-7.5, 8.5]);
        assert_eq!(offset.x, -7.5);
        assert_eq!(offset.y, 8.5);
    }

    #[test]
    fn offset_to_point_conversion() {
        let offset = Offset::new(15.0, 25.0);
        let point: Point = offset.into();
        assert_eq!(point.x, 15.0);
        assert_eq!(point.y, 25.0);
    }

    #[test]
    fn offset_ref_to_point_conversion() {
        let offset = Offset::new(15.0, 25.0);
        let point: Point = (&offset).into();
        assert_eq!(point.x, 15.0);
        assert_eq!(point.y, 25.0);
    }

    #[test]
    fn offset_is_copy() {
        let offset1 = Offset::new(1.0, 2.0);
        let offset2 = offset1;
        // Both should be usable since Offset implements Copy
        assert_eq!(offset1.x, 1.0);
        assert_eq!(offset2.x, 1.0);
    }

    #[test]
    fn offset_is_clone() {
        let offset1 = Offset::new(3.0, 4.0);
        let offset2 = offset1.clone();
        assert_eq!(offset1.x, offset2.x);
        assert_eq!(offset1.y, offset2.y);
    }
}
