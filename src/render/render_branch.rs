use crate::plant::{branch, Plant};
use crate::render::{camera::CameraState, Vertex};
use crate::render::branch_model::generate_branch_model;
use crate::render::mat4_def::Mat4;
use crate::render::vector_math;

use glium::{DrawParameters, Surface};

impl Plant {
    pub fn render_branch(
        &self, 
        branch_index: usize, 
        plant_vertices: &mut Vec<Vertex>,
        plant_indices: &mut Vec<u32>,
        matrix: Mat4
    ) {
        let color: [f32; 4] = [self.branches[branch_index].strength, self.branches[branch_index].photoreceptiveness, self.branches[branch_index].water_intake, 1.0];
        let branch_length_real = 1.0 + self.branches[branch_index].length * 10.0;

        let (mut vertices, mut indices) = generate_branch_model(color, Mat4::scale(4.0, branch_length_real, 4.0) * matrix, plant_vertices.len() as u32);

        plant_vertices.append(&mut vertices);
        plant_indices.append(&mut indices);
    }
}