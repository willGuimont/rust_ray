use crate::marching::Vec3;

pub trait Geometry {
    fn distance_from(&self, pos: Vec3) -> f32;
}


#[macro_export]
macro_rules! geom {
    ( $( $x:expr ),* ) => {
        {
            $(
                Box::new($x) as Box<dyn Geometry>
            )*
        }
    };
}
