use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_double_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_int_between(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0.0..1.0);
    min + ((((max - min + 1) as f64) * num) as i32)
}
