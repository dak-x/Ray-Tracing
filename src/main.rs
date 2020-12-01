mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

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

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, INFINITY) {
        return 0.5 * (rec.normal + Color::default());
    }

    let unit_dir = r.dir().unit_vector();
    let t: f32 = 0.5 * (unit_dir.y() + 1.0);

    (1.0 - t) * Color::default() + t * Color::new(0.5, 0.7, 1.0)
}

// *=======================================================
fn main() {
    let mut writer = stdout();

    // * IMAGE
    const aspect_ratio: f32 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f32 / aspect_ratio) as i32;

    // * WORLD
    let sph1: Rc<dyn Hittable> = Rc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5));
    let sph2: Rc<dyn Hittable> = Rc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0));

    let mut world = HittableList::new();
    world.add(&sph1);
    world.add(&sph2);
    
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

            let pixel: Color = ray_color(&r, &world);

            color::write_color(pixel, &mut writer);
        }
    }

    writer.flush().expect("Couldnt Flush Writer");
    eprintln! {"Done"};
}
