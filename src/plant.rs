use glium::{DrawParameters, Surface};

use crate::camera::CameraState;
use crate::terrain::{Terrain, TERRAIN_CELL_WIDTH};
use crate::vertex_def::Vertex;

pub struct Plant {
    pub position: (f32, f32, f32)
}

impl Plant {
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
                position: [ 0.0,  0.0,  0.0], color: [1.0, 0.0, 0.0, 1.0]
            },
            Vertex {    
                position: [ 2.0, 10.0,  2.0], color: [1.0, 0.0, 0.0, 1.0]
            },
            Vertex {    
                position: [ 2.0, 10.0, -2.0], color: [1.0, 0.0, 0.0, 1.0]
            },
            Vertex {    
                position: [-2.0, 10.0, -2.0], color: [1.0, 0.0, 0.0, 1.0]
            },
            Vertex {    
                position: [-2.0, 10.0,  2.0], color: [1.0, 0.0, 0.0, 1.0]
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
            offset: [self.position.0 * TERRAIN_CELL_WIDTH, self.position.1, self.position.2 * TERRAIN_CELL_WIDTH]
        };

        target.draw(&vertex_buffer, &index_buffer, program, &uniforms, params).unwrap();
    }

    pub fn new (x: f32, z: f32, terrain: &Terrain) -> Plant {
        return Plant {
            position: (x, terrain.get_height(x, z), z)
        }
    }
}