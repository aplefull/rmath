use approx::assert_relative_eq;
use rmath::aggregate::*;

#[test]
fn test_sum_product() {
    assert_relative_eq!(sum(&[1.0, 2.0, 3.0, 4.0]), 10.0, epsilon = 1e-10);
    assert_relative_eq!(sum(&[]), 0.0, epsilon = 1e-10);
    assert!(sum(&[1.0, f64::NAN, 3.0]).is_nan());

    assert_relative_eq!(product(&[2.0, 3.0, 4.0]), 24.0, epsilon = 1e-10);
    assert_relative_eq!(product(&[]), 1.0, epsilon = 1e-10);
    assert!(product(&[1.0, f64::NAN, 3.0]).is_nan());
}

#[test]
fn test_mean() {
    assert_relative_eq!(mean(&[1.0, 2.0, 3.0, 4.0]), 2.5, epsilon = 1e-10);
    assert_relative_eq!(mean(&[10.0]), 10.0, epsilon = 1e-10);
    assert!(mean(&[]).is_nan());
    assert!(mean(&[1.0, f64::NAN]).is_nan());
}

#[test]
fn test_geometric_mean() {
    assert_relative_eq!(
        geometric_mean(&[1.0, 4.0, 9.0]),
        (1.0 * 4.0 * 9.0_f64).powf(1.0 / 3.0),
        epsilon = 1e-10
    );
    assert_relative_eq!(geometric_mean(&[2.0, 8.0]), 4.0, epsilon = 1e-10);
    assert!(geometric_mean(&[]).is_nan());
    assert!(geometric_mean(&[-1.0, 2.0]).is_nan());
    assert!(geometric_mean(&[0.0, 2.0]).is_nan());
}

#[test]
fn test_harmonic_mean() {
    assert_relative_eq!(
        harmonic_mean(&[1.0, 2.0, 4.0]),
        3.0 / (1.0 + 0.5 + 0.25),
        epsilon = 1e-10
    );
    assert_relative_eq!(harmonic_mean(&[2.0, 2.0]), 2.0, epsilon = 1e-10);
    assert!(harmonic_mean(&[]).is_nan());
    assert!(harmonic_mean(&[0.0, 1.0]).is_nan());
}

#[test]
fn test_variance_std() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let var = variance(&data);
    let std = standard_deviation(&data);

    assert_relative_eq!(var, 2.5, epsilon = 1e-10);
    assert_relative_eq!(std, var.sqrt(), epsilon = 1e-10);

    assert!(variance(&[1.0]).is_nan());
    assert!(variance(&[]).is_nan());
}

#[test]
fn test_median() {
    assert_relative_eq!(median(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0, epsilon = 1e-10);
    assert_relative_eq!(median(&[1.0, 2.0, 3.0, 4.0]), 2.5, epsilon = 1e-10);
    assert_relative_eq!(median(&[5.0]), 5.0, epsilon = 1e-10);
    assert!(median(&[]).is_nan());
    assert!(median(&[1.0, f64::NAN]).is_nan());
}

#[test]
fn test_range() {
    assert_relative_eq!(range(&[1.0, 5.0, 3.0, 9.0, 2.0]), 8.0, epsilon = 1e-10);
    assert_relative_eq!(range(&[5.0]), 0.0, epsilon = 1e-10);
    assert!(range(&[]).is_nan());
    assert!(range(&[1.0, f64::NAN]).is_nan());
}

#[test]
fn test_quantile() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_relative_eq!(quantile(&data, 0.0), 1.0, epsilon = 1e-10);
    assert_relative_eq!(quantile(&data, 0.5), 3.0, epsilon = 1e-10);
    assert_relative_eq!(quantile(&data, 1.0), 5.0, epsilon = 1e-10);
    assert_relative_eq!(quantile(&data, 0.25), 2.0, epsilon = 1e-10);

    assert!(quantile(&[], 0.5).is_nan());
    assert!(quantile(&data, -0.1).is_nan());
    assert!(quantile(&data, 1.1).is_nan());
}

#[test]
fn test_integer_functions() {
    assert_eq!(sum_i64(&[1, 2, 3, 4]), 10);
    assert_eq!(product_i64(&[2, 3, 4]), 24);
    assert_eq!(product_i64(&[]), 1);

    assert_eq!(max_i64(&[1, 5, 3, 2]), Some(5));
    assert_eq!(min_i64(&[1, 5, 3, 2]), Some(1));
    assert_eq!(max_i64(&[]), None);
    assert_eq!(min_i64(&[]), None);
}

#[test]
fn test_cumulative_functions() {
    assert_eq!(
        cumulative_sum(&[1.0, 2.0, 3.0, 4.0]),
        vec![1.0, 3.0, 6.0, 10.0]
    );
    assert_eq!(cumulative_product(&[2.0, 3.0, 4.0]), vec![2.0, 6.0, 24.0]);
    assert_eq!(cumulative_sum(&[]), Vec::<f64>::new());
}

#[test]
fn test_pairwise_differences() {
    assert_eq!(
        pairwise_differences(&[1.0, 3.0, 6.0, 10.0]),
        vec![2.0, 3.0, 4.0]
    );
    assert_eq!(pairwise_differences(&[5.0]), Vec::<f64>::new());
    assert_eq!(pairwise_differences(&[]), Vec::<f64>::new());
}

#[test]
fn test_statistical_properties() {
    let data = [2.0, 4.0, 6.0, 8.0];

    assert_relative_eq!(mean(&data), median(&data), epsilon = 1e-10);

    let equal_data = [5.0, 5.0, 5.0, 5.0];
    assert_relative_eq!(mean(&equal_data), 5.0, epsilon = 1e-10);
    assert_relative_eq!(geometric_mean(&equal_data), 5.0, epsilon = 1e-10);
    assert_relative_eq!(harmonic_mean(&equal_data), 5.0, epsilon = 1e-10);
    assert_relative_eq!(variance(&equal_data), 0.0, epsilon = 1e-10);
}
