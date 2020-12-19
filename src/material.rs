use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{clamp, color};
use crate::{hittable::HitRecord, vec3::random_f64};
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
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: color::Color, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Metal { albedo, fuzz }
        } else {
            Metal { albedo, fuzz: 1f64 }
        }
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
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        *attenuation = self.albedo;

        if scattered.dir().dot(&rec.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    #[inline]
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (f64::powi(1.0 - cosine, 5))
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut color::Color,
    ) -> Option<Ray> {
        *attenuation = Vec3(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = ray_in.dir().unit_vector();
        let cos_theta = clamp((-unit_dir).dot(&rec.normal), -1.0, 1.0);
        let sin_theta = (1f64 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) < random_f64()
        {
            Vec3::reflect(&unit_dir, &rec.normal)
        } else {
            Vec3::refract(&unit_dir, &rec.normal, refraction_ratio)
        };

        Some(Ray::new(rec.p, direction))
    }
}

use std::convert::From;

impl From<Lambertian> for Rc<dyn Material> {
    #[inline]
    fn from(material: Lambertian) -> Self {
        let trait_object: Rc<dyn Material> = Rc::new(material);
        trait_object
    }
}

impl From<Metal> for Rc<dyn Material> {
    #[inline]
    fn from(material: Metal) -> Self {
        let trait_object: Rc<dyn Material> = Rc::new(material);
        trait_object
    }
}

impl From<Dielectric> for Rc<dyn Material> {
    #[inline]
    fn from(material: Dielectric) -> Self {
        let trait_object: Rc<dyn Material> = Rc::new(material);
        trait_object
    }
}
