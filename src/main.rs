mod color;
mod vec3;

use color::Color;
use std::io::stdout;
use std::io::Write;
const IMG_WIDTH: i32 = 256;
const IMG_HEIGHT: i32 = 256;

fn main() {
    let mut writer = stdout();

    writeln!(writer, "P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);
    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            let r: f32 = i as f32 / (IMG_WIDTH - 1) as f32;
            let g: f32 = j as f32 / (IMG_WIDTH - 1) as f32;
            let b: f32 = 0.25;
            let pixel: Color = Color::new(r, g, b);

            color::write_color(pixel, &mut writer);
        }
    }

    writer.flush().expect("Couldnt Flush Writer");
    eprintln! {"Done"};
}
