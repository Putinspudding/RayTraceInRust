mod camera;
mod color;
mod hittable;
mod hittable_list;
mod rayh;
mod rtweekend;
mod sphere;
use camera::*;
use color::*;
use hittable::*;
use hittable_list::*;
use ray::*;
use rayh::*;
use rtweekend::*;
use sphere::*;

const IMAGE_WIDTH: u16 = 400;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const SAMPLES_PER_PIXEL: u16 = 100;
const MAX_DEPTH: u16 = 50;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: u16) -> Color {
    let mut rec = Hit_record {
        p: Vec3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: true,
    };
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.0, infinity, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5
            * ray_color(
                &Ray {
                    orig: rec.p,
                    dir: target - rec.p,
                },
                world,
                depth - 1,
            );
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

    let cam = Camera::new();

    println! {"P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT};
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining:{}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random_double(None, None)) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random_double(None, None)) / (IMAGE_HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            print!("{}", write_color(pixel_color, SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("\nDone");
}
