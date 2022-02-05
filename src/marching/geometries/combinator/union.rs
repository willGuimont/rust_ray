use crate::marching::geometries::geometry::Geometry;
use crate::marching::{Geom, Vec3};

pub struct Union {
    shapes: Vec<Geom>,
}

impl Union {
    pub fn new(shapes: Vec<Geom>) -> Union {
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
