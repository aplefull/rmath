use approx::assert_relative_eq;
use rmath::easing::*;

#[test]
fn test_smoothstep_boundary_values() {
    assert_eq!(smoothstep(-1.0), 0.0);
    assert_eq!(smoothstep(0.0), 0.0);
    assert_eq!(smoothstep(1.0), 1.0);
    assert_eq!(smoothstep(2.0), 1.0);
}

#[test]
fn test_smoothstep_midpoint() {
    assert_relative_eq!(smoothstep(0.5), 0.5, epsilon = 1e-10);
}

#[test]
fn test_smoothstep_monotonic() {
    let x1 = 0.25;
    let x2 = 0.75;
    assert!(smoothstep(x1) < smoothstep(x2));
}

#[test]
fn test_smootherstep_boundary_values() {
    assert_eq!(smootherstep(-1.0), 0.0);
    assert_eq!(smootherstep(0.0), 0.0);
    assert_eq!(smootherstep(1.0), 1.0);
    assert_eq!(smootherstep(2.0), 1.0);
}

#[test]
fn test_smootherstep_midpoint() {
    assert_relative_eq!(smootherstep(0.5), 0.5, epsilon = 1e-10);
}

#[test]
fn test_smootherstep_monotonic() {
    let x1 = 0.25;
    let x2 = 0.75;
    assert!(smootherstep(x1) < smootherstep(x2));
}

#[test]
fn test_smootherstep_smoother_than_smoothstep() {
    let x = 0.25;
    let smooth = smoothstep(x);
    let smoother = smootherstep(x);

    assert!(smoother < smooth);
}

#[test]
fn test_lerp_basic() {
    assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
    assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
}

#[test]
fn test_lerp_negative_values() {
    assert_eq!(lerp(-5.0, 5.0, 0.5), 0.0);
    assert_eq!(lerp(-10.0, -5.0, 0.5), -7.5);
}

#[test]
fn test_lerp_extrapolation() {
    assert_eq!(lerp(0.0, 10.0, -0.5), -5.0);
    assert_eq!(lerp(0.0, 10.0, 1.5), 15.0);
}

#[test]
fn test_lerp_identical_values() {
    assert_eq!(lerp(5.0, 5.0, 0.3), 5.0);
    assert_eq!(lerp(5.0, 5.0, 1.7), 5.0);
}

#[test]
fn test_clamp_within_range() {
    assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
    assert_eq!(clamp(0.0, 0.0, 10.0), 0.0);
    assert_eq!(clamp(10.0, 0.0, 10.0), 10.0);
}

#[test]
fn test_clamp_below_range() {
    assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
    assert_eq!(clamp(-100.0, -50.0, 50.0), -50.0);
}

#[test]
fn test_clamp_above_range() {
    assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    assert_eq!(clamp(100.0, -50.0, 50.0), 50.0);
}

#[test]
fn test_clamp_negative_range() {
    assert_eq!(clamp(-3.0, -10.0, -1.0), -3.0);
    assert_eq!(clamp(-15.0, -10.0, -1.0), -10.0);
    assert_eq!(clamp(5.0, -10.0, -1.0), -1.0);
}
