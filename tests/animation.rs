use approx::assert_relative_eq;
use rmath::animation::*;

#[test]
fn test_ease_in_quad_basic() {
    assert_eq!(ease_in_quad(0.0), 0.0);
    assert_eq!(ease_in_quad(1.0), 1.0);
    assert_eq!(ease_in_quad(0.5), 0.25);
}

#[test]
fn test_ease_in_quad_curve() {
    assert!(ease_in_quad(0.25) < 0.25);
    assert!(ease_in_quad(0.75) > 0.5);
}

#[test]
fn test_ease_out_quad_basic() {
    assert_eq!(ease_out_quad(0.0), 0.0);
    assert_eq!(ease_out_quad(1.0), 1.0);
    assert_eq!(ease_out_quad(0.5), 0.75);
}

#[test]
fn test_ease_out_quad_curve() {
    assert!(ease_out_quad(0.25) > 0.25);
    assert!(ease_out_quad(0.75) < 1.0);
}

#[test]
fn test_ease_in_out_quad_basic() {
    assert_eq!(ease_in_out_quad(0.0), 0.0);
    assert_eq!(ease_in_out_quad(1.0), 1.0);
    assert_eq!(ease_in_out_quad(0.5), 0.5);
}

#[test]
fn test_ease_in_out_quad_symmetry() {
    assert_relative_eq!(ease_in_out_quad(0.25), 1.0 - ease_in_out_quad(0.75), epsilon = 1e-10);
    assert_relative_eq!(ease_in_out_quad(0.1), 1.0 - ease_in_out_quad(0.9), epsilon = 1e-10);
}

#[test]
fn test_ease_in_out_quad_curve() {
    assert!(ease_in_out_quad(0.25) < 0.25);
    assert!(ease_in_out_quad(0.75) > 0.75);
}

#[test]
fn test_easing_monotonic() {
    let values = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];

    for i in 0..values.len() - 1 {
        assert!(ease_in_quad(values[i]) <= ease_in_quad(values[i + 1]));
        assert!(ease_out_quad(values[i]) <= ease_out_quad(values[i + 1]));
        assert!(ease_in_out_quad(values[i]) <= ease_in_out_quad(values[i + 1]));
    }
}

#[test]
fn test_bounce_basic() {
    assert_eq!(bounce(0.0), 0.0);
    assert_eq!(bounce(1.0), 1.0);
    assert_eq!(bounce(2.0), 0.0);
}

#[test]
fn test_bounce_symmetry() {
    assert_relative_eq!(bounce(0.5), bounce(1.5), epsilon = 1e-10);
    assert_relative_eq!(bounce(0.3), bounce(1.7), epsilon = 1e-10);
}

#[test]
fn test_bounce_negative() {
    assert_eq!(bounce(-0.5), bounce(0.5));
    assert_eq!(bounce(-1.0), bounce(1.0));
}

#[test]
fn test_bounce_range() {
    for i in 0..100 {
        let x = i as f64 * 0.1;
        let b = bounce(x);
        assert!(b >= 0.0 && b <= 2.0);
    }
}

#[test]
fn test_elastic_boundary() {
    assert_eq!(elastic(0.0), 0.0);
    assert_eq!(elastic(1.0), 1.0);
}

#[test]
fn test_elastic_oscillation() {
    let mid_values = [0.1, 0.3, 0.5, 0.7, 0.9];
    for &x in &mid_values {
        let val = elastic(x);
        assert!(val.abs() > 0.0);
    }
}

#[test]
fn test_elastic_range() {
    for i in 1..100 {
        let x = i as f64 * 0.01;
        let e = elastic(x);
        assert!(e.is_finite());
    }
}

#[test]
fn test_animation_curves_at_endpoints() {
    let functions = [ease_in_quad, ease_out_quad, ease_in_out_quad];

    for func in &functions {
        assert_eq!(func(0.0), 0.0);
        assert_eq!(func(1.0), 1.0);
    }
}

#[test]
fn test_bounce_periodicity() {
    for i in 0..10 {
        let offset = i as f64 * 2.0;
        assert_relative_eq!(bounce(0.3), bounce(0.3 + offset), epsilon = 1e-10);
        assert_relative_eq!(bounce(0.7), bounce(0.7 + offset), epsilon = 1e-10);
    }
}
