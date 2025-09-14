pub fn ease_in_quad(x: f64) -> f64 {
    x * x
}

pub fn ease_out_quad(x: f64) -> f64 {
    1.0 - (1.0 - x) * (1.0 - x)
}

pub fn ease_in_out_quad(x: f64) -> f64 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - 2.0 * (1.0 - x) * (1.0 - x)
    }
}

pub fn bounce(x: f64) -> f64 {
    let t = x.abs() % 2.0;
    if t < 1.0 {
        t
    } else {
        2.0 - t
    }
}

pub fn elastic(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else {
        let c4 = (2.0 * std::f64::consts::PI) / 3.0;
        -(2.0_f64.powf(10.0 * (x - 1.0))) * ((x - 1.0) * c4 - std::f64::consts::PI / 2.0).sin()
    }
}