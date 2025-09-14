const ERF_A: [f64; 5] = [
    0.254829592,
    -0.284496736,
    1.421413741,
    -1.453152027,
    1.061405429,
];

const ERF_P: f64 = 0.3275911;

pub fn erf(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }

    if x.is_infinite() {
        return if x.is_sign_positive() { 1.0 } else { -1.0 };
    }

    if x == 0.0 {
        return 0.0;
    }

    let sign = x.signum();
    let x = x.abs();

    if x > 6.0 {
        return sign;
    }

    // Abramowitz and Stegun approximation
    let t = 1.0 / (1.0 + ERF_P * x);
    let poly = ERF_A[0] * t
        + ERF_A[1] * t * t
        + ERF_A[2] * t * t * t
        + ERF_A[3] * t * t * t * t
        + ERF_A[4] * t * t * t * t * t;
    let result = 1.0 - poly * (-x * x).exp();

    sign * result
}

pub fn erfc(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }

    if x.is_infinite() {
        return if x.is_sign_positive() { 0.0 } else { 2.0 };
    }

    if x > 6.0 {
        return 0.0;
    }

    if x < -6.0 {
        return 2.0;
    }

    1.0 - erf(x)
}
