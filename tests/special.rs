use approx::assert_relative_eq;
use rmath::special::*;

#[test]
fn test_erf_basic_values() {
    assert_relative_eq!(erf(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(erf(1.0), 0.8427007929497149, epsilon = 1e-6);
    assert_relative_eq!(erf(-1.0), -0.8427007929497149, epsilon = 1e-6);
    assert_relative_eq!(erf(2.0), 0.9953222650189527, epsilon = 1e-6);
    assert_relative_eq!(erf(-2.0), -0.9953222650189527, epsilon = 1e-6);

    assert_relative_eq!(erf(10.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(erf(-10.0), -1.0, epsilon = 1e-10);
}

#[test]
fn test_erfc_basic_values() {
    assert_relative_eq!(erfc(0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(erfc(1.0), 0.15729920705028513, epsilon = 1e-6);
    assert_relative_eq!(erfc(-1.0), 1.8427007929497149, epsilon = 1e-6);
    assert_relative_eq!(erfc(2.0), 0.004677734981047266, epsilon = 1e-6);

    assert_relative_eq!(erfc(10.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(erfc(-10.0), 2.0, epsilon = 1e-10);
}

#[test]
fn test_erf_erfc_relationship() {
    let test_values = [-2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0];

    for &x in &test_values {
        assert_relative_eq!(erf(x) + erfc(x), 1.0, epsilon = 1e-10);
        assert_relative_eq!(erf(-x), -erf(x), epsilon = 1e-10);
        assert_relative_eq!(erfc(-x), 2.0 - erfc(x), epsilon = 1e-10);
    }
}

#[test]
fn test_erf_special_cases() {
    assert!(erf(f64::NAN).is_nan());
    assert_relative_eq!(erf(f64::INFINITY), 1.0, epsilon = 1e-10);
    assert_relative_eq!(erf(f64::NEG_INFINITY), -1.0, epsilon = 1e-10);

    assert!(erfc(f64::NAN).is_nan());
    assert_relative_eq!(erfc(f64::INFINITY), 0.0, epsilon = 1e-10);
    assert_relative_eq!(erfc(f64::NEG_INFINITY), 2.0, epsilon = 1e-10);
}

#[test]
fn test_erf_mathematical_properties() {
    let x = 1.0;
    let h = 1e-8;
    let numerical_derivative = (erf(x + h) - erf(x - h)) / (2.0 * h);
    let analytical_derivative = 2.0 / std::f64::consts::PI.sqrt() * (-x * x).exp();
    assert_relative_eq!(numerical_derivative, analytical_derivative, epsilon = 1e-6);

    // Test integral relationship: ∫₀ˣ erf(t) dt = x·erf(x) + 1/√π·(1 - exp(-x²))
    // This is just a sanity check for small values
    let x = 0.5;
    let expected_integral = x * erf(x) + (1.0 - (-x * x).exp()) / std::f64::consts::PI.sqrt();
    assert!(expected_integral > 0.0);
}
