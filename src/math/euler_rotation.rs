use std::ops::{AddAssign, Mul, MulAssign};
use crate::math::quaternion::Quaternion;
use crate::math::rotation::Rotation;
use crate::math::vector::Vector;
use crate::math::vertex::Vertex;

pub struct EulerRotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl EulerRotation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn from_degrees(x: f32, y: f32, z: f32) -> Self {
        Self::new(x.to_radians(), y.to_radians(), z.to_radians())
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}
impl Rotation for EulerRotation {
    fn rotate_vector(&self, v: Vector) -> Vector {
        let q: Quaternion = v.into();
        q * v
    }
}

impl From<Vector> for EulerRotation {
    fn from(v: Vector) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl Mul<f32> for EulerRotation {
    type Output = EulerRotation;
    fn mul(self, rhs: f32) -> Self::Output {
        EulerRotation::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl AddAssign<EulerRotation> for EulerRotation {
    fn add_assign(&mut self, rhs: EulerRotation) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f32> for EulerRotation {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}