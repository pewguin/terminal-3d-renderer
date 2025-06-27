use crate::geometry::vertex::Vertex;

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

impl From<Vertex> for Point {
    fn from(vertex: Vertex) -> Self {
        Point { x: vertex.x.round() as i32, y: vertex.y.round() as i32 }
    }
}