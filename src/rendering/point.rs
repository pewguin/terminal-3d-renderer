use crate::math::vertex::Vertex;

#[derive(Copy, Clone, Debug)]
pub struct Point{
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}