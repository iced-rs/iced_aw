//! Helper functions for calculating the clock

use iced_core::Point;
use std::fmt::Display;

/// The size of the period on the clock based on the clock's size.
pub const PERIOD_PERCENTAGE: f32 = 0.1;

/// The radius of the hour on the clock based on the clock's size.
pub const HOUR_RADIUS_PERCENTAGE: f32 = 0.4;

/// The radius of the minute on the clock based on the clock's size.
pub const MINUTE_RADIUS_PERCENTAGE: f32 = 0.65;

/// The radius of the second on the clock based on the clock's size.
pub const SECOND_RADIUS_PERCENTAGE: f32 = 0.9;

/// The radius of the hour without the seconds on the clock based on the
/// clock's size.
pub const HOUR_RADIUS_PERCENTAGE_NO_SECONDS: f32 = 0.5;

/// The radius of the minute without the seconds on the clock based on the
/// clock's size.
pub const MINUTE_RADIUS_PERCENTAGE_NO_SECONDS: f32 = 0.9;

/// The current period of the clock.
#[derive(Clone, Debug)]
pub enum Period {
    /// Ante meridiem: Before noon.
    AM,
    /// Post meridiem: After noon.
    PM,
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AM => "AM",
                Self::PM => "PM",
            }
        )
    }
}

/// The radius nearest to the cursor position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NearestRadius {
    /// Nothing is near to the cursor position.
    None,
    /// Period is the nearest radius to the cursor position.
    Period,
    /// Hour is the nearest radius to the cursor position.
    Hour,
    /// Minute is the nearest radius to the cursor position.
    Minute,
    /// Second is the nearest radius to the cursor position.
    Second,
}

/// Distributes the amount of points on a circle with the given radius around the
/// center.
#[must_use]
pub fn circle_points(distance_radius: f32, center: Point, amount: u16) -> Vec<Point> {
    let part = std::f32::consts::TAU / f32::from(amount);

    let rotation =
        |(x, y): (f32, f32), t: f32| (x * t.cos() - y * t.sin(), x * t.sin() + y * t.cos());

    let points: Vec<(f32, f32)> = (0..amount).fold(Vec::new(), |mut v, i| {
        v.push(rotation((0.0, -distance_radius), part * f32::from(i)));
        v
    });

    let points: Vec<Point> = points
        .iter()
        .map(|p| Point::new(p.0 + center.x, p.1 + center.y))
        .collect();

    points
}

/// # Panics
/// Determines the nearest point with the smallest distance to the cursor
/// position. The index of the point is returned.
/// Will panic if distance vec can not compare a and b
#[must_use]
pub fn nearest_point(points: &[Point], cursor_position: Point) -> usize {
    let mut distance_vec: Vec<(usize, f32)> = points
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.distance(cursor_position)))
        .collect();

    distance_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Should be comparable"));

    distance_vec[0].0
}

/// # Panics
/// Determining the nearest radius to the position of the cursor position based
/// on the distance to the center.
/// Will panic if distance vec can not compare a and b
#[must_use]
pub fn nearest_radius(
    radii: &[(f32, NearestRadius)],
    cursor_position: Point,
    center: Point,
) -> NearestRadius {
    let distance = cursor_position.distance(center);

    let mut distance_vec: Vec<(f32, &NearestRadius)> = radii
        .iter()
        .map(|(r, n)| ((r - distance).abs(), n))
        .collect();

    distance_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("Should be comparable"));

    distance_vec[0].1.clone()
}

#[cfg(test)]
mod tests {
    use iced_core::{Point, Vector};

    use super::{NearestRadius, circle_points, nearest_point, nearest_radius};

