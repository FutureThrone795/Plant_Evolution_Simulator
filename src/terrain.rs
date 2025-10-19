pub const TERRAIN_GRID_ROWS: usize = 100;
pub const TERRAIN_CELL_WIDTH: f32 = 10.0;
pub const WORLD_WIDTH: f32 = TERRAIN_GRID_ROWS as f32 * TERRAIN_CELL_WIDTH;

use crate::{camera::CameraState, Vertex};

use glium::{DrawParameters, Surface};

use rand::Rng;
use noise::Perlin;
use crate::noise::NoiseFn;

use crate::generate_terrain_mesh::generate_terrain_mesh;

fn perlin_util(seed: u32, x: f32, y: f32, multiplier: f32, min: f32, max: f32) -> f32 {
    return ((Perlin::new(seed).get([(x * multiplier) as f64, (y * multiplier) as f64]) as f32 + 1.0) / 2.0) * (max - min) + min;
}

#[derive(Clone, Copy)]
pub enum GroundType {
    Grass,
    Rock,
    Sand,
    Swamp,
    Snow
}

impl GroundType {
    pub fn color(&self) -> [f32; 4] {
        match self {
            Self::Grass => [0.15, 0.65, 0.20, 1.0],
            Self::Rock  => [0.50, 0.50, 0.52, 1.0],
            Self::Sand  => [0.90, 0.85, 0.45, 1.0],
            Self::Swamp => [0.55, 0.30, 0.10, 1.0],
            Self::Snow  => [0.98, 1.00, 0.98, 1.0]
        }
    }

    pub fn random() -> GroundType {
        match rand::rng().random_range(0..5) {
            0 => Self::Grass,
            1 => Self::Rock,
            2 => Self::Sand,
            3 => Self::Swamp,
            4 => Self::Snow,

            _ => Self::Grass //Grass is the default variant when something goes wrong
        }
    }

    pub fn from_grid_node(grid_node: &TerrainGridNode) -> GroundType {
        if grid_node.height > 38.0 && grid_node.dryness < 0.6 {
            return GroundType::Snow;
        }
        if grid_node.height > 30.0 || grid_node.rockiness > 0.8 {
            return GroundType::Rock;
        }
        if grid_node.dryness > 0.7 || (grid_node.dryness > 0.6 && grid_node.height < 0.0) {
            return GroundType::Sand;
        }
        if (grid_node.dryness < 0.3 && grid_node.height < 15.0) || (grid_node.dryness < 0.4 && grid_node.height < 0.0) {
            return GroundType::Swamp;
        } 
        if grid_node.height < -1.0 {
            return GroundType::Sand;
        }

        return GroundType::Grass;
    }
}

#[derive(Clone, Copy)]
pub struct TerrainGridNode {
    pub height: f32,
    pub local_height: f32,
    pub ground_type: GroundType,

    pub dryness: f32,
    pub rockiness: f32
}

pub struct Terrain {
    pub grid: [[TerrainGridNode; TERRAIN_GRID_ROWS]; TERRAIN_GRID_ROWS],
    pub water_height: f32,

    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub water_vertices: Vec<Vertex>,
    pub water_indices: Vec<u32>
}

impl Terrain {
    pub fn empty() -> Terrain {
        return Terrain {grid: [[TerrainGridNode {
            height: 0.0,
            local_height: 0.0,
            ground_type: GroundType::Grass,

            dryness: 0.0,
            rockiness: 0.0,
        }; TERRAIN_GRID_ROWS]; TERRAIN_GRID_ROWS],
        water_height: 0.0,
        vertices: vec![],
        indices: vec![],
        water_vertices: vec![],
        water_indices: vec![],
        };
    }

    pub fn random() -> Terrain {
        let mut terrain: Terrain = Terrain::empty();

        for (x, row) in terrain.grid.iter_mut().enumerate() {
            for (y, grid_node) in row.iter_mut().enumerate() {
                let x_pos: f32 = 0.5 * x as f32;
                let y_pos: f32 = 0.5 * y as f32;

                grid_node.dryness = perlin_util(0, x_pos, y_pos, 0.2, 0.0, 1.0);
                grid_node.rockiness = perlin_util(1, x_pos, y_pos, 0.3, 0.0, 1.0);

                grid_node.height =  (grid_node.dryness * grid_node.rockiness) * perlin_util(4, x_pos, y_pos, 3.0, -10.0, 30.0) + 
                                    perlin_util(5, x_pos, y_pos, 0.1, -40.0, 40.0) + 
                                    perlin_util(6, x_pos, y_pos, 0.5, -10.0, 10.0) +
                                    20.0;

                grid_node.ground_type = GroundType::random();
            }
        }

        for x in 0..TERRAIN_GRID_ROWS {
            for y in 0..TERRAIN_GRID_ROWS {
                let mut local_max = terrain.grid[x][y].height;
                let mut local_min = terrain.grid[x][y].height;

                let signed_x = x as isize;
                let signed_y = x as isize;

                for local_x in (signed_x - 1) .. (signed_x + 1) {
                    for local_y in (signed_y - 1) .. (signed_y + 1) {
                        let local_x_mapped = local_x.rem_euclid(TERRAIN_GRID_ROWS as isize) as usize;
                        let local_y_mapped = local_y.rem_euclid(TERRAIN_GRID_ROWS as isize) as usize;

                        let local_mapped_height = terrain.grid[local_x_mapped][local_y_mapped].height;

                        if local_max < local_mapped_height {
                            local_max = local_mapped_height;
                        }
                        if local_min > local_mapped_height {
                            local_min = local_mapped_height;
                        }
                    }
                }

                terrain.grid[x][y].local_height = local_max - local_min;
                terrain.grid[x][y].ground_type = GroundType::from_grid_node(&terrain.grid[x][y]);
            }
        }

        (terrain.vertices, terrain.indices, terrain.water_vertices, terrain.water_indices) = generate_terrain_mesh(&terrain);

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
        let vertex_buffer = glium::VertexBuffer::new(display, &self.vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap();

        let water_vertex_buffer = glium::VertexBuffer::new(display, &self.water_vertices).unwrap();
        let water_index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.water_indices).unwrap();     

        for x in -1..2 {
            for z in -1..2 {
                let uniforms = uniform! {
                    view: camera.get_view(),
                    perspective: camera.get_perspective(),
                    offset: [WORLD_WIDTH * x as f32, 0.0, WORLD_WIDTH * z as f32]
                };

                target.draw(&vertex_buffer, &index_buffer, program, &uniforms, params).unwrap();
            }
        }
        for x in -1..2 {
            for z in -1..2 {
                let uniforms = uniform! {
                    view: camera.get_view(),
                    perspective: camera.get_perspective(),
                    offset: [WORLD_WIDTH * x as f32, self.water_height, WORLD_WIDTH * z as f32]
                };

                target.draw(&water_vertex_buffer, &water_index_buffer, program, &uniforms, params).unwrap();
            }
        }
    }
}