use crate::math::mesh::Mesh;
use crate::math::projection_type::ProjectionType;
use crate::rendering::camera::Camera;

pub struct InputContext<'a> {
    pub camera: &'a mut Camera,
    pub mesh: &'a mut Mesh,
    pub projection_type: &'a mut ProjectionType,
    pub exit: &'a mut bool,
}