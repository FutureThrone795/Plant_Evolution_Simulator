use rand::rng;

use crate::plant::branch::Branch;
use crate::plant::Plant;
use crate::terrain::Terrain;

use crate::rand::Rng;

struct GenomeVal {
    val: f32,
    rand_factor: f32
}

enum OffshootSelection {
    One,
    Two,
    Longest,
    Random
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

enum RuleOutcome {
    Exit,
    JumpToRule(usize),
    KillOffshoot(OffshootSelection),
    ChangeSelfProperty{
        strength: f32, 
        photoreceptiveness: f32, 
        water_intake: f32,
        length: f32
    },
    RequestNewOffshoot{
        priority: f32, 
        pitch: f32, 
        yaw: f32, 
        strength: f32, 
        photoreceptiveness: f32, 
        water_intake: f32,
        length: f32
    }
}

pub struct GenomeRule {
    req: RuleReq,
    min: f32,
    max: f32,
    outcome: RuleOutcome
}

impl GenomeRule {
    pub fn execute(&self, depth: usize, branch: &Branch, plant: &Plant, terrain: &Terrain) -> Option<&RuleOutcome> {
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
            min: rng().random_range(0.0 .. 20.0), 
            max: rng().random_range(0.0 .. 20.0), 
            outcome: RuleOutcome::Exit //TEMP 
        }
    }
}

pub struct PlantGenome {
    pub min_enegy_for_growth: f32,
    pub make_baby_req_energy: f32,

    pub rules: [GenomeRule; 4]
}

impl PlantGenome {
    pub fn random() -> PlantGenome {
        return PlantGenome { 
            min_enegy_for_growth: 1_000.0, 
            make_baby_req_energy: 10_000.0, 
            rules: [GenomeRule::random(), GenomeRule::random(), GenomeRule::random(), GenomeRule::random()] 
        }
    }
}