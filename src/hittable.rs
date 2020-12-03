use crate::ray::Ray;
use crate::vec3;

/// SOME DEFINITIONS
#[derive(Default)]
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn set_normal_face(&mut self, r: &Ray, outward_normal: vec3::Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

/// Trait which describes if a hit with a ray and returns a HitRecord
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: &Rc<dyn Hittable>) {
        self.objects.push(object.clone());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, t_max) {
                if rec.t < closest_so_far {
                    closest_so_far = temp_rec.t;
                    temp_rec = rec;
                    hit_anything = true;
                }
            }
        }

        if hit_anything {
            Some(temp_rec)
        } else {
            None
        }
    }
}
