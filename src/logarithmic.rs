pub fn log(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z == 0.0 {
        return f64::NEG_INFINITY;
    }

    if z < 0.0 {
        return f64::NAN;
    }

    if z == 1.0 {
        return 0.0;
    }

    if z.is_infinite() && z.is_sign_positive() {
        return f64::INFINITY;
    }

    z.ln()
}

pub fn log_base(a: f64, z: f64) -> f64 {
    if a.is_nan() || z.is_nan() {
        return f64::NAN;
    }

    if a == 1.0 {
        return f64::NAN;
    }

    if a <= 0.0 {
        return f64::NAN;
    }

    if z == 0.0 {
        return f64::NEG_INFINITY;
    }

    if z < 0.0 {
        return f64::NAN;
    }

    if z == 1.0 {
        return 0.0;
    }

    if z == a {
        return 1.0;
    }

    if z.is_infinite() && z.is_sign_positive() {
        if a > 1.0 {
            return f64::INFINITY;
        } else if a < 1.0 && a > 0.0 {
            return f64::NEG_INFINITY;
        }
    }

    if a.is_infinite() && a.is_sign_positive() {
        if z > 1.0 {
            return 0.0;
        } else if z < 1.0 {
            return -0.0;
        }
    }

    z.ln() / a.ln()
}

pub fn log10(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z == 0.0 {
        return f64::NEG_INFINITY;
    }

    if z < 0.0 {
        return f64::NAN;
    }

    if z == 1.0 {
        return 0.0;
    }

    if z == 10.0 {
        return 1.0;
    }

    if z.is_infinite() && z.is_sign_positive() {
        return f64::INFINITY;
    }

    z.log10()
}

pub fn log2(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z == 0.0 {
        return f64::NEG_INFINITY;
    }

    if z < 0.0 {
        return f64::NAN;
    }

    if z == 1.0 {
        return 0.0;
    }

    if z == 2.0 {
        return 1.0;
    }

    if z.is_infinite() && z.is_sign_positive() {
        return f64::INFINITY;
    }

    z.log2()
}
