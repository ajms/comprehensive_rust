// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn magnitude(self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }
    pub fn dist(self, other: Self) -> f64 {
        (self - other).magnitude()
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new() -> Self {
        Polygon { points: Vec::new() }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    pub fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|point| point.x).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    pub fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        } else {
            let mut result: f64 = 0.0;
            let mut prev_point = self.points[0];
            for point in &self.points[1..] {
                result += prev_point.dist(*point);
                prev_point = *point;
            }
            result += prev_point.dist(self.points[0]);
            return result;
        }
    }
}

pub struct Circle {
    radius: i32,
    center: Point,
}

impl Circle {
    pub fn new(radius: i32, center: Point) -> Self {
        Circle { radius, center }
    }

    pub fn circumference(&self) -> f64 {
        2.0 * f64::from(self.radius) * std::f64::consts::PI
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circle) => circle.circumference(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(5, Point::new(10, 20))),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
pub fn exercise7() {}
