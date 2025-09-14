fn normalize_phase(x: f64) -> f64 {
    let t = x % 1.0;
    if t < 0.0 {
        t + 1.0
    } else {
        t
    }
}

pub fn square_wave(x: f64) -> f64 {
    let t = normalize_phase(x);
    if t < 0.5 {
        1.0
    } else {
        -1.0
    }
}

pub fn sawtooth(x: f64) -> f64 {
    let t = normalize_phase(x);
    2.0 * t - 1.0
}

pub fn triangle_wave(x: f64) -> f64 {
    let t = normalize_phase(x);
    if t < 0.5 {
        4.0 * t - 1.0
    } else {
        3.0 - 4.0 * t
    }
}

pub fn pulse_wave(x: f64, duty: f64) -> f64 {
    let duty_cycle = duty.max(0.0).min(1.0);
    let t = normalize_phase(x);
    if t < duty_cycle {
        1.0
    } else {
        -1.0
    }
}
