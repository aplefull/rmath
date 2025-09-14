use approx::assert_relative_eq;
use rmath::geometry::*;

#[test]
fn test_step_basic() {
    assert_eq!(step(0.0, -1.0), 0.0);
    assert_eq!(step(0.0, 0.0), 1.0);
    assert_eq!(step(0.0, 1.0), 1.0);
}

#[test]
fn test_step_different_edges() {
    assert_eq!(step(5.0, 3.0), 0.0);
    assert_eq!(step(5.0, 5.0), 1.0);
    assert_eq!(step(5.0, 7.0), 1.0);

    assert_eq!(step(-2.0, -5.0), 0.0);
    assert_eq!(step(-2.0, -2.0), 1.0);
    assert_eq!(step(-2.0, 0.0), 1.0);
}

#[test]
fn test_step_continuity() {
    let edge = 3.5;
    assert_eq!(step(edge, edge - 0.1), 0.0);
    assert_eq!(step(edge, edge), 1.0);
    assert_eq!(step(edge, edge + 0.1), 1.0);
}

#[test]
fn test_sign_positive() {
    assert_eq!(sign(1.0), 1.0);
    assert_eq!(sign(0.1), 1.0);
    assert_eq!(sign(100.0), 1.0);
    assert_eq!(sign(f64::INFINITY), 1.0);
}

#[test]
fn test_sign_negative() {
    assert_eq!(sign(-1.0), -1.0);
    assert_eq!(sign(-0.1), -1.0);
    assert_eq!(sign(-100.0), -1.0);
    assert_eq!(sign(f64::NEG_INFINITY), -1.0);
}

#[test]
fn test_sign_zero() {
    assert_eq!(sign(0.0), 0.0);
    assert_eq!(sign(-0.0), 0.0);
}

#[test]
fn test_sign_nan() {
    assert_eq!(sign(f64::NAN), 0.0);
}

#[test]
fn test_fmod_basic() {
    assert_relative_eq!(fmod(5.0, 3.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(fmod(7.0, 2.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(fmod(8.0, 4.0), 0.0, epsilon = 1e-10);
}

#[test]
fn test_fmod_negative_dividend() {
    assert_relative_eq!(fmod(-5.0, 3.0), -2.0, epsilon = 1e-10);
    assert_relative_eq!(fmod(-7.0, 2.0), -1.0, epsilon = 1e-10);
}

#[test]
fn test_fmod_negative_divisor() {
    assert_relative_eq!(fmod(5.0, -3.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(fmod(7.0, -2.0), 1.0, epsilon = 1e-10);
}

#[test]
fn test_fmod_both_negative() {
    assert_relative_eq!(fmod(-5.0, -3.0), -2.0, epsilon = 1e-10);
    assert_relative_eq!(fmod(-7.0, -2.0), -1.0, epsilon = 1e-10);
}

#[test]
fn test_fmod_fractional() {
    assert_relative_eq!(fmod(5.5, 2.0), 1.5, epsilon = 1e-10);
    assert_relative_eq!(fmod(3.14, 1.0), 0.14, epsilon = 1e-10);
}

#[test]
fn test_fmod_zero_dividend() {
    assert_relative_eq!(fmod(0.0, 5.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(fmod(0.0, -3.0), 0.0, epsilon = 1e-10);
}

#[test]
fn test_fmod_zero_divisor() {
    assert!(fmod(5.0, 0.0).is_nan());
    assert!(fmod(-3.0, 0.0).is_nan());
}

#[test]
fn test_fmod_same_values() {
    assert_relative_eq!(fmod(5.0, 5.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(fmod(-3.0, -3.0), 0.0, epsilon = 1e-10);
}

#[test]
fn test_fmod_small_divisor() {
    assert_relative_eq!(fmod(1.0, 0.3), 0.1, epsilon = 1e-10);
}
