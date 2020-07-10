mod color;
mod rayh;
use color::*;
use ray::Vec3;

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

fn main() {
    println! {"P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT};
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining:{}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Vec3::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.25 as f32,
            );
            print!("{}", write_color(pixel_color));
        }
    }
    eprintln!("\nDone");
}
