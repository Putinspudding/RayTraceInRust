use ray::Point3;
use ray::Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
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
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
