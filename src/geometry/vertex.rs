use std::cmp::Ordering;
use std::ops;
use crate::geometry::vector::Vector;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y)
    }
}

impl ops::Add<Vertex> for Vertex {
    type Output = Vertex;
    fn add(self, rhs: Vertex) -> Vertex {
        Vertex::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Neg for Vertex {
    type Output = Vertex;
    fn neg(self) -> Vertex {
        Vertex::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Sub<Vertex> for Vertex {
    type Output = Vertex;
    fn sub(self, rhs: Vertex) -> Vertex {
        Vertex::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl ops::Add<Vector> for Vertex {
    type Output = Vertex;
    fn add(self, rhs: Vector) -> Vertex {
        Vertex::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}