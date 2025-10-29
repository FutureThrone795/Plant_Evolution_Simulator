use std::rc::Rc;

use crate::plant::branch::Branch;
use crate::plant::Plant;
use crate::terrain::Terrain;

use crate::rand::Rng;

const MAX_GENOME_RULE_COUNT: usize = 8;

struct GenomeVal {
    val: f32,
    rand_factor: f32
}

pub enum OffshootSelection {
    One,
    Two
}

enum RuleReq {
    BranchDepthReq,
    BranchStrengthReq,
    BranchPhotoreceptivenessReq,
    BranchWaterIntakeReq,
    BranchLengthReq,

    PlantEnergyReq,
    PlantWaterReq,
    PlantSunlightReq,

    TerrainHeightReq,
    TerrainDrynessReq,
    TerrainRockinessReq,
    TerrainWaterlog
}

pub enum RuleOutcome {
    Exit,
    //JumpToRule(usize), //Removed for concerns of infinite loops
    KillOffshoot(OffshootSelection),
    ChangeSelfProperty{
        strength_factor: f32, 
        photoreceptiveness_factor: f32, 
        water_intake_factor: f32,
        length_factor: f32
    },
    RequestNewOffshoot{
        priority: f32,
        placement_straightness: f32,
        strength: f32,
        photoreceptiveness: f32,
        water_intake: f32,
        length: f32,
    }
}

pub struct GenomeRule {
    req: RuleReq,
    min: f32,
    max: f32,
    outcome: RuleOutcome
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

            RuleReq::TerrainHeightReq => plant.root_position.1,
            RuleReq::TerrainDrynessReq => 0.0,
            RuleReq::TerrainRockinessReq => 0.0,
            RuleReq::TerrainWaterlog => 0.0
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
    pub make_baby_req_energy: f32,

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
            make_baby_req_energy: 100.0, 

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
            make_baby_req_energy: 100.0, 

            sapling_strength: 0.3, 
            sapling_photoreceptiveness: 0.9, 
            sapling_water_intake: 0.5, 
            sapling_length: 0.5,

            rules: vec![
                GenomeRule {
                    req: RuleReq::BranchDepthReq,
                    min: 6.5,
                    max: 10.0,
                    outcome: RuleOutcome::Exit
                },
                GenomeRule {
                    req: RuleReq::PlantEnergyReq,
                    min: -1.0,
                    max: 5.0,
                    outcome: RuleOutcome::Exit
                },
                GenomeRule {
                    req: RuleReq::BranchDepthReq,
                    min: -1.0,
                    max: 4.5,
                    outcome: RuleOutcome::RequestNewOffshoot { 
                        priority: 10.0,
                        placement_straightness: 0.5, 
                        strength: 0.3,
                        photoreceptiveness: 0.9,
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
                        placement_straightness: 0.0,
                        strength: 0.1,
                        photoreceptiveness: 1.0,
                        water_intake: 0.1,
                        length: 0.1,
                    }
                },
                GenomeRule {
                    req: RuleReq::PlantEnergyReq,
                    min: -1.0,
                    max: 10.0,
                    outcome: RuleOutcome::Exit
                },
                GenomeRule {
                    req: RuleReq::BranchDepthReq,
                    min: -1.0,
                    max: 3.0,
                    outcome: RuleOutcome::ChangeSelfProperty { 
                        strength_factor: 1.0, 
                        photoreceptiveness_factor: -1.0, 
                        water_intake_factor: -1.0, 
                        length_factor: 1.0 
                    }
                }
            ] 
        }
    }
}