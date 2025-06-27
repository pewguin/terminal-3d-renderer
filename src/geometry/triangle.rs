use crate::geometry::vertex::Vertex;

pub struct Triangle {
    pub verts: [Vertex; 3],
}

impl Triangle {
    pub fn from_vertexes(p0: Vertex, p1: Vertex, p2: Vertex) -> Self {
        Self { verts: [p0, p1, p2] }
    }
    pub fn from_array(p: [Vertex; 3]) -> Self {
        Self { verts: p }
    }
}