use mathlib::{
    io::write_to_file,
    visual::{canvas::Canvas, color::Col},
};

fn main() {
    let mut canvas = Canvas::new(5, 3);

    for (i, row) in canvas.arr.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = Col::new(i as f32 / 30.0, 0.8, 0.6);
        }
    }

    write_to_file("./out.ppm", canvas.canvas_to_ppm());
}
