use crate::marching::geometries::geometry::Geometry;
use crate::marching::Vec3;

pub struct Difference {
    shape: Box<dyn Geometry>,
    sub: Box<dyn Geometry>,
}

impl Difference {
    pub fn new(shape: Box<dyn Geometry>, sub: Box<dyn Geometry>) -> Difference {
        Difference { shape, sub }
    }
}

impl Geometry for Difference {
    fn distance_from(&self, pos: Vec3) -> f32 {
        let from_shape = self.shape.distance_from(pos);
        let from_sub = self.sub.distance_from(pos);
        f32::max(from_shape, -from_sub)
    }
}
