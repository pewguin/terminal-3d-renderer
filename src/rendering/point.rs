use crate::math::vertex::Vertex;

#[derive(Copy, Clone, Debug)]
pub struct Point{
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}