use std::cmp::Ordering;
use std::time::Duration;
use crate::debug::debug_logger::log;
use crate::interface::input::{ActiveCommand, CommandType, InterpolationMode};
use crate::math::euler_rotation::EulerRotation;
use crate::math::mesh::Mesh;
use crate::math::triangle::Triangle;
use crate::math::projection_type::ProjectionType;
use crate::math::quaternion::Quaternion;
use crate::math::vector::Vector;
use crate::rendering::camera::Camera;
use crate::rendering::object::Object;
use crate::rendering::rasterizer::draw_triangle;
use crate::rendering::screen_buffer::ScreenBuffer;

pub struct RenderBuffer {
    objs: Vec<Object>,
}

impl RenderBuffer {
    pub fn new() -> Self {
        RenderBuffer {
            objs: Vec::new(),
        }
    }
    pub fn add_mesh_worldspace(&mut self, obj: Object, camera: &Camera) {
        self.objs.push(obj);
    }
    pub fn add_command_to_obj(&mut self, cmd: ActiveCommand, obj_id: usize) {
        self.objs[obj_id].active_commands.push(cmd);
    }
    pub fn clear(&mut self) {
        self.objs.clear();
    }
    pub fn order_tris_by_z(mut tris: Vec<Triangle>) -> Vec<Triangle> {
        tris.sort_by(|t, o| {
            if t.avg_z() > o.avg_z() {
                return Ordering::Greater
            }
            else if t.avg_z() < o.avg_z() {
                return Ordering::Less
            }
            Ordering::Equal
        });
        tris
    }
    pub fn write_meshes_to_buffer(meshes: Vec<Mesh>, buffer: &mut ScreenBuffer, prj_type: &ProjectionType, camera: &Camera) {
        let tris = meshes.iter().flat_map(|m| m.tris.iter().copied()).collect();
        let tris = Self::order_tris_by_z(tris);
        
        for tri in tris {
            draw_triangle(buffer, &tri, camera, prj_type);
        }
    }
    pub fn pass_obj_time(&mut self, time: Duration) {
        for obj in self.objs.iter_mut() {
            obj.pass_time(time);
        }
    }
    pub fn meshes_from_objects(&mut self) -> Vec<Mesh> {
        self.objs.iter_mut().map(|obj| { obj.apply_commands() }).collect()
    }
}