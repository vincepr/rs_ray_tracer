// using rayon for multi threading the process

use std::time::Instant;

use mathlib::visual::{camera::Camera, canvas::Canvas, world::World};
use rayon::prelude::*;
pub fn render_parallel(camera: &Camera, world: &World) -> Canvas {
    let now = Instant::now();
    let mut canvas = Canvas::new(camera.width, camera.height);
    canvas.arr.par_iter_mut().enumerate().for_each(|(y, row)| {
        row.par_iter_mut().enumerate().for_each(|(x, col)| {
            let ray = camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray);
            *col = color;
        });
    });
    println!("total render took: {} seconds.", now.elapsed().as_secs());
    canvas
}
