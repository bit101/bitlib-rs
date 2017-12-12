pub mod point;
pub mod rect;
pub mod circle;

use math::lerp;
use self::point::Point;
use self::circle::Circle;

pub fn dot_product(p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> f64 {
    let dx0 = p1.x - p0.x;
    let dy0 = p1.y - p0.y;
    let dx1 = p3.x - p2.x;
    let dy1 = p3.y - p2.y;
    dx0 * dx1 + dy0 * dy1
}

pub fn angle_between(p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> f64 {
    let dp = dot_product(p0, p1, p2, p3);
    let mag0 = dist(p0, p1);
    let mag1 = dist(p2, p3);
    (dp / mag0 / mag1).acos()
}


pub fn dist(p0: &Point, p1: &Point) -> f64 {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn lerp_point(p0: Point, p1: Point, t: f64) -> Point {
    Point::new(lerp(p0.x, p1.x, t), lerp(p0.y, p1.y, t))
}

pub fn bezier_point(p0: Point, p1: Point, p2: Point, p3: Point, t: f64) -> Point {
    let one_minus_t = 1.0 - t;
    let m0 = one_minus_t * one_minus_t * one_minus_t;
    let m1 = 3.0 * one_minus_t * one_minus_t * t;
    let m2 = 3.0 * one_minus_t * t * t;
    let m3 = t * t * t;
    Point {
        x: m0 * p0.x + m1 * p1.x + m2 * p2.x + m3 * p3.x,
        y: m0 * p0.y + m1 * p1.y + m2 * p2.y + m3 * p3.y
    }
}

pub fn quadratic_point(p0: Point, p1: Point, p2: Point, t: f64) -> Point {
    let one_minus_t = 1.0 - t;
    let m0 = one_minus_t * one_minus_t;
    let m1 = 2.0 * one_minus_t * t;
    let m2 = t * t;
    Point {
        x: m0 * p0.x + m1 * p1.x + m2 * p2.x,
        y: m0 * p0.y + m1 * p1.y + m2 * p2.y
    }
}

pub fn segment_intersect(p0: Point, p1: Point, p2: Point, p3: Point) -> Option<Point> {
    let a1 = p1.y - p0.y;
    let b1 = p0.x - p1.x;
    let c1 = a1 * p0.x + b1 * p0.y;
    let a2 = p3.y - p2.y;
    let b2 = p2.x - p3.x;
    let c2 = a2 * p2.x + b2 * p2.y;
    let denominator = a1 * b2 - a2 * b1;

    if denominator == 0.0 {
        None 
    }
    else {
        let intersect_x = (b2 * c1 - b1 * c2) / denominator;
        let intersect_y = (a1 * c2 - a2 * c1) / denominator;
        let rx0 = (intersect_x - p0.x) / (p1.x - p0.x);
        let ry0 = (intersect_y - p0.y) / (p1.y - p0.y);
        let rx1 = (intersect_x - p2.x) / (p3.x - p2.x);
        let ry1 = (intersect_y - p2.y) / (p3.y - p2.y);

        if ((rx0 >= 0.0 && rx0 <= 1.0) || (ry0 >= 0.0 && ry0 <= 1.0)) &&
           ((rx1 >= 0.0 && rx1 <= 1.0) || (ry1 >= 0.0 && ry1 <= 1.0)) {
            Some(Point {
                x: intersect_x,
                y: intersect_y
            })
        }
        else {
            None
        }
    }
}

pub fn tangent_point_to_circle(point: &Point, circle: &Circle, anticlockwise: bool) -> Point {
    let d = dist(point, &circle.center());
    let mut dir = -1.0;
    if anticlockwise {
        dir = 1.0;
    }
    let angle = (-circle.r / d).cos() * dir;
    let base_angle = (circle.y - point.y).atan2(circle.x - point.x);
    let total_angle = base_angle + angle;

    Point {
        x: circle.x + (total_angle).cos() * circle.r,
        y: circle.y + (total_angle).sin() * circle.r
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // fn almost_eq(m: f64, n: f64) -> bool {
    //     let max_diff = 0.0001;
    //     let diff = f64::abs(m - n);
    //     diff < max_diff
    // }

    #[test]
    fn test_intersect() {
        let p0 = Point::new(-10.0, 0.0);
        let p1 = Point::new( 10.0, 0.0);
        let p2 = Point::new(0.0, -10.0);
        let p3 = Point::new(0.0,  10.0);
        assert_eq!(segment_intersect(p0, p1, p2, p3).expect("no intersection"), Point::new(0.0, 0.0));
    }

    #[test]
    fn test_no_intersect() {
        let p0 = Point::new(-10.0, 0.0);
        let p1 = Point::new( 10.0, 0.0);
        let p2 = Point::new(20.0, -10.0);
        let p3 = Point::new(20.0,  10.0);
        assert_eq!(segment_intersect(p0, p1, p2, p3), None);
    }

}
