use crate::math::vertex::Vertex;
use crate::math::triangle::Triangle;
use crate::rendering::camera::Camera;
use crate::math::geometry::signed_area;
use crate::math::projection_type::ProjectionType;
use crate::rendering::point::Point;
use crate::rendering::screen_buffer::ScreenBuffer;

pub fn draw_triangle(buf: &mut ScreenBuffer, tri: &Triangle, cam: &Camera, prj_type: &ProjectionType) {
    let vertexes = tri.verts.map(|v| cam.project(v, prj_type));
    
    let min_x = (*vertexes.map(|v| v.x.round() as i32).iter().min().unwrap()).max(0);
    let min_y = (*vertexes.map(|v| v.y.round() as i32).iter().min().unwrap()).max(0);
    let max_x = (*vertexes.map(|v| v.x.round() as i32).iter().max().unwrap()).min(buf.width as i32);
    let max_y = (*vertexes.map(|v| v.y.round() as i32).iter().max().unwrap()).min(buf.height as i32);
    
    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = Point::new(x as f32 + 0.5, y as f32 + 0.5);
            
            let e = [
                (vertexes[0], vertexes[1]),
                (vertexes[1], vertexes[2]),
                (vertexes[2], vertexes[0]),
            ];
            
            if signed_area(vertexes[0].into(), vertexes[1].into(), p) > 0.0 &&
                signed_area(vertexes[1].into(), vertexes[2].into(), p) > 0.0 &&
                signed_area(vertexes[2].into(), vertexes[0].into(), p) > 0.0 {
                buf.set_pixel(x as u16, y as u16, tri.stroke);
            }
        }
    }
}