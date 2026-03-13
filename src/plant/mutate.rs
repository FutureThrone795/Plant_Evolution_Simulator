use std::ops::Range;

use rand::random_range;
use rand::seq::IndexedMutRandom;

use crate::PlantGenome;
use crate::plant::genome::{self, GenomeRule, RuleOutcome, RuleReq};

pub const MUTATION_COUNT_RANGE: Range<i32> = 4..10;

impl RuleOutcome {
    pub fn apply_single_mutation(&mut self) {
        match self {
            RuleOutcome::Exit => {
                return;
            },
            RuleOutcome::KillOffshoot(_) => {
                // Whatever lol
                return;
            },
            RuleOutcome::RequestModifyBranch{
                priority,
                strength_factor, 
                photoreceptiveness_factor, 
                water_intake_factor,
                length_factor
            } => {
                let mut items = [priority, strength_factor, photoreceptiveness_factor, water_intake_factor, length_factor];
                **items.choose_mut(&mut rand::rng()).unwrap() += random_range(-0.5..0.5);
            },
            RuleOutcome::RequestNewOffshoot{
                priority,
        
                placement_upness,
                placement_rightness,
                placement_forwardness,
                placement_randomness,

                strength,
                photoreceptiveness,
                water_intake,
                length
            } => {
                let mut items = [priority, placement_upness, placement_rightness, placement_forwardness, placement_randomness, strength, photoreceptiveness, water_intake, length];
                **items.choose_mut(&mut rand::rng()).unwrap() += random_range(-0.5..0.5);
            }
        }
    }
}

impl GenomeRule {
    pub fn apply_single_mutation(&mut self) {
        let choice = random_range(0..=10);
        match choice {
            0..=2 => {
                self.max += random_range(-0.5..0.5);
            },
            3..=5 => {
                self.min += random_range(-0.5..0.5);
            },
            6..=9 => {
                self.outcome.apply_single_mutation();
            },
            _ => {
                self.req = RuleReq::random();
            }
        }
    }
}

impl PlantGenome {
    pub fn apply_single_mutation(&mut self) {
        let choice = random_range(0..=20);
        match choice {
            0 => {
                self.min_enegy_for_growth += random_range(-0.5..0.5);
            },
            1 => {
                self.baby_energy += random_range(-0.5..0.5);
            },
            2 => {
                self.sapling_strength += random_range(-0.5..0.5);
            },
            3 => {
                self.sapling_photoreceptiveness += random_range(-0.5..0.5);
            },
            4 => {
                self.sapling_water_intake += random_range(-0.5..0.5);
            },
            5 => {
                self.sapling_length += random_range(-0.5..0.5)
            },
            6 => {
                let current_rule_count = self.rules.len();
                let random_genome_rule_index = random_range(..current_rule_count);
                if current_rule_count == genome::MAX_GENOME_RULE_COUNT {
                    // Already at the maximum number of rules
                    // Let's remove one just to keep things clean
                    self.rules.remove(random_genome_rule_index);
                    return;
                }
                self.rules.insert(random_genome_rule_index, GenomeRule::random());
            },
            7 => {
                let random_genome_rule_index = random_range(..self.rules.len());
                self.rules.remove(random_genome_rule_index);
            },
            _ => {
                self.rules.choose_mut(&mut rand::rng()).expect("Random selection of genome rules returned None in PlantGenome::apply_single_mutation()").apply_single_mutation();
            }

        }
    }

    pub fn apply_random_mutations(&mut self) {
        for _ in 0..random_range(MUTATION_COUNT_RANGE) {
            self.apply_single_mutation();
        }
    }
}