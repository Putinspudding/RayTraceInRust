use rand::{thread_rng, Rng};
pub const infinity: f32 = f32::INFINITY;
pub const PI: f32 = 3.14159274f32;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double(min: Option<f32>, max: Option<f32>) -> f32 {
    let mut rng = thread_rng();
    let n: f32 = rng.gen_range(min.unwrap_or(0.0), max.unwrap_or(1.0));
    n
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
