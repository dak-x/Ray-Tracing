use crate::clamp;
use crate::vec3::*;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(
    out: &mut impl Write,
    pixel: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let static_cast = |x: f64| (256.0 * clamp(x / samples_per_pixel as f64, 0.0, 0.999)) as i32;
    out.write(format!("{} {} {}\n",
    static_cast(pixel.0),
    static_cast(pixel.1),
    static_cast(pixel.2)).as_bytes())?;
    Ok(())
    // write! {"{} {} {}\n",
    // static_cast(pixel.0),
    // static_cast(pixel.1),
    // static_cast(pixel.2)}
}
