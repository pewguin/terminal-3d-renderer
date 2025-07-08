use std::ops;
use crate::geometry::euler_rotation::EulerRotation;
use crate::geometry::rotation::Rotation;
use crate::geometry::vector::Vector;
use crate::geometry::vertex::Vertex;

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x, y, z, w }
    }
    pub fn identity() -> Quaternion {
        Quaternion::new(0.0, 0.0, 0.0, 1.0)
    }
    pub fn from_axis_angle(axis: Vector, angle: f32) -> Quaternion {
        let half_angle = angle / 2.0;
        let cos = half_angle.cos();
        let sin = half_angle.sin();

        Quaternion::new(axis.x * sin, axis.y * sin, axis.z * sin, cos)
    }
    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    pub fn normalized(&self) -> Quaternion {
        *self / self.len()
    }
}

impl Rotation for Quaternion {
    fn rotate_vector(&self, v: Vector) -> Vector {
        *self * v
    }
}

impl From<EulerRotation> for Quaternion {
    fn from(e: EulerRotation) -> Quaternion {
        let (cx, sx) = ((e.x * 0.5).cos(), (e.x * 0.5).sin());
        let (cy, sy) = ((e.y * 0.5).cos(), (e.y * 0.5).sin());
        let (cz, sz) = ((e.z * 0.5).cos(), (e.z * 0.5).sin());
        
        let x = sx * cy * cz - cx * sy * sz;
        let y = cx * sy * cz + sx * cy * sz;
        let z = cx * cy * sz - sx * sy * cz;
        let w = cx * cy * cz + sx * sy * sz;

        Quaternion { x, y, z, w }
    }
}
impl From<Vector> for Quaternion {
    fn from(v: Vector) -> Quaternion {
        Quaternion::new(v.x, v.y, v.z, 0.0)
    }
}
impl Into<Vector> for Quaternion {
    fn into(self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }
}
impl ops::Mul<Vector> for Quaternion {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        let q_ = self.conjugate();
        let v_q: Quaternion = rhs.into();
        (self * v_q * q_).into()
    }
}
impl ops::Mul<f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f32) -> Quaternion {
        Quaternion::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs,
        )
    }
}
impl ops::Div<f32> for Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: f32) -> Quaternion {
        self * (1.0 / rhs)
    }
}