use std::f64::consts;

pub static PI: f64 = consts::PI; 
pub static TWO_PI: f64 = consts::PI * 2.0;
pub static HALF_PI: f64 = consts::PI / 2.0;

pub fn norm(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}

pub fn lerp(min: f64, max: f64, t: f64) -> f64 {
    min + (max - min) * t
}

pub fn map(src_value: f64, src_min: f64, src_max: f64, dst_min: f64, dst_max: f64) -> f64 {
    let norm = norm(src_value, src_min, src_max);
    lerp(dst_min, dst_max, norm)
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    // let min and max be reversed and still work.
    let mut real_min = min;
    let mut real_max = max;
    if min > max {
        real_min = max;
        real_max = min;
    }
    let mut result = value;
    if value < real_min {
        result = real_min;
    }
    if value > real_max {
        result = real_max;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm() {
        // in range
        assert_eq!(norm(25.0, 0.0, 100.0), 0.25);
        assert_eq!(norm(50.0, 0.0, 100.0), 0.5);
        assert_eq!(norm(0.0, 0.0, 100.0), 0.0);
        assert_eq!(norm(100.0, 0.0, 100.0), 1.0);
        // reverse range
        assert_eq!(norm(25.0, 100.0, 0.0), 0.75);
        // below range
        assert_eq!(norm(-25.0, 0.0, 100.0), -0.25);
        // above range
        assert_eq!(norm(125.0, 0.0, 100.0), 1.25);
    }

    #[test]
    fn test_lerp() {
        // in range
        assert_eq!(lerp(0.0, 100.0, 0.25), 25.0);
        assert_eq!(lerp(0.0, 100.0, 0.5), 50.0);
        assert_eq!(lerp(0.0, 100.0, 0.75), 75.0);
        assert_eq!(lerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 100.0, 1.0), 100.0);
        // reverse range
        assert_eq!(lerp(100.0, 0.0, 0.25), 75.0);
        // below range
        assert_eq!(lerp(0.0, 100.0, -0.25), -25.0);
        // above range
        assert_eq!(lerp(0.0, 100.0, 1.25), 125.0);
    }

    #[test]
    fn test_clamp() {
        // in range
        assert_eq!(clamp(0.0, 0.0, 100.0), 0.0);
        assert_eq!(clamp(100.0, 0.0, 100.0), 100.0);
        // out of range
        assert_eq!(clamp(-50.0, 0.0, 100.0), 0.0);
        assert_eq!(clamp(110.0, 0.0, 100.0), 100.0);
        // reversed min/max
        assert_eq!(clamp(-50.0, 100.0, 0.0), 0.0);
        assert_eq!(clamp(110.0, 100.0, 0.0), 100.0);
    }
}

