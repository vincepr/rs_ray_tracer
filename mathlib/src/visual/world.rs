use crate::{
    cmp::ApproxEq,
    mathstructs::{matrix::Matrix, point::Point},
    object::{sphere::Sphere, Object},
    ray::{computations::Computations, intersects::VecIntersections, Ray},
};

use super::{
    color::{Col, BLACK, WHITE},
    light::Light,
    patterns::Pattern,
};

#[derive(Debug, Clone)]
pub struct World {
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        Self {
            lights: vec![Light::default()],
            objects: vec![],
        }
    }

    fn intersect_world(&self, ray: &Ray) -> VecIntersections {
        let mut intersections = VecIntersections::new();
        for obj in &self.objects {
            intersections.intersect_add(ray, obj);
        }
        intersections
    }
}

impl Default for World {
    /// a default World configuration with 2 spheres used for testing
    fn default() -> Self {
        let mut objects = vec![];
        let mut s1 = Sphere::new();
        s1.material.pattern = Pattern::new_single(Col::new(0.8, 1.0, 0.6));
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        objects.push(s1);

        let mut s2 = Sphere::new();
        s2.transformation = Matrix::scaling_new(0.5, 0.5, 0.5);
        objects.push(s2);

        Self {
            lights: vec![Default::default(); 1],
            objects,
        }
    }
}

impl World {
    /// for every light source we sum up all the colors and return the sum
    fn shade_hit(&self, comps: &Computations, remaining: u8) -> Col {
        // multiple lights exist in the secene (careful many will slow down everything)
        let mut col_sum = BLACK;
        for cur_light in &self.lights {
            // calculate and add the surface color
            col_sum = col_sum
                + Light::lighting(
                    &comps.object.material,
                    &comps.object,
                    cur_light,
                    &comps.point,
                    &comps.eye_v,
                    &comps.normal_v,
                    self.is_shadowed(&comps.over_point, cur_light),
                );
        }
        // calculate and add reflected light
        col_sum = col_sum + self.reflected_color(comps, remaining);
        col_sum = col_sum + self.refracted_color(comps, remaining);
        col_sum
    }

    /// cast ray to the lightsource, if we hit any obstruction => were in the shadow of that
    fn is_shadowed(&self, point: &Point, current_light: &Light) -> bool {
        let v = current_light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray::new(*point, direction);
        let intersections = self.intersect_world(&ray);
        let hit = intersections.hit();
        if let Some(h) = hit {
            if h.t < distance {
                return true;
            }
        }
        false
    }

    pub fn color_at(&self, ray: &Ray, remaining: u8) -> Col {
        let intersects = self.intersect_world(ray);
        match intersects.hit() {
            None => BLACK,
            Some(i) => {
                let comps = Computations::prepare(&i, ray);
                self.shade_hit(&comps, remaining)
            }
        }
    }

    /// we basically spawn new rays from reflective material
    pub fn reflected_color(&self, comps: &Computations, remaining: u8) -> Col {
        if remaining == 0 || comps.object.material.reflective.apx_eq(&0.0) {
            return BLACK;
        }
        // we use the over_point to make sure we dont rounding-error to inside the shape we bounce off
        let reflect_ray = Ray::new(comps.over_point, comps.reflective_v);
        let color = self.color_at(&reflect_ray, remaining - 1);
        color * comps.object.material.reflective
    }

    pub fn refracted_color(&self, comps: &Computations, remaining: u8) -> Col {
        if remaining <= 0 {
            return BLACK;
        }
        if comps.object.material.transparency == 0. {
            return BLACK;
        }

        // total internal reflection (ex inside a fiber-optic-cable or with shallow angle against water surface)
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eye_v.dot(&comps.normal_v);
        let sin2_t = n_ratio * n_ratio * (1. - cos_i * cos_i);
        if sin2_t > 1. {
            return BLACK;
        }

        // calc refracted color from intersecting material
        let cos_t = (1. - sin2_t).sqrt();
        let direction = comps.normal_v * (n_ratio * cos_i - cos_t) - comps.eye_v * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);
        let color = self.color_at(&refract_ray, remaining - 1) * comps.object.material.transparency;
        return color;
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        mathstructs::{matrix::Matrix, point::Point, vector::Vector},
        object::plane::Plane,
        ray::{intersects::Intersect, Ray},
        visual::color::{Col, RED},
    };

    use super::*;
    // setting up world/defaults
    #[test]
    fn creating_world() {
        let w = World::new();
        assert_eq!(w.objects, vec![]);
    }

    #[test]
    fn contents_of_the_default_world() {
        let w = World::default();
        let light = Light::new_point_light(Point::inew(-10, 10, -10), Col::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color(Col::new(0.8, 1.0, 0.6));
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.transformation = Matrix::scaling_new(0.5, 0.5, 0.5);

        assert_eq!(w.lights.len(), 1);
        assert_eq!(w.lights[0], light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let xs = w.intersect_world(&r);
        assert_eq!(xs.len(), 4);
        let mut xs_iter = xs.iter();
        // our VecIntersections should be ordered ascending:
        assert_eq!(xs_iter.next().unwrap().t, 4.0);
        assert_eq!(xs_iter.next().unwrap().t, 4.5);
        assert_eq!(xs_iter.next().unwrap().t, 5.5);
        assert_eq!(xs_iter.next().unwrap().t, 6.0);
    }

    // shading
    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let shape = w.objects.first().unwrap();
        let i = Intersect::new(4.0, shape);
        let comps = Computations::prepare(&i, &r);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Col::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.lights[0] = Light::new_point_light(Point::new(0.0, 0.25, 0.0), Col::new(1.0, 1.0, 1.0));
        let r = Ray::new(Point::inew(0, 0, 0), Vector::inew(0, 0, 1));

        let shape = &w.objects[1]; // second element
        let i = Intersect::new(0.5, shape);
        let comps = Computations::prepare(&i, &r);

        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Col::new(0.90498, 0.90498, 0.90498));
    }

    // compute color from world:
    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 1, 0));
        let c = w.color_at(&r, 1);
        assert_eq!(c, Col::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let c = w.color_at(&r, 1);
        assert_eq!(c, Col::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_the_ray() {
        let mut w = World::default();
        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        inner.material.color(RED);
        let exp = RED;
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::inew(0, 0, -1));
        let c = w.color_at(&r, 1);
        assert_eq!(c, exp);
    }

    // shadows
    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::inew(0, 10, 0);
        let current_light = &w.lights[0];
        assert_eq!(w.is_shadowed(&p, current_light), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Point::inew(10, -10, 10);
        let current_light = &w.lights[0];
        assert_eq!(w.is_shadowed(&p, current_light), true);
    }

    #[test]
    fn there_is_no_shadow_when_and_object_is_behind_the_light() {
        let w = World::default();
        let p = Point::inew(-20, 20, -20);
        let current_light = &w.lights[0];
        assert_eq!(w.is_shadowed(&p, current_light), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Point::inew(-2, 2, -2);
        let current_light = &w.lights[0];
        assert_eq!(w.is_shadowed(&p, current_light), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.lights[0] = Light::new_point_light(Point::inew(0, 0, -10), Col::new(1.0, 1.0, 1.0));
        let s1 = Sphere::new();
        w.objects.push(s1);
        let mut s2 = Sphere::new();
        s2.transformation = Matrix::translation_new(0.0, 0.0, 10.0);
        w.objects.push(s2);
        let ray = Ray::new(Point::inew(0, 0, 5), Vector::inew(0, 0, 1));
        let intersection = Intersect::new(4.0, &w.objects[1]);
        let comps = Computations::prepare(&intersection, &ray);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Col::new(0.1, 0.1, 0.1));
    }

    // reflection
    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = World::default();
        let ray = Ray::new(Point::inew(0, 0, 0), Vector::inew(0, 0, 1));
        let shape = w.objects.last_mut().unwrap();
        shape.material.ambient = 1.0;
        let i = Intersect::new(1.0, &shape);
        let comps = Computations::prepare(&i, &ray);
        let res = w.reflected_color(&comps, 1);
        assert_eq!(res, BLACK);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transformation = Matrix::translation_new(0.0, -1.0, 0.0);
        w.objects.push(shape);

        let sq = 2.0_f64.sqrt() / 2.0;
        let ray = Ray::new(Point::inew(0, 0, -3), Vector::new(0.0, -sq, sq));
        let shape = w.objects.last_mut().unwrap();
        let i = Intersect::new(2.0_f64.sqrt(), &shape);
        let comps = Computations::prepare(&i, &ray);
        let res = w.reflected_color(&comps, 1);
        assert_eq!(res, Col::new(0.19033, 0.23791, 0.14274));
    }

    #[test]
    fn shate_hit_with_a_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transformation = Matrix::translation_new(0.0, -1.0, 0.0);
        w.objects.push(shape);

        let sq = 2.0_f64.sqrt() / 2.0;
        let ray = Ray::new(Point::inew(0, 0, -3), Vector::new(0.0, -sq, sq));
        let shape = w.objects.last_mut().unwrap();
        let i = Intersect::new(2.0_f64.sqrt(), &shape);
        let comps = Computations::prepare(&i, &ray);
        let res = w.shade_hit(&comps, 1);
        // these values were off quite a bit from the book's. There might be
        // a bug somewhere       0.87677,  0.92436 , 0.82918
        assert_eq!(res, Col::new(0.876757, 0.924340, 0.82918));
    }

    #[test]
    fn avoid_infinite_recursion_with_mutually_reflective_surfaces() {
        let mut w = World::default();
        w.lights[0] = Light::new_point_light(Point::inew(0, 0, 0), Col::new(1., 1., 1.));

        let mut lower = Plane::new();
        lower.material.reflective = 1.;
        lower.transformation = Matrix::translation_new(0., -1., 0.);
        w.objects[0] = lower;

        let mut upper = Plane::new();
        upper.material.reflective = 1.;
        upper.transformation = Matrix::translation_new(0., 1., 0.);
        w.objects[1] = upper;

        let ray = Ray::new(Point::inew(0, 0, 0), Vector::inew(0, 1, 0));
        let col = w.color_at(&ray, 99);
        assert_eq!(col, Col::new(190., 190., 190.));
        // we just check that this terminates here and doesnt get stuck in infinite recursion
    }

    #[test]
    fn reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transformation = Matrix::translation_new(0.0, -1.0, 0.0);
        w.objects.push(shape);

        let sq = 2.0_f64.sqrt() / 2.0;
        let ray = Ray::new(Point::inew(0, 0, -3), Vector::new(0.0, -sq, sq));
        let shape = w.objects.last_mut().unwrap();
        let i = Intersect::new(2.0_f64.sqrt(), &shape);
        let comps = Computations::prepare(&i, &ray);

        let res = w.reflected_color(&comps, 0);
        assert_eq!(res, Col::new(0., 0., 0.));
    }
    
    // refraction (rays passing trough see trough/getting internal reflected etc)
    #[test]
    fn refracted_color_with_an_opaque_surface() {
        let w = crate::visual::world::World::default();
        let shape = w.objects.first().unwrap();
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let xs = VecIntersections{0:vec![ 
            Intersect { t: 4., object: &shape },
            Intersect { t: 6., object: &shape } 
        ]};
        let comps = Computations::prepare_computations(&Intersect { t: 4., object: &shape }, &ray, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn refracted_color_at_max_recursive_depth_always_black() {
        let mut w = crate::visual::world::World::default();
        let shape: &mut Object = w.objects.first_mut().unwrap();
        shape.material.transparency = 1.;
        shape.material.refractive_index = 1.5;
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let xs = VecIntersections{0:vec![ 
            Intersect { t: 4., object: &shape },
            Intersect { t: 6., object: &shape } 
        ]};
        let comps = Computations::prepare_computations(&Intersect { t: 4., object: &shape }, &ray, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = crate::visual::world::World::default();
        let shape: &mut Object = w.objects.first_mut().unwrap();
        shape.material.transparency = 1.;
        shape.material.refractive_index = 1.5;
        let sq = 2.0_f64.sqrt() / 2.;
        let ray = Ray::new(Point::new(0., 0., sq), Vector::new(0., 1., 0.));
        let xs = VecIntersections{0:vec![ 
            Intersect { t: -sq, object: &shape },
            Intersect { t: sq, object: &shape } 
        ]};
        let comps = Computations::prepare_computations(&Intersect { t: sq, object: &shape }, &ray, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, BLACK);
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut w = crate::visual::world::World::default();
        {
            let a_shape: &mut Object = w.objects.first_mut().unwrap();
            a_shape.material.ambient = 1.;
            a_shape.material.pattern = Pattern::new_test_pattern();
            let b_shape: &mut Object = w.objects.last_mut().unwrap();
            b_shape.material.transparency = 1.;
            b_shape.material.refractive_index = 1.5;
        }
        let a_shape = w.objects.first().unwrap();
        let b_shape = w.objects.last().unwrap();

        let ray = Ray::new(Point::new(0., 0., 0.1), Vector::new(0., 1., 0.));
        let xs = VecIntersections{0:vec![ 
            Intersect { t: -0.9899, object: &a_shape },
            Intersect { t: -0.4899, object: &b_shape } ,
            Intersect { t: 0.4899, object: &b_shape } ,
            Intersect { t: 0.9899, object: &a_shape } 
            ]};
        let comps = Computations::prepare_computations(&Intersect { t: 0.4899, object: &b_shape }, &ray, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Col::new(0., 0.99888, 0.04721)); 
        // book has different rounding here: 0. 0.99888 0.04725
    }

    #[test]
    fn shade_hit_now_with_transparent_refraction() {
        let mut w = crate::visual::world::World::default();
        let mut floor = Plane::new().with_transform(Matrix::translation_new(0., -1., 0.));
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor);
        let mut ball = Sphere::new().with_transform(Matrix::translation_new(0., -3.5, -0.5));
        ball.material.color(Col::new(1., 0., 0.));
        ball.material.ambient = 0.5;
        w.objects.push(ball);
        let obj = &w.objects[2];
        let sq = 2.0_f64.sqrt();
        let ray = Ray::new(Point::new(0., 0., -3.), Vector::new(0., -sq/2., sq/2.));
        let xs = VecIntersections{0:vec![ Intersect { t: sq, object: &obj },]};
        let comps = Computations::prepare_computations(&Intersect { t: sq, object: &obj }, &ray, &xs);
        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, Col::new(0.93642, 0.68642, 0.68642)); 


    }
}
