const TERRAIN_GRID_WIDTH: usize = 10;

use crate::{camera::CameraState, Vertex};

use glium::{DrawParameters, Surface};

use rand::Rng;

#[derive(Clone, Copy)]
pub enum GroundType {
    Grass,
    Rock,
    Sand,
    Swamp    
}

impl GroundType {
    pub fn color(&self) -> [f32; 3] {
        match self {
            Self::Grass => [0.15, 0.65, 0.20],
            Self::Rock  => [0.50, 0.50, 0.55],
            Self::Sand  => [0.90, 0.85, 0.45],
            Self::Swamp => [0.55, 0.30, 0.10]
        }
    }

    pub fn random() -> GroundType {
        match rand::rng().random_range(0..4) {
            0 => Self::Grass,
            1 => Self::Rock,
            2 => Self::Sand,
            3 => Self::Swamp,

            _ => Self::Grass //Grass is the default variant when something goes wrong
        }
    }
}

#[derive(Clone, Copy)]
pub struct TerrainGridNode {
    height: f32,
    ground_type: GroundType
}

pub struct Terrain {
    grid: [[TerrainGridNode; TERRAIN_GRID_WIDTH]; TERRAIN_GRID_WIDTH] 
}

impl Terrain {
    pub fn empty() -> Terrain {
        return Terrain {grid: [[TerrainGridNode {
            height: 0.0,
            ground_type: GroundType::Grass
        }; TERRAIN_GRID_WIDTH]; TERRAIN_GRID_WIDTH]};
    }

    pub fn random() -> Terrain {
        let mut terrain: Terrain = Terrain::empty();
        
        for (_x, row) in terrain.grid.iter_mut().enumerate() {
            for (_y, grid_node) in row.iter_mut().enumerate() {
                grid_node.height = rand::rng().random_range(0.0..10.0);
                grid_node.ground_type = GroundType::random();
            }
        }

        return terrain;
    }
    
    pub fn render(
        &self, 
        target: &mut glium::Frame, 
        program: &glium::Program, 
        display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>,
        camera: &CameraState,
        params: &DrawParameters
    ) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(TERRAIN_GRID_WIDTH * TERRAIN_GRID_WIDTH);
        let mut indices: Vec<u32> = Vec::new();

        for (x, row) in self.grid.iter().enumerate() {
            for (z, grid_node) in row.iter().enumerate() {
                vertices.push(Vertex {
                    position: [x as f32 * 10.0, grid_node.height, z as f32 * 10.0],
                    color: grid_node.ground_type.color()
                });

                if x != TERRAIN_GRID_WIDTH - 1 && z != TERRAIN_GRID_WIDTH - 1 {
                    let top_left = (x * TERRAIN_GRID_WIDTH + z) as u32;
                    let top_right = (x * TERRAIN_GRID_WIDTH + (z + 1)) as u32;
                    let bottom_left = ((x + 1) * TERRAIN_GRID_WIDTH + z) as u32;
                    let bottom_right = ((x + 1) * TERRAIN_GRID_WIDTH + (z + 1)) as u32;

                    // first triangle
                    indices.push(top_left);
                    indices.push(bottom_left);
                    indices.push(bottom_right);

                    // second triangle
                    indices.push(top_left);
                    indices.push(bottom_right);
                    indices.push(top_right);
                }
            }
        }

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
        let uniforms = uniform! {
            view: camera.get_view(),
            perspective: camera.get_perspective()
        };

        target.draw(&vertex_buffer, &index_buffer, program, &uniforms, params).unwrap();
    }
}