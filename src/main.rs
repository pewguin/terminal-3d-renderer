mod rendering;
mod geometry;
mod math;

use std::cmp::Ordering;
use termion::{async_stdin, raw::IntoRawMode, terminal_size};
use std::io::{Write, stdin, stdout, Read};
use std::panic;
use std::thread::sleep;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use rendering::screen_buffer::ScreenBuffer;
use crate::geometry::euler_rotation::EulerRotation;
use crate::geometry::triangle::Triangle;
use crate::geometry::vertex::Vertex;
use crate::math::camera::Camera;
use crate::rendering::point::Point;
use crate::rendering::rasterizer::draw_triangle;
use crate::rendering::stroke::Stroke;

const FRAME_TIME: Duration = Duration::from_micros(16_667);
fn main() {
    let mut stdout = stdout().into_raw_mode().expect("unrecoverable: failed to convert terminal to raw mode");
    let mut stdin = async_stdin().keys();

    panic::set_hook(Box::new(|panic_info| {
        write!(std::io::stdout(), "{}", termion::screen::ToMainScreen).unwrap();
    }));

    write!(stdout, "{}{}", termion::cursor::Hide, termion::screen::ToAlternateScreen).unwrap();

    let term_dims = terminal_size().expect("unrecoverable: failed to obtain terminal dimensions");
    let mut buffer = ScreenBuffer::new(term_dims.0, term_dims.1);

    for y in 0..buffer.height {
        for x in 0..buffer.width {
            buffer.set_pixel(x, y, Stroke::new([255, 255, 255], ' '));
        }
    }
    
    let cam = Camera::new(Vertex::new(0.0, 0.0, 0.0), buffer.width, buffer.height);

    let sz = 20.0;
    let mut time = 0.0;
    let mut rot = EulerRotation::zero();
    let mut d_rot = 0.1;

    'frame: loop {
        let mut tris: Vec<Triangle> = Vec::new();

        let v = (
            Vertex::new(sz, sz, sz),
            Vertex::new(-sz, -sz, sz),
            Vertex::new(-sz, sz, -sz),
            Vertex::new(sz, -sz, -sz),
        );
        tris.push(
            Triangle::from_vertexes(
                v.0, v.2, v.1,
                Stroke::new([255, 0, 0], '█')));
        tris.push(
            Triangle::from_vertexes(
                v.0, v.1, v.3,
                Stroke::new([0, 255, 0], '█')));
        tris.push(
            Triangle::from_vertexes(
                v.2, v.0, v.3,
                Stroke::new([0, 0, 255], '█')));
        tris.push(
            Triangle::from_vertexes(
                v.2, v.3, v.1,
                Stroke::new([255, 0, 255], '█')));

        tris.sort_by(|t, o| {
            if t.avg_z() > o.avg_z() {
                return Ordering::Greater
            }
            else if t.avg_z() < o.avg_z() {
                return Ordering::Less
            }
            Ordering::Equal
        });
        for tri in tris {
            let tri = tri.rotate(&rot);
            draw_triangle(&mut buffer, &tri, &cam);
        }
        buffer.fill_string(format!("{}", time).as_str(), Point::new(1, (buffer.height-2) as i32));
        
        if let Some(Ok(c)) = stdin.next() {
            match c {
                Key::Ctrl('c') => break 'frame,
                Key::Char('d') => rot.y += d_rot,
                Key::Char('a') => rot.y -= d_rot,
                Key::Char('w') => rot.x += d_rot,
                Key::Char('s') => rot.x -= d_rot,
                Key::Char('e') => rot.z += d_rot,
                Key::Char('q') => rot.z -= d_rot,
                Key::Up => d_rot += 0.01,
                Key::Down => d_rot -= 0.01,
                _ => {}
            }
        }
        
        buffer.fill_string(format!("{}", d_rot).as_str(), Point::new(1, (buffer.height-1) as i32));
        buffer.write(&mut stdout);
        buffer.clear();
        
        sleep(FRAME_TIME);
        time += FRAME_TIME.as_secs_f32();
    }

    write!(stdout, "{}{}", termion::cursor::Show, termion::screen::ToMainScreen).unwrap();
}