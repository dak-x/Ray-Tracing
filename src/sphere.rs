use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::*,
};
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, m: &Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat_ptr: m.clone(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig() - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
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
        rec.mat_ptr = Some(self.mat_ptr.clone());

        Some(rec)
    }
}

impl std::convert::From<Sphere> for Rc<dyn Hittable> {
    fn from(sph: Sphere) -> Self {
        let trait_object: Rc<dyn Hittable> = Rc::new(sph);
        trait_object
    }
}
