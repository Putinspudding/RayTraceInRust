use crate::rayh::*;
use ray::*;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        Camera {
            origin: Vec3(0.0, 0.0, 0.0),
            horizontal: Vec3(viewport_width, 0.0, 0.0),
            vertical: Vec3(0.0, viewport_height, 0.0),
            lower_left_corner: Vec3(0.0, 0.0, 0.0)
                - Vec3(viewport_width, 0.0, 0.0) / 2.0
                - Vec3(0.0, viewport_height, 0.0) / 2.0
                - Vec3(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
