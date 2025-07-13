use std::ops::{AddAssign, Mul, MulAssign};
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
    pub fn rotate_x(&self, radians: f32) -> Vector {
        let sin = radians.sin();
        let cos = radians.cos();
        Vector {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }
    pub fn rotate_y(&self, radians: f32) -> Vector {
        let sin = radians.sin();
        let cos = radians.cos();
        Vector {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }
    pub fn rotate_z(&self, radians: f32) -> Vector {
        let sin = radians.sin();
        let cos = radians.cos();
        Vector {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
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