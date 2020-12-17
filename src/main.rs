/*
    Identifying all the various modules in the crate
*/
mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
/* ======================================================= */

use camera::*;
use color::Color;
use hittable::*;
use material::*;
use ray::*;
use sphere::*;
use std::io::{stdout, BufWriter, Write};
use vec3::*;
/* ======================================================= */

const INFINITY: f64 = std::f64::INFINITY;
// const PI: f64 = std::f64::consts::PI;
//
// #[inline]
// fn deg_to_rad(deg: f64) -> f64 {
// deg * PI / 180.0
// }

#[inline]
fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth < 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let mut attenuation = Color::default();

        if let Some(ref scattered) =
            rec.mat_ptr
                .as_ref()
                .unwrap()
                .scatter(r, &rec, &mut attenuation)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
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

fn main() {
    let mut writer = BufWriter::new(stdout());
    // * IMAGE
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    // * CAMERA
    let cam = Camera::new();

    // * WORLD
    let world = init_world();

    // * RENDER
    writeln!(writer, "P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT).unwrap();
    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMG_WIDTH - 1) as f64;
                let v = (j as f64 + random_f64()) / (IMG_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut writer, pixel_color, SAMPLES_PER_PIXEL).unwrap();
        }
    }

    writer.flush().expect("Couldnt Flush Writer");
    eprintln! {"Done"};
}

// * Init the objects in the world.
// ! This looks ugly!!!!
fn init_world() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0)).into();
    let material_center = Lambertian::new(Vec3(0.7, 0.3, 0.3)).into();
    let material_left = Metal::new(Vec3(0.8, 0.8, 0.8), 0.3).into();
    let material_right = Metal::new(Vec3(0.8, 0.6, 0.2), 1.0).into();

    world.add(&Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, &material_ground).into());
    world.add(&Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, &material_center).into());
    world.add(&Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5, &material_left).into());
    world.add(&Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, &material_right).into());

    world
}
