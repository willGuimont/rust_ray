use crate::marching::geometries::Geometry;
use crate::marching::Vec3;

const MAX_NUM_STEPS: i32 = 64;
const EPSILON: f32 = 0.00001;
const MAX_TRACE_DISTANCE: i32 = 1000;

pub struct Camera {
    width: u32,
    height: u32,
    position: Vec3,
    forward: Vec3,
    up: Vec3,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }
}

pub struct Image {
    pub data: Vec<Vec<Color>>,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            width,
            height,
            position: Vec3::new(0., 0., 0.),
            forward: Vec3::new(0., 1., 0.),
            up: Vec3::new(0., 0., 1.),
        }
    }

    pub fn render(&self, geometry: &Box<dyn Geometry>) -> Image {
        let mut data = vec![vec![Color::black(); self.height as usize]; self.width as usize];
        let left = self.up.cross(self.forward);

        for i in 0..self.width {
            let u = i as f32 / self.width as f32 * 2. - 1.;
            for j in 0..self.height {
                let v = j as f32 / self.height as f32 * 2. - 1.;
                let pt_img_plane = self.position + self.forward + left * u - self.up * v;
                let dir = (pt_img_plane - self.position).normalized();

                let color = self.ray_march(&dir);
                data[i as usize][j as usize] = color;
            }
        }

        Image { data, width: self.width, height: self.height }
    }

    fn ray_march(&self, direction: &Vec3) -> Color {
        Color::black()
    }

    fn compute_normal(position: &Vec3) -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
}
