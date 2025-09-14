use approx::assert_relative_eq;
use rmath::lambert_w::*;
use std::f64::consts::E;

#[test]
fn test_product_log_basic_values() {
    assert_relative_eq!(product_log(0.0), 0.0, epsilon = 1e-10);

    assert_relative_eq!(product_log(E), 1.0, epsilon = 1e-10);

    let branch_point = -1.0 / E;
    assert_relative_eq!(product_log(branch_point), -1.0, epsilon = 1e-10);

    assert_relative_eq!(product_log(1.0), 0.5671432904097838, epsilon = 1e-10);
}

#[test]
fn test_product_log_domain() {
    let branch_point = -1.0 / E;

    assert!(product_log(0.0).is_finite());
    assert!(product_log(1.0).is_finite());
    assert!(product_log(branch_point).is_finite());

    assert!(product_log(branch_point - 1e-10).is_nan());
    assert!(product_log(-1.0).is_nan());
    assert!(product_log(-2.0).is_nan());
}

#[test]
fn test_product_log_definition() {
    // The defining property: W(x) * exp(W(x)) = x
    let test_values = [0.0, 0.1, 0.5, 1.0, 2.0, 5.0, 10.0];

    for x in test_values.iter() {
        let w = product_log(*x);
        let verification = w * w.exp();
        assert_relative_eq!(verification, *x, epsilon = 1e-12);
    }

    let branch_point = -1.0 / E;
    let near_branch = [-0.367, -0.35, -0.3, -0.2, -0.1];

    for x in near_branch.iter() {
        if *x >= branch_point {
            let w = product_log(*x);
            let verification = w * w.exp();
            assert_relative_eq!(verification, *x, epsilon = 1e-10);
        }
    }
}

#[test]
fn test_product_log_special_cases() {
    assert!(product_log(f64::NAN).is_nan());

    let w_inf = product_log(f64::INFINITY);
    assert!(w_inf.is_infinite() && w_inf.is_sign_positive());

    let branch_point = -1.0 / E;
    let epsilon = 1e-15;
    let w_near = product_log(branch_point + epsilon);
    assert!(w_near.is_finite());
    assert!((w_near - (-1.0)).abs() < 0.1);
}

#[test]
fn test_product_log_accuracy() {
    struct TestCase {
        x: f64,
        expected: f64,
    }

    let test_cases = [
        // Small values near branch point
        TestCase {
            x: -0.35787944117,
            expected: -0.78322919896673503,
        },
        TestCase {
            x: -0.2158928365968376,
            expected: -0.2879278822628465,
        },
        TestCase {
            x: -0.06390623202223294,
            expected: -0.06843261212733538,
        },
        TestCase {
            x: 0.08808037255237178,
            expected: 0.08121009988290029,
        },
        TestCase {
            x: 0.2400669771269764,
            expected: 0.1971175754367965,
        },
        TestCase {
            x: 0.3920535817015811,
            expected: 0.2925981958438676,
        },
        TestCase {
            x: 0.5440401862761859,
            expected: 0.374208904991127,
        },
        TestCase {
            x: 0.6960267908507904,
            expected: 0.4457129707320664,
        },
        TestCase {
            x: 0.8480133954253952,
            expected: 0.5094887196941887,
        },
        TestCase {
            x: 1.0,
            expected: 0.5671432904097838,
        },
        // Regular values from 0 to 100
        TestCase {
            x: 0.0,
            expected: 0.0,
        },
        TestCase {
            x: 10.0,
            expected: 1.745528002740699,
        },
        TestCase {
            x: 20.0,
            expected: 2.20500327802406,
        },
        TestCase {
            x: 30.0,
            expected: 2.489225688157678,
        },
        TestCase {
            x: 40.0,
            expected: 2.696809898661708,
        },
        TestCase {
            x: 50.0,
            expected: 2.860890177982211,
        },
        TestCase {
            x: 60.0,
            expected: 2.996799632233992,
        },
        TestCase {
            x: 70.0,
            expected: 3.112930633639478,
        },
        TestCase {
            x: 80.0,
            expected: 3.21438926062197,
        },
        TestCase {
            x: 90.0,
            expected: 3.304518803996878,
        },
        TestCase {
            x: 100.0,
            expected: 3.38563014029005,
        },
        // Large values from 1e3 to 1e7
        TestCase {
            x: 1000.0,
            expected: 5.249602852401596,
        },
        TestCase {
            x: 10000.0,
            expected: 7.231846038093373,
        },
        TestCase {
            x: 100000.0,
            expected: 9.284571428622108,
        },
        TestCase {
            x: 1000000.0,
            expected: 11.38335808614005,
        },
        TestCase {
            x: 10000000.0,
            expected: 13.51434401030609,
        },
        // Misc values
        TestCase {
            x: 1e-06,
            expected: 9.999990000015e-07,
        },
        TestCase {
            x: 0.0001,
            expected: 9.999000149973339e-05,
        },
        TestCase {
            x: 0.01,
            expected: 0.009901473843595012,
        },
        TestCase {
            x: 0.01,
            expected: 0.009901473843595012,
        },
        TestCase {
            x: 100.42,
            expected: 3.388866029689645,
        },
        TestCase {
            x: 10000.42,
            expected: 7.231882935193733,
        },
        TestCase {
            x: 1000000.42,
            expected: 11.383358472223485359,
        },
    ];

    for case in test_cases.iter() {
        let computed = product_log(case.x);
        assert_relative_eq!(computed, case.expected, epsilon = 1e-10);
    }
}

// TODO
/*#[test]
fn test_product_log_k_branches() {
    assert_relative_eq!(product_log_k(0, 1.0), product_log(1.0), epsilon = 1e-10);
    assert_relative_eq!(product_log_k(0, E), product_log(E), epsilon = 1e-10);

    let branch_point = -1.0 / E;
    let test_values = [-0.3, -0.25, -0.2, -0.1, -0.05];

    for x in test_values.iter() {
        if *x > branch_point && *x < 0.0 {
            let w = product_log_k(-1, *x);
            assert!(w.is_finite());

            let verification = w * w.exp();
            assert_relative_eq!(verification, *x, epsilon = 1e-10);

            assert!(w < -1.0);
        }
    }

    assert!(product_log_k(1, 1.0).is_nan());
    assert!(product_log_k(-2, 1.0).is_nan());
    assert!(product_log_k(-1, 1.0).is_nan());
}*/
