pub use std::sync::Arc;

use crate::marching::Vec3;

pub trait Geometry: Sync {
    fn distance_from(&self, pos: Vec3) -> f32;
}

pub type Geom = Box<dyn Geometry>;

#[macro_export]
macro_rules! geom {
    ( $( $x:expr ),* ) => {
        {
            $(
                Box::new($x) as Geom
            )*
        }
    };
}
