mod rendering;
mod geometry;
mod math;

use termion::{async_stdin, raw::IntoRawMode, terminal_size};
use std::io::{Write, stdin, stdout, Read};
use std::thread::sleep;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use rendering::screen_buffer::ScreenBuffer;
use crate::geometry::triangle::Triangle;
use crate::geometry::vertex::Vertex;
use crate::rendering::rasterizer::draw_triangle;

const FRAME_TIME: Duration = Duration::from_micros(16_667);
fn main() {
    let mut stdout = stdout().into_raw_mode().expect("unrecoverable: failed to convert terminal to raw mode");
    let mut stdin = async_stdin().keys();

    write!(stdout, "{}{}", termion::cursor::Hide, termion::screen::ToAlternateScreen).unwrap();

    let term_dims = terminal_size().expect("unrecoverable: failed to obtain terminal dimensions");
    let mut buffer = ScreenBuffer::new(term_dims.0, term_dims.1);

    for y in 0..buffer.height {
        for x in 0..buffer.width {
            buffer.set_pixel(x, y, ' ');
        }
    }

    'frame: loop {
        draw_triangle(&mut buffer, &Triangle::from_vertexes(Vertex::new(10.0, 2.0, 0.0), Vertex::new(80.0, 30.0, 0.0), Vertex::new(10.0, 10.0, 0.0)), '█');

        buffer.write(&mut stdout);
        if let Some(c) = stdin.next() {
            match c.unwrap() {
                Key::Char('q') => break 'frame,
                _ => {}
            }
        }
        sleep(FRAME_TIME);
    }

    write!(stdout, "{}{}", termion::cursor::Show, termion::screen::ToMainScreen).unwrap();
}