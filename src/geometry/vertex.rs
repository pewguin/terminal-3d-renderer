use std::cmp::Ordering;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn rotate_x(&self, radians: f32) -> Vertex {
        let sin = radians.sin();
        let cos = radians.cos();
        Vertex {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }
    pub fn rotate_y(&self, radians: f32) -> Vertex{
        let sin = radians.sin();
        let cos = radians.cos();
        Vertex {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }
    pub fn rotate_z(&self, radians: f32) -> Vertex{
        let sin = radians.sin();
        let cos = radians.cos();
        Vertex {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y)
    }
}