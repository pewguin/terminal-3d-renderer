use crate::rendering::point::Point;
pub fn signed_area(a: Point, b: Point, c: Point) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y -a.y) * (c.x - a.x)
}