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
use std::io::stdout;
use std::io::Write;
use std::rc::Rc;
use vec3::*;
// *=======================================================

const INFINITY: f32 = std::f32::INFINITY;
const PI: f32 = std::f32::consts::PI;

#[inline]
fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

#[inline]
/// Returns a random real in [0.1)
fn random_f32() -> f32 {
    rand::random::<f32>() / (std::f32::MAX + 1.0)
}
#[inline]
fn random_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_f32()
}

#[inline]
fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, INFINITY) {
        return 0.5 * (rec.normal + Color::default());
    }
    let unit_dir = r.dir().unit_vector();
    let t: f32 = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::default() + t * Color::new(0.5, 0.7, 1.0)
}

#[inline]
fn clamp(x: f32, min: f32, max: f32) -> f32 {
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
    let mut writer = stdout();

    // * IMAGE
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

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
                let u = (i as f32 + random_f32()) / (IMG_WIDTH - 1) as f32;
                let v = (j as f32 + random_f32()) / (IMG_HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            color::write_color(&mut writer, pixel_color, SAMPLES_PER_PIXEL).unwrap();
        }
    }

    writer.flush().expect("Couldnt Flush Writer");
    eprintln! {"Done"};
}
