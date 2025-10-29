use glium::DrawParameters;

use crate::render::camera::CameraState;
use crate::plant::plant_option_vec::PlantOptionVec;
use crate::terrain::Terrain;

pub struct World {
    pub terrain: Terrain,
    pub plants: PlantOptionVec
}

impl World {
    pub fn world_init() -> World {
        return World {
            terrain: Terrain::random(),
            plants: PlantOptionVec::new()
        };
    }

    pub fn tick(&mut self, total_ticks: u64, display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>, camera: &CameraState) {
        self.terrain.water_height = (total_ticks as f32 * 0.1).sin() as f32 * 5.0 + (total_ticks as f32 * 0.0271).sin() as f32 * 5.0;

        self.plants.tick(&self.terrain, total_ticks, display, camera);
    }

    pub fn render(
        &self, 
        target: &mut glium::Frame, 
        program: &glium::Program, 
        display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>,
        camera: &CameraState,
        params: &DrawParameters
    ) {
        self.plants.render(target, program, display, camera, params);

        self.terrain.render(target, program, display, camera, params);
    }
}
