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
use std::io::{stdout,Write};
use vec3::*;
use write_buf::*;
/* ======================================================= */

const INFINITY: f64 = std::f64::INFINITY;
const ASPECT_RATIO: f64 = 4.0 / 3.0;
const IMG_WIDTH: i32 = 800;
const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;

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
    let mut writer = WriteBufVec::new(stdout());
    // * IMAGE
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // * WORLD and CAMERA
    let (world, cam) = random_scene();

    // * RENDER
    writeln!(writer, "P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT).unwrap();
    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Lines Remaining: {}", j);
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
fn init_world() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    // =================================================================
    // For Fuzzy two metals:
    // let material_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0)).into();
    // let material_center = Lambertian::new(Vec3(0.7, 0.3, 0.3)).into();
    // let material_left = Metal::new(Vec3(0.8, 0.8, 0.8), 0.3).into();
    // let material_right = Metal::new(Vec3(0.8, 0.6, 0.2), 1.0).into();
    // =================================================================

    // =================================================================
    // Making center and right balls glass (dielectric):
    let material_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0)).into();
    let material_center = Lambertian::new(Vec3(0.1, 0.2, 0.5)).into();
    let material_left = Dielectric::new(1.5).into();
    let material_right = Metal::new(Vec3(0.8, 0.6, 0.2), 0.0).into();
    // =================================================================

    world.add(&Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, &material_ground).into());
    world.add(&Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, &material_center).into());
    world.add(&Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5, &material_left).into());
    world.add(&Sphere::new(Vec3(-1.0, 0.0, -1.0), -0.45, &material_left).into());
    world.add(&Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, &material_right).into());

    let lookfrom = Vec3(-2.0, 2.0, 1.0);
    let lookat = Vec3(0.0, 0.0, -1.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.01;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    (world, cam)
}

fn init_world2() -> (HittableList, Camera) {
    // ==========================================================================
    // Testing FOV in the camera
    let r = (PI / 4.0).cos();
    let mut world = HittableList::new();
    let material_left = Lambertian::new(Vec3(0.0, 0.0, 1.0)).into();
    let material_right = Lambertian::new(Vec3(1.0, 0.0, 0.0)).into();

    world.add(&Sphere::new(Vec3(-r, 0.0, -1.0), r, &material_left).into());
    world.add(&Sphere::new(Vec3(r, 0.0, -1.0), r, &material_right).into());
    //==========================================================================
    let lookfrom = Vec3(-2.0, 2.0, 1.0);
    let lookat = Vec3(0.0, 0.0, -1.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    (world, cam)
}

fn random_scene() -> (HittableList, Camera) {
    // * RANDOM SCENE:
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Vec3(0.5, 0.5, 0.5)).into();

    world.add(&Sphere::new(Vec3(0.0, -1000.0, 0.0), 1000.0, &ground_material).into());

    for a in -11i32..11 {
        for b in -11i32..11 {
            let choose_mat = random_f64();
            let center = Vec3(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sph_mat = if choose_mat < 0.8 {
                    // Diffuse
                    Lambertian::new(Color::random() * Color::random()).into()
                } else if choose_mat < 0.95 {
                    // metal
                    Metal::new(Color::random_range(0.5, 1.0), random_range(0.5, 1.0)).into()
                } else {
                    //glass
                    Dielectric::new(1.5).into()
                };
                world.add(&Sphere::new(center, 0.2, &sph_mat).into());
            }
        }
    }

    let mat1 = Dielectric::new(1.5).into();
    world.add(&Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, &mat1).into());

    let mat2 = Lambertian::new(Vec3(0.4, 0.2, 0.1)).into();
    world.add(&Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, &mat2).into());

    let mat3 = Metal::new(Vec3(0.7, 0.6, 0.5), 0.0).into();
    world.add(&Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, &mat3).into());

    // * CAMERA
    let lookfrom = Vec3(13.0, 5.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    (world, cam)
}
