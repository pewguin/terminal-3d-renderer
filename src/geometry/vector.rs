use crate::geometry::vertex::Vertex;

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