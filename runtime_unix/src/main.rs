use mathlib::{
    io::ppm::write_to_file,
    mathstructs::point::Point,
    objects::sphere::Sphere,
    ray::{intersects::VecIntersections, Ray},
    visual::{canvas::Canvas, color::Col, light::Light},
};

fn main() {
    manually_cast_rays_at_sphere_infront_canvas();
}

pub fn manually_cast_rays_at_sphere_infront_canvas() {
    let ray_origin = Point::inew(0, 0, -5);
    let wall_z: f32 = 10.0;
    let wall_size = 7.0;

    let canvas_size_px = 100;
    let mut canvas = Canvas::new(canvas_size_px, canvas_size_px);
    let pixel_size = wall_size / canvas_size_px as f32;
    let half = wall_size / 2.0;

    let mut shape = Sphere::new();
    shape.material.color = Col::new(1.0, 0.2, 1.0);

    // add a lighting source
    let light_position = Point::inew(-10, 10, -10);
    let light_color = Col::new(1.0, 1.0, 1.0);
    let light = Light::new_point_light(light_position, light_color);

    // shape.set_transform(Matrix::shearingi_new(1, 0, 0, 0, 0, 0) * Matrix::scaling_new(0.5, 1.0, 1.0),);

    // for each pixel:
    for (y, row) in canvas.arr.iter_mut().enumerate() {
        // compute current world y choordinate
        let world_y = half - pixel_size * y as f32;

        for (x, col) in row.iter_mut().enumerate() {
            // compute the world x choordinate
            let world_x = -half + pixel_size * x as f32;
            // Point the ray will hit:
            let position = Point::new(world_x, world_y, wall_z);

            // cast the ray:
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = VecIntersections::new().intersect_add(&ray, &shape);
            let pos_hit = xs.hit();
            if let Some(hit) = pos_hit {
                // apply light
                let point = ray.position(hit.t);
                let normal_v = hit.object.normal_at(&point);
                let eye_v = -ray.dir;
                let color_with_lighting = Light::lighting(
                    &hit.object.material, &light, &point, &eye_v, &normal_v);
                *col = color_with_lighting;
            }
        }
    }

    write_to_file("./out.ppm", canvas.canvas_to_ppm());
    canvas.canvas_png_save("./out.png");
}

pub fn manually_create_gradient_file() {
    let mut canvas = Canvas::new(100, 200);
    for (i, row) in canvas.arr.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = Col::new(i as f32 / 100.0, 0.8, j as f32 / 200.0);
        }
    }
    write_to_file("./out.ppm", canvas.canvas_to_ppm());
}
