use crate::marching::geometries::geometry::Geometry;
use crate::marching::Vec3;

pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius }
    }
}

impl Geometry for Sphere {
    fn distance_from(&self, pos: Vec3) -> f32 {
        pos.norm() - self.radius
    }
}
