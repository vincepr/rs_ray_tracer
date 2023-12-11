use crate::mathstructs::point::Point;

use super::color::Col;

/// Pattern - texture over the bodies
#[derive(Debug, Clone)]
pub enum Pattn {
    Single(Col),
    Stripe(Col, Col),
}
impl Pattn {
    pub fn single(a: Col) -> Self {
        Self::Single(a)
    }
    pub fn stripe(a: Col, b: Col) -> Self {
        Self::Stripe(a, b)
    }
    pub fn at(&self, point: &Point) -> &Col {
        match self {
            Pattn::Stripe(a, b) => stripe_at(point, a, b),
            Pattn::Single(a) => a,
        }
    }
}

impl PartialEq for Pattn {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(l0), Self::Single(r0)) => l0 == r0,
            (Self::Stripe(l0, l1), Self::Stripe(r0, r1)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}

fn stripe_at<'a>(point: &Point, a: &'a Col, b: &'a Col) -> &'a Col {
    if point.x.floor() % 2.0 == 0.0 {
        return a;
    }
    b
}

#[cfg(test)]
mod tests {
    use crate::{
        mathstructs::vector::Vector,
        visual::{
            color::{Col, BLACK, WHITE},
            light::Light,
            material::Material,
        },
    };

    use super::*;
    // setting up world/defaults
    #[test]
    fn static_colors() {
        assert_eq!(BLACK, Col::new(0.0, 0.0, 0.0));
        assert_eq!(WHITE, Col::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn creating_a_stripe_patter() {
        let pattern = Pattn::stripe(WHITE, BLACK);
        // constant y:
        assert_eq!(*pattern.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(*pattern.at(&Point::inew(0, 1, 0)), WHITE);
        assert_eq!(*pattern.at(&Point::inew(0, 2, 0)), WHITE);
        // constant z:
        assert_eq!(*pattern.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(*pattern.at(&Point::inew(0, 0, 1)), WHITE);
        assert_eq!(*pattern.at(&Point::inew(0, 0, 2)), WHITE);
        // alternating in x:
        assert_eq!(*pattern.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(*pattern.at(&Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(*pattern.at(&Point::new(1.0, 0.0, 0.0)), BLACK);

        assert_eq!(*pattern.at(&Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(*pattern.at(&Point::new(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(*pattern.at(&Point::new(-1.1, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let mut material = Material::new();
        material.pattern = Pattn::stripe(WHITE, BLACK);
        material.ambient = 1.0;
        material.diffuse = 0.0;
        material.specular = 0.0;

        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Point::inew(0, 0, -10), Col::new(1.0, 1.0, 1.0));
        let c1 = Light::lighting(
            &material,
            &light,
            &Point::new(0.9, 0.0, 0.0),
            &eye_v,
            &normal_v,
            false,
        );
        let c2 = Light::lighting(
            &material,
            &light,
            &Point::new(1.1, 0.0, 0.0),
            &eye_v,
            &normal_v,
            false,
        );
        assert_eq!(c1, WHITE);
        assert_eq!(c2, BLACK);
    }
}
