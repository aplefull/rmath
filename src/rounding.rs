pub fn floor(z: f64) -> f64 {
    if z.is_nan() || z.is_infinite() {
        return z;
    }
    z.floor()
}

pub fn ceiling(z: f64) -> f64 {
    if z.is_nan() || z.is_infinite() {
        return z;
    }
    z.ceil()
}

pub fn round(z: f64) -> f64 {
    if z.is_nan() || z.is_infinite() {
        return z;
    }
    z.round()
}

pub fn integer_part(z: f64) -> f64 {
    if z.is_nan() || z.is_infinite() {
        return z;
    }
    z.trunc()
}

pub fn fractional_part(z: f64) -> f64 {
    if z.is_nan() || z.is_infinite() {
        return f64::NAN;
    }
    z - z.trunc()
}

pub fn modulo(m: f64, n: f64) -> f64 {
    if m.is_nan() || n.is_nan() {
        return f64::NAN;
    }

    if n == 0.0 {
        return f64::NAN;
    }

    if m.is_infinite() {
        return f64::NAN;
    }

    if n.is_infinite() {
        return m;
    }

    let result = m % n;
    if result.signum() != n.signum() && result != 0.0 {
        result + n
    } else {
        result
    }
}

pub fn quotient(m: f64, n: f64) -> f64 {
    if m.is_nan() || n.is_nan() {
        return f64::NAN;
    }

    if n == 0.0 {
        return if m == 0.0 { f64::NAN } else { f64::INFINITY * m.signum() };
    }

    if m.is_infinite() {
        return if n.is_infinite() { f64::NAN } else { f64::INFINITY * m.signum() * n.signum() };
    }

    if n.is_infinite() {
        return 0.0;
    }

    floor(m / n)
}