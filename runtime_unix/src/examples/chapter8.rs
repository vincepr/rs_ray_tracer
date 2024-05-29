// shadows

use std::f64::consts::PI;

use mathlib_renderer::{
    io::ppm::write_to_file,
    mathstructs::{matrix::Matrix, point::Point, vector::Vector},
    object::{plane::Plane, sphere::Sphere},
    visual::{camera::Camera, color::Col, light::Light, material::Material, world::World},
};

use crate::png_io::canvas_png_save;

pub fn build_example() {
    let mut base_mat = Material::new();
    base_mat.color(Col::new(1.0, 0.9, 0.9));
    base_mat.specular = 0.0;

    // a plane as the floor
    let mut floor = Plane::new();
    floor.material = base_mat.clone();

    // colored red sphere in the middle:
    let mut middle = Sphere::new();
    middle.transformation = Matrix::translation_new(-0.5, 1.0, 0.5);
    middle.material = base_mat.clone();
    middle.material.color(Col::new(0.1, 1.0, 0.5));
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // colored green halfsize sphere on the right:
    let mut right = Sphere::new();
    right.transformation =
        Matrix::translation_new(1.2, 0.5, 0.7) * Matrix::scaling_new(0.5, 0.5, 0.5);
    right.material = base_mat.clone();
    right.material.color(Col::new(0.5, 1.0, 0.1));
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // colored smallest sphere to the left:
    let mut left = Sphere::new();
    left.transformation =
        Matrix::translation_new(-1.5, 1.77, -0.75) * Matrix::scaling_new(0.33, 0.33, 0.33);
    left.material = base_mat;
    left.material.color(Col::new(1.0, 0.8, 0.1));
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.objects.push(floor);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(left);

    world.lights[0] =
        Light::new_point_light(Point::new(-10.0, 10.0, -10.0), Col::new(1.0, 1.0, 1.0));

    let camera = Camera::new(1000, 500, PI / 3.0).with_transform(Matrix::view_transform_new(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    ));

    let canvas = crate::parallel::render_parallel(camera, world);
    write_to_file("./out.ppm", canvas.canvas_to_ppm());
    canvas_png_save(&canvas, "./out.png");
}
