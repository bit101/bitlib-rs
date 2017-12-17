pub fn approx_eq(m: f64, n: f64) -> bool {
    (m - n).abs() < 0.000001
}

#[test]
    fn test_approx() {
        assert!(approx_eq(3.141592, 355.0 / 113.0));
    }

