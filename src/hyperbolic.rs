pub fn sinh(x: f64) -> f64 {
    x.sinh()
}

pub fn cosh(x: f64) -> f64 {
    x.cosh()
}

pub fn tanh(x: f64) -> f64 {
    x.tanh()
}

pub fn coth(x: f64) -> f64 {
    1.0 / x.tanh()
}

pub fn csch(x: f64) -> f64 {
    1.0 / x.sinh()
}

pub fn sech(x: f64) -> f64 {
    1.0 / x.cosh()
}

pub fn arsinh(x: f64) -> f64 {
    x.asinh()
}

pub fn arcosh(x: f64) -> f64 {
    if x < 1.0 {
        return f64::NAN;
    }
    x.acosh()
}

pub fn artanh(x: f64) -> f64 {
    if x <= -1.0 || x >= 1.0 {
        return f64::NAN;
    }
    x.atanh()
}

pub fn arcoth(x: f64) -> f64 {
    if x.abs() <= 1.0 {
        return f64::NAN;
    }
    (1.0 / x).atanh()
}

pub fn arccsch(x: f64) -> f64 {
    if x == 0.0 {
        return f64::INFINITY;
    }
    (1.0 / x).asinh()
}

pub fn arcsech(x: f64) -> f64 {
    if x <= 0.0 || x > 1.0 {
        return f64::NAN;
    }
    (1.0 / x).acosh()
}
