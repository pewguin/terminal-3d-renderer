use std::fmt::{Display, Formatter};
use termion::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Stroke {
    pub color: [u8; 3],
    pub tex: char
}

impl Stroke {
    pub fn new(color: [u8; 3], tex: char) -> Self {
        Self { color, tex }
    }
}

impl Display for Stroke {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[38;2;{};{};{}m", self.color[0], self.color[1], self.color[2])
    }
}