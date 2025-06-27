use crate::geometry::vertex::Vertex;
use crate::geometry::triangle::Triangle;
use crate::math::geometry::signed_area;
use crate::rendering::point::Point;
use crate::rendering::screen_buffer::ScreenBuffer;

pub fn draw_triangle(buf: &mut ScreenBuffer, tri: &Triangle, fill: char) {
    let verts = tri.verts;
    
    for y in 0..buf.height {
        for x in 0..buf.width {
            let p = Point::new(x as i32, y as i32);
            if signed_area(verts[0].into(), verts[1].into(), p) > 0 &&
                signed_area(verts[1].into(), verts[2].into(), p) > 0 &&
                signed_area(verts[2].into(), verts[0].into(), p) > 0 {
                buf.set_pixel(x, y, fill);
            }
        }
    }
}