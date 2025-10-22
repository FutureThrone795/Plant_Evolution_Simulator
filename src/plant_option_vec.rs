use crate::{plant::Plant, terrain::Terrain};

use glium::{DrawParameters};
use crate::camera::CameraState;

pub struct PlantOptionVec {
    pub internal_vec: Vec<Option<Box<Plant>>>,
    first_none: Option<usize>
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

    pub fn tick(&mut self, terrain: &Terrain) {
        let mut is_plant_deleted = false;

        for (i, item) in self.internal_vec.iter_mut().enumerate() {
            match item {
                Some(plant) => {
                    if !plant.tick(terrain) {
                        //Delete from internal vector if it returns false
                        *item = None;
                        is_plant_deleted = true;

                        match self.first_none {
                            Some(index) => {
                                if i < index {
                                    self.first_none = Some(i)
                                }
                            },
                            None => {
                                self.first_none = Some(i);
                            }
                        }

                        continue;
                    }
                },
                None => ()
            }
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