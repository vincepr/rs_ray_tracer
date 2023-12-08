use crate::{
    mathstructs::matrix::Matrix,
    objects::{object::Object, sphere::Sphere},
    ray::{computations::Computations, intersects::VecIntersections, Ray},
};

use super::{color::Col, light::Light};

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
            intersections.intersect_add(ray, &obj);
        }
        intersections
    }
}

impl Default for World {
    /// a default World configuration with 2 spheres used for testing
    fn default() -> Self {
        let mut objects = vec![];
        let mut s1 = Sphere::new();
        s1.material.color = Col::new(0.8, 1.0, 0.6);
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
    fn shade_hit(&self, comps: &Computations) -> Col {
        if self.lights.len() == 1 {
            Light::lighting(
                &comps.object.material,
                &self.lights[0],
                &comps.point,
                &comps.eye_v,
                &comps.normal_v,
            )
        } else {
            // multiple lights exist in the secene (careful will slow down everything)
            let mut col_sum = Col::new_black();
            for cur_light in &self.lights {
                col_sum = col_sum
                    + Light::lighting(
                        &comps.object.material,
                        cur_light,
                        &comps.point,
                        &comps.eye_v,
                        &comps.normal_v,
                    );
            }
            return col_sum;
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Col {
        let intersects = self.intersect_world(ray);
        match intersects.hit() {
            None => return Col::new_black(),
            Some(i) => {
                let comps = Computations::prepare(&i, &ray);
                return self.shade_hit(&comps);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        mathstructs::{matrix::Matrix, point::Point, vector::Vector},
        objects::sphere::Sphere,
        ray::{intersects::Intersect, Ray},
        visual::color::Col,
    };

    use super::*;
    // setting up world/defaults
    #[test]
    fn creating_world() {
        let w = World::new();
        assert_eq!(w.objects, vec![]);
    }

    #[test]
    fn default_world() {
        let w = World::default();
        let light = Light::new_point_light(Point::inew(-10, 10, -10), Col::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Col::new(0.8, 1.0, 0.6);
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
        let c = w.shade_hit(&comps);
        assert_eq!(c, Col::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.lights[0] = Light::new_point_light(Point::new(0.0, 0.25, 0.0), Col::new(1.0, 1.0, 1.0));
        let r = Ray::new(Point::inew(0, 0, 0), Vector::inew(0, 0, 1));

        let shape = w.objects.last().unwrap(); // second element (but we only have 2 in default world)
        let i = Intersect::new(0.5, shape);
        let comps = Computations::prepare(&i, &r);

        let c = w.shade_hit(&comps);
        assert_eq!(c, Col::new(0.90498, 0.90498, 0.90498));
    }

    // compute color from world:
    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 1, 0));
        let c = w.color_at(&r);
        assert_eq!(c, Col::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let c = w.color_at(&r);
        assert_eq!(c, Col::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_the_ray() {
        let mut w = World::default();
        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        let exp = inner.material.color.clone();
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::inew(0, 0, -1));
        let c = w.color_at(&r);
        assert_eq!(c, exp);
    }
}
