use crate::clamp;
use crate::vec3::*;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(
    out: &mut impl Write,
    pixel: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let static_cast = |x: f32| (256.0 * clamp(x / samples_per_pixel as f32, 0.0, 0.999)) as i32;
    write! {out, "{} {} {}\n",
    static_cast(pixel.0),
    static_cast(pixel.1),
    static_cast(pixel.2)}
}
