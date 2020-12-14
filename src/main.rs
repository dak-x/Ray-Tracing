mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;
// *=======================================================

use camera::*;
use color::Color;
use hittable::*;
use ray::*;
use sphere::*;
use std::io::{Write,BufWriter,stdout};
use std::rc::Rc;
use vec3::*;
// *=======================================================

const INFINITY: f64 = std::f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

#[inline]
fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

#[inline]
fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth < 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.0, INFINITY) {
        let target: Point3 = rec.p + rec.normal + Vec3::randon_in_unit_sphere();
        let new_ray = Ray::new(rec.p, target - rec.p);
        return 0.5 * ray_color(&new_ray, world, depth - 1);
    }
    let unit_dir = r.dir().unit_vector();
    let t: f64 = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::default() + t * Color::new(0.5, 0.7, 1.0)
}

#[inline]
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

// *=======================================================
fn main() {
    let mut writer = BufWriter::new(stdout());
    
    // * IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    // * CAMERA
    let cam = Camera::new();

    // * WORLD
    // Todo: THIS LOOKS UGLY!!!!!
    let sph1: Rc<dyn Hittable> = Rc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5));
    let sph2: Rc<dyn Hittable> = Rc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0));
    let mut world = HittableList::new();
    world.add(&sph1);
    world.add(&sph2);

    // * RENDER
    writeln!(writer, "P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);
    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMG_WIDTH - 1) as f64;
                let v = (j as f64 + random_f64()) / (IMG_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world,MAX_DEPTH);
            }
            color::write_color(&mut writer, pixel_color, SAMPLES_PER_PIXEL).unwrap();
        }
    }

    writer.flush().expect("Couldnt Flush Writer");
    eprintln! {"Done"};
}
