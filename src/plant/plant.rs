use std::fmt::{Debug, Formatter};

use glium::{DrawParameters, Surface};

use crate::render::camera::CameraState;
use crate::terrain::{Terrain, TERRAIN_CELL_WIDTH};
use crate::render::Vertex;

pub struct Plant {
    //pub genome: <What type is this?>,

    pub root_position: (f32, f32, f32),
    //pub root_branch: Box<Branch>,
    
    pub current_energy: f32,
    pub current_water: f32,
    pub current_sunlight: f32
}

impl Debug for Plant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        return write!(f, "Plant")
    }
}

impl Plant {
    pub fn tick(&mut self, terrain: &Terrain) -> bool {
        //Returns false when the plant has died and should be removed

        let mut homeostasis: f32 = 2.0;

        //recursively tick root branch

        if self.current_water > self.current_sunlight {
            self.current_energy += self.current_water;
            self.current_sunlight -= self.current_water;
            self.current_energy = 0.0;
        } else {
            self.current_energy += self.current_sunlight;
            self.current_sunlight = 0.0;
            self.current_energy -= self.current_sunlight;
        }
        
        self.current_energy -= homeostasis;

        return self.current_energy > 0.0;
    }

    pub fn render(
        &self, 
        target: &mut glium::Frame, 
        program: &glium::Program, 
        display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>,
        camera: &CameraState,
        params: &DrawParameters
    ) {
        let vertices: Vec<Vertex> = vec![
            Vertex {    
                position: [ 0.0,  0.0,  0.0], color: [self.current_energy / 1000.0, 0.0, 0.0, 1.0]
            },
            Vertex {    
                position: [ 2.0, 10.0,  2.0], color: [self.current_energy / 1000.0, 0.0, 0.0, 1.0]
            },
            Vertex {    
                position: [ 2.0, 10.0, -2.0], color: [self.current_energy / 1000.0, 0.0, 0.0, 1.0]
            },
            Vertex {    
                position: [-2.0, 10.0, -2.0], color: [self.current_energy / 1000.0, 0.0, 0.0, 1.0]
            },
            Vertex {    
                position: [-2.0, 10.0,  2.0], color: [self.current_energy / 1000.0, 0.0, 0.0, 1.0]
            }
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2, 
            0, 2, 3, 
            0, 3, 4, 
            0, 4, 1, 
            1, 3, 2, 
            1, 4, 3
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        let uniforms = uniform! {
            view: camera.get_view(),
            perspective: camera.get_perspective(),
            offset: [self.root_position.0 * TERRAIN_CELL_WIDTH, self.root_position.1, self.root_position.2 * TERRAIN_CELL_WIDTH]
        };

        target.draw(&vertex_buffer, &index_buffer, program, &uniforms, params).unwrap();
    }

    pub fn new (x: f32, z: f32, starting_energy: f32, terrain: &Terrain) -> Plant {
        return Plant {
            root_position: (x, terrain.get_height(x, z), z),
            current_energy: starting_energy,
            current_sunlight: 0.0,
            current_water: 0.0
        }
    }
}