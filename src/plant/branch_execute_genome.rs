use rand::random_range;

use crate::plant::{Plant, plant::PLANT_MAX_BRANCH_COUNT};
use crate::plant::genome::{OffshootSelection, RuleOutcome};
use crate::terrain::Terrain;

use std::collections::BinaryHeap;
use crate::plant::growth_priority_item::GrowthPriorityItem;
use crate::render::branch_model::PlantModelMode;

use crate::render::Vertex;
use crate::render::mat4_def::Mat4;

impl Plant {
    pub fn execute_branch_and_update_model_recursive(
        &mut self, homeostasis: &mut f32, 
        branch_index: usize, 
        growth_priority_heap: &mut BinaryHeap<GrowthPriorityItem>, 
        depth: usize, 
        terrain: &Terrain,

        plant_vertices: &mut Vec<Vertex>,
        plant_indices: &mut Vec<u32>,
        matrix: Mat4,

        model_mode: PlantModelMode
    ) {
        *homeostasis += self.branches[branch_index].calculate_homeostasis();

        self.current_sunlight += self.branches[branch_index].calculate_collect_sunlight(depth);
        self.current_water += self.branches[branch_index].calculate_collect_water(depth);

        self.execute_branch_genome(branch_index, growth_priority_heap, depth, terrain);

        let branch_length_real = 1.0 + self.branches[branch_index].length * 10.0;
        match &self.branches[branch_index].offshoot_1 {
            Some(branch_connection) => {
                let offshoot_1_matrix = Mat4::rotation_y(branch_connection.yaw) * 
                                              Mat4::rotation_x(branch_connection.pitch) * 
                                              Mat4::translation(0.0, branch_connection.along_length * branch_length_real, 0.0) * 
                                              matrix.clone();

                self.execute_branch_and_update_model_recursive(homeostasis, branch_connection.branch_index, growth_priority_heap, depth + 1, terrain, plant_vertices, plant_indices, offshoot_1_matrix, model_mode);
            },
            None => ()
        }
        match &self.branches[branch_index].offshoot_2 {
            Some(branch_connection) => {
                let offshoot_2_matrix = Mat4::rotation_y(branch_connection.yaw) * 
                                              Mat4::rotation_x(branch_connection.pitch) * 
                                              Mat4::translation(0.0, branch_connection.along_length * branch_length_real, 0.0) * 
                                              matrix.clone();

                self.execute_branch_and_update_model_recursive(homeostasis, branch_connection.branch_index, growth_priority_heap, depth + 1, terrain, plant_vertices, plant_indices, offshoot_2_matrix, model_mode);
            },
            None => ()
        }
        
        match model_mode {
            PlantModelMode::NoModelUpdate => {
                return;
            }
            _ => {
                self.push_branch_model(branch_index, plant_vertices, plant_indices, matrix, model_mode);
            }
        }
    }   

    fn execute_branch_genome(&mut self, branch_index: usize, growth_priority_heap: &mut BinaryHeap<GrowthPriorityItem>, depth: usize, terrain: &Terrain) {
        for genome_rule in &self.genome.rules {



            // Giant match statement as Rust god (Ferris) intended
            match genome_rule.evaluate(depth, &self.branches[branch_index], self, terrain) {
                Some(rule_outcome) => match rule_outcome {
                    
                    /////////////////////////////////////////////////////////////////////////////////////////////////////
                    // BEGIN MEAT AND POTATOES
                    /////////////////////////////////////////////////////////////////////////////////////////////////////

                    RuleOutcome::Exit => {
                        break;
                    },



                    RuleOutcome::KillOffshoot(offshoot_selection) => {
                        let branch_target_index: usize;
                        let returned_energy: f32;

                        match offshoot_selection {
                            OffshootSelection::One => {
                                match &self.branches[branch_index].offshoot_1 {
                                    Some(connection) => {
                                        returned_energy = self.branches[connection.branch_index].calculate_cost();
                                        branch_target_index = connection.branch_index;
                                    }
                                    None => {
                                        continue;
                                    }
                                }
                            },
                            OffshootSelection::Two => {
                                match &self.branches[branch_index].offshoot_2 {
                                    Some(connection) => {
                                        returned_energy = self.branches[connection.branch_index].calculate_cost();
                                        branch_target_index = connection.branch_index;
                                    }
                                    None => {
                                        continue;
                                    }
                                }
                            }
                        }

                        self.delete_branch_recursive(branch_target_index);
                        self.current_energy += returned_energy;
                        break;
                    },



                    RuleOutcome::RequestModifyBranch { 
                        strength_factor, 
                        photoreceptiveness_factor, 
                        water_intake_factor, 
                        length_factor,
                        priority
                    } => {
                        growth_priority_heap.push(GrowthPriorityItem::new_modify_branch_request(
                            branch_index, 
                            *strength_factor, 
                            *photoreceptiveness_factor, 
                            *water_intake_factor, 
                            *length_factor, 
                            *priority + random_range(-1.0 .. 1.0)
                        ));
                        break;
                    },



                    RuleOutcome::RequestNewOffshoot { 
                        priority,
                        placement_straightness,
                        strength,
                        photoreceptiveness,
                        water_intake,
                        length,
                    } => {
                        if self.branches.len() > PLANT_MAX_BRANCH_COUNT || (self.branches[branch_index].offshoot_1.is_some() && self.branches[branch_index].offshoot_2.is_some()) {
                            continue;
                        }
                        growth_priority_heap.push(GrowthPriorityItem::new_offshoot_request(
                            branch_index, 
                            *placement_straightness, 
                            *strength, 
                            *photoreceptiveness, 
                            *water_intake, 
                            *length, 
                            *priority + random_range(-1.0 .. 1.0)
                        ));
                        break;
                    }

                    /////////////////////////////////////////////////////////////////////////////////////////////////////
                    // END MEAT AND POTATOES
                    /////////////////////////////////////////////////////////////////////////////////////////////////////

                }
                None => ()
            }



        }
    }
}