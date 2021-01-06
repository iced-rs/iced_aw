//! Helper functions for calculating the clock

use std::fmt::Display;

use iced_graphics::Point;

/// TODO
pub const PERIOD_PERCENTAGE: f32 = 0.1;

/// TODO
pub const HOUR_RADIUS_PERCENTAGE: f32 = 0.4;

/// TODO
pub const MINUTE_RADIUS_PERCENTAGE: f32 = 0.65;

/// TODO
pub const SECOND_RADIUS_PERCENTAGE: f32 = 0.9;

/// TODO
pub const HOUR_RADIUS_PERCENTAGE_NO_SECONDS: f32 = 0.5;

/// TODO
pub const MINUTE_RADIUS_PERCENTAGE_NO_SECONDS: f32 = 0.9;

/// TODO
#[derive(Clone, Debug)]
pub enum Period {
    /// TODO
    AM,
    /// TODO
    PM
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Period::AM => "AM",
            Period::PM => "PM",
        })
    }
}

/// TODO
#[derive(Clone, Debug, PartialEq)]
pub enum NearestRadius {
    /// TODO
    None,
    /// TODO
    Period,
    /// TODO
    Hour,
    /// TODO
    Minute,
    /// TODO
    Second,
}

/// TODO
pub fn nearest_radius(radii: &[(f32, NearestRadius)], cursor_position: Point, center: Point) -> NearestRadius {
    let distance = cursor_position.distance(center);

    let mut distance_vec: Vec<(f32, &NearestRadius)> = radii.iter()
        .map(|(r, n)| ((r - distance).abs(), n) )
        .collect();

    distance_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    distance_vec[0].1.clone()
}

/// TODO
pub fn nearest_point(points: &[Point], cursor_position: Point) -> usize {

    let mut distance_vec: Vec<(usize, f32)> = points.iter()
        .enumerate()
        .map(|(i, p)| (i, p.distance(cursor_position)))
        .collect();

    distance_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    distance_vec[0].0
}

/// TODO
pub fn circle_points(distance_radius: f32, center: Point, amount: u32) -> Vec<Point> {
    let part = std::f32::consts::TAU / amount as f32;

    let rotation = |(x, y): (f32, f32), t: f32| {
        (
            x * t.cos() - y * t.sin(),
            x * t.sin() + y * t.cos()
        )
    };

    let points: Vec<(f32, f32)> = (0..amount).into_iter()
        .fold(
            Vec::new(),
            |mut v, i| {
                v.push(rotation((0.0, -distance_radius), part * i as f32));
                v
            }
        );

    let points: Vec<Point> = points.iter()
        .map(|p| Point::new(p.0 + center.x, p.1 + center.y))
        .collect();

    points
}

#[cfg(test)]
mod tests {
    use iced_graphics::{Point, Vector};

    use super::{NearestRadius, circle_points, nearest_point, nearest_radius};

    #[test]
    fn circle_points_test() {
        let points = circle_points(10.0, Point::new(0.0, 0.0), 10);
        let expected = vec![
            Point { x: 0.0, y: -10.0 },
            Point { x: 5.8778524, y: -8.09017 },
            Point { x: 9.510566, y: -3.0901697 },
            Point { x: 9.510565, y: 3.0901704 },
            Point { x: 5.877852, y: 8.090171 },
            Point { x: -0.0000008742278, y: 10.0 },
            Point { x: -5.8778534, y: 8.09017 },
            Point { x: -9.510565, y: 3.0901709 },
            Point { x: -9.510565, y: -3.0901713 },
            Point { x: -5.8778496, y: -8.090173 }
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

        let index = 5;
        let cursor_position = points[index] + Vector::new(1.0, -1.0);
        let result = nearest_point(&points, cursor_position);
        assert_eq!(index, result);

        let index = 0;
        let cursor_position = points[index] + Vector::new(-1.0, 1.0);
        let result = nearest_point(&points, cursor_position);
        assert_eq!(index, result);
    }
}