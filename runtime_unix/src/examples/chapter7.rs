// a scene with 6 spheres:

use std::f32::consts::PI;


use mathlib::{objects::{material::Material, sphere::Sphere}, visual::{color::Col, world::World, light::Light, camera::Camera}, mathstructs::{matrix::Matrix, point::Point, vector::Vector}, io::ppm::write_to_file};

use crate::png_io::canvas_png_save;

pub fn build_example() {
    let mut base_mat = Material::new();
    base_mat.color = Col::new(1.0, 0.9, 0.9);
    base_mat.specular = 0.0;

    // a flattened sphere has to make do for the floor
    let mut floor = Sphere::new();
    floor.transformation = Matrix::scaling_new(10.0, 0.01, 10.0);
    floor.material = base_mat.clone();

    let mut left_wall = Sphere::new();
    left_wall.transformation = Matrix::translation_new(0.0, 0.0, 5.0)
        * Matrix::rotation_y_new(-PI / 4.0)
        * Matrix::rotation_x_new(PI / 2.0)
        * Matrix::scaling_new(10.0, 0.01, 10.0);
    left_wall.material = base_mat.clone();

    let mut right_wall = Sphere::new();
    right_wall.transformation = Matrix::translation_new(0.0, 0.0, 5.0)
        * Matrix::rotation_y_new(PI / 4.0)
        * Matrix::rotation_x_new(PI / 2.0)
        * Matrix::scaling_new(10.0, 0.01, 10.0);
    right_wall.material = base_mat.clone();

    // colored red sphere in the middle:
    let mut middle = Sphere::new();
    middle.transformation = Matrix::translation_new(-0.5, 1.0, 0.5);
    middle.material = base_mat.clone();
    middle.material.color = Col::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // colored green halfsize sphere on the right:
    let mut right = Sphere::new();
    right.transformation =
        Matrix::translation_new(1.5, 0.5, -0.5) * Matrix::scaling_new(0.5, 0.5, 0.5);
    right.material = base_mat.clone();
    right.material.color = Col::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // colored smallest sphere to the left:
    let mut left = Sphere::new();
    left.transformation =
        Matrix::translation_new(-1.5, 0.33, -0.75) * Matrix::scaling_new(0.33, 0.33, 0.33);
    left.material = base_mat;
    left.material.color = Col::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.objects.push(floor);
    world.objects.push(left_wall);
    world.objects.push(right_wall);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(left);

    world.lights[0] = Light::new_point_light(Point::new(-10.0, 10.0, -10.0), Col::new(1.0, 1.0, 1.0));

    let camera = Camera::new(2000, 1000, PI / 3.0).with_transform(Matrix::view_transform_new(
    // let camera = Camera::new(500, 250, PI / 3.0).with_transform(Matrix::view_transform_new(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render_with_progress(world);
    write_to_file("./out.ppm", canvas.canvas_to_ppm());
    canvas_png_save(&canvas, "./out.png");
}

