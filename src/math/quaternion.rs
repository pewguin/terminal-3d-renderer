use std::ops;
use crate::math::euler_rotation::EulerRotation;
use crate::math::rotation::Rotation;
use crate::math::vector::Vector;
use crate::math::vertex::Vertex;

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
    pub fn from_euler_angles(x: f32, y: f32, z: f32) -> Quaternion {
        let e = EulerRotation::new(x, y, z);
        e.into()
    }
    pub fn from_euler_vec(v: Vector) -> Quaternion {
        let e: EulerRotation = v.into();
        e.into()
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
    
    fn dot(self, other: &Quaternion) -> f32 {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn slerp(&self, q2: Quaternion, t: f32) -> Quaternion {
        let mut q1 = self.normalized();
        let mut q2 = q2.normalized();

        let mut dot = q1.dot(&q2);

        // If dot < 0, slerp the long way around the sphere
        if dot < 0.0 {
            dot = -dot;
            q2 = -q2;
        }

        const EPSILON: f32 = 1e-6;

        if dot > 1.0 - EPSILON {
            // If angle is small, use lerp and normalize
            let lerped = Quaternion {
                w: q1.w + t * (q2.w - q1.w),
                x: q1.x + t * (q2.x - q1.x),
                y: q1.y + t * (q2.y - q1.y),
                z: q1.z + t * (q2.z - q1.z),
            };
            return lerped.normalized();
        }

        let theta_0 = dot.acos();          // angle between q1 and q2
        let theta = theta_0 * t;           // angle between q1 and result

        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();

        let s1 = ((theta_0 - theta).sin()) / sin_theta_0;
        let s2 = sin_theta / sin_theta_0;

        Quaternion {
            w: q1.w * s1 + q2.w * s2,
            x: q1.x * s1 + q2.x * s2,
            y: q1.y * s1 + q2.y * s2,
            z: q1.z * s1 + q2.z * s2,
        }
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

impl ops::Neg for Quaternion {
    type Output = Quaternion;
    fn neg(self) -> Quaternion {
        Quaternion::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl From<Quaternion> for EulerRotation {
    fn from(q: Quaternion) -> Self {
        let q = q.normalized();

        let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
        let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        let sinp = 2.0 * (q.w * q.y - q.z * q.x);
        let pitch = if sinp.abs() >= 1.0 {
            std::f32::consts::FRAC_PI_2.copysign(sinp)
        } else {
            sinp.asin()
        };

        let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
        let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        EulerRotation::new(roll, pitch, yaw)
    }
}
