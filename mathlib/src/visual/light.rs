use crate::{
    mathstructs::{point::Point, vector::Vector},
    object::Object,
};

use super::{
    color::{Col, BLACK},
    material::Material,
    patterns::Texture,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Light {
    pub position: Point,
    pub intensity: Col,
}

impl Light {
    pub fn new_point_light(position: Point, intensity: Col) -> Self {
        Light {
            position,
            intensity,
        }
    }

    /// keeping this arround till i refactor the unit tests that dont use object for the lighting testing
    pub fn lighting(
        material: &Material,
        object: &Object,
        light: &Light,
        point: &Point,
        eye_v: &Vector,
        normal_v: &Vector,
        in_shadow: bool,
    ) -> Col {
        let material_color = match material.pattern.texture {
            Texture::Single(col) => col,
            _ => *material.pattern.at_with_obj(object, point),
        };
        Self::lighting_calculations(
            material,
            light,
            point,
            eye_v,
            normal_v,
            in_shadow,
            material_color,
        )
    }

    /// keeping this arround because rewriting unit tests would suck
    #[allow(dead_code)]
    fn lighting_without_obj(
        material: &Material,
        light: &Light,
        point: &Point,
        eye_v: &Vector,
        normal_v: &Vector,
        in_shadow: bool,
    ) -> Col {
        let material_color = match material.pattern.texture {
            Texture::Single(col) => col,
            _ => *material.pattern.texture.at(point),
        };
        Self::lighting_calculations(
            material,
            light,
            point,
            eye_v,
            normal_v,
            in_shadow,
            material_color,
        )
    }

    /// phong-reflection-model combines material and light source to shading
    fn lighting_calculations(
        material: &Material,
        light: &Light,
        point: &Point,
        eye_v: &Vector,
        normal_v: &Vector,
        in_shadow: bool,
        material_color: Col,
    ) -> Col {
        // combine the surface color with the lights's color/intensity
        let effective_col = material_color * light.intensity;
        // find the direction to the light source
        let light_v = (light.position - *point).normalize();
        // compute the ambient contribution
        let ambient = effective_col * material.ambient;

        // when in shadow we ignore diffure & specular -> only ambient lighting left:
        if in_shadow {
            return ambient;
        }

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface
        let diffuse;
        let specular;

        let light_dot_normal = light_v.dot(normal_v);
        if light_dot_normal < 0.0 {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            // compute the diffuse contribution
            diffuse = effective_col * material.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means
            // the light reflects away from the eye.
            let reflect_v = Vector::reflect(&(-light_v), normal_v);
            let reflect_dot_eye = reflect_v.dot(eye_v);

            if reflect_dot_eye <= 0.0 {
                specular = BLACK;
            } else {
                // compute the specular contribution
                let factor = f64::powf(reflect_dot_eye, material.shininess);
                specular = light.intensity * material.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

impl Default for Light {
    /// A default light configuration used for testing. at Point (-10, 10, -10)
    fn default() -> Self {
        Light::new_point_light(Point::inew(-10, 10, -10), Col::new(1.0, 1.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mathstructs::vector::Vector,
        visual::{color::WHITE, patterns::Pattern},
    };

    use super::*;

    #[test]
    fn test_point_light() {
        let intensity = Col::new(1.0, 1.0, 1.0);
        let position = Point::inew(0, 0, 0);
        let light = Light::new_point_light(position, intensity);
        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }

    fn setup_mat_pos() -> (Material, Point) {
        let m = Material::new();
        let position = Point::inew(0, 0, 0);
        (m, position)
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface() {
        let (m, position) = setup_mat_pos();
        let v_eye = Vector::inew(0, 0, -1);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 0, -10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting_without_obj(&m, &light, &position, &v_eye, &v_normal, false);
        assert_eq!(res, Col::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface_eye_offset_45deg() {
        let (m, position) = setup_mat_pos();
        let sq = 2.0_f64.sqrt() / 2.0;
        let v_eye = Vector::new(0.0, sq, -sq);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 0, -10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting_without_obj(&m, &light, &position, &v_eye, &v_normal, false);
        assert_eq!(res, Col::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45deg() {
        let (m, position) = setup_mat_pos();
        let v_eye = Vector::inew(0, 0, -1);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 10, -10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting_without_obj(&m, &light, &position, &v_eye, &v_normal, false);
        assert_eq!(res, Col::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn light_with_eye_in_the_path_of_reflection_vector() {
        let (m, position) = setup_mat_pos();
        let sq = 2.0_f64.sqrt() / 2.0;
        let v_eye = Vector::new(0.0, -sq, -sq);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 10, -10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting_without_obj(&m, &light, &position, &v_eye, &v_normal, false);
        assert_eq!(res, Col::new(1.63639, 1.63639, 1.63639));
    }

    #[test]
    fn light_with_light_behind_the_surface() {
        let (m, position) = setup_mat_pos();
        let v_eye = Vector::inew(0, 0, -1);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 0, 10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting_without_obj(&m, &light, &position, &v_eye, &v_normal, false);
        assert_eq!(res, Col::new(0.1, 0.1, 0.1));
    }

    // shadows
    #[test]
    fn lighting_with_surface_in_shadow() {
        let (m, position) = setup_mat_pos();
        let eye_v = Vector::inew(0, 0, -1);
        let normal_v = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 0, -10), Col::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let res = Light::lighting_without_obj(&m, &light, &position, &eye_v, &normal_v, in_shadow);
        assert_eq!(res, Col::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let mut material = Material::new();
        material.pattern = Pattern::stripe(WHITE, BLACK);
        material.ambient = 1.0;
        material.diffuse = 0.0;
        material.specular = 0.0;

        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Point::inew(0, 0, -10), Col::new(1.0, 1.0, 1.0));
        let c1 = Light::lighting_without_obj(
            &material,
            &light,
            &Point::new(0.9, 0.0, 0.0),
            &eye_v,
            &normal_v,
            false,
        );
        let c2 = Light::lighting_without_obj(
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
