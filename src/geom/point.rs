#[derive(PartialEq)]
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn from_polar(angle: f64, radius: f64) -> Point {
        Point::new((angle).cos() * radius, (angle).sin() * radius)
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dist(&self, other: &Point) -> f64 {
        super::dist(self, other)
    }

    pub fn translate(&self, x: f64, y: f64) -> Point {
        Point::new(self.x + x, self.y + y)
    }

    pub fn scale(&self, scale_x: f64, scale_y: f64) -> Point {
        Point::new(self.x * scale_x, self.y * scale_y)
    }

    pub fn rotate(&self, angle: &f64) -> Point {
        let x = self.x * angle.cos() + self.y * angle.sin();
        let y = self.y * angle.cos() - self.x * angle.sin();
        Point::new(x, y)
    }

    pub fn clone(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn almost_eq(m: f64, n: f64) -> bool {
        let max_diff = 0.0001;
        let diff = f64::abs(m - n);
        diff < max_diff
    }

    #[test]
    fn test_new_point() {
        let point = Point::new(0.0, 0.0);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
        let point = Point::new(10.0, -20.0);
        assert_eq!(point.x, 10.0);
        assert_eq!(point.y, -20.0);
    }

    #[test]
    fn test_from_polar() {
        let point = Point::from_polar(0.0, 100.0);
        assert_eq!(point.x, 100.0);
        assert_eq!(point.y, 0.0);
        let point = Point::from_polar(-PI / 2.0, 100.0);
        assert!(almost_eq(point.x, 0.0));
        assert!(almost_eq(point.y, -100.0));
        let point = Point::from_polar(PI, 100.0);
        assert!(almost_eq(point.x, -100.0));
        assert!(almost_eq(point.y, 0.0));
        let point = Point::from_polar(PI / 2.0, 100.0);
        assert!(almost_eq(point.x, 0.0));
        assert!(almost_eq(point.y, 100.0));
        let point = Point::from_polar(0.0, 0.0);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }

    #[test]
    fn test_angle() {
        let point = Point::new(100.0, 0.0);
        assert_eq!(point.angle(), 0.0);
        let point = Point::new(100.0, 100.0);
        assert_eq!(point.angle(), PI / 4.0);
        let point = Point::new(0.0, 100.0);
        assert_eq!(point.angle(), PI / 2.0);
        let point = Point::new(-100.0, 0.0);
        assert_eq!(point.angle(), PI);
        let point = Point::new(0.0, -100.0);
        assert_eq!(point.angle(), -PI / 2.0);
    }

    #[test]
    fn test_magnitude() {
        let point = Point::new(100.0, 0.0);
        assert_eq!(point.magnitude(), 100.0);
        let point = Point::new(0.0, 100.0);
        assert_eq!(point.magnitude(), 100.0);
        let point = Point::new(-100.0, 0.0);
        assert_eq!(point.magnitude(), 100.0);
        let point = Point::new(0.0, -100.0);
        assert_eq!(point.magnitude(), 100.0);
        let point = Point::new(3.0, 4.0);
        assert_eq!(point.magnitude(), 5.0);
        let point = Point::new(0.0, 0.0);
        assert_eq!(point.magnitude(), 0.0);
    }

    #[test]
    fn test_dist() {
        let point_a = Point::new(10.0, 10.0); 
        let point_b = Point::new(13.0, 14.0); 
        assert_eq!(point_a.dist(&point_b), 5.0);
        let point_a = Point::new(-15.0, -10.0); 
        let point_b = Point::new(25.0, 20.0); 
        assert_eq!(point_a.dist(&point_b), 50.0);
    }
}


