pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn fmax(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn degrees_to_radians(phi: f64) -> f64 {
    phi * std::f64::consts::PI / 180.0
}
