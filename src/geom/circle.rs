use super::point::Point;
use super::dist;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl Circle {
    pub fn contains_point(&self, p: &Point) -> bool {
        dist(p, &self.center()) <= self.r
    }

    pub fn intersects_circle(&self, other: Circle) -> bool {
        let d = dist(&self.center(), &other.center());
        d < self.r + other.r
    }

    pub fn center(&self) -> Point {
        Point::new(self.x, self.y)
    }
}
