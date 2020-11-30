use crate::{ray, vec3::*, HitRecord, Hittable};
use ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        unimplemented!() // TODO
    }
}
