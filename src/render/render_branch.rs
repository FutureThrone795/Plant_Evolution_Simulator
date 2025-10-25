use crate::plant::Plant;
use crate::render::{camera::CameraState, Vertex};
use crate::render::branch_model::generate_branch_model;
use crate::render::mat4_def::Mat4;

use glium::{DrawParameters, Surface};

impl Plant {
    pub fn render_branch_recursive(
        &self, 
        branch_index: usize, 
        target: &mut glium::Frame, 
        program: &glium::Program, 
        display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>,
        camera: &CameraState,
        params: &DrawParameters,
        matrix: Mat4
    ) {
        let color: [f32; 4] = [self.branches[branch_index].strength, self.branches[branch_index].photoreceptiveness, self.branches[branch_index].water_intake, 1.0];
        let (vertices, indices) = generate_branch_model(color);

        let model_matrix = Mat4::scale(4.0, 1.0 + self.branches[branch_index].length * 10.0, 4.0) * matrix.clone();

        let uniforms = uniform! {
            view: camera.get_view(),
            perspective: camera.get_perspective().0,
            model: model_matrix.0
        };
        
        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        target.draw(&vertex_buffer, &index_buffer, program, &uniforms, params).unwrap();

        match &self.branches[branch_index].offshoot_1 {
            Some(branch_connection) => {
                let offshoot_1_matrix = Mat4::translation(2.0, 3.0, 0.0) * matrix.clone();

                self.render_branch_recursive(branch_connection.branch_index, target, program, display, camera, params, offshoot_1_matrix);
            },
            None => ()
        }
        match &self.branches[branch_index].offshoot_2 {
            Some(branch_connection) => {
                let offshoot_2_matrix = Mat4::translation(-2.0, 3.0, 0.0) * matrix.clone();

                self.render_branch_recursive(branch_connection.branch_index, target, program, display, camera, params, offshoot_2_matrix);
            },
            None => ()
        }
    }
}