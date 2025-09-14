use approx::{assert_relative_eq};
use rmath::rounding::*;

#[test]
fn test_floor_basic() {
    assert_relative_eq!(floor(3.7), 3.0, epsilon = 1e-10);
    assert_relative_eq!(floor(-2.3), -3.0, epsilon = 1e-10);
    assert_relative_eq!(floor(5.0), 5.0, epsilon = 1e-10);
    assert_relative_eq!(floor(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(floor(-0.9), -1.0, epsilon = 1e-10);
}

#[test]
fn test_ceiling_basic() {
    assert_relative_eq!(ceiling(3.7), 4.0, epsilon = 1e-10);
    assert_relative_eq!(ceiling(-2.3), -2.0, epsilon = 1e-10);
    assert_relative_eq!(ceiling(5.0), 5.0, epsilon = 1e-10);
    assert_relative_eq!(ceiling(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(ceiling(0.1), 1.0, epsilon = 1e-10);
}

#[test]
fn test_round_basic() {
    assert_relative_eq!(round(3.7), 4.0, epsilon = 1e-10);
    assert_relative_eq!(round(3.3), 3.0, epsilon = 1e-10);
    assert_relative_eq!(round(-2.7), -3.0, epsilon = 1e-10);
    assert_relative_eq!(round(-2.3), -2.0, epsilon = 1e-10);
    assert_relative_eq!(round(5.0), 5.0, epsilon = 1e-10);
    assert_relative_eq!(round(0.5), 1.0, epsilon = 1e-10);
    assert_relative_eq!(round(-0.5), -1.0, epsilon = 1e-10);
}

#[test]
fn test_integer_and_fractional_parts() {
    assert_relative_eq!(integer_part(3.7), 3.0, epsilon = 1e-10);
    assert_relative_eq!(integer_part(-2.3), -2.0, epsilon = 1e-10);
    assert_relative_eq!(integer_part(5.0), 5.0, epsilon = 1e-10);

    assert_relative_eq!(fractional_part(3.7), 0.7, epsilon = 1e-10);
    assert_relative_eq!(fractional_part(-2.3), -0.3, epsilon = 1e-10);
    assert_relative_eq!(fractional_part(5.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(fractional_part(-5.0), 0.0, epsilon = 1e-10);

    let x = 7.42;
    assert_relative_eq!(x, integer_part(x) + fractional_part(x), epsilon = 1e-10);
}

#[test]
fn test_modulo_basic() {
    assert_relative_eq!(modulo(7.0, 3.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(modulo(-7.0, 3.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(modulo(7.0, -3.0), -2.0, epsilon = 1e-10);
    assert_relative_eq!(modulo(-7.0, -3.0), -1.0, epsilon = 1e-10);
    assert_relative_eq!(modulo(5.5, 2.0), 1.5, epsilon = 1e-10);
}

#[test]
fn test_quotient_basic() {
    assert_relative_eq!(quotient(7.0, 3.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(quotient(-7.0, 3.0), -3.0, epsilon = 1e-10);
    assert_relative_eq!(quotient(7.0, -3.0), -3.0, epsilon = 1e-10);
    assert_relative_eq!(quotient(-7.0, -3.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(quotient(5.5, 2.0), 2.0, epsilon = 1e-10);
}

#[test]
fn test_special_cases() {
    assert!(floor(f64::NAN).is_nan());
    assert!(ceiling(f64::NAN).is_nan());
    assert!(round(f64::NAN).is_nan());
    assert!(integer_part(f64::NAN).is_nan());
    assert!(fractional_part(f64::NAN).is_nan());

    assert!(floor(f64::INFINITY).is_infinite());
    assert!(ceiling(f64::INFINITY).is_infinite());
    assert!(round(f64::INFINITY).is_infinite());

    assert!(modulo(5.0, 0.0).is_nan());
    assert!(quotient(5.0, 0.0).is_infinite());
    assert!(quotient(0.0, 0.0).is_nan());
}

#[test]
fn test_modulo_quotient_relationship() {
    let test_cases = [(7.0, 3.0), (-7.0, 3.0), (7.0, -3.0), (-7.0, -3.0), (5.5, 2.0)];

    for (m, n) in test_cases.iter() {
        let q = quotient(*m, *n);
        let r = modulo(*m, *n);
        assert_relative_eq!(*m, q * *n + r, epsilon = 1e-10);
    }
}
