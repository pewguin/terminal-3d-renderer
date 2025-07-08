use crate::geometry::vertex::Vertex;
use crate::geometry::triangle::Triangle;
use crate::rendering::camera::Camera;
use crate::math::geometry::signed_area;
use crate::math::projection_type::ProjectionType;
use crate::rendering::point::Point;
use crate::rendering::screen_buffer::ScreenBuffer;

pub fn draw_triangle(buf: &mut ScreenBuffer, tri: &Triangle, cam: &Camera, prj_type: &ProjectionType) {
    let verts = tri.verts.map(|v| cam.project(v, &prj_type));
    
    for y in 0..buf.height {
        for x in 0..buf.width {
            let p = Point::new(x as i32, y as i32);
            if signed_area(verts[0].into(), verts[1].into(), p) > 0 &&
                signed_area(verts[1].into(), verts[2].into(), p) > 0 &&
                signed_area(verts[2].into(), verts[0].into(), p) > 0 {
                buf.set_pixel(x, y, tri.stroke);
            }
        }
    }
}