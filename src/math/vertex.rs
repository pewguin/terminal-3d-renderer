use std::cmp::Ordering;
use std::ops;
use crate::math::vector::Vector;

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
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
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
impl ops::Mul<f32> for Vertex {
    type Output = Vertex;
    fn mul(self, rhs: f32) -> Vertex {
        Vertex::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}