use crate::hittable::*;
use crate::rayh::*;
use ray::*;

pub struct Hittable_list<T> {
    pub objects: Vec<T>,
}

impl<T: Hittable> Hittable_list<T> {
    pub fn new(object: T) -> Hittable_list<T> {
        Hittable_list {
            objects: vec![object],
        }
    }
    pub fn clear(&mut self) {
        (*self).objects.truncate(0);
    }
    pub fn add(&mut self, object: T) {
        (*self).objects.push(object);
    }
}
impl<T: Hittable> Hittable for Hittable_list<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut Hit_record) -> bool {
        let mut temp_rec = Hit_record {
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut ar_temp_rec = temp_rec;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                ar_temp_rec = temp_rec;
            }
        }
        *rec = ar_temp_rec;
        hit_anything
    }
}
