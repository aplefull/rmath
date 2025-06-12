use approx::{assert_relative_eq};
use rmath::hyperbolic::*;
use std::f64::consts::E;

#[test]
fn test_sinh_basic_values() {
    assert_relative_eq!(sinh(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(sinh(1.0), (E - 1.0/E) / 2.0, epsilon = 1e-10);
    assert_relative_eq!(sinh(-1.0), -(E - 1.0/E) / 2.0, epsilon = 1e-10);
}

#[test]
fn test_cosh_basic_values() {
    assert_relative_eq!(cosh(0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(cosh(1.0), (E + 1.0/E) / 2.0, epsilon = 1e-10);
    assert_relative_eq!(cosh(-1.0), (E + 1.0/E) / 2.0, epsilon = 1e-10);
}

#[test]
fn test_tanh_basic_values() {
    assert_relative_eq!(tanh(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(tanh(1.0), (E*E - 1.0) / (E*E + 1.0), epsilon = 1e-10);
    assert_relative_eq!(tanh(-1.0), -(E*E - 1.0) / (E*E + 1.0), epsilon = 1e-10);
}

#[test]
fn test_reciprocal_hyperbolic_functions() {
    let x = 1.5;
    assert_relative_eq!(csch(x), 1.0 / sinh(x), epsilon = 1e-10);
    assert_relative_eq!(sech(x), 1.0 / cosh(x), epsilon = 1e-10);
    assert_relative_eq!(coth(x), 1.0 / tanh(x), epsilon = 1e-10);
}

#[test]
fn test_arsinh() {
    assert_relative_eq!(arsinh(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(arsinh(1.0), (1.0 + 2.0_f64.sqrt()).ln(), epsilon = 1e-10);
    assert_relative_eq!(arsinh(-1.0), -(1.0 + 2.0_f64.sqrt()).ln(), epsilon = 1e-10);
}

#[test]
fn test_arcosh() {
    assert_relative_eq!(arcosh(1.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(arcosh(E), 1.6574544541530771, epsilon = 1e-10);

    assert!(arcosh(0.5).is_nan());
}

#[test]
fn test_artanh() {
    assert_relative_eq!(artanh(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(artanh(0.5), 0.5493061443340549, epsilon = 1e-10);

    assert!(artanh(1.0).is_nan());
    assert!(artanh(-1.0).is_nan());
    assert!(artanh(1.5).is_nan());

    assert_relative_eq!(tanh(100.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(tanh(-100.0), -1.0, epsilon = 1e-10);
}

#[test]
fn test_inverse_reciprocal_hyperbolic() {
    assert_relative_eq!(arcoth(2.0), 0.5 * (3.0_f64.ln()), epsilon = 1e-10);
    assert!(arcoth(0.5).is_nan());

    assert_relative_eq!(arccsch(1.0), (1.0 + 2.0_f64.sqrt()).ln(), epsilon = 1e-10);
    assert!(arccsch(0.0).is_infinite());

    assert_relative_eq!(arcsech(1.0), 0.0, epsilon = 1e-10);
    assert!(arcsech(0.0).is_nan());
    assert!(arcsech(1.5).is_nan());
}

#[test]
fn test_hyperbolic_identities() {
    let values = [-2.0, -1.0, 0.0, 1.0, 2.0];

    for x in values.iter() {
        let sh = sinh(*x);
        let ch = cosh(*x);
        assert_relative_eq!(ch * ch - sh * sh, 1.0, epsilon = 1e-10);

        if ch.abs() > 1e-10 {
            assert_relative_eq!(tanh(*x), sh / ch, epsilon = 1e-10);
        }

        assert_relative_eq!(sinh(-*x), -sinh(*x), epsilon = 1e-10);
        assert_relative_eq!(cosh(-*x), cosh(*x), epsilon = 1e-10);
    }
}

#[test]
fn test_inverse_hyperbolic_consistency() {
    let values = [-5.0, -1.0, 0.0, 1.0, 5.0];

    for val in values.iter() {
        assert_relative_eq!(sinh(arsinh(*val)), *val, epsilon = 1e-10);
    }

    let cosh_values = [1.0, 1.5, 2.0, 5.0];
    for val in cosh_values.iter() {
        assert_relative_eq!(cosh(arcosh(*val)), *val, epsilon = 1e-10);
    }

    let tanh_values = [-0.9, -0.5, 0.0, 0.5, 0.9];
    for val in tanh_values.iter() {
        assert_relative_eq!(tanh(artanh(*val)), *val, epsilon = 1e-10);
    }
}
