use crate::plant::Plant;

use crate::render::{camera::CameraState, Vertex};

use glium::{DrawParameters, Surface};

impl Plant {
    pub fn render_branch_recursive(
        &self, 
        branch_index: usize, 
        target: &mut glium::Frame, 
        program: &glium::Program, 
        display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>,
        camera: &CameraState,
        params: &DrawParameters
    ) {

    }
}