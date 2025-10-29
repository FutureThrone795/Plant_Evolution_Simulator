use std::fmt::{Debug, Formatter};

use glium::DrawParameters;
use crate::glium::Surface;

use crate::render::camera::CameraState;
use crate::render::mat4_def::Mat4;
use crate::terrain::{Terrain, TERRAIN_CELL_WIDTH};
use crate::plant::growth_priority_item::GrowthPriorityItem;
use crate::plant::genome::PlantGenome;
use crate::plant::branch::{Branch, BranchConnection};
use crate::render::Vertex;

use std::collections::BinaryHeap;

pub struct Plant {
    pub genome: PlantGenome,
    pub age_ticks: u64,

    pub root_position: (f32, f32, f32),
    pub branches: Vec<Branch>,
    
    pub current_energy: f32,
    pub current_water: f32,
    pub current_sunlight: f32,

    pub cached_model: Option<(glium::VertexBuffer<Vertex>, glium::IndexBuffer<u32>)>
}

impl Debug for Plant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        return write!(f, "Plant: (Branch count: {:?}, Energy: {})", self.branches.len(), self.current_energy)
    }
}

impl Plant {
    pub fn tick(&mut self, terrain: &Terrain, display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>, is_ldm: bool) -> bool {
        //Returns false when the plant has died and should be removed

        self.age_ticks += 1;

        let mut homeostasis: f32 = 2.0;
        let mut growth_priority_heap: BinaryHeap<GrowthPriorityItem> = BinaryHeap::new();

        let matrix = Mat4::identity();

        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];

        self.execute_branch_and_update_model_recursive(&mut homeostasis, 0, &mut growth_priority_heap, 0, terrain, &mut vertices, &mut indices, matrix, is_ldm);

        self.cached_model = Some((
            glium::VertexBuffer::new(display, &vertices).unwrap(), 
            glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap()
        ));



        if self.current_water > self.current_sunlight {
            self.current_energy += self.current_water;
            self.current_sunlight -= self.current_water;
            self.current_water = 0.0;
        } else {
            self.current_energy += self.current_sunlight;
            self.current_water -= self.current_sunlight;
            self.current_sunlight = 0.0;
        }
        
        self.current_energy -= homeostasis;

        if self.current_energy < 0.0 {
            return false;
        }

        while !growth_priority_heap.is_empty() && self.current_energy > self.genome.min_enegy_for_growth {
            if self.current_energy - growth_priority_heap.peek().unwrap().cost() > self.genome.min_enegy_for_growth {
                let growth_priority_item: GrowthPriorityItem = growth_priority_heap.pop().unwrap();
                let new_index: usize = self.branches.len();
                let new_offshoot = BranchConnection::new(&growth_priority_item, new_index);
                
                self.branches.push(Branch::from(&growth_priority_item));
                self.branches[growth_priority_item.parent_branch_index].add_offshoot(new_offshoot);
            } else {
                break;
            }
        }

        return true;
    }

    pub fn delete_branch_recursive(&mut self, branch_index: usize) {
        match &self.branches[branch_index].offshoot_1 {
            Some(connection) => {
                self.delete_branch_recursive(connection.branch_index);
            }
            None => ()
        }
        match &self.branches[branch_index].offshoot_2 {
            Some(connection) => {
                self.delete_branch_recursive(connection.branch_index);
            }
            None => ()
        }
        self.branches.remove(branch_index);
    }

    pub fn render(
        &self, 
        target: &mut glium::Frame, 
        program: &glium::Program, 
        display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>,
        camera: &CameraState,
        params: &DrawParameters
    ) {
        match &self.cached_model {
            Some((vertex_buffer, index_buffer)) => {
                let uniforms = uniform! {
                    view: camera.get_view(),
                    perspective: camera.get_perspective().0,
                    model: Mat4::translation(self.root_position.0 * TERRAIN_CELL_WIDTH, self.root_position.1, self.root_position.2 * TERRAIN_CELL_WIDTH).0
                };

                target.draw(vertex_buffer, index_buffer, program, &uniforms, params).unwrap();
            }
            None => ()
        }
        
    }

    pub fn new (genome: PlantGenome, x: f32, z: f32, starting_energy: f32, terrain: &Terrain) -> Plant {
        return Plant {
            branches: vec![Branch::new(genome.sapling_strength, genome.sapling_photoreceptiveness, genome.sapling_water_intake, genome.sapling_length)],
            age_ticks: 0,

            genome: genome,
            root_position: (x, terrain.get_height(x, z), z),
            current_energy: starting_energy,
            current_sunlight: 0.0,
            current_water: 0.0,

            cached_model: None
        }
    }
}