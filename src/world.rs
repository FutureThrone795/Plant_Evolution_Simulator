use glium::DrawParameters;

use crate::{camera::CameraState, terrain::Terrain};

pub struct World {
    terrain: Terrain
}

impl World {
    pub fn world_init() -> World {
        return World {terrain: Terrain::random()};
    }

    pub fn world_loop(&mut self) {

    }

    pub fn world_render(
        &self, 
        target: &mut glium::Frame, 
        program: &glium::Program, 
        display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>,
        camera: &CameraState,
        params: &DrawParameters
    ) {
        self.terrain.render(target, program, display, camera, params);
    }
}
