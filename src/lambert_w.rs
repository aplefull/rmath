fn product_log_branch_minus1(z: f64) -> f64 {
    if z >= 0.0 || z <= -1.0 / std::f64::consts::E {
        return f64::NAN;
    }

    let p = (std::f64::consts::E * z + 1.0).sqrt();
    let mut w = -1.0 + p - p * p / 3.0 + 11.0 * p * p * p / 72.0;

    const MAX_ITERS: usize = 20;
    const EPSILON: f64 = 1e-15;

    for _ in 0..MAX_ITERS {
        let ew = w.exp();
        let wew = w * ew;
        let f = wew - z;
        let f1 = ew * (w + 1.0);
        let f2 = ew * (w + 2.0);

        let delta = f * f1 / (f1 * f1 - 0.5 * f * f2);
        w -= delta;

        if delta.abs() < EPSILON * (1.0 + w.abs()) {
            break;
        }
    }

    w
}

pub fn product_log(z: f64) -> f64 {
    if z.is_nan() {
        return f64::NAN;
    }

    if z.is_infinite() {
        return if z.is_sign_positive() {
            f64::INFINITY
        } else {
            f64::NAN
        }
    }

    let branch_point = -1.0 / std::f64::consts::E;
    if z < branch_point {
        return f64::NAN;
    }

    if z == 0.0 {
        return 0.0;
    }

    if z == branch_point {
        return -1.0;
    }

    let mut w: f64;

    if z < 0.0 {
        let p = (2.0 * (std::f64::consts::E * z + 1.0)).sqrt();
        w = -1.0 + p - p * p / 3.0 + 11.0 * p * p * p / 72.0;
    } else if z < 0.1 {
        let z2 = z * z;
        let z3 = z2 * z;
        let z4 = z3 * z;
        let z5 = z4 * z;
        w = z - z2 + 1.5 * z3 - 8.0/3.0 * z4 + 125.0/24.0 * z5;
    } else if z < 3.0 {
        w = z / (1.0 + z);

        for _ in 0..3 {
            let ew = w.exp();
            let wew = w * ew;
            let f = wew - z;
            let df = ew * (w + 1.0);
            w -= f / df;
        }
    } else {
        let lz = z.ln();
        let llz = lz.ln();
        w = lz - llz + llz / lz;
    }

    const MAX_ITERS: usize = 50;
    const EPSILON: f64 = 1e-15;

    for _ in 0..MAX_ITERS {
        let ew = w.exp();
        let wew = w * ew;
        let f = wew - z;

        if f.abs() < EPSILON * (1.0 + z.abs()) {
            break;
        }

        let f1 = ew * (w + 1.0);

        if f1.abs() < 1e-15 {
            break;
        }

        let f2 = ew * (w + 2.0);

        let denominator = f1 * f1 - 0.5 * f * f2;
        let delta = if denominator.abs() > 1e-15 {
            f * f1 / denominator
        } else {
            f / f1
        };

        w -= delta;

        if delta.abs() < EPSILON * (1.0 + w.abs()) {
            break;
        }
    }

    w
}
pub fn product_log_k(k: i32, z: f64) -> f64 {
    if k == 0 {
        return product_log(z);
    }

    if k == -1 && z < 0.0 && z > -1.0 / std::f64::consts::E {
        return product_log_branch_minus1(z);
    }

    f64::NAN
}