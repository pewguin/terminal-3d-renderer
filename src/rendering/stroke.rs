use std::fmt::{Display, Formatter};
use termion::color::Color;
use crate::math::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Stroke {
    pub color: [u8; 3],
    pub tex: char,
}

impl Stroke {
    pub fn new(color: [u8; 3], tex: char) -> Self {
        Self { color, tex }
    }
    pub fn as_str(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.color[0], self.color[1], self.color[2])
    }
    pub fn shaded(&self, normal: Vector) -> Stroke {
        let m = (normal.normalized().dot(Vector::new(0.0, 1.0, 0.0)) + 1.0) / 2.0;
        let c = self.color.map(|v| (v as f32 * m).round() as u8);

        Self::new(c, self.tex)
    }
}