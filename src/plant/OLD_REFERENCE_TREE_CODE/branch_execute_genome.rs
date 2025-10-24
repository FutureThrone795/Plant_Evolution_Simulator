use crate::plant::branch::{Branch, BranchConnection};
use crate::plant::Plant;
use crate::plant::genome::{self, RuleOutcome};
use crate::terrain::Terrain;

use std::collections::BinaryHeap;
use crate::plant::growth_priority_item::GrowthPriorityItem;

impl Branch {
    // This is fucked
    fn branch_execute_genome<'a>(&'a mut self, growth_priority_heap: &mut BinaryHeap<GrowthPriorityItem<'a>>, depth: usize, plant: &Plant, terrain: &Terrain) {
        let mut rule_index: usize = 0;
        while rule_index < plant.genome.rules.len() {
            let genome_rule = &plant.genome.rules[rule_index];

            match genome_rule.execute(depth, self, plant, terrain) {
                Some(rule_outcome) => match rule_outcome {
                    RuleOutcome::Exit => {
                        break;
                    },
                    RuleOutcome::KillOffshoot(offshoot_selection) => {
                        
                    },
                    RuleOutcome::ChangeSelfProperty { 
                        strength, 
                        photoreceptiveness, 
                        water_intake, 
                        length 
                    } => {

                    },
                    RuleOutcome::RequestNewOffshoot { 
                        priority, 
                        placement_straightness, 
                        strength, 
                        photoreceptiveness, 
                        water_intake, 
                        length 
                    } => {
                        growth_priority_heap.push(
                            GrowthPriorityItem::new_from_branch(
                                self, 
                                BranchConnection { 
                                    branch: Box::new(Branch {
                                        strength: *strength, 
                                        photoreceptiveness: *photoreceptiveness, 
                                        water_intake: *water_intake, 
                                        length: *length, 
                                        offshoot_1: None, 
                                        offshoot_2: None}),
                                    yaw: 0.0,
                                    pitch: 0.0
                                }, 
                                *priority))
                    }
                },
                None => ()
            }
        }
    }
}