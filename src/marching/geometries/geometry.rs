use crate::marching::Vec3;

pub trait Geometry {
    fn distance_from(&self, pos: Vec3) -> f32;
}
