use crate::color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;
pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut color::Color)
        -> Option<Ray>;
}
/* ========================================== */
#[derive(Default)]
pub struct Lambertian {
    pub albedo: color::Color,
}

impl Lambertian {
    pub fn new(albedo: color::Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, attenuation: &mut color::Color) -> Option<Ray> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        *attenuation = self.albedo;
        Some(Ray::new(rec.p, scatter_dir))
    }
}
/* ============================================= */
#[derive(Default)]
pub struct Metal {
    albedo: color::Color,
}
impl Metal {
    pub fn new(albedo: color::Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut color::Color,
    ) -> Option<Ray> {
        let reflected = Vec3::reflect(&ray_in.dir().unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);

        *attenuation = self.albedo;

        if scattered.dir().dot(&rec.normal) > 0.0 {    
            Some(scattered)
        } else {
            None
        }
    }
}

use std::convert::From;

impl From<Lambertian> for Rc<dyn Material> {
    fn from(material: Lambertian) -> Self {
        let trait_object: Rc<dyn Material> = Rc::new(material);
        trait_object
    }
}

impl From<Metal> for Rc<dyn Material> {
    fn from(material: Metal) -> Self {
        let trait_object: Rc<dyn Material> = Rc::new(material);
        trait_object
    }
}
