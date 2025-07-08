use crate::geometry::vector::Vector;
use crate::geometry::vertex::Vertex;

pub trait Rotation {
    fn rotate_vector(&self, v: Vector) -> Vector;
    fn rotate_vertex(&self, v: Vertex) -> Vertex {
        self.rotate_vector(v.into()).into()
    }
}