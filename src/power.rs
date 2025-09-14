pub fn sqrt(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z == 0.0 {
        return 0.0;
    }

    if z < 0.0 {
        return f64::NAN;
    }

    if z.is_infinite() && z.is_sign_positive() {
        return f64::INFINITY;
    }

    z.sqrt()
}

pub fn power(z: f64, a: f64) -> f64 {
    if z.is_nan() || a.is_nan() {
        return f64::NAN;
    }

    if a == 0.0 {
        return 1.0;
    }

    if z == 0.0 {
        if a > 0.0 {
            return 0.0;
        } else if a < 0.0 {
            return f64::INFINITY;
        }
    }

    if z == 1.0 {
        return 1.0;
    }

    if a.fract() == 0.0 && a.abs() < 1e10 {
        let n = a as i64;
        return integer_power(z, n);
    }

    if z < 0.0 && a.fract() != 0.0 {
        return f64::NAN;
    }

    if z < 0.0 && a.fract() == 0.0 {
        let n = a as i64;
        let is_odd = n % 2 != 0;
        let abs_result = (-z).powf(a);
        return if is_odd { -abs_result } else { abs_result };
    }

    z.powf(a)
}

fn integer_power(z: f64, n: i64) -> f64 {
    if n == 0 {
        return 1.0;
    }

    if z == 0.0 {
        return if n > 0 { 0.0 } else { f64::INFINITY };
    }

    if n < 0 {
        return 1.0 / integer_power(z, -n);
    }

    let mut result = 1.0;
    let mut base = z;
    let mut exp = n;

    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }

    result
}

pub fn square(z: f64) -> f64 {
    z * z
}

pub fn cube(z: f64) -> f64 {
    z * z * z
}

pub fn nth_root(z: f64, n: i32) -> f64 {
    if n == 0 {
        return f64::NAN;
    }

    if z == 0.0 {
        return 0.0;
    }

    if z < 0.0 {
        return if n % 2 == 0 {
            f64::NAN
        } else {
            -(-z).powf(1.0 / n as f64)
        };
    }

    if n == 2 {
        return z.sqrt();
    }

    if n == 3 {
        return z.cbrt();
    }

    z.powf(1.0 / n as f64)
}

pub fn cbrt(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z == 0.0 || z.is_infinite() {
        return z;
    }

    z.cbrt()
}

pub fn exp(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z == 0.0 {
        return 1.0;
    }

    if z.is_infinite() {
        return if z.is_sign_positive() {
            f64::INFINITY
        } else {
            0.0
        };
    }

    z.exp()
}

pub fn exp2(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z == 0.0 {
        return 1.0;
    }

    if z.is_infinite() {
        return if z.is_sign_positive() {
            f64::INFINITY
        } else {
            0.0
        };
    }

    z.exp2()
}

pub fn exp10(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z == 0.0 {
        return 1.0;
    }

    if z.is_infinite() {
        return if z.is_sign_positive() {
            f64::INFINITY
        } else {
            0.0
        };
    }

    const LN_10: f64 = std::f64::consts::LN_10;
    (z * LN_10).exp()
}
