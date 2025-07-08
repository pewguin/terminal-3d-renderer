use std::ops;
use std::path::Path;
use crate::geometry::rotation::Rotation;
use crate::geometry::triangle::Triangle;
use crate::geometry::vector::Vector;
use crate::geometry::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle>) -> Self {
        Self { tris }
    }
    pub fn translate(&self, v: &Vector) -> Self {
        let tris: Vec<Triangle> = self.tris.iter().map(|t| t.translate(v)).collect();
        Self::new(tris)
    }
    pub fn rotate<R: Rotation>(&self, rot: &R) -> Self {
        let tris = self.tris.iter().map(|t| t.rotate(rot)).collect();
        Self::new(tris)
    }
}

impl ops::Add<Vector> for Mesh {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self::Output {
        Self::new(self.tris.iter().map(|t| *t + rhs).collect())
    }
}