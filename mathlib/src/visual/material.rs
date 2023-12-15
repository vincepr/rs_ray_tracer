use crate::{cmp::ApproxEq, visual::color::Col};

use super::{color::WHITE, patterns::Pattern};

/// Phong Reflection Model uses these values to express lighting
#[derive(Debug, Clone)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Pattern,
}

impl Material {
    pub fn new() -> Self {
        Self::default()
    }

    /// 'setter' for a single color
    pub fn color(&mut self, color: Col) {
        self.pattern = Pattern::single(color);
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: Pattern::single(WHITE),
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
            && self.ambient.apx_eq(&other.ambient)
            && self.diffuse.apx_eq(&other.diffuse)
            && self.specular.apx_eq(&other.specular)
            && self.shininess.apx_eq(&other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use crate::object::sphere::Sphere;

    use super::*;

    #[test]
    fn ctor_material_values() {
        let m = Material::new();
        assert_eq!(m.pattern, Pattern::single(Col::new(1.0, 1.0, 1.0)));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut new_mat = Material::new();
        new_mat.ambient = 11.1;
        s.material = new_mat.clone();
        assert_eq!(s.material, new_mat);
    }
}
