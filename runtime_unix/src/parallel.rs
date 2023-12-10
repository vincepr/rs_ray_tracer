// using rayon for multi threading the process

use indicatif::{ProgressBar, ProgressStyle};
use mathlib::visual::{camera::Camera, canvas::Canvas, world::World};
use rayon::prelude::*;
pub fn render_parallel(camera: Camera, world: World) -> Canvas {
    let bar = ProgressBar::new((camera.height) as u64);
    bar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] [{eta} left] {wide_bar:.green/white} {percent}% [speed-in-rows: {bytes_per_sec}] [total-rows: {pos} rows] ").unwrap());

    let mut canvas = Canvas::new(camera.width, camera.height);
    canvas.arr.par_iter_mut().enumerate().for_each(|(y, row)| {
        row.par_iter_mut().enumerate().for_each(|(x, col)| {
            let ray = camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray);
            *col = color;
        });
        bar.inc(1);
    });
    bar.finish();
    canvas
}
