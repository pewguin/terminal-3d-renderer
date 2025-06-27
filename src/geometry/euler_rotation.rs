use crate::geometry::rotation::Rotation;
use crate::geometry::vertex::Vertex;

pub struct EulerRotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl EulerRotation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}
impl Rotation for EulerRotation {
    fn rotate(&self, v: Vertex) -> Vertex {
        let v = v.rotate_x(self.x);
        let v = v.rotate_y(self.y);
        v.rotate_z(self.z)
    }
}