pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn cos(x: f64) -> f64 {
    x.cos()
}

pub fn tan(x: f64) -> f64 {
    x.tan()
}

pub fn cot(x: f64) -> f64 {
    1.0 / x.tan()
}

pub fn csc(x: f64) -> f64 {
    1.0 / x.sin()
}

pub fn sec(x: f64) -> f64 {
    1.0 / x.cos()
}

pub fn arcsin(x: f64) -> f64 {
    if x < -1.0 || x > 1.0 {
        return f64::NAN;
    }
    x.asin()
}

pub fn arccos(x: f64) -> f64 {
    if x < -1.0 || x > 1.0 {
        return f64::NAN;
    }
    x.acos()
}

pub fn arctan(x: f64) -> f64 {
    x.atan()
}

pub fn arccot(x: f64) -> f64 {
    (1.0 / x).atan()
}

pub fn arccsc(x: f64) -> f64 {
    if x.abs() < 1.0 {
        return f64::NAN;
    }
    (1.0 / x).asin()
}

pub fn arcsec(x: f64) -> f64 {
    if x.abs() < 1.0 {
        return f64::NAN;
    }
    (1.0 / x).acos()
}

pub fn sinc(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }

    if x == 0.0 {
        return 1.0;
    }

    if x.is_infinite() {
        return 0.0;
    }

    x.sin() / x
}