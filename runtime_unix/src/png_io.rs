use image::{ImageBuffer, ImageFormat, RgbImage};
use mathlib_renderer::{
    io::ppm::COLOR_MAXVAL,
    visual::{canvas::Canvas, color::Col},
};

pub fn canvas_png_save(canvas: &Canvas, path: &str) {
    let mut buffer: RgbImage = ImageBuffer::new(canvas.width as u32, canvas.height as u32);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let x: usize = x.try_into().unwrap_or_default();
        let y: usize = y.try_into().unwrap_or_default();
        let col: Col = canvas[y][x];
        *pixel = image::Rgb([base_255(col.r), base_255(col.g), base_255(col.b)]);
    }
    buffer
        .save_with_format(path, ImageFormat::Png)
        .expect("unable to write file");
    // buffer.write_to(writer, ImageOutputFormat::Png)
}

// uses COLOR_MAXVAL to translate 0-1 range into percentage of that value
fn base_255(f: f64) -> u8 {
    match f {
        n if n < 0.0 => 0,
        n if n > COLOR_MAXVAL as f64 => 255,
        n => (n * ((COLOR_MAXVAL + 1) as f64)).floor() as u8,
    }
}