    #[test]
    fn circle_points_test() {
        let points = circle_points(10.0, Point::new(0.0, 0.0), 10);
        let expected = vec![
            Point { x: 0.0, y: -10.0 },
            Point {
                x: 5.877_852_4,
                y: -8.09017,
            },
            Point {
                x: 9.510_566,
                y: -3.090_169_7,
            },
            Point {
                x: 9.510_565,
                y: 3.090_170_4,
            },
            Point {
                x: 5.877_852,
                y: 8.090_171,
            },
            Point {
                x: -0.000_000_874_227_8,
                y: 10.0,
            },
            Point {
                x: -5.877_853_4,
                y: 8.09017,
            },
            Point {
                x: -9.510_565,
                y: 3.090_170_9,
            },
            Point {
                x: -9.510_565,
                y: -3.090_171_3,
            },
            Point {
                x: -5.877_849_6,
                y: -8.090_173,
            },
        ];

        assert_eq!(points, expected);
    }

    #[test]
    fn nearest_radius_test() {
        let radii = vec![
            (10.0, NearestRadius::Period),
            (20.0, NearestRadius::Hour),
            (30.0, NearestRadius::Minute),
            (40.0, NearestRadius::Second),
        ];

        let center = Point::new(0.0, 0.0);

        let cursor_position = Point::new(5.0, 0.0);
        let result = nearest_radius(&radii, cursor_position, center);
        assert_eq!(result, NearestRadius::Period);

        let cursor_position = Point::new(16.0, 0.0);
        let result = nearest_radius(&radii, cursor_position, center);
        assert_eq!(result, NearestRadius::Hour);

        let cursor_position = Point::new(26.0, 0.0);
        let result = nearest_radius(&radii, cursor_position, center);
        assert_eq!(result, NearestRadius::Minute);

        let cursor_position = Point::new(36.0, 0.0);
        let result = nearest_radius(&radii, cursor_position, center);
        assert_eq!(result, NearestRadius::Second);
    }

    #[test]
    fn nearest_point_test() {
        let points = circle_points(10.0, Point::new(0.0, 0.0), 10);

        let mut index = 5;
        let cursor_position = points[index] + Vector::new(1.0, -1.0);
        let mut result = nearest_point(&points, cursor_position);
        assert_eq!(index, result);

        index = 0;
        let cursor_position = points[index] + Vector::new(-1.0, 1.0);
        result = nearest_point(&points, cursor_position);
        assert_eq!(index, result);
    }

    #[test]
    fn period_display_am() {
        use super::Period;
        let period = Period::AM;
        assert_eq!(format!("{period}"), "AM");
    }

    #[test]
    fn period_display_pm() {
        use super::Period;
        let period = Period::PM;
        assert_eq!(format!("{period}"), "PM");
    }

    #[test]
    fn circle_points_with_offset_center() {
        let points = circle_points(5.0, Point::new(10.0, 20.0), 4);
        assert_eq!(points.len(), 4);
        // Top point should be at (10, 15) - center.y - radius
        assert!((points[0].x - 10.0).abs() < 0.001);
        assert!((points[0].y - 15.0).abs() < 0.001);
    }

    #[test]
    fn circle_points_single_point() {
        let points = circle_points(10.0, Point::new(0.0, 0.0), 1);
        assert_eq!(points.len(), 1);
        // Single point should be at top (0, -10)
        assert!((points[0].x - 0.0).abs() < 0.001);
        assert!((points[0].y - (-10.0)).abs() < 0.001);
    }

    #[test]
    fn circle_points_twelve_points() {
        let points = circle_points(10.0, Point::new(0.0, 0.0), 12);
        assert_eq!(points.len(), 12);
        // 12 points should be evenly distributed (like clock hours)
        // First point should be at top
        assert!((points[0].x - 0.0).abs() < 0.001);
        assert!((points[0].y - (-10.0)).abs() < 0.001);
    }

    #[test]
    fn nearest_point_single_point() {
        let points = vec![Point::new(5.0, 5.0)];
        let cursor = Point::new(100.0, 100.0);
        let result = nearest_point(&points, cursor);
        assert_eq!(result, 0);
    }

