use crate::{
    mathstructs::{matrix::Matrix, point::Point},
    object::Object, cmp::ApproxEq,
};

use super::color::Col;

#[derive(Debug, Clone, PartialEq)]
pub enum Texture {
    Single(Col),
    Stripe(Col, Col),
    Gradient(Col, Col),
    Ring(Col, Col),
    Checker(Col, Col),
}

impl Texture {
    pub fn at(&self, point: &Point) -> Col {
        match self {
            Texture::Stripe(a, b) => stripe_at(point, a, b),
            Texture::Single(a) => *a,
            Texture::Gradient(a, b) => gradient_at(point, a, b),
            Texture::Ring(a, b) => ring_at(point, a, b),
            Texture::Checker(a, b) => checker_at(point, a, b),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Pattern {
    pub texture: Texture,
    pub transform: Option<Matrix>,
}

impl Pattern {
    /// sets pattern transform. (default is None)
    pub fn pattern_transform(mut self, transform: Matrix) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn new_single(a: Col) -> Self {
        Self {
            texture: Texture::Single(a),
            transform: None,
        }
    }

    pub fn new_stripe(a: Col, b: Col) -> Self {
        Self {
            texture: Texture::Stripe(a, b),
            transform: None,
        }
    }

    pub fn new_gradient(a: Col, b: Col) -> Self {
        Self {
            texture: Texture::Gradient(a, b),
            transform: None,
        }
    }

    pub fn new_ring(a: Col, b: Col) -> Self {
        Self {
            texture: Texture::Ring(a, b),
            transform: None,
        }
    }

    pub fn new_checkers(a: Col, b: Col) -> Self {
        Self {
            texture: Texture::Checker(a, b),
            transform: None,
        }
    }

    pub fn at_with_obj(&self, object: &Object, world_point: &Point) -> Col {
        let object_point = object.transformation.inverse() * *world_point;
        match self.transform {
            Some(t) => self.texture.at(&(t.inverse() * object_point)),
            None => self.texture.at(&object_point),
        }
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.texture == other.texture && self.transform == other.transform
    }
}

fn stripe_at<'a>(point: &Point, a: &'a Col, b: &'a Col) -> Col {
    if point.x.floor() % 2.0 == 0.0 {
        return *a;
    }
    *b
}

fn gradient_at<'a>(point: &Point, a: &'a Col, b: &'a Col) -> Col {
    *a + (*b - *a) * (point.x - point.x.floor())
}

fn ring_at<'a>(point: &Point, a: &'a Col, b: &'a Col) -> Col {
    if ((point.x*point.x + point.z*point.z).sqrt()).floor() % 2.0 == 0.0 {
        return *a;
    }
    *b
}

fn checker_at<'a>(point: &Point, a: &'a Col, b: &'a Col) -> Col {
    let sum = f64::floor(point.x) + f64::floor(point.y) + f64::floor(point.z);
        if (sum % 2.0).apx_eq(&0.0) {
            return *a;
        }
        *b
        
    // if ((point.x.abs() + point.y.abs() + point.z.abs()) % 2.0) == 0.0 {
    //     return *a;
    // }
    // *b
}

#[cfg(test)]
mod tests {
    use crate::{
        mathstructs::matrix::Matrix,
        object::sphere::Sphere,
        visual::color::{BLACK, WHITE},
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
        let pattern = Pattern::new_stripe(WHITE, BLACK);
        // constant y:
        assert_eq!(pattern.texture.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::inew(0, 1, 0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::inew(0, 2, 0)), WHITE);
        // constant z:
        assert_eq!(pattern.texture.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::inew(0, 0, 1)), WHITE);
        assert_eq!(pattern.texture.at(&Point::inew(0, 0, 2)), WHITE);
        // alternating in x:
        assert_eq!(pattern.texture.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(1.0, 0.0, 0.0)), BLACK);

        assert_eq!(pattern.texture.at(&Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.texture.at(&Point::new(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.texture.at(&Point::new(-1.1, 0.0, 0.0)), WHITE);
    }

    // patterns transforming independently
    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Sphere::new();
        object.set_transform(Matrix::scaling_new(2.0, 2.0, 2.0));
        let pattern = Pattern::new_stripe(WHITE, BLACK);
        let res = pattern.at_with_obj(&object, &Point::new(1.5, 0.0, 0.0));
        assert_eq!(res, WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::new();
        let mut pattern = Pattern::new_stripe(WHITE, BLACK);
        pattern.transform = Some(Matrix::scaling_new(2.0, 2.0, 2.0)); 
        let res = pattern.at_with_obj(&object, &Point::new(1.5, 0.0, 0.0));
        assert_eq!(res, WHITE);
    }

    #[test]
    fn stripes_with_obj_and_pattern_transformation() {
        let mut object = Sphere::new();
        object.set_transform(Matrix::scaling_new(2.0, 2.0, 2.0));
        let mut pattern = Pattern::new_stripe(WHITE, BLACK);
        pattern.transform = Some(Matrix::translation_new(0.5, 0.0, 0.0)); 
        let res = pattern.at_with_obj(&object, &Point::new(2.5, 0.0, 0.0));
        assert_eq!(res, WHITE);
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::new_gradient(WHITE, BLACK);
        assert_eq!(pattern.texture.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(0.25, 0.0, 0.0)), Col::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.texture.at(&Point::new(0.5, 0.0, 0.0)), Col::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.texture.at(&Point::new(0.75, 0.0, 0.0)), Col::new(0.25, 0.25, 0.25));
    }

    #[test]
    fn a_ring_should_extend_in_both_x_z() {
        let pattern = Pattern::new_ring(WHITE, BLACK);
        assert_eq!(pattern.texture.at(&Point::inew(0, 0, 0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.texture.at(&Point::new(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(pattern.texture.at(&Point::new(0.708, 0.0, 0.708)), BLACK);
    }

    #[test]
    fn checkers_should_repeat(){
        let pattern = Pattern::new_checkers(WHITE, BLACK);
        // repeats x direction
        assert_eq!(pattern.texture.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(1.01, 0.0, 0.0)), WHITE);
        // repeats y direction
        assert_eq!(pattern.texture.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(0.0, 1.01, 0.0)), WHITE);
        // repeats z direction
        assert_eq!(pattern.texture.at(&Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(pattern.texture.at(&Point::new(0.0, 0.0, 1.01)), WHITE);
    }
}
