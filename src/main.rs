mod rendering;
mod geometry;
mod math;
mod debug;
mod interface;
mod parser;

use std::cmp::Ordering;
use termion::{async_stdin, raw::IntoRawMode, terminal_size};
use std::io::{Write, stdin, stdout, Read};
use std::panic;
use std::thread::sleep;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use rendering::screen_buffer::ScreenBuffer;
use crate::debug::debug_logger::{get_logs, log, log_disp};
use crate::interface::move_mode::MoveMode;
use math::euler_rotation::EulerRotation;
use math::triangle::Triangle;
use math::vector::Vector;
use math::vertex::Vertex;
use rendering::camera::Camera;
use math::mesh::Mesh;
use crate::interface::input::Input;
use crate::interface::input_context::InputContext;
use crate::math::projection_type::ProjectionType;
use crate::rendering::object::Object;
use crate::rendering::point::Point;
use crate::rendering::render_buffer::RenderBuffer;
use crate::rendering::stroke::Stroke;

const FRAME_TIME: Duration = Duration::from_micros(16_667);
fn main() {
    let mut stdout = stdout().into_raw_mode().expect("unrecoverable: failed to convert terminal to raw mode");
    let mut stdin = async_stdin().keys();

    panic::set_hook(Box::new(|panic_info| {
        write!(std::io::stdout(), "{}", termion::screen::ToMainScreen).unwrap();
        println!("{}", panic_info);
    }));

    write!(stdout, "{}{}", termion::cursor::Hide, termion::screen::ToAlternateScreen).unwrap();

    let term_dims = terminal_size().expect("unrecoverable: failed to obtain terminal dimensions");
    let mut screen_buffer = ScreenBuffer::new(term_dims.0, term_dims.1);

    for y in 0..screen_buffer.height {
        for x in 0..screen_buffer.width {
            screen_buffer.set_pixel(x, y, Stroke::new([255, 255, 255], ' '));
        }
    }
    
    let mut cam = Camera::new(Vertex::new(0.0, 0.0, -50.0), screen_buffer.width, screen_buffer.height);
    let mut render_buffer = RenderBuffer::new();
    let mut input = Input::new();

    let size = 20.0;
    let mut time = 0.0;
    let mut rot = 0.1;
    let mut pos = 1.0;
    let mut prj_type = ProjectionType::Perspective;
    let mut mv_mode = MoveMode::Rotation;

    let v = (
        Vertex::new(size, size, size),
        Vertex::new(-size, -size, size),
        Vertex::new(-size, size, -size),
        Vertex::new(size, -size, -size),
    );
    let mut tetrahedron = Mesh::new(vec![
        Triangle::from_vertexes(
            v.0, v.1, v.2,
            Stroke::new([255, 0, 0], '█')),
        Triangle::from_vertexes(
            v.0, v.3, v.1,
            Stroke::new([0, 255, 0], '█')),
        Triangle::from_vertexes(
            v.2, v.3, v.0,
            Stroke::new([0, 0, 255], '█')),
        Triangle::from_vertexes(
            v.2, v.1, v.3,
            Stroke::new([255, 0, 255], '█'))]
    );
    let mut tetrahedron = Object::new(tetrahedron);
    
    render_buffer.add_mesh_worldspace(tetrahedron, &cam);

    'frame: loop {
        let ctx = &mut InputContext {
            camera: &mut cam,
            buffer: &mut render_buffer,
            projection_type: &mut prj_type,
            exit: &mut false,
        };

        while let Some(Ok(key)) = stdin.next() {
            input.process_key(key, ctx);
        }

        if *ctx.exit {
            break 'frame
        }
        
        render_buffer.pass_obj_time(FRAME_TIME);
        RenderBuffer::write_meshes_to_buffer(render_buffer.meshes_from_objects(),
            &mut screen_buffer, &prj_type, &cam);

        log_disp(1, &input);
        log(2, &prj_type);
        log(3, &mv_mode);

        for l in get_logs(screen_buffer.height) {
            screen_buffer.fill_string(l.1.as_str(), Point::new(1, l.0 - 1));
        }
        screen_buffer.write(&mut stdout);
        screen_buffer.clear();

        sleep(FRAME_TIME);
        time += FRAME_TIME.as_secs_f32();
    }

    write!(stdout, "{}{}", termion::cursor::Show, termion::screen::ToMainScreen).unwrap();
}