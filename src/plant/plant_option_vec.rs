use std::fmt::{Debug, Formatter};

use crate::plant::Plant;
use crate::terrain::Terrain;

use glium::{DrawParameters};
use crate::render::camera::CameraState;

pub struct PlantOptionVec {
    pub internal_vec: Vec<Option<Box<Plant>>>,
    first_none: Option<usize>
}

impl Debug for PlantOptionVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        return write!(f, "{:?}", self.internal_vec);
    }
}

impl PlantOptionVec {
    pub fn new() -> PlantOptionVec {
        return PlantOptionVec { internal_vec: vec![], first_none: None }
    }

    fn move_first_move_up(&mut self) {
        match self.first_none {
            Some(mut index) => {
                while self.internal_vec[index].is_some() {
                    index += 1;
                    if index + 1 > self.internal_vec.len() {
                        self.first_none = None;
                        return;
                    }
                }
                self.first_none = Some(index);
            },
            None => ()
        }
    }
    
    pub fn add_plant(&mut self, plant: Plant) {
        match self.first_none {
            Some(index) => {
                self.internal_vec[index] = Some(Box::new(plant));
                self.move_first_move_up();
            },
            None => {
                self.internal_vec.push(Some(Box::new(plant)));
            }
        };
    }

    pub fn tick(&mut self, terrain: &Terrain, total_ticks: u64, display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>) {
        let mut is_plant_deleted = false;
        static PLANT_TICK_MOD: u64 = 10;

        let mut i = total_ticks.rem_euclid(PLANT_TICK_MOD);
        while i < self.internal_vec.len() as u64 {
            let item = &mut self.internal_vec[i as usize];
            
            match item {
                Some(plant) => {
                    if !plant.tick(terrain, display) {
                        //Delete from internal vector if it returns false
                        *item = None;
                        is_plant_deleted = true;

                        match self.first_none {
                            Some(index) => {
                                if i < index as u64 {
                                    self.first_none = Some(i as usize)
                                }
                            },
                            None => {
                                self.first_none = Some(i as usize);
                            }
                        }

                        continue;
                    }
                },
                None => ()
            }

            i += PLANT_TICK_MOD;
        }

        if is_plant_deleted {
            println!("Plant deleted, new PlantOptionVec: {:?}", self.internal_vec);
        }
    }

    pub fn render(
        &self, 
        target: &mut glium::Frame, 
        program: &glium::Program, 
        display: &glium::backend::glutin::Display<glium::glutin::surface::WindowSurface>,
        camera: &CameraState,
        params: &DrawParameters
    ) {
        for item in &self.internal_vec {
            match item {
                Some(plant) => {
                    plant.render(target, program, display, camera, params);
                }
                None => ()
            }
        }
    }
}