use crate::geometry::vertex::Vertex;

pub trait Rotation {
    fn rotate(&self, v: Vertex) -> Vertex;
}