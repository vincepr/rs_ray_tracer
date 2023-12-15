use std::f64::consts::PI;

use mathlib::{
    mathstructs::{matrix::Matrix, point::Point, vector::Vector},
    object::{plane::Plane, sphere::Sphere},
    visual::{
        camera::Camera,
        color::{self, Col},
        light::Light,
        material::Material,
        world::World,
    },
};

pub fn build_example(width: u32, height: u32) -> Vec<u8> {
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

    let camera = Camera::new(width as usize, height as usize, PI / 3.0).with_transform(
        Matrix::view_transform_new(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        ),
    );

    render(camera, world)
}

pub fn render(camera: Camera, world: World) -> Vec<u8> {
    let mut raw_pixels: Vec<u8> = Vec::new();

    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "generating for: w:{} x h:{}",
        camera.width, camera.height
    )));

    for y in 0..camera.height {
        // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&y.to_string()));
        for x in 0..camera.width {
            let ray = camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray);
            raw_pixels.push(color::base_255(color.r));
            raw_pixels.push(color::base_255(color.g));
            raw_pixels.push(color::base_255(color.b));
            raw_pixels.push(255); // transparency
        }
    }
    raw_pixels
}
