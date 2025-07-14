use std::io::Write;
use termion::color;
use termion::color::{Color, Fg};
use termion::cursor::Goto;
use crate::rendering::point::Point;
use crate::rendering::stroke::Stroke;

pub struct ScreenBuffer {
    pub width: u16,
    pub height: u16,
    buffer: Vec<Stroke>,
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16) -> ScreenBuffer {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            buffer: vec![Stroke::new([255, 255, 255], ' '); size]
        }
    }

    fn index_of(&self, x: u16, y: u16) -> usize {
        (y * self.width + x) as usize
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, s: Stroke) {
        let i = self.index_of(x, y);
        self.buffer[i] = s;
    }
    
    pub fn fill_string(&mut self, s: &str, p: Point) {
        let cs = s.chars().collect::<Vec<_>>();
        for i in 0..s.len() {
            let idx = self.index_of(p.x as u16 + i as u16, p.y as u16);
            self.buffer[idx] = Stroke::new([255, 255, 255], cs[i]) ;
        }
    }

    pub fn fill(&mut self, s: Stroke) {
        self.buffer.fill(s);
    }

    pub fn clear(&mut self) {
        self.buffer.fill(Stroke::new([255, 255, 255], ' '));
    }

    pub fn write<W: Write>(&self, w: &mut W) {
        let mut output = String::with_capacity(((self.width + 1) * self.height + 8) as usize);
        output.push_str("\x1B[1;1H");
        for y in 0..self.height {
            for x in 0..self.width {
                let s = self.buffer[self.index_of(x, y)];
                output.push_str(&format!("{}{}", s.as_str(), Goto(x + 1, y + 1)));
                output.push(s.tex)
            }
        }

        w.write_all(output.as_bytes()).unwrap();
        w.flush().unwrap();
    }
}