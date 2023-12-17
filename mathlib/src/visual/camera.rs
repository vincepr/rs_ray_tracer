use std::time::Instant;

use crate::{
    mathstructs::{matrix::Matrix, point::Point},
    ray::Ray,
};

use super::{canvas::Canvas, world::World};

pub struct Camera {
    /// width in pixels
    pub width: usize,
    /// heigth in pixels
    pub height: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    /// calculate the pixel size for a given camera
    pub fn new(width: usize, height: usize, fow: f64) -> Self {
        let half_view = (fow / 2.0).tan();
        let aspect = width as f64 / height as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / width as f64;
        Self {
            width,
            height,
            field_of_view: fow,
            transform: Matrix::new_identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn with_transform(mut self, t: Matrix) -> Self {
        self.transform = t;
        self
    }

    /// for each canvas pixel get the correspoinding world_choordinates and then the ray from it
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let offset_x = (px as f64 + 0.5) * self.pixel_size;
        let offset_y = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        // using the camera matrix transform the canvas point and the origin
        // and then compute the ray's directin vector;
        let inverse = self.transform.inverse();
        let pixel = inverse * Point::new(world_x, world_y, -1.0);
        let origin = inverse * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    /// for given camera and world we render out the pixels to a canvas
    pub fn render(&self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);
        canvas.arr.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, col)| {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, 1);
                *col = color;
            });
        });

        canvas
    }

    pub fn render_with_progress_procedural(&self, world: World) -> Canvas {
        let now = Instant::now();
        let mut canvas = Canvas::new(self.width, self.height);
        let mut nxt_percent = (1, self.height / 10, "::".to_string());
        for (y, row) in canvas.arr.iter_mut().enumerate() {
            if y == nxt_percent.1 {
                println!(
                    "{} {}0 % took: {}s",
                    nxt_percent.2,
                    nxt_percent.0,
                    now.elapsed().as_secs()
                );
                nxt_percent = (
                    nxt_percent.0 + 1,
                    (nxt_percent.0 + 1) * self.height / 10,
                    nxt_percent.2 + "::",
                );
            }
            for (x, col) in row.iter_mut().enumerate() {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, 1);
                *col = color;
            }
        }
        println!("total render took: {} seconds.", now.elapsed().as_secs());
        canvas
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        cmp::ApproxEq,
        mathstructs::{matrix::Matrix, point::Point, vector::Vector},
        visual::{color::Col, world::World},
    };

    use super::*;

    // translation
    #[test]
    fn constructing_a_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.width, 160);
        assert_eq!(c.height, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix::new_identity())
    }

    #[test]
    fn pixel_size_for_a_canvas() {
        //horizontal canvas
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(c.pixel_size.apx_eq(&0.01));
        //vertical canvas
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(c.pixel_size.apx_eq(&0.01));
    }

    #[test]
    fn constructing_ray_trough_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::inew(0, 0, 0));
        assert_eq!(r.direction, Vector::inew(0, 0, -1));
    }

    #[test]
    fn constructing_a_ray_trough_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Point::inew(0, 0, 0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Matrix::rotation_y_new(PI / 4.0) * Matrix::translation_new(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::inew(0, 2, -5));
        let sq = 2.0_f64.sqrt() / 2.0;
        assert_eq!(r.direction, Vector::new(sq, 0.0, -sq));
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let from = Point::inew(0, 0, -5);
        let to = Point::inew(0, 0, 0);
        let up = Vector::inew(0, 1, 0);
        let c =
            Camera::new(11, 11, PI / 2.0).with_transform(Matrix::view_transform_new(from, to, up));
        let image = c.render(w);
        assert_eq!(image[5][5], Col::new(0.38066, 0.47583, 0.2855));
    }
}
