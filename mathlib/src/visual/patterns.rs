use crate::{mathstructs::{point::Point, matrix::Matrix}, object::Object};

use super::color::Col;

#[derive(Debug, Clone, PartialEq)]
pub enum Texture {
    Single(Col),
    Stripe(Col, Col),
}

impl Texture {
    pub fn at(&self, point: &Point) -> &Col {
        match self {
            Texture::Stripe(a, b) => stripe_at(point, &a, &b),
            Texture::Single(a) => &a,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub texture: Texture,
    pub transform: Option<Matrix>,
} 

impl Pattern {
    pub fn single(a: Col) -> Self {
        Self{
            texture: Texture::Single(a),
            transform: None
        }
    }
    pub fn stripe(a: Col, b: Col) -> Self {
        Self{
            texture: Texture::Stripe(a, b),
            transform: None
        }
    }

    pub fn at_with_obj(&self, object: &Object, world_point: &Point) -> &Col {
        let object_point = object.transformation.inverse() * *world_point;
        match self.transform {
            Some(t) => self.texture.at(&(t.inverse() * object_point)),
            None =>  self.texture.at(&object_point)
        }

    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.texture == other.texture && self.transform == other.transform
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
        mathstructs::{vector::Vector, matrix::Matrix},
        visual::{
            color::{Col, BLACK, WHITE},
            light::Light,
            material::Material,
        }, object::sphere::Sphere,
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
        let pattern = Pattern::stripe(WHITE, BLACK);
        // constant y:
        assert_eq!(*pattern.texture.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(*pattern.texture.at(&Point::inew(0, 1, 0)), WHITE);
        assert_eq!(*pattern.texture.at(&Point::inew(0, 2, 0)), WHITE);
        // constant z:
        assert_eq!(*pattern.texture.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(*pattern.texture.at(&Point::inew(0, 0, 1)), WHITE);
        assert_eq!(*pattern.texture.at(&Point::inew(0, 0, 2)), WHITE);
        // alternating in x:
        assert_eq!(*pattern.texture.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(*pattern.texture.at(&Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(*pattern.texture.at(&Point::new(1.0, 0.0, 0.0)), BLACK);

        assert_eq!(*pattern.texture.at(&Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(*pattern.texture.at(&Point::new(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(*pattern.texture.at(&Point::new(-1.1, 0.0, 0.0)), WHITE);
    }



    // patterns transforming independently
    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Sphere::new();
        object.set_transform(Matrix::scaling_new(2.0, 2.0, 2.0));
        let pattern = Pattern::stripe(WHITE, BLACK);
    }
}
