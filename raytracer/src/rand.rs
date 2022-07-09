use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_double_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
