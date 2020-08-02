mod color;
mod hittable;
mod hittable_list;
mod rayh;
mod rtweekend;
mod sphere;
pub use color::*;
pub use hittable::*;
pub use hittable_list::*;
pub use ray::*;
pub use rayh::*;
use rtweekend::*;
pub use sphere::*;

const IMAGE_WIDTH: u16 = 400;
const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    let mut rec = Hit_record {
        p: Vec3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: true,
    };
    if world.hit(r, 0.0, infinity, &mut rec) {
        return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}
fn main() {
    let IMAGE_HEIGHT = unsafe { (IMAGE_WIDTH as f32 / ASPECT_RATIO).to_int_unchecked::<u16>() };

    let mut world = Hittable_list {
        objects: Vec::new(),
    };
    world.add(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    println! {"P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT};
    let viewport_height = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2 as f32 - vertical / 2 as f32 - Vec3(0.0, 0.0, focal_length);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining:{}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color = ray_color(&r, &world);
            print!("{}", write_color(pixel_color));
        }
    }
    eprintln!("\nDone");
}
