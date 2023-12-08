use crate::{mathstructs::{matrix::Matrix, point::Point}, ray::Ray};

pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f32,
    pub transform: Matrix,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
}

impl Camera {
    /// calculate the pixel size for a given camera
    pub fn new(hsize: u32, vsize: u32, fow: f32) -> Self {
        let half_view = (fow/2.0).tan();
        let aspect = hsize as f32 / vsize as f32;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / hsize as f32;
        Self {
            hsize,
            vsize,
            field_of_view: fow,
            transform: Matrix::new_identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    /// for each canvas pixel get the correspoinding world_choordinates and then the ray from it
    pub fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        let px = px as f32;
        let py = py as f32;
        let offset_x = (px + 0.5) * self.pixel_size;
        let offset_y = (py + 0.5) * self.pixel_size;

        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        // using the camera matrix transform the canvas point and the origin
        // and then compute the ray's directin vector;
        let pixel = self.transform.inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::{mathstructs::{matrix::Matrix, vector::Vector, point::Point}, cmp::ApproxEq};

    use super::*;

    // translation
    #[test]
    fn constructing_a_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
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

    // #[test]
    // fn constructing_a_ray_when_camera_is_transformed() {
    //     let mut c = Camera::new(201, 101, PI / 2.0);
    //     //c.transform = Matrix::new_identity().rotate_y(PI/4.0).translate(0.0, -2.0, 5.0);
    //     c.transform = Matrix::rotation_y_new(PI/4.0) * Matrix::translation_new(0.0, -2.0, 5.0);
    //     Matrix::new_identity().rotate_y(PI/4.0).translate(0.0, -2.0, 5.0);
    //     let r = c.ray_for_pixel(100, 50);
    //     assert_eq!(r.origin, Point::inew(0, 2, -5));
    //     let sq = 2.0_f32.sqrt();
    //     assert_eq!(r.direction, Vector::new(sq, 0.0, -sq));
    // }
}
