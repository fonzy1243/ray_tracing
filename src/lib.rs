use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn random_int(min: i32, max: i32) -> i32 {
    random_double_r(min as f64, (max + 1) as f64).round() as i32
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0f64..1.)
}

pub fn random_double_r(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
