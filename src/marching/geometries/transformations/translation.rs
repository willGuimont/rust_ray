use crate::marching::geometries::geometry::Geometry;
use crate::marching::Vec3;

pub struct Translation {
    translation: Vec3,
    geometry: Box<dyn Geometry>,
}

impl Translation {
    pub fn new(translation: Vec3, geometry: Box<dyn Geometry>) -> Translation {
        Translation { translation, geometry }
    }
}

impl Geometry for Translation {
    fn distance_from(&self, pos: Vec3) -> f32 {
        let p = pos - self.translation;
        self.geometry.distance_from(p)
    }
}
