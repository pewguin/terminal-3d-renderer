use std::ops;
use crate::geometry::rotation::Rotation;
use crate::geometry::vector::Vector;
use crate::geometry::vertex::Vertex;
use crate::rendering::stroke::Stroke;

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub verts: [Vertex; 3],
    pub stroke: Stroke
}

impl Triangle {
    pub fn from_vertexes(p0: Vertex, p1: Vertex, p2: Vertex, s: Stroke) -> Self {
        Self { verts: [p0, p1, p2], stroke: s }
    }
    pub fn from_array(p: [Vertex; 3], s: Stroke) -> Self {
        Self { verts: p, stroke: s }
    }
    pub fn avg_z(&self) -> f32 {
        return self.verts.iter().map(|v| v.z).sum::<f32>() / self.verts.len() as f32;
    }
    pub fn translate(&self, v: &Vector) -> Triangle {
        let vs = self.verts.map(|vtx| vtx + *v);
        Self::from_array(vs, self.stroke)
    }
    pub fn rotate<R: Rotation>(&self, r: &R) -> Triangle {
        let vs = self.verts.map(|v| r.rotate_vertex(v));
        Self::from_array(vs, self.stroke)
    }
}

impl ops::Add<Vector> for Triangle {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self {
        Triangle::from_array(self.verts.map(|v| v + rhs), self.stroke)
    }
}