mod color;
mod rayh;
use color::*;
use ray::*;
use rayh::*;

const IMAGE_WIDTH: u16 = 384;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let oc: Vec3 = &(r.origin()) - &center;
    let a = dot(&(r.direction()), &(r.direction()));
    let b = 2.0 * dot(&oc, &(r.direction()));
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4 as f32 * a * c;
    discriminant > 0 as f32
}
fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Vec3(0 as f32, 0 as f32, -1 as f32), 0.5, &r) {
        return Vec3(1 as f32, 0 as f32, 0 as f32);
    }
    let unit_direction = unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    &((1.0 - t) * &Vec3(1.0, 1.0, 1.0)) + &(t * &Vec3(0.5, 0.7, 1.0))
}
fn main() {
    let IMAGE_HEIGHT = unsafe { (IMAGE_WIDTH as f32 / ASPECT_RATIO).to_int_unchecked::<u16>() };
    println! {"P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT};
    let viewport_height = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = &(&(&origin - &(&horizontal / 2 as f32)) - &(&vertical / 2 as f32))
        - &Vec3(0.0, 0.0, focal_length);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining:{}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray {
                orig: origin,
                dir: &(&(&lower_left_corner + &(u * &horizontal)) + &(v * &vertical)) - &origin,
            };
            let pixel_color = ray_color(&r);
            print!("{}", write_color(pixel_color));
        }
    }
    eprintln!("\nDone");
}
