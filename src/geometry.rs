pub fn step(edge: f64, x: f64) -> f64 {
    if x < edge {
        0.0
    } else {
        1.0
    }
}

pub fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

pub fn fmod(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        f64::NAN
    } else {
        x - y * (x / y).trunc()
    }
}