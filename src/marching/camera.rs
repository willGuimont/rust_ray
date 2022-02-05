use crate::marching::geometries::Geometry;
use crate::marching::Vec3;

const MAX_NUM_STEPS: i32 = 64;
const EPSILON: f32 = 0.00001;
const MAX_TRACE_DISTANCE: f32 = 1000.;

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
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color::new(0, 0, 0)
    }
}

pub struct Image {
    pub data: Vec<Vec<Color>>,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
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

                let color = self.ray_march(&dir, geometry);
                data[i as usize][j as usize] = color;
            }
        }

        Image { data, width: self.width, height: self.height }
    }

    fn ray_march(&self, direction: &Vec3, geometry: &Box<dyn Geometry>) -> Color {
        let mut distance_traveled = 0.;
        'running: for _ in 0..MAX_NUM_STEPS {
            let current_pos = self.position + distance_traveled * direction;
            let distance_to_closest = geometry.distance_from(current_pos);

            if distance_to_closest < EPSILON {
                let normal = self.compute_normal(&current_pos, geometry);
                // TODO allow movable light
                let light_pos = Vec3::new(1., 1., 1.);
                let dir_light = (light_pos - current_pos).normalized();
                let diffuse_intensity = f32::max(0., normal.dot(dir_light));
                let c = (normal * 0.5 + 0.5) * 255. * diffuse_intensity;
                return Color::new(c.x as u8, c.y as u8, c.z as u8);
            } else if distance_traveled > MAX_TRACE_DISTANCE {
                break 'running;
            } else {
                distance_traveled += distance_to_closest;
            }
        }

        Color::black()
    }

    fn compute_normal(&self, position: &Vec3, geometry: &Box<dyn Geometry>) -> Vec3 {
        let dx = geometry.distance_from(position + Vec3::new(EPSILON, 0., 0.));
        let dx2 = geometry.distance_from(position - Vec3::new(EPSILON, 0., 0.));
        let dy = geometry.distance_from(position + Vec3::new(0., EPSILON, 0.));
        let dy2 = geometry.distance_from(position - Vec3::new(0., EPSILON, 0.));
        let dz = geometry.distance_from(position + Vec3::new(0., 0., EPSILON));
        let dz2 = geometry.distance_from(position - Vec3::new(0., 0., EPSILON));

        Vec3::new(dx - dx2, dy - dy2, dz - dz2).normalized()
    }
}
