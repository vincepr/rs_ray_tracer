use crate::{
    mathstructs::{point::Point, vector::Vector},
    objects::object::Object,
};

use super::{intersects::Intersect, Ray};

/// set of precomputed values that will get used a lot
#[derive(Debug, Clone)]
pub struct Computations {
    pub t: f32,
    pub object: Object,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
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

        Self {
            t: intersection.t,
            object: intersection.object.clone(),
            point: point,
            eye_v: eye_v,
            normal_v: normal_v,
            inside: hit_is_inside_object,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        objects::sphere::Sphere,
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
}
