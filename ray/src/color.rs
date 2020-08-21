use crate::rtweekend::*;
use ray::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: u16) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    unsafe {
        format!(
            "{} {} {}\n",
            (256.0 * clamp(r, 0.0, 0.999)).to_int_unchecked::<u16>(),
            (256.0 * clamp(g, 0.0, 0.999)).to_int_unchecked::<u16>(),
            (256.0 * clamp(b, 0.0, 0.999)).to_int_unchecked::<u16>(),
        )
    }
}
