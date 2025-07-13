use crate::math::mesh::Mesh;
use crate::math::projection_type::ProjectionType;
use crate::rendering::camera::Camera;
use crate::rendering::object::Object;
use crate::rendering::render_buffer::RenderBuffer;

pub struct InputContext<'a> {
    pub camera: &'a mut Camera,
    pub buffer: &'a mut RenderBuffer,
    pub projection_type: &'a mut ProjectionType,
    pub exit: &'a mut bool,
}