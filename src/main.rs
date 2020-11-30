/// SOME DEFINITIONS
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
}

/// Trait which describes if a hit with a ray and returns a HitRecord
pub trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

// *=======================================================
mod color;
mod ray;
mod sphere;
mod vec3;

use color::Color;
use ray::*;
use std::io::stdout;
use std::io::Write;
use vec3::*;
// *=======================================================

fn main() {
    let mut writer = stdout();
    // * IMAGE
    const aspect_ratio: f32 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f32 / aspect_ratio) as i32;

    // * CAMERA
    const viewport_height: f32 = 2.0;
    const viewport_width: f32 = aspect_ratio * viewport_height;
    const focal_length: f32 = 1.0;

    let origin = Point3::new(0f32, 0f32, 0f32);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3(0.0, 0.0, focal_length);

    // * RENDER

    writeln!(writer, "P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);
    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            let u: f32 = i as f32 / (IMG_WIDTH - 1) as f32;
            let v: f32 = j as f32 / (IMG_HEIGHT - 1) as f32;
            let b: f32 = 0.25;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel: Color = ray_color(&r);

            color::write_color(pixel, &mut writer);
        }
    }

    writer.flush().expect("Couldnt Flush Writer");
    eprintln! {"Done"};
}

fn ray_color(r: &Ray) -> Color {
    if let Some(t) = hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, r) {
        let N: Vec3 = (r.at(t) - Vec3(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }

    let unit_dir = r.dir().unit_vector();
    let t: f32 = 0.5 * (unit_dir.y() + 1.0);

    (1.0 - t) * Color::default() + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> Option<f32> {
    let oc = r.orig() - *center;
    let a = r.dir.length_squared();
    let half_b = oc.dot(r.dir);
    let c = oc.length_squared() - radius * radius;
    let disc = half_b * half_b - a * c;
    if disc < 0.0 {
        None
    } else {
        Some((-half_b - disc.sqrt()) / a)
    }
}
