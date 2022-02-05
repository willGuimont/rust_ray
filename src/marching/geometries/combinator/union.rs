use crate::marching::geometries::geometry::Geometry;
use crate::marching::Vec3;

pub struct Union {
    shapes: Vec<Box<dyn Geometry>>,
}

impl Union {
    pub fn new(shapes: Vec<Box<dyn Geometry>>) -> Union {
        Union { shapes }
    }
}

impl Geometry for Union {
    fn distance_from(&self, pos: Vec3) -> f32 {
        self.shapes.iter()
            .map(|x| { x.distance_from(pos) })
            .min_by(|a, b| {a.partial_cmp(b).unwrap()})
            .unwrap_or(f32::INFINITY)
    }
}
