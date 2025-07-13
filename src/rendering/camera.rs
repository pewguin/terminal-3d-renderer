use crate::debug::debug_logger::log;
use crate::math::triangle::Triangle;
use crate::math::vector::Vector;
use crate::math::vertex::Vertex;
use crate::math::projection_type::ProjectionType;
use crate::rendering::point::Point;

pub const SCALE: f32 = 20.0;
pub struct Camera {
    pub origin: Vertex,
    screen_width: u16,
    screen_height: u16,
    fov: f32
}
impl Camera {
    pub fn new(origin: Vertex, screen_width: u16, screen_height: u16) -> Self {
        Self { origin, screen_width, screen_height, fov: 90.0_f32.to_radians() }
    }
    pub fn mv(&mut self, v: Vector) {
        self.origin = self.origin + v;
    }
    pub fn project(&self, v: Vertex, prj_type: &ProjectionType) -> Point {
        let center_x = self.screen_width as f32 / 2.0;
        let center_y = self.screen_height as f32 / 2.0;
        let v = v - self.origin;
        
        let p = match prj_type {
            ProjectionType::Perspective => {
                let f = 1.0 / (self.fov / 2.0).tan();
                (v.x * f * SCALE / -v.z,
                v.y * f * SCALE / -v.z)
            }
            ProjectionType::Orthographic => {
                (v.x,
                v.y)
            }
        };

        Point::new(
            (p.0 * 2.0 + center_x).round() as i32,
            (p.1 + center_y).round() as i32
        )
    }
}