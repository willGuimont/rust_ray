use crate::marching::{Geom, Vec3};
use crate::marching::geometries::geometry::Geometry;

pub struct Smooth {
    k: f32,
    a: Geom,
    b: Geom,
}

impl Smooth {
    pub fn new(k: f32, a: Geom, b: Geom) -> Smooth {
        Smooth { k, a, b }
    }
}

impl Geometry for Smooth {
    fn distance_from(&self, pos: Vec3) -> f32 {
        let from_a = self.a.distance_from(pos);
        let from_b = self.b.distance_from(pos);
        let h = f32::max(self.k - f32::abs(from_a - from_b), 0.) / self.k;
        f32::min(from_a, from_b) - f32::powf(h, 3.) * self.k / 6.
    }
}
