use approx::assert_relative_eq;
use rmath::waves::*;

#[test]
fn test_square_wave_basic() {
    assert_eq!(square_wave(0.0), 1.0);
    assert_eq!(square_wave(0.25), 1.0);
    assert_eq!(square_wave(0.49), 1.0);
    assert_eq!(square_wave(0.5), -1.0);
    assert_eq!(square_wave(0.75), -1.0);
    assert_eq!(square_wave(0.99), -1.0);
}

#[test]
fn test_square_wave_periodicity() {
    assert_eq!(square_wave(0.0), square_wave(1.0));
    assert_eq!(square_wave(0.25), square_wave(1.25));
    assert_eq!(square_wave(0.75), square_wave(1.75));
    assert_eq!(square_wave(-0.25), square_wave(0.75));
}

#[test]
fn test_sawtooth_basic() {
    assert_eq!(sawtooth(0.0), -1.0);
    assert_eq!(sawtooth(0.5), 0.0);
    assert_relative_eq!(sawtooth(1.0), -1.0, epsilon = 1e-10);
}

#[test]
fn test_sawtooth_linearity() {
    assert_eq!(sawtooth(0.25), -0.5);
    assert_eq!(sawtooth(0.75), 0.5);
}

#[test]
fn test_sawtooth_periodicity() {
    assert_relative_eq!(sawtooth(0.0), sawtooth(1.0), epsilon = 1e-10);
    assert_relative_eq!(sawtooth(0.3), sawtooth(1.3), epsilon = 1e-10);
    assert_relative_eq!(sawtooth(-0.7), sawtooth(0.3), epsilon = 1e-10);
}

#[test]
fn test_triangle_wave_basic() {
    assert_eq!(triangle_wave(0.0), -1.0);
    assert_eq!(triangle_wave(0.25), 0.0);
    assert_eq!(triangle_wave(0.5), 1.0);
    assert_eq!(triangle_wave(0.75), 0.0);
}

#[test]
fn test_triangle_wave_symmetry() {
    assert_relative_eq!(triangle_wave(0.125), -triangle_wave(0.375), epsilon = 1e-10);
    assert_relative_eq!(triangle_wave(0.625), -triangle_wave(0.875), epsilon = 1e-10);
}

#[test]
fn test_triangle_wave_periodicity() {
    assert_relative_eq!(triangle_wave(0.0), triangle_wave(1.0), epsilon = 1e-10);
    assert_relative_eq!(triangle_wave(0.2), triangle_wave(1.2), epsilon = 1e-10);
}

#[test]
fn test_triangle_wave_range() {
    for i in 0..100 {
        let x = i as f64 * 0.01;
        let value = triangle_wave(x);
        assert!(value >= -1.0 && value <= 1.0);
    }
}

#[test]
fn test_pulse_wave_50_percent_duty() {
    assert_eq!(pulse_wave(0.0, 0.5), 1.0);
    assert_eq!(pulse_wave(0.25, 0.5), 1.0);
    assert_eq!(pulse_wave(0.49, 0.5), 1.0);
    assert_eq!(pulse_wave(0.5, 0.5), -1.0);
    assert_eq!(pulse_wave(0.75, 0.5), -1.0);
}

#[test]
fn test_pulse_wave_25_percent_duty() {
    assert_eq!(pulse_wave(0.0, 0.25), 1.0);
    assert_eq!(pulse_wave(0.2, 0.25), 1.0);
    assert_eq!(pulse_wave(0.24, 0.25), 1.0);
    assert_eq!(pulse_wave(0.25, 0.25), -1.0);
    assert_eq!(pulse_wave(0.5, 0.25), -1.0);
    assert_eq!(pulse_wave(0.75, 0.25), -1.0);
}

#[test]
fn test_pulse_wave_75_percent_duty() {
    assert_eq!(pulse_wave(0.0, 0.75), 1.0);
    assert_eq!(pulse_wave(0.5, 0.75), 1.0);
    assert_eq!(pulse_wave(0.74, 0.75), 1.0);
    assert_eq!(pulse_wave(0.75, 0.75), -1.0);
    assert_eq!(pulse_wave(0.9, 0.75), -1.0);
}

#[test]
fn test_pulse_wave_duty_clamping() {
    assert_eq!(pulse_wave(0.0, -0.5), -1.0);
    assert_eq!(pulse_wave(0.0, 1.5), 1.0);
    assert_eq!(pulse_wave(0.5, 1.5), 1.0);
}

#[test]
fn test_pulse_wave_zero_duty() {
    assert_eq!(pulse_wave(0.0, 0.0), -1.0);
    assert_eq!(pulse_wave(0.5, 0.0), -1.0);
}

#[test]
fn test_pulse_wave_full_duty() {
    assert_eq!(pulse_wave(0.0, 1.0), 1.0);
    assert_eq!(pulse_wave(0.5, 1.0), 1.0);
    assert_eq!(pulse_wave(0.99, 1.0), 1.0);
}

#[test]
fn test_pulse_wave_periodicity() {
    assert_eq!(pulse_wave(0.0, 0.3), pulse_wave(1.0, 0.3));
    assert_eq!(pulse_wave(0.2, 0.6), pulse_wave(1.2, 0.6));
}

#[test]
fn test_wave_functions_range() {
    for i in 0..100 {
        let x = i as f64 * 0.1;

        let sq = square_wave(x);
        assert!(sq == 1.0 || sq == -1.0);

        let saw = sawtooth(x);
        assert!(saw >= -1.0 && saw <= 1.0);

        let tri = triangle_wave(x);
        assert!(tri >= -1.0 && tri <= 1.0);

        let pulse = pulse_wave(x, 0.3);
        assert!(pulse == 1.0 || pulse == -1.0);
    }
}
