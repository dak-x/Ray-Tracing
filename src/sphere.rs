use crate::{
    hittable::{HitRecord, Hittable},
    ray,
    vec3::*,
};
use ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig() - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return None;
        }
        let sqrtd = disc.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec: HitRecord = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.set_normal_face(r, (rec.p - self.center) / self.radius);

        Some(rec)
    }
}
