use approx::assert_relative_eq;
use rmath::logarithmic::*;
use std::f64::consts::{E, LN_10, LN_2};

#[test]
fn test_log_basic_values() {
    assert_relative_eq!(log(1.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(log(E), 1.0, epsilon = 1e-10);
    assert_relative_eq!(log(E * E), 2.0, epsilon = 1e-10);
    assert_relative_eq!(log(1.0 / E), -1.0, epsilon = 1e-10);

    assert!(log(0.0).is_infinite() && log(0.0).is_sign_negative());
    assert!(log(-1.0).is_nan());
    assert!(log(f64::INFINITY).is_infinite() && log(f64::INFINITY).is_sign_positive());
}

#[test]
fn test_log10_basic_values() {
    assert_relative_eq!(log10(1.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(log10(10.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(log10(100.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(log10(1000.0), 3.0, epsilon = 1e-10);
    assert_relative_eq!(log10(0.1), -1.0, epsilon = 1e-10);
    assert_relative_eq!(log10(0.01), -2.0, epsilon = 1e-10);

    assert!(log10(0.0).is_infinite() && log10(0.0).is_sign_negative());
    assert!(log10(-1.0).is_nan());
    assert!(log10(f64::INFINITY).is_infinite() && log10(f64::INFINITY).is_sign_positive());
}

#[test]
fn test_log2_basic_values() {
    assert_relative_eq!(log2(1.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(log2(2.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(log2(4.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(log2(8.0), 3.0, epsilon = 1e-10);
    assert_relative_eq!(log2(16.0), 4.0, epsilon = 1e-10);
    assert_relative_eq!(log2(0.5), -1.0, epsilon = 1e-10);
    assert_relative_eq!(log2(0.25), -2.0, epsilon = 1e-10);

    assert!(log2(0.0).is_infinite() && log2(0.0).is_sign_negative());
    assert!(log2(-1.0).is_nan());
    assert!(log2(f64::INFINITY).is_infinite() && log2(f64::INFINITY).is_sign_positive());
}

#[test]
fn test_log_base_basic() {
    assert_relative_eq!(log_base(10.0, 100.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(log_base(2.0, 8.0), 3.0, epsilon = 1e-10);
    assert_relative_eq!(log_base(5.0, 25.0), 2.0, epsilon = 1e-10);
    assert_relative_eq!(log_base(3.0, 27.0), 3.0, epsilon = 1e-10);

    assert_relative_eq!(log_base(7.0, 7.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(log_base(13.0, 13.0), 1.0, epsilon = 1e-10);

    assert_relative_eq!(log_base(2.0, 1.0), 0.0, epsilon = 1e-10);
    assert_relative_eq!(log_base(10.0, 1.0), 0.0, epsilon = 1e-10);
}

#[test]
fn test_log_base_special_cases() {
    assert!(log_base(1.0, 10.0).is_nan());
    assert!(log_base(0.0, 10.0).is_nan());
    assert!(log_base(-2.0, 10.0).is_nan());

    assert!(log_base(2.0, 0.0).is_infinite() && log_base(2.0, 0.0).is_sign_negative());
    assert!(log_base(2.0, -1.0).is_nan());

    assert!(
        log_base(2.0, f64::INFINITY).is_infinite()
            && log_base(2.0, f64::INFINITY).is_sign_positive()
    );
    assert!(
        log_base(0.5, f64::INFINITY).is_infinite()
            && log_base(0.5, f64::INFINITY).is_sign_negative()
    );
}

#[test]
fn test_logarithm_properties() {
    let a = 3.0;
    let x = 2.0;
    let y = 7.0;

    // log(xy) = log(x) + log(y)
    assert_relative_eq!(log(x * y), log(x) + log(y), epsilon = 1e-10);
    assert_relative_eq!(log10(x * y), log10(x) + log10(y), epsilon = 1e-10);
    assert_relative_eq!(log2(x * y), log2(x) + log2(y), epsilon = 1e-10);

    // log(x/y) = log(x) - log(y)
    assert_relative_eq!(log(x / y), log(x) - log(y), epsilon = 1e-10);
    assert_relative_eq!(log10(x / y), log10(x) - log10(y), epsilon = 1e-10);
    assert_relative_eq!(log2(x / y), log2(x) - log2(y), epsilon = 1e-10);

    // log(x^a) = a * log(x)
    assert_relative_eq!(log(x.powf(a)), a * log(x), epsilon = 1e-10);
    assert_relative_eq!(log10(x.powf(a)), a * log10(x), epsilon = 1e-10);
    assert_relative_eq!(log2(x.powf(a)), a * log2(x), epsilon = 1e-10);
}

#[test]
fn test_change_of_base_formula() {
    let bases = [2.0, 3.0, 5.0, 7.0, 10.0];
    let values = [1.5, 2.0, 4.0, 8.0, 10.0];

    for base in bases.iter() {
        for val in values.iter() {
            // log_a(x) = ln(x) / ln(a)
            assert_relative_eq!(
                log_base(*base, *val),
                log(*val) / log(*base),
                epsilon = 1e-10
            );

            if (*base - 10.0).abs() < 1e-10 {
                assert_relative_eq!(log_base(*base, *val), log10(*val), epsilon = 1e-10);
            }
            if (*base - 2.0).abs() < 1e-10 {
                assert_relative_eq!(log_base(*base, *val), log2(*val), epsilon = 1e-10);
            }
            if (*base - E).abs() < 1e-10 {
                assert_relative_eq!(log_base(*base, *val), log(*val), epsilon = 1e-10);
            }
        }
    }
}

#[test]
fn test_logarithm_consistency() {
    let values = [0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 100.0];

    for &val in values.iter() {
        if val > 0.0 {
            assert_relative_eq!(log(val), val.ln(), epsilon = 1e-15);
            assert_relative_eq!(log10(val), val.log10(), epsilon = 1e-15);
            assert_relative_eq!(log2(val), val.log2(), epsilon = 1e-15);

            assert_relative_eq!(log10(val), log(val) / LN_10, epsilon = 1e-10);
            assert_relative_eq!(log2(val), log(val) / LN_2, epsilon = 1e-10);
        }
    }
}

#[test]
fn test_exp_log_inverse() {
    use rmath::power::{exp, exp10, exp2};

    let values: [f64; 6] = [-2.0, -1.0, 0.0, 1.0, 2.0, 3.0];

    for &val in values.iter() {
        // e^(ln(e^x)) = e^x (for reasonable values)
        if val.abs() < 100.0 {
            let exp_val = exp(val);
            if exp_val > 0.0 && exp_val.is_finite() {
                assert_relative_eq!(exp(log(exp_val)), exp_val, epsilon = 1e-10);
            }
        }

        // 2^(log2(2^x)) = 2^x
        if val.abs() < 100.0 {
            let exp2_val = exp2(val);
            if exp2_val > 0.0 && exp2_val.is_finite() {
                assert_relative_eq!(exp2(log2(exp2_val)), exp2_val, epsilon = 1e-10);
            }
        }

        // 10^(log10(10^x)) = 10^x
        if val.abs() < 100.0 {
            let exp10_val = exp10(val);
            if exp10_val > 0.0 && exp10_val.is_finite() {
                assert_relative_eq!(exp10(log10(exp10_val)), exp10_val, epsilon = 1e-10);
            }
        }
    }
}

#[test]
fn test_logarithm_limits() {
    assert!(log(1e-100).is_finite());
    assert!(log(1e-100) < 0.0);

    assert!(log(1e100).is_finite());
    assert!(log(1e100) > 0.0);

    assert!(log10(1e-100).is_finite());
    assert!(log2(1e-100).is_finite());
    assert!(log10(1e100).is_finite());
    assert!(log2(1e100).is_finite());
}
