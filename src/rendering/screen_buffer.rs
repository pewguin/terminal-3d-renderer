use std::io::Write;
use termion::cursor::Goto;

pub struct ScreenBuffer {
    pub width: u16,
    pub height: u16,
    buffer: Vec<char>
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16) -> ScreenBuffer {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            buffer: vec![' '; size]
        }
    }

    fn index_of(&self, x: u16, y: u16) -> usize {
        (y * self.width + x) as usize
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, c: char) {
        let i = self.index_of(x, y);
        self.buffer[i] = c;
    }

    pub fn fill(&mut self, c: char) {
        self.buffer.fill(c);
    }

    pub fn clear(&mut self) {
        self.buffer.fill(' ');
    }

    pub fn write<W: Write>(&self, w: &mut W) {
        let mut output = String::with_capacity(((self.width + 1) * self.height + 8) as usize);
        output.push_str("\x1B[1;1H");
        for y in 0..self.height {
            let s = (y * self.width) as usize;
            let e = s + self.width as usize;
            output.push_str(&format!("{}", Goto(1, y + 1)));
            output.extend(&self.buffer[s..e]);
        }

        w.write_all(output.as_bytes()).unwrap();
        w.flush().unwrap();
    }
}