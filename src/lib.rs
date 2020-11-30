mod color;
mod ray;
mod sphere;
mod vec3;

use ray::Ray;
use vec3::*;

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
}

/// Trait which describes if a hit with a ray and returns a HitRecord
pub trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
