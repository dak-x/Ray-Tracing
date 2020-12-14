use crate::color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::Vec3;
pub trait Material {
    fn scatter(
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut color::Color,
        ray_scattered: &mut Ray,
    ) -> bool;
}
