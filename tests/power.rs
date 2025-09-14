use approx::assert_relative_eq;
use rmath::power::*;
use std::f64::consts::E;

#[test]
fn test_sqrt_basic() {
    assert_relative_eq!(sqrt(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(sqrt(1.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(sqrt(4.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(sqrt(9.0), 3.0, epsilon = 1e-10);
    assert_relative_eq!(sqrt(2.0), 2.0_f64.sqrt(), epsilon = 1e-10);

    assert!(sqrt(-1.0).is_nan());

    assert!(sqrt(f64::INFINITY).is_infinite());
    assert!(sqrt(f64::INFINITY).is_sign_positive());
}

#[test]
fn test_power_basic() {
    assert_relative_eq!(power(2.0, 3.0), 8.0, epsilon = 1e-10);
    assert_relative_eq!(power(3.0, 2.0), 9.0, epsilon = 1e-10);
    assert_relative_eq!(power(5.0, 0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(power(0.0, 0.0), 1.0, epsilon = 1e-10);

    assert_relative_eq!(power(-2.0, 3.0), -8.0, epsilon = 1e-10);
    assert_relative_eq!(power(-2.0, 2.0), 4.0, epsilon = 1e-10);
    assert_relative_eq!(power(-3.0, 4.0), 81.0, epsilon = 1e-10);

    assert!(power(-2.0, 0.5).is_nan());
}

#[test]
fn test_power_special_cases() {
    assert_relative_eq!(power(1.0, 100.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(power(1.0, -100.0), 1.0, epsilon = 1e-10);

    assert_relative_eq!(power(0.0, 2.0), 0.0, epsilon = 1e-10);
    assert!(power(0.0, -1.0).is_infinite());

    assert_relative_eq!(power(8.0, 1.0 / 3.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(power(16.0, 0.25), 2.0, epsilon = 1e-10);
}

#[test]
fn test_square_and_cube() {
    assert_relative_eq!(square(3.0), 9.0, epsilon = 1e-10);
    assert_relative_eq!(square(-4.0), 16.0, epsilon = 1e-10);
    assert_relative_eq!(square(0.0), 0.0, epsilon = 1e-10);

    assert_relative_eq!(cube(2.0), 8.0, epsilon = 1e-10);
    assert_relative_eq!(cube(-3.0), -27.0, epsilon = 1e-10);
    assert_relative_eq!(cube(0.0), 0.0, epsilon = 1e-10);
}

#[test]
fn test_nth_root() {
    assert_relative_eq!(nth_root(8.0, 3), 2.0, epsilon = 1e-10);
    assert_relative_eq!(nth_root(16.0, 4), 2.0, epsilon = 1e-10);
    assert_relative_eq!(nth_root(32.0, 5), 2.0, epsilon = 1e-10);

    assert_relative_eq!(nth_root(-8.0, 3), -2.0, epsilon = 1e-10);
    assert_relative_eq!(nth_root(-32.0, 5), -2.0, epsilon = 1e-10);

    assert!(nth_root(-4.0, 2).is_nan());
    assert!(nth_root(-16.0, 4).is_nan());

    assert!(nth_root(4.0, 0).is_nan());
}

#[test]
fn test_cbrt() {
    assert_relative_eq!(cbrt(8.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(cbrt(27.0), 3.0, epsilon = 1e-10);
    assert_relative_eq!(cbrt(-8.0), -2.0, epsilon = 1e-10);
    assert_relative_eq!(cbrt(0.0), 0.0, epsilon = 1e-10);

    assert_relative_eq!(cbrt(-27.0), -3.0, epsilon = 1e-10);
}

#[test]
fn test_exp_functions() {
    assert_relative_eq!(exp(0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(exp(1.0), E, epsilon = 1e-10);
    assert_relative_eq!(exp(2.0), E * E, epsilon = 1e-10);

    assert_relative_eq!(exp2(0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(exp2(1.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(exp2(3.0), 8.0, epsilon = 1e-10);
    assert_relative_eq!(exp2(10.0), 1024.0, epsilon = 1e-10);

    assert_relative_eq!(exp10(0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(exp10(1.0), 10.0, epsilon = 1e-10);
    assert_relative_eq!(exp10(2.0), 100.0, epsilon = 1e-10);
    assert_relative_eq!(exp10(3.0), 1000.0, epsilon = 1e-10);
}

#[test]
fn test_exp_special_cases() {
    assert!(exp(f64::INFINITY).is_infinite());
    assert_relative_eq!(exp(f64::NEG_INFINITY), 0.0, epsilon = 1e-10);

    assert!(exp2(f64::INFINITY).is_infinite());
    assert_relative_eq!(exp2(f64::NEG_INFINITY), 0.0, epsilon = 1e-10);

    assert!(exp10(f64::INFINITY).is_infinite());
    assert_relative_eq!(exp10(f64::NEG_INFINITY), 0.0, epsilon = 1e-10);
}

#[test]
fn test_integer_power_efficiency() {
    assert_relative_eq!(power(2.0, 10.0), 1024.0, epsilon = 1e-10);
    assert_relative_eq!(power(3.0, 5.0), 243.0, epsilon = 1e-10);
    assert_relative_eq!(power(2.0, -3.0), 0.125, epsilon = 1e-10);

    assert_relative_eq!(power(2.0, 20.0), 1048576.0, epsilon = 1e-8);
}

#[test]
fn test_power_consistency() {
    let bases = [0.5, 1.0, 2.0, 3.0, 10.0];
    let exponents = [0.0, 0.5, 1.0, 2.0, 3.0];

    for base in bases.iter() {
        for exp in exponents.iter() {
            if *base > 0.0 {
                assert_relative_eq!(power(*base, *exp), base.powf(*exp), epsilon = 1e-10);
            }
        }
    }
}

#[test]
fn test_root_power_consistency() {
    let values = [1.0, 4.0, 8.0, 16.0, 27.0, 64.0];

    for val in values.iter() {
        assert_relative_eq!(sqrt(*val), power(*val, 0.5), epsilon = 1e-10);

        assert_relative_eq!(cbrt(*val), power(*val, 1.0 / 3.0), epsilon = 1e-10);

        assert_relative_eq!(nth_root(*val, 2), power(*val, 0.5), epsilon = 1e-10);
        assert_relative_eq!(nth_root(*val, 3), power(*val, 1.0 / 3.0), epsilon = 1e-10);
    }
}
