pub const TERRAIN_GRID_ROWS: usize = 100;
pub const TERRAIN_CELL_WIDTH: f32 = 10.0;
pub const WORLD_WIDTH: f32 = TERRAIN_GRID_ROWS as f32 * TERRAIN_CELL_WIDTH;
pub const TERRAIN_RENDER_SPREAD: i32 = 1;

use std::f32::consts::PI;

use crate::{camera::CameraState, Vertex};

use glium::{DrawParameters, Surface};

use rand::{rng, Rng, RngCore};
use noise::Perlin;
use crate::noise::NoiseFn;

use crate::generate_terrain_mesh::generate_terrain_mesh;

#[inline]
fn map_helper(val: f32, curr_min: f32, curr_max: f32, new_min: f32, new_max: f32) -> f32 {
    return (val - curr_min) / (curr_max - curr_min) * (new_max - new_min) + new_min;
}

fn old_perlin_helper(seed: u32, x: f32, y: f32, multiplier: f32, min: f32, max: f32) -> f32 {
    return ((Perlin::new(seed).get([(x * multiplier) as f64, (y * multiplier) as f64]) as f32 + 1.0) / 2.0) * (max - min) + min;
}

//Tileable version
fn perlin_helper(perlin: Perlin, seed: u32, x: f32, y: f32, multiplier: f32, min: f32, max: f32) -> f32 {
    let s: f32 = x / TERRAIN_GRID_ROWS as f32;
    let t: f32 = y / TERRAIN_GRID_ROWS as f32;

    //Changes the multiplier to work in 4-space
    let scale_factor_4d = TERRAIN_GRID_ROWS as f32 * multiplier / (2.0 * PI);

    let nx = seed as f32 + (s * 2.0 * PI).cos() * scale_factor_4d;
    let ny = seed as f32 - (t * 2.0 * PI).cos() * scale_factor_4d;
    let nz = seed as f32 + (s * 2.0 * PI).sin() * scale_factor_4d;
    let nw = seed as f32 - (t * 2.0 * PI).sin() * scale_factor_4d;

    let sample = perlin.get([nx as f64, ny as f64, nz as f64, nw as f64]) as f32;

    //To counteract the higher-dimensional smoothing artifacts
    let contrast_curve = sample.signum() * sample.abs().powf(0.7);

    return map_helper(contrast_curve, -1.0, 1.0, min, max);
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
            Self::Snow  => [0.95, 0.98, 0.95, 1.0]
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
        if grid_node.height > 55.0 && (grid_node.dryness < 0.6 || grid_node.local_height > 7.0) && grid_node.local_height > -2.0 {
            return GroundType::Snow;
        }
        if grid_node.height > 45.0 || grid_node.rockiness > 0.8 {
            return GroundType::Rock;
        }
        if grid_node.dryness > 0.6 || (grid_node.dryness > 0.4 && grid_node.height < 0.0) {
            return GroundType::Sand;
        }
        if (grid_node.dryness < 0.3 && grid_node.height < 25.0) || (grid_node.dryness < 0.4 && grid_node.height < 5.0) || grid_node.dryness < 0.1 {
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
    pub gradient: (f32, f32),
    pub ground_type: GroundType,

    pub dryness: f32,
    pub rockiness: f32
}

pub struct Terrain {
    pub grid: Box<[[TerrainGridNode; TERRAIN_GRID_ROWS]; TERRAIN_GRID_ROWS]>,
    pub water_height: f32,

    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub water_vertices: Vec<Vertex>,
    pub water_indices: Vec<u32>
}

impl Terrain {
    pub fn get_height(&self, x: f32, z: f32) -> f32 {
        let x_mapped = (x as f32).rem_euclid(TERRAIN_GRID_ROWS as f32);
        let z_mapped = (z as f32).rem_euclid(TERRAIN_GRID_ROWS as f32);

        let neg_x_index = x_mapped.floor() as usize;
        let pos_x_index = (neg_x_index + 1).rem_euclid(TERRAIN_GRID_ROWS);
        let x_offset = x_mapped.rem_euclid(1.0);

        let neg_z_index = z_mapped.floor() as usize;
        let pos_z_index = (neg_z_index + 1).rem_euclid(TERRAIN_GRID_ROWS);
        let z_offset = z_mapped.rem_euclid(1.0);

        let pos_x_pos_z_height = self.grid[pos_x_index][pos_z_index].height;
        let neg_x_pos_z_height = self.grid[neg_x_index][pos_z_index].height;
        let pos_x_neg_z_height = self.grid[pos_x_index][neg_z_index].height;
        let neg_x_neg_z_height = self.grid[neg_x_index][neg_z_index].height;
        
        return  pos_x_pos_z_height*(x_offset)*(z_offset) + 
                neg_x_pos_z_height*(1.0-x_offset)*(z_offset) +
                pos_x_neg_z_height*(x_offset)*(1.0-z_offset) +
                neg_x_neg_z_height*(1.0-x_offset)*(1.0-z_offset);
    }

    pub fn empty() -> Terrain {
        return Terrain {grid: Box::new([[TerrainGridNode {
            height: 0.0,
            local_height: 0.0,
            gradient: (0.0, 0.0),
            ground_type: GroundType::Grass,

            dryness: 0.0,
            rockiness: 0.0,
        }; TERRAIN_GRID_ROWS]; TERRAIN_GRID_ROWS]),
        water_height: 0.0,
        vertices: vec![],
        indices: vec![],
        water_vertices: vec![],
        water_indices: vec![],
        };
    }

    pub fn random() -> Terrain {
        let mut terrain: Terrain = Terrain::empty();
        let perlin: Perlin = Perlin::new(rand::prelude::SmallRng::from(rand::SeedableRng::from_os_rng()).next_u32());

        for (x, row) in terrain.grid.iter_mut().enumerate() {
            for (y, grid_node) in row.iter_mut().enumerate() {
                let x_pos: f32 = x as f32;
                let y_pos: f32 = y as f32;

                grid_node.dryness = perlin_helper(perlin, 0, x_pos, y_pos, 0.07, 0.0, 1.0);
                grid_node.rockiness = perlin_helper(perlin, 1, x_pos, y_pos, 0.2, 0.0, 1.0);

                grid_node.height =  ((grid_node.dryness * grid_node.rockiness) * perlin_helper(perlin, 4, x_pos, y_pos, 1.5, 30.0, 80.0) + 
                                    perlin_helper(perlin, 5, x_pos, y_pos, 0.05, -70.0, 100.0) + 
                                    perlin_helper(perlin, 6, x_pos, y_pos, 0.25, -10.0, 10.0) +
                                    perlin_helper(perlin, 7, x_pos, y_pos, 0.15, -20.0, 20.0) +
                                    if grid_node.dryness < 0.3 { -5.0 } else { 0.0 } +
                                    20.0) * 0.6;

                //grid_node.height = perlin_helper(50, x_pos, y_pos, 0.07, 0.0, 500.0);
                //grid_node.height = old_perlin_helper(50, x_pos, y_pos, 0.07, 0.0, 500.0);

                if grid_node.height > 5.0 {
                    if grid_node.dryness < 0.4 {
                        grid_node.height -= 5.0;
                    }
                } else {
                    grid_node.height -= 20.0;
                }
                
                grid_node.ground_type = GroundType::random();
            }
        }

        for x in 0..TERRAIN_GRID_ROWS {
            for z in 0..TERRAIN_GRID_ROWS {
                let mut x_grad: f32 = 0.0;
                let mut z_grad: f32 = 0.0;
                let mut local_average: f32 = 0.0;

                let signed_x = x as isize;
                let signed_z = z as isize;

                for local_x in (signed_x - 1) .. (signed_x + 2) {
                    for local_z in (signed_z - 1) .. (signed_z + 2) {
                        let local_x_mapped = local_x.rem_euclid(TERRAIN_GRID_ROWS as isize) as usize;
                        let local_z_mapped = local_z.rem_euclid(TERRAIN_GRID_ROWS as isize) as usize;

                        let local_mapped_height = terrain.grid[local_x_mapped][local_z_mapped].height;

                        let x_offset: isize = local_x - signed_x;
                        let z_offset: isize = local_z - signed_z;

                        let dist: f32 = ((x_offset as f32).powi(2) + (z_offset as f32).powi(2)).sqrt();

                        local_average += local_mapped_height;

                        if dist != 0.0 {
                            x_grad += local_mapped_height * (x_offset.signum() as f32) / dist;
                            z_grad += local_mapped_height * (z_offset.signum() as f32) / dist;
                        }
                    }
                }

                local_average /= 9.0; // 3x3 square

                terrain.grid[x][z].local_height = terrain.grid[x][z].height - local_average;
                terrain.grid[x][z].ground_type = GroundType::from_grid_node(&terrain.grid[x][z]);
                terrain.grid[x][z].gradient = (x_grad, z_grad);
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

        for x in (-TERRAIN_RENDER_SPREAD)..(TERRAIN_RENDER_SPREAD + 1) {
            for z in (-TERRAIN_RENDER_SPREAD)..(TERRAIN_RENDER_SPREAD + 1) {
                let uniforms = uniform! {
                    view: camera.get_view(),
                    perspective: camera.get_perspective(),
                    offset: [WORLD_WIDTH * x as f32, 0.0, WORLD_WIDTH * z as f32]
                };

                target.draw(&vertex_buffer, &index_buffer, program, &uniforms, params).unwrap();
            }
        }
        for x in (-TERRAIN_RENDER_SPREAD)..(TERRAIN_RENDER_SPREAD + 1) {
            for z in (-TERRAIN_RENDER_SPREAD)..(TERRAIN_RENDER_SPREAD + 1) {
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