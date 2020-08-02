pub const infinity: f32 = f32::INFINITY;
pub const PI: f32 = 3.14159274f32;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
