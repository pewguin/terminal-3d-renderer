use crate::math::vector::Vector;
use crate::math::vertex::Vertex;

pub trait Rotation {
    fn rotate_vector(&self, v: Vector) -> Vector;
    fn rotate_vertex(&self, v: Vertex) -> Vertex {
        self.rotate_vector(v.into()).into()
    }
}