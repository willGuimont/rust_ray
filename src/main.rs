use crate::marching::{Camera, Sphere};

mod marching;

fn main() {
    let cam = Camera::new(160, 160);
    let geometry = Sphere::new(1.);

    let img = cam.render(Box::new(geometry));
    for i in 0..img.width {
        for j in 0..img.height {
            !todo!()
        }
    }
}
