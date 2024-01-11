use crate::{
    cmp::EPSILON,
    mathstructs::{point::Point, vector::Vector},
    object::Object,
};

use super::{
    intersects::{Intersect, VecIntersections},
    Ray,
};

/// set of precomputed values that will get used a lot
#[derive(Debug, Clone)]
pub struct Computations {
    pub t: f64,
    pub object: Object,
    /// the point of the intersection
    pub point: Point,
    /// to avoid acne-rounding errors we need to add a slightly offset-point = Point * EPSILON
    pub over_point: Point,
    /// to avoid acne-rounding errors, lies just beneath the intersected surface
    pub under_point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    /// used for reflections. create a plane and positions a ray at 45 deg
    pub reflective_v: Vector,
    /// indicates if the hit occurs inside the object. (In that case the normal will be inverted)
    pub inside: bool,
    /// refractive index of element to one side of the current element
    pub n1: f64,
    /// refractive index of element to the other side of the current element
    pub n2: f64,
}

impl Computations {
    /// precomputes the point in world space where the intersection occurred, and information relating to it
    /// - eye vector pointing back toward the camera and a the normal vector
    pub fn prepare(intersection: &Intersect, ray: &Ray) -> Self {
        let (point, eye_v, normal_v, hit_is_inside_object, over_point, under_point, reflective_v) =
            Self::calculations(ray, intersection);

        Self {
            t: intersection.t,
            object: intersection.object.clone(),
            point,
            over_point,
            under_point,
            eye_v,
            normal_v,
            reflective_v,
            inside: hit_is_inside_object,
            n1: 1.0,
            n2: 1.0,
        }
    }

    fn prepare_with_n1_n2(intersection: &Intersect, ray: &Ray, n1: f64, n2: f64) -> Self {
        let (point, eye_v, normal_v, hit_is_inside_object, over_point, under_point, reflective_v) =
            Self::calculations(ray, intersection);

        Self {
            t: intersection.t,
            object: intersection.object.clone(),
            point,
            over_point,
            under_point,
            eye_v,
            normal_v,
            reflective_v,
            inside: hit_is_inside_object,
            n1,
            n2,
        }
    }

    /// just wrapping the old prepare() with a new parameter. To add transparency and refraction calculations
    // with default parameters we could do this a bit more elegant probably
    // all we do here is set n1 and n2
    pub fn prepare_computations(
        intersection: &Intersect,
        ray: &Ray,
        xs: &VecIntersections,
    ) -> Self {
        let mut n1: f64 = 0.;
        let mut n2: f64 = 0.;
        let mut containers: Vec<&Object> = vec![];

        for i in xs.iter() {
            if i == intersection {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().material.refractive_index;
                }
            }

            if let Some(index) = containers.iter().position(|x| *x == i.object) {
                containers.remove(index);
            } else {
                containers.push(i.object);
            }

            if i == intersection {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material.refractive_index;
                }
                break;
            }
        }
        Self::prepare_with_n1_n2(intersection, ray, n1, n2)
    }

    fn calculations(
        ray: &Ray,
        intersection: &Intersect<'_>,
    ) -> (Point, Vector, Vector, bool, Point, Point, Vector) {
        let point = ray.position(intersection.t);
        let eye_v = -ray.direction;
        let mut normal_v = intersection.object.normal_at(&point);

        let hit_is_inside_object = if normal_v.dot(&eye_v) < 0.0 {
            normal_v = -normal_v;
            true
        } else {
            false
        };

        let over_point = point + normal_v * EPSILON;
        let under_point = point - normal_v * EPSILON;
        let reflective_v = Vector::reflect(&ray.direction, &normal_v);
        (
            point,
            eye_v,
            normal_v,
            hit_is_inside_object,
            over_point,
            under_point,
            reflective_v,
        )
    }

    /// approximation for fresnel-effect. Like how Water close by is seetrough, but far away reflecs more.
    /// returns 'reflectance': from 0 all light is refracted(internally). to 1 all light is reflected. 
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye_v.dot(&self.normal_v);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n * n * (1. - cos * cos);
            if sin2_t > 1. {
                return 1.;
            }
            // compute cosine of theta_t using trig identity
            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + ( 1. - r0) * ( 1. - cos).powi(5)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mathstructs::matrix::Matrix,
        object::{plane::Plane, sphere::Sphere},
        ray::{
            intersects::{Intersect, VecIntersections},
            Ray,
        }, cmp::ApproxEq,
    };

    use super::*;

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersect::new(4.0, &shape);
        let comps = Computations::prepare(&i, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, *i.object);
        assert_eq!(comps.point, Point::inew(0, 0, -1));
        assert_eq!(comps.eye_v, Vector::inew(0, 0, -1));
        assert_eq!(comps.normal_v, Vector::inew(0, 0, -1));
    }

    #[test]
    fn hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersect::new(4.0, &shape);
        let comps = Computations::prepare(&i, &r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::inew(0, 0, 0), Vector::inew(0, 0, 1));
        let shape = Sphere::new();
        let i = Intersect::new(1.0, &shape);
        let comps = Computations::prepare(&i, &r);
        assert_eq!(comps.point, Point::inew(0, 0, 1));
        assert_eq!(comps.eye_v, Vector::inew(0, 0, -1));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normal_v, Vector::inew(0, 0, -1)); // this is the inverted normal
    }

    #[test]
    fn the_hit_should_offset_overpoint_by_epsilon() {
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let mut shape = Sphere::new();
        shape.transformation = Matrix::translation_new(0.0, 0.0, 1.0);
        let i = Intersect::new(5.0, &shape);
        let comps = Computations::prepare(&i, &r);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::new();
        let sq = 2.0_f64.sqrt() / 2.0;
        let ray = Ray::new(Point::inew(0, 1, -1), Vector::new(0.0, -sq, sq));
        let i = Intersect::new(2.0_f64, &shape);
        let comps = Computations::prepare(&i, &ray);
        assert_eq!(comps.reflective_v, Vector::new(0.0, sq, sq));
    }

    fn assert_n1_n2(n1: f64, n2: f64, comp: &Computations, iteration: usize) {
        assert_eq!(
            n1, comp.n1,
            "{n1}!={} at iteration:{iteration} -> failed at comp: {:#?}",
            comp.n1, comp
        );
        assert_eq!(
            n2, comp.n2,
            "{n2}!={} -at iteration:{iteration} -> failed at comp: {:#?}",
            comp.n2, comp
        );
    }

    #[test]
    fn finding_ne_and_n2_at_various_intersections() {
        let a = Sphere::new_glass_sphere()
            .with_transform(Matrix::scaling_new(2., 2., 2.))
            .with_refrative_index(1.5);
        let b = Sphere::new_glass_sphere()
            .with_transform(Matrix::translation_new(0.0, 0.0, -0.25))
            .with_refrative_index(2.0);
        let c = Sphere::new_glass_sphere()
            .with_transform(Matrix::translation_new(0., 0., 0.25))
            .with_refrative_index(2.5);
        let ray = Ray::new(Point::inew(0, 0, -4), Vector::inew(0, 0, 1));
        let mut xs = VecIntersections::new();
        xs.intersect_add(&ray, &a);
        xs.intersect_add(&ray, &b);
        xs.intersect_add(&ray, &c);
        for (i, x) in xs.iter().enumerate() {
            let comp = Computations::prepare_computations(x, &ray, &xs);
            match i {
                0 => assert_n1_n2(1.0, 1.5, &comp, i),
                1 => assert_n1_n2(1.5, 2.0, &comp, i),
                2 => assert_n1_n2(2.0, 2.5, &comp, i),
                3 => assert_n1_n2(2.5, 2.5, &comp, i), // overlap with same on both sides
                4 => assert_n1_n2(2.5, 1.5, &comp, i),
                5 => assert_n1_n2(1.5, 1.0, &comp, i),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let shape = Sphere::new_glass_sphere().with_transform(Matrix::translation_new(0., 0., 1.));
        let xs = VecIntersections::new();
        let i = Intersect {
            t: 5.,
            object: &shape,
        };
        let comps = Computations::prepare_computations(&i, &ray, &xs);
        assert!(comps.under_point.z > EPSILON / 2.);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_aprox_under_total_internal_reflection() {
        let shape = Sphere::new_glass_sphere();
        let sq = 2.0_f64.sqrt() /2.;
        let ray = Ray::new(Point::new(0., 0., sq), Vector::new(0., 1., 0.));
        let xs = VecIntersections {
            0: vec![ Intersect::new(-sq, &shape), Intersect::new(sq, &shape),]
        };
        let i = Intersect::new(sq, &shape);
        let comps = Computations::prepare_computations(&i, &ray, &xs);
        assert_eq!(comps.schlick(), 1.);
    }

    #[test]
    fn schlick_aprox_with_perpendicular_viewing_angle() {
        let shape = Sphere::new_glass_sphere();
        let ray = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 1., 0.));
        let xs = VecIntersections {0: vec![ Intersect::new(-1., &shape), Intersect::new(1., &shape),]};
        let i = Intersect::new(1., &shape);
        let comps = Computations::prepare_computations(&i, &ray, &xs);
        assert!(comps.schlick().apx_eq(&0.04));
    }

    #[test]
    fn schlick_aprox_with_small_angle_and_n2_gt_n1() {
        let shape = Sphere::new_glass_sphere();
        let ray = Ray::new(Point::new(0., 0.99, -2.), Vector::new(0., 0., 1.));
        let xs = VecIntersections {0: vec![ Intersect::new(1.8589, &shape),]};
        let i = Intersect::new(1.8589, &shape);
        let comps = Computations::prepare_computations(&i, &ray, &xs);
        assert!(comps.schlick().apx_eq(&0.48873));
    }
}
