use crate::{
    mathstructs::{point::Point, vector::Vector},
    objects::material::Material,
};

use super::color::Col;

pub struct Light {
    position: Point,
    intensity: Col,
}

impl Light {
    pub fn new_point_light(position: Point, intensity: Col) -> Self {
        Light {
            position,
            intensity,
        }
    }

    /// phong-reflection-model combines material and light source to shading
    pub fn lighting(
        material: &Material,
        light: &Light,
        point: &Point,
        eye_v: &Vector,
        normal_v: &Vector,
    ) -> Col {
        // combine the surface color with the lights's color/intensity
        let effective_col = material.color * light.intensity;
        // find the direction to the light source
        let light_v = (light.position - *point).normalize();
        // compute the ambient contribution
        let ambient = effective_col * material.ambient;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface
        let diffuse;
        let specular;

        let light_dot_normal = light_v.dot(normal_v);
        if light_dot_normal < 0.0 {
            diffuse = Col::new_black();
            specular = Col::new_black();
        } else {
            // compute the diffuse contribution
            diffuse = effective_col * material.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means
            // the light reflects away from the eye.
            let reflect_v = Vector::reflect(&(-light_v), normal_v);
            let reflect_dot_eye = reflect_v.dot(eye_v);

            if reflect_dot_eye <= 0.0 {
                specular = Col::new_black();
            } else {
                // compute the specular contribution
                let factor = f32::powf(reflect_dot_eye, material.shininess);
                specular = light.intensity * material.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::{mathstructs::vector::Vector, objects::material::Material};

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
        let res = Light::lighting(&m, &light, &position, &v_eye, &v_normal);
        assert_eq!(res, Col::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface_eye_offset_45deg() {
        let (m, position) = setup_mat_pos();
        let sq = 2.0_f32.sqrt() / 2.0;
        let v_eye = Vector::new(0.0, sq, -sq);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 0, -10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting(&m, &light, &position, &v_eye, &v_normal);
        assert_eq!(res, Col::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45deg() {
        let (m, position) = setup_mat_pos();
        let v_eye = Vector::inew(0, 0, -1);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 10, -10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting(&m, &light, &position, &v_eye, &v_normal);
        assert_eq!(res, Col::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn light_with_eye_in_the_path_of_reflection_vector() {
        let (m, position) = setup_mat_pos();
        let sq = 2.0_f32.sqrt() / 2.0;
        let v_eye = Vector::new(0.0, -sq, -sq);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 10, -10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting(&m, &light, &position, &v_eye, &v_normal);
        assert_eq!(res, Col::new(1.63638, 1.63638, 1.63638));
    }

    #[test]
    fn light_with_light_behind_the_surface() {
        let (m, position) = setup_mat_pos();
        let v_eye = Vector::inew(0, 0, -1);
        let v_normal = Vector::inew(0, 0, -1);
        let light = Light::new_point_light(Point::inew(0, 0, 10), Col::new(1.0, 1.0, 1.0));
        let res = Light::lighting(&m, &light, &position, &v_eye, &v_normal);
        assert_eq!(res, Col::new(0.1, 0.1, 0.1));
    }
}
