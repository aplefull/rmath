use approx::{assert_relative_eq};
use rmath::trigonometric::*;
use std::f64::consts::{PI, SQRT_2};

#[test]
fn test_sin_basic_values() {
    assert_relative_eq!(sin(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(sin(PI / 6.0), 0.5, epsilon = 1e-10);
    assert_relative_eq!(sin(PI / 4.0), SQRT_2 / 2.0, epsilon = 1e-10);
    assert_relative_eq!(sin(PI / 3.0), (3.0_f64.sqrt()) / 2.0, epsilon = 1e-10);
    assert_relative_eq!(sin(PI / 2.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(sin(PI), 0.0, epsilon = 1e-10);
    assert_relative_eq!(sin(3.0 * PI / 2.0), -1.0, epsilon = 1e-10);
    assert_relative_eq!(sin(2.0 * PI), 0.0, epsilon = 1e-10);
}

#[test]
fn test_cos_basic_values() {
    assert_relative_eq!(cos(0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(cos(PI / 6.0), (3.0_f64.sqrt()) / 2.0, epsilon = 1e-10);
    assert_relative_eq!(cos(PI / 4.0), SQRT_2 / 2.0, epsilon = 1e-10);
    assert_relative_eq!(cos(PI / 3.0), 0.5, epsilon = 1e-10);
    assert_relative_eq!(cos(PI / 2.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(cos(PI), -1.0, epsilon = 1e-10);
    assert_relative_eq!(cos(3.0 * PI / 2.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(cos(2.0 * PI), 1.0, epsilon = 1e-10);
}

#[test]
fn test_tan_basic_values() {
    assert_relative_eq!(tan(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(tan(PI / 6.0), 1.0 / (3.0_f64.sqrt()), epsilon = 1e-10);
    assert_relative_eq!(tan(PI / 4.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(tan(PI / 3.0), 3.0_f64.sqrt(), epsilon = 1e-10);
    assert_relative_eq!(tan(PI), 0.0, epsilon = 1e-10);
}

#[test]
fn test_reciprocal_functions() {
    let x = PI / 6.0;
    assert_relative_eq!(csc(x), 1.0 / sin(x), epsilon = 1e-10);
    assert_relative_eq!(sec(x), 1.0 / cos(x), epsilon = 1e-10);
    assert_relative_eq!(cot(x), 1.0 / tan(x), epsilon = 1e-10);
}

#[test]
fn test_arcsin() {
    assert_relative_eq!(arcsin(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(arcsin(0.5), PI / 6.0, epsilon = 1e-10);
    assert_relative_eq!(arcsin(SQRT_2 / 2.0), PI / 4.0, epsilon = 1e-10);
    assert_relative_eq!(arcsin(1.0), PI / 2.0, epsilon = 1e-10);
    assert_relative_eq!(arcsin(-1.0), -PI / 2.0, epsilon = 1e-10);

    assert!(arcsin(1.1).is_nan());
    assert!(arcsin(-1.1).is_nan());
}

#[test]
fn test_arccos() {
    assert_relative_eq!(arccos(1.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(arccos(SQRT_2 / 2.0), PI / 4.0, epsilon = 1e-10);
    assert_relative_eq!(arccos(0.5), PI / 3.0, epsilon = 1e-10);
    assert_relative_eq!(arccos(0.0), PI / 2.0, epsilon = 1e-10);
    assert_relative_eq!(arccos(-1.0), PI, epsilon = 1e-10);

    assert!(arccos(1.1).is_nan());
    assert!(arccos(-1.1).is_nan());
}

#[test]
fn test_arctan() {
    assert_relative_eq!(arctan(0.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(arctan(1.0), PI / 4.0, epsilon = 1e-10);
    assert_relative_eq!(arctan(3.0_f64.sqrt()), PI / 3.0, epsilon = 1e-10);
    assert_relative_eq!(arctan(-1.0), -PI / 4.0, epsilon = 1e-10);
}

#[test]
fn test_sinc() {
    assert_relative_eq!(sinc(0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(sinc(PI / 6.0), sin(PI / 6.0) / (PI / 6.0), epsilon = 1e-10);
    assert_relative_eq!(sinc(PI / 4.0), sin(PI / 4.0) / (PI / 4.0), epsilon = 1e-10);
    assert_relative_eq!(sinc(PI / 3.0), sin(PI / 3.0) / (PI / 3.0), epsilon = 1e-10);
}

#[test]
fn test_inverse_reciprocal_functions() {
    assert_relative_eq!(arcsec(1.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(arcsec(2.0), PI / 3.0, epsilon = 1e-10);
    assert!(arcsec(0.5).is_nan());

    assert_relative_eq!(arccsc(1.0), PI / 2.0, epsilon = 1e-10);
    assert_relative_eq!(arccsc(2.0), PI / 6.0, epsilon = 1e-10);
    assert!(arccsc(0.5).is_nan());

    assert_relative_eq!(arccot(1.0), PI / 4.0, epsilon = 1e-10);
    assert_relative_eq!(arccot(3.0_f64.sqrt()), PI / 6.0, epsilon = 1e-10);
}

#[test]
fn test_trig_identities() {
    let angles = [0.0, PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, PI];

    for angle in angles.iter() {
        let s = sin(*angle);
        let c = cos(*angle);
        assert_relative_eq!(s * s + c * c, 1.0, epsilon = 1e-10);

        if c.abs() > 1e-10 {
            assert_relative_eq!(tan(*angle), s / c, epsilon = 1e-10);
        }
    }
}

#[test]
fn test_inverse_trig_consistency() {
    let values = [-0.9, -0.5, 0.0, 0.5, 0.9];

    for val in values.iter() {
        assert_relative_eq!(sin(arcsin(*val)), *val, epsilon = 1e-10);
        assert_relative_eq!(cos(arccos(*val)), *val, epsilon = 1e-10);
    }

    let tan_values = [-10.0, -1.0, 0.0, 1.0, 10.0];
    for val in tan_values.iter() {
        assert_relative_eq!(tan(arctan(*val)), *val, epsilon = 1e-10);
    }
}

#[test]
fn test_large_angle_reduction() {
    let large_angle = 1000.0 * PI;
    assert_relative_eq!(sin(large_angle), 0.0, epsilon = 1e-10);
    assert_relative_eq!(cos(large_angle), 1.0, epsilon = 1e-10);
    assert_relative_eq!(tan(large_angle), 0.0, epsilon = 1e-10);
    
    let large_angle = 1000.0 * PI + PI / 4.0;
    assert_relative_eq!(sin(large_angle), SQRT_2 / 2.0, epsilon = 1e-10);
    assert_relative_eq!(cos(large_angle), SQRT_2 / 2.0, epsilon = 1e-10);
    assert_relative_eq!(tan(large_angle), 1.0, epsilon = 1e-10);
}