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

    let e = [
        (vertexes[0], vertexes[1]),
        (vertexes[1], vertexes[2]),
        (vertexes[2], vertexes[0]),
    ];


    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = Point::new(x as f32 + 0.5, y as f32 + 0.5);

            let mut inside = true;
            for (i, (p0, p1)) in e.iter().enumerate() {
                let a = signed_area(*p0, *p1, p);
                let top_left = is_top_left(*p0, *p1);
                if a < 0.0 || (a == 0.0 && !top_left) {
                    inside = false;
                    break;
                }
            }
            
            if inside {
                buf.set_pixel(x as u16, y as u16, tri.stroke.shaded(tri.normal()));
            }
        }
    }
}


fn is_top_left(a: Point, b: Point) -> bool {
    // Top edge: y1 == y2 and x1 < x2 (horizontal, left to right)
    // Left edge: y1 < y2 (vertical, top to bottom)
    (a.y == b.y && a.x < b.x) || (a.y < b.y)
}