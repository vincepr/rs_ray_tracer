use crate::{
    cmp::EPSILON,
    mathstructs::{point::Point, vector::Vector},
    object::Object,
};

use super::{intersects::Intersect, Ray};

/// set of precomputed values that will get used a lot
#[derive(Debug, Clone)]
pub struct Computations {
    pub t: f64,
    pub object: Object,
    /// the point of the intersection
    pub point: Point,
    /// to avoid acne-rounding errors we need to add a slightly offset-point = Point * EPSILON
    pub over_point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    /// used for reflections. create a plane and positions a ray at 45 deg
    pub reflective_v: Vector,
    /// indicates if the hit occurs inside the object. (In that case the normal will be inverted)
    pub inside: bool,
}

impl Computations {
    /// precomputes the point in world space where the intersection occurred, and information relating to it
    /// - eye vector pointing back toward the camera and a the normal vector
    pub fn prepare(intersection: &Intersect, ray: &Ray) -> Self {
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
        let reflective_v = Vector::reflect(&ray.direction, &normal_v);

        Self {
            t: intersection.t,
            object: intersection.object.clone(),
            point,
            over_point,
            eye_v,
            normal_v,
            reflective_v,
            inside: hit_is_inside_object,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mathstructs::matrix::Matrix,
        object::{sphere::Sphere, plane::Plane},
        ray::{intersects::Intersect, Ray},
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
}
