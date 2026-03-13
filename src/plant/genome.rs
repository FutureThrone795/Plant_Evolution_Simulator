use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::plant::branch::Branch;
use crate::plant::Plant;
use crate::terrain::Terrain;

use crate::rand::Rng;

pub const MAX_GENOME_RULE_COUNT: usize = 8;

pub enum OffshootSelection {
    One,
    Two
}

#[derive(EnumIter)]
pub enum RuleReq {
    BranchDepthReq,
    BranchStrengthReq,
    BranchPhotoreceptivenessReq,
    BranchWaterIntakeReq,
    BranchLengthReq,

    PlantEnergyReq,
    PlantWaterReq,
    PlantSunlightReq,
    PlantBranchReq,

    TerrainHeightReq,
    TerrainDrynessReq,
    TerrainRockinessReq,
    TerrainWaterlog
}

impl RuleReq {
    pub fn random() -> RuleReq {
        return RuleReq::iter().choose(&mut rand::rng()).expect("Random RuleReq selection returned None in RuleReq::random()");
    }
}

pub enum RuleOutcome {
    Exit,
    KillOffshoot(OffshootSelection),
    RequestModifyBranch{
        priority: f32,
        strength_factor: f32, 
        photoreceptiveness_factor: f32, 
        water_intake_factor: f32,
        length_factor: f32
    },
    RequestNewOffshoot{
        priority: f32,
        
        placement_upness: f32,
        placement_rightness: f32,
        placement_forwardness: f32,
        placement_randomness: f32,

        strength: f32,
        photoreceptiveness: f32,
        water_intake: f32,
        length: f32,
    }
}

pub struct GenomeRule {
    pub req: RuleReq,
    pub min: f32,
    pub max: f32,
    pub outcome: RuleOutcome
}

impl GenomeRule {
    pub fn evaluate(&self, depth: usize, branch: &Branch, plant: &Plant, terrain: &Terrain) -> Option<&RuleOutcome> {
        let comp_val = match self.req {
            RuleReq::BranchDepthReq => depth as f32,
            RuleReq::BranchStrengthReq => branch.strength,
            RuleReq::BranchPhotoreceptivenessReq => branch.photoreceptiveness,
            RuleReq::BranchWaterIntakeReq => branch.water_intake,
            RuleReq::BranchLengthReq => branch.length,

            RuleReq::PlantEnergyReq => plant.current_energy,
            RuleReq::PlantWaterReq => plant.current_water,
            RuleReq::PlantSunlightReq => plant.current_sunlight,
            RuleReq::PlantBranchReq => plant.branches.len() as f32,

            RuleReq::TerrainHeightReq => plant.root_position.1,
            RuleReq::TerrainDrynessReq => terrain.get_dryness(plant.root_position.0, plant.root_position.2),
            RuleReq::TerrainRockinessReq => terrain.get_rockiness(plant.root_position.0, plant.root_position.2),
            RuleReq::TerrainWaterlog => todo!()
        };

        if self.min <= comp_val && comp_val <= self.max {
            return Some(&self.outcome);
        }
        return None;
    }

    pub fn random() -> GenomeRule {
        return GenomeRule { 
            req: RuleReq::BranchDepthReq, 
            min: rand::rng().random_range(0.0 .. 20.0), 
            max: rand::rng().random_range(0.0 .. 20.0), 
            outcome: RuleOutcome::Exit //TEMP 
        }
    }
}

pub struct PlantGenome {
    pub min_enegy_for_growth: f32,
    pub baby_energy: f32,

    pub sapling_strength: f32,
    pub sapling_photoreceptiveness: f32,
    pub sapling_water_intake: f32,
    pub sapling_length: f32,

    pub rules: Vec<GenomeRule>
}

impl PlantGenome {
    pub fn random() -> PlantGenome {
        return PlantGenome { 
            min_enegy_for_growth: 10.0, 
            baby_energy: 100.0, 

            sapling_strength: 0.0, 
            sapling_photoreceptiveness: 0.0, 
            sapling_water_intake: 0.0, 
            sapling_length: 10.0,

            rules: vec![GenomeRule::random(), GenomeRule::random(), GenomeRule::random(), GenomeRule::random()] 
        }
    }
    pub fn human_made_tree_genome() -> PlantGenome {
        return PlantGenome { 
            min_enegy_for_growth: 10.0, 
            baby_energy: 80.0, 

            sapling_strength: 0.3, 
            sapling_photoreceptiveness: 0.9, 
            sapling_water_intake: 0.8, 
            sapling_length: 0.5,

            rules: vec![
                GenomeRule {
                    req: RuleReq::BranchDepthReq,
                    min: 6.5,
                    max: 10.0,
                    outcome: RuleOutcome::Exit
                },
                GenomeRule {
                    req: RuleReq::BranchDepthReq,
                    min: -1.0,
                    max: 4.5,
                    outcome: RuleOutcome::RequestNewOffshoot { 
                        priority: 10.0,

                        placement_upness: 0.0,
                        placement_rightness: 0.0,
                        placement_forwardness: 1.0,
                        placement_randomness: 0.1, 

                        strength: 0.3,
                        photoreceptiveness: 1.0,
                        water_intake: 0.5,
                        length: 0.3,
                    }
                },
                GenomeRule {
                    req: RuleReq::BranchDepthReq,
                    min: 4.5,
                    max: 10.0,
                    outcome: RuleOutcome::RequestNewOffshoot { 
                        priority: 5.0, 
                        
                        placement_upness: 0.1,
                        placement_rightness: 0.0,
                        placement_forwardness: 0.5,
                        placement_randomness: 1.0,

                        strength: 0.1,
                        photoreceptiveness: 1.0,
                        water_intake: 0.1,
                        length: 0.1,
                    }
                },
                GenomeRule {
                    req: RuleReq::BranchDepthReq,
                    min: 2.5,
                    max: 99.0,
                    outcome: RuleOutcome::Exit
                },
                GenomeRule {
                    req: RuleReq::PlantBranchReq,
                    min: 40.5,
                    max: 99.0,
                    outcome: RuleOutcome::RequestModifyBranch { 
                        priority: 5.0,
                        strength_factor: 0.5, 
                        photoreceptiveness_factor: -1.0, 
                        water_intake_factor: 0.3, 
                        length_factor: 0.8
                    }
                },
                GenomeRule {
                    req: RuleReq::PlantWaterReq,
                    min: 0.0,
                    max: 30.0,
                    outcome: RuleOutcome::RequestModifyBranch { 
                        priority: 4.9,
                        strength_factor: 0.2, 
                        photoreceptiveness_factor: -0.3, 
                        water_intake_factor: 0.5, 
                        length_factor: 0.5
                    }
                }
            ] 
        }
    }
}