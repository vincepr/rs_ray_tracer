

use image::{ImageBuffer, ImageFormat, RgbImage}; // first img library i found might look for a smalller ones later

use crate::{
    io::ppm::COLOR_MAXVAL,
    visual::{canvas::Canvas, color::Col},
};

impl Canvas {
    pub fn canvas_png_save(&self, path: &str) {
        let mut buffer: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let x: usize = x.try_into().unwrap_or_default();
            let y: usize = y.try_into().unwrap_or_default();
            let col: Col = self[y][x];
            *pixel = image::Rgb([base_255(col.r), base_255(col.g), base_255(col.b)]);
        }
        buffer
            .save_with_format(path, ImageFormat::Png)
            .expect("unable to write file");
        // buffer.write_to(writer, ImageOutputFormat::Png)
    }
}

// uses COLOR_MAXVAL to translate 0-1 range into percentage of that value
fn base_255(f: f32) -> u8 {
    match f {
        n if n < 0.0 => 0,
        n if n > COLOR_MAXVAL as f32 => 255,
        n => (n * ((COLOR_MAXVAL + 1) as f32)).floor() as u8,
    }
}
