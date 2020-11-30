use crate::vec3::*;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(pixel: Color, out: &mut impl Write) -> std::io::Result<()> {
    let static_cast = |x: f32| (x * 255.999) as i32;
    write! {out, "{} {} {}\n",
    static_cast(pixel.0),
    static_cast(pixel.1),
    static_cast(pixel.2)}
}
