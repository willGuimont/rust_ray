use crate::marching::{Geom, Vec3};
use crate::marching::geometries::geometry::Geometry;

pub struct Translation {
    translation: Vec3,
    geometry: Geom,
}

impl Translation {
    pub fn new(translation: Vec3, geometry: Geom) -> Translation {
        Translation { translation, geometry }
    }
}

impl Geometry for Translation {
    fn distance_from(&self, pos: Vec3) -> f32 {
        let p = pos - self.translation;
        self.geometry.distance_from(p)
    }
}
