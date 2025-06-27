use crate::geometry::vertex::Vertex;
use crate::rendering::point::Point;

pub struct Camera {
    origin: Vertex,
    screen_width: u16,
    screen_height: u16,
}
impl Camera {
    pub fn new(origin: Vertex, screen_width: u16, screen_height: u16) -> Self {
        Self { origin, screen_width, screen_height }
    }
    pub fn project(&self, v: Vertex) -> Point {
        let center_x = self.screen_width as f32 / 2.0;
        let center_y = self.screen_height as f32 / 2.0;
        Point::new(
            (v.x * 2.0 + center_x).round() as i32,
            (v.y + center_y).round() as i32
        )
    }
}