use std::cmp::Ordering;

use super::{sphere::Sphere, Ray};

/// the interface we use for all objects that our rays can hit/intersect with
pub trait IntersectsRay {
    fn intersect(&self, ray: &Ray) -> Option<(f32, f32)>;
}

/// keeps reference to intersections our rays we cast find
#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    t: f32,
    object: &'a Sphere,
}

impl<'a> Eq for Intersection<'a> {} // cant use derive macro this will just use PartialEq for Eq

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Self {
        Self { t, object }
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if !self.t.is_finite() {
            return Ordering::Greater;
        } else if !other.t.is_finite() {
            return Ordering::Less;
        } else if self.t > other.t {
            return Ordering::Greater;
        } else if self.t < other.t {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

/// collection of Intersections
#[derive(Debug)]
pub struct VecIntersections<'a>(Vec<Intersection<'a>>);
impl<'a> VecIntersections<'a> {
    pub fn new() -> Self {
        Self(Vec::with_capacity(32))
    }

    pub fn push(&mut self, add: Intersection<'a>) {
        self.0.push(add);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn hit(&self) -> Option<Intersection<'_>> {
        Some(
            self.0
                .iter()
                .filter(|inter| inter.t.is_sign_positive())
                .min()?
                .clone(),
        ) // or maybe just return by reference? might be
    }

    // unclean because of the mutability (unless we hit often then this might be faster)
    /// sorts the vec permanently. might be optimal if hit is called often.
    pub fn hit_permanent(&mut self) -> Option<Intersection<'_>> {
        self.0.sort();
        Some(
            self.0
                .iter()
                .find(|inter| inter.t.is_sign_positive())?
                .clone(),
        )
    }

    // pub fn all_positive(&self) -> impl Iterator<Item = &Intersection> {
    //     self.0.iter()
    //         .filter(|f| f.t.is_sign_positive())
    // }

    /// adds a possible intersection to the collection
    pub fn intersections(&mut self, intersect: Option<(f32, f32)>, obj: &'a Sphere) {
        match intersect {
            None => {}
            Some((t1, t2)) => {
                self.0.push(Intersection::new(t1, obj));
                self.0.push(Intersection::new(t2, obj)); // TODO: we could not remove double in case of tangent?
            }
        }
    }
}

impl<'a> Default for VecIntersections<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let sphere = Sphere::new();
        let i = Intersection::new(3.5, &sphere);
        assert_eq!(i.t, 3.5);
        // we compare if both point to the same space in memory by casting as raw pointers and comparing the memory-adress
        assert_eq!(i.object as *const _, &sphere as *const _);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let i3 = Intersection::new(3.0, &s);
        let mut intersections = VecIntersections::new();
        intersections.push(i1);
        intersections.push(i2);
        intersections.push(i3);
        assert_eq!(intersections.0.len(), 3);
        assert_eq!(intersections.0[0].t, 1.0);
        assert_eq!(intersections.0[1].t, 2.0);
        assert_eq!(intersections.0[2].t, 3.0);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut ins = VecIntersections::new();
        ins.push(i1.clone());
        ins.push(i2);
        let res = ins.hit();
        assert_eq!(res, Some(i1));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut ins = VecIntersections::new();
        ins.push(i1);
        ins.push(i2.clone());
        let res = ins.hit();
        assert_eq!(res, Some(i2));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-3.0, &s);
        let i2 = Intersection::new(-5.0, &s);
        let mut ins = VecIntersections::new();
        ins.push(i1);
        ins.push(i2);
        let res = ins.hit();
        assert_eq!(res, None);
    }

    #[test]
    fn hit_is_always_lowest_non_negative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let i5 = Intersection::new(3.0, &s);
        let mut ins = VecIntersections::new();
        ins.push(i1);
        ins.push(i2);
        ins.push(i3);
        ins.push(i4.clone());
        ins.push(i5);
        let res = ins.hit();
        assert_eq!(res, Some(i4));
    }
}
