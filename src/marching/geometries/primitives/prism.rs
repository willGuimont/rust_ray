use crate::marching::geometries::geometry::Geometry;
use crate::marching::Vec3;

pub struct Prism {
    size: Vec3,
}

impl Prism {
    pub fn new(size: Vec3) -> Prism {
        Prism { size }
    }
}

impl Geometry for Prism {
    fn distance_from(&self, pos: Vec3) -> f32 {
        let x = f32::max(f32::abs(pos.x) - self.size.x, 0.);
        let y = f32::max(f32::abs(pos.y) - self.size.y, 0.);
        let z = f32::max(f32::abs(pos.y) - self.size.z, 0.);

        Vec3::new(x, y, z).norm()
    }
}