    #[test]
    fn nearest_point_exact_match() {
        let points = circle_points(10.0, Point::new(0.0, 0.0), 8);
        // Cursor exactly on point 3
        let cursor = points[3];
        let result = nearest_point(&points, cursor);
        assert_eq!(result, 3);
    }

    #[test]
    fn nearest_radius_at_exact_radius() {
        let radii = vec![
            (10.0, NearestRadius::Period),
            (20.0, NearestRadius::Hour),
            (30.0, NearestRadius::Minute),
        ];
        let center = Point::new(0.0, 0.0);
        // Cursor exactly at radius 20
        let cursor = Point::new(20.0, 0.0);
        let result = nearest_radius(&radii, cursor, center);
        assert_eq!(result, NearestRadius::Hour);
    }

    #[test]
    fn nearest_radius_at_center() {
        let radii = vec![
            (10.0, NearestRadius::Period),
            (20.0, NearestRadius::Hour),
            (30.0, NearestRadius::Minute),
        ];
        let center = Point::new(0.0, 0.0);
        // Cursor at center
        let cursor = Point::new(0.0, 0.0);
        let result = nearest_radius(&radii, cursor, center);
        // Should pick the smallest radius (Period at 10.0)
        assert_eq!(result, NearestRadius::Period);
    }

    #[test]
    fn nearest_radius_beyond_largest() {
        let radii = vec![
            (10.0, NearestRadius::Period),
            (20.0, NearestRadius::Hour),
            (30.0, NearestRadius::Minute),
        ];
        let center = Point::new(0.0, 0.0);
        // Cursor far beyond all radii
        let cursor = Point::new(100.0, 0.0);
        let result = nearest_radius(&radii, cursor, center);
        // Should still pick the closest (Minute at 30.0)
        assert_eq!(result, NearestRadius::Minute);
    }

    #[test]
    fn nearest_radius_with_offset_center() {
        let radii = vec![(5.0, NearestRadius::Period), (15.0, NearestRadius::Hour)];
        let center = Point::new(50.0, 50.0);
        let cursor = Point::new(62.0, 50.0);
        // Distance from center is 12, closer to Hour (15) than Period (5)
        // |5 - 12| = 7, |15 - 12| = 3, so Hour is closer
        let result = nearest_radius(&radii, cursor, center);
        assert_eq!(result, NearestRadius::Hour);
    }

    #[test]
    fn nearest_radius_none_variant() {
        let radii = vec![(10.0, NearestRadius::None), (20.0, NearestRadius::Hour)];
        let center = Point::new(0.0, 0.0);
        let cursor = Point::new(11.0, 0.0);
        let result = nearest_radius(&radii, cursor, center);
        // Should pick None as it's closer to radius 10
        assert_eq!(result, NearestRadius::None);
    }

    #[test]
    fn circle_points_large_amount() {
        let points = circle_points(10.0, Point::new(0.0, 0.0), 60);
        assert_eq!(points.len(), 60);
        // All points should be approximately 10.0 away from center
        let center = Point::new(0.0, 0.0);
        for point in points {
            let distance = point.distance(center);
            assert!((distance - 10.0).abs() < 0.001);
        }
    }

    #[test]
    fn constants_are_defined() {
        use super::*;
        // Just verify the constants are accessible and have expected values

        assert_eq!(PERIOD_PERCENTAGE, 0.1);
        assert_eq!(HOUR_RADIUS_PERCENTAGE, 0.4);
        assert_eq!(MINUTE_RADIUS_PERCENTAGE, 0.65);
        assert_eq!(SECOND_RADIUS_PERCENTAGE, 0.9);
        assert_eq!(HOUR_RADIUS_PERCENTAGE_NO_SECONDS, 0.5);
        assert_eq!(MINUTE_RADIUS_PERCENTAGE_NO_SECONDS, 0.9);
    }
}
