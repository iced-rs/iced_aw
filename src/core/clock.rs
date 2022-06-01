//! Helper functions for calculating the clock

use std::fmt::Display;

use iced_graphics::Point;

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

/// Determining the nearest radius to the position of the cursor position based
/// on the distance to the center.
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

/// Determines the nearest point with the smallest distance to the cursor
/// position. The index of the point is returned.
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

/// Distributes the amount of points on a circle with the given radius around the
/// center.
#[must_use]
pub fn circle_points(distance_radius: f32, center: Point, amount: u16) -> Vec<Point> {
    let part = std::f32::consts::TAU / f32::from(amount);

    let rotation =
        |(x, y): (f32, f32), t: f32| (x * t.cos() - y * t.sin(), x * t.sin() + y * t.cos());

    let points: Vec<(f32, f32)> = (0..amount).into_iter().fold(Vec::new(), |mut v, i| {
        v.push(rotation((0.0, -distance_radius), part * f32::from(i)));
        v
    });

    let points: Vec<Point> = points
        .iter()
        .map(|p| Point::new(p.0 + center.x, p.1 + center.y))
        .collect();

    points
}

#[cfg(test)]
mod tests {
    use iced_graphics::{Point, Vector};

    use super::{circle_points, nearest_point, nearest_radius, NearestRadius};

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
}
