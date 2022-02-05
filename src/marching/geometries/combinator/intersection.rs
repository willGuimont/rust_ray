use crate::marching::geometries::geometry::Geometry;
use crate::marching::Vec3;

pub struct Intersection {
    shapes: Vec<Box<dyn Geometry>>,
}

impl Intersection {
    pub fn new(shapes: Vec<Box<dyn Geometry>>) -> Intersection {
        Intersection { shapes }
    }
}

impl Geometry for Intersection {
    fn distance_from(&self, pos: Vec3) -> f32 {
        self.shapes.iter()
            .map(|x| { x.distance_from(pos) })
            .max_by(|a, b| {a.partial_cmp(b).unwrap()})
            .unwrap_or(f32::INFINITY)
    }
}
