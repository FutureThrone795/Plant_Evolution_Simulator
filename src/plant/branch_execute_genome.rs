use crate::plant::{self, Plant, branch};
use crate::plant::genome::{OffshootSelection, RuleOutcome};
use crate::terrain::Terrain;

use std::collections::BinaryHeap;
use crate::plant::growth_priority_item::GrowthPriorityItem;

use crate::render::Vertex;
use crate::render::mat4_def::Mat4;

fn modify_self_property_helper(original_val: f32, change_factor: f32) -> f32 {
    //Original val must be between 0.0 and 1.0, change factor changes this - positive go up, negative go down, kinda lmao its not a science idk go graph it yourself its weird
    return (original_val + (change_factor / 10.0).tanh()).tanh();
}

fn modify_self_length_property_helper(original_len: f32, change_factor: f32) -> f32 {
    return 10.0 * modify_self_property_helper(original_len / 10.0, change_factor);
}

impl Plant {
    pub fn execute_branch_and_update_model_recursive(
        &mut self, homeostasis: &mut f32, 
        branch_index: usize, 
        growth_priority_heap: &mut BinaryHeap<GrowthPriorityItem>, 
        depth: usize, 
        terrain: &Terrain,

        plant_vertices: &mut Vec<Vertex>,
        plant_indices: &mut Vec<u32>,
        matrix: Mat4
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

                self.execute_branch_and_update_model_recursive(homeostasis, branch_connection.branch_index, growth_priority_heap, depth + 1, terrain, plant_vertices, plant_indices, offshoot_1_matrix);
            },
            None => ()
        }
        match &self.branches[branch_index].offshoot_2 {
            Some(branch_connection) => {
                let offshoot_2_matrix = Mat4::rotation_y(branch_connection.yaw) * 
                                              Mat4::rotation_x(branch_connection.pitch) * 
                                              Mat4::translation(0.0, branch_connection.along_length * branch_length_real, 0.0) * 
                                              matrix.clone();

                self.execute_branch_and_update_model_recursive(homeostasis, branch_connection.branch_index, growth_priority_heap, depth + 1, terrain, plant_vertices, plant_indices, offshoot_2_matrix);
            },
            None => ()
        }
        
        self.render_branch(branch_index, plant_vertices, plant_indices, matrix);
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



                    RuleOutcome::ChangeSelfProperty { 
                        strength_factor, 
                        photoreceptiveness_factor, 
                        water_intake_factor, 
                        length_factor 
                    } => {
                        self.branches[branch_index].strength = modify_self_property_helper(self.branches[branch_index].strength, *strength_factor);
                        self.branches[branch_index].photoreceptiveness = modify_self_property_helper(self.branches[branch_index].photoreceptiveness, *photoreceptiveness_factor);
                        self.branches[branch_index].water_intake = modify_self_property_helper(self.branches[branch_index].water_intake, *water_intake_factor);
                        self.branches[branch_index].length = modify_self_property_helper(self.branches[branch_index].length, *length_factor);
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
                        if self.branches[branch_index].offshoot_1.is_some() && self.branches[branch_index].offshoot_2.is_some() {
                            // If a branch attempts to grow a new offshoot and fails, it exits the genome evaluation process
                            break;
                        }
                        growth_priority_heap.push(GrowthPriorityItem::new(branch_index, *placement_straightness, *strength, *photoreceptiveness, *water_intake, *length, *priority));
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