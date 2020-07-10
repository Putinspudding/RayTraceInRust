use ray::Color;
use ray::Point3;
use ray::Vec3;

struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            orig: Default::default(),
            dir: Default::default(),
        }
    }
}

impl Ray {
    fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    fn origin(&self) -> Point3 {
        self.orig
    }

    fn direction(&self) -> Vec3 {
        self.dir
    }

    fn at(&self, t: f32) -> Point3 {
        &self.orig + &(t * &self.dir)
    }
}
