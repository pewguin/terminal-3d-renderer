use std::ops::{AddAssign, Div, Mul, MulAssign};
use crate::math::vertex::Vertex;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }
    pub fn zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }
    pub fn dot(&self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn len(&self) -> f32 {
        self.dot(*self).sqrt()
    }
    pub fn normalized(&self) -> Vector {
        *self / self.len()
    }
    pub fn cross(self, other: Vector) -> Vector {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Into<Vertex> for Vector {
    fn into(self) -> Vertex {
        Vertex::new(self.x, self.y, self.z)
    }
}

impl From<Vertex> for Vector {
    fn from(v: Vertex) -> Vector {
        Vector::new(v.x, v.y, v.z)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Vector {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Vector {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}