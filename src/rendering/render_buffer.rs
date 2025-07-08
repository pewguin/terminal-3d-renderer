use std::cmp::Ordering;
use crate::debug::debug_logger::log;
use crate::geometry::mesh::Mesh;
use crate::geometry::triangle::Triangle;
use crate::math::projection_type::ProjectionType;
use crate::rendering::camera::Camera;
use crate::rendering::rasterizer::draw_triangle;
use crate::rendering::screen_buffer::ScreenBuffer;

pub struct RenderBuffer {
    meshes: Vec<Mesh>,
}

impl RenderBuffer {
    pub fn new() -> Self {
        RenderBuffer {
            meshes: Vec::new(),
        }
    }
    pub fn add_mesh_worldspace(&mut self, mesh: &Mesh, camera: &Camera) {
        let mesh = mesh.clone();
        self.meshes.push(mesh);
    }
    pub fn clear(&mut self) {
        self.meshes.clear();
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
    pub fn flush_meshes_to_buffer(&self, buffer: &mut ScreenBuffer, prj_type: &ProjectionType, camera: &Camera) {
        let tris = self.meshes.iter().flat_map(|m| m.tris.iter().copied()).collect();
        let tris = Self::order_tris_by_z(tris);
        
        for tri in tris {
            draw_triangle(buffer, &tri, camera, prj_type);
        }
    }
}