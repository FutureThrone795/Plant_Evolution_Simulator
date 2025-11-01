use std::f32::consts::PI;

use crate::plant::growth_priority_item::NewOffshootPriorityItem;
use crate::rand::Rng;

pub struct BranchConnection{
    pub branch_index: usize, 
    pub yaw: f32, 
    pub pitch: f32,
    pub along_length: f32
}

impl BranchConnection {
    pub fn new(new_offshoot_priority_item: &NewOffshootPriorityItem, new_index: usize) -> BranchConnection {
        let along_length: f32 = 1.0 - (1.0 - new_offshoot_priority_item.placement_straightness) * rand::rng().random_range(0.0 .. 1.0);

        return BranchConnection { 
            branch_index: new_index, 
            yaw: rand::rng().random_range(0.0 .. 2.0*PI), 
            pitch: (1.0 - along_length) * PI * 0.5,
            along_length
        }
    }
}

pub struct Branch {
    pub strength: f32,              //red
    pub photoreceptiveness: f32,    // green
    pub water_intake: f32,          // blue

    pub length: f32,

    pub offshoot_1: Option<BranchConnection>,
    pub offshoot_2: Option<BranchConnection>
}

impl Branch {
    pub fn new(strength: f32, photoreceptiveness: f32, water_intake: f32, length: f32) -> Branch {
        return Branch {
            strength: strength + rand::random_range(-0.03 .. 0.03),
            photoreceptiveness: photoreceptiveness + rand::random_range(-0.03 .. 0.03),
            water_intake: water_intake + rand::random_range(-0.03 .. 0.03),
            length: length * rand::random_range(0.9 .. 1.1),
            offshoot_1: None,
            offshoot_2: None,
        }
    }

    pub fn calculate_cost_from_individual_parts(strength: f32, photoreceptiveness: f32, water_intake: f32, length: f32) -> f32 {
        return 5.0 + 4.0 * length * (strength + photoreceptiveness + water_intake).powi(2);
    }

    pub fn calculate_cost(&self) -> f32 {
        return Self::calculate_cost_from_individual_parts(self.strength, self.photoreceptiveness, self.water_intake, self.length);
    }

    pub fn calculate_homeostasis(&self) -> f32 {
        return 0.02 + 0.05 * self.length * (self.strength + self.photoreceptiveness + self.water_intake).powi(2);
    }

    pub fn calculate_collect_sunlight(&self, depth: usize) -> f32 {
        return 1.0 * self.length * self.photoreceptiveness * (1.0 + depth as f32 / 5.0); //TODO: Make this depend on height
    }

    pub fn calculate_collect_water(&self, depth: usize) -> f32 {
        return 2.5 * self.length * self.water_intake / (1.0 + depth as f32 / 5.0) as f32;
    }

    pub fn add_offshoot(&mut self, branch_connection: BranchConnection) {
        match self.offshoot_1 {
            None => {
                self.offshoot_1 = Some(branch_connection);
                return;
            }
            Some(_) => ()
        }
        match self.offshoot_2 {
            None => {
                self.offshoot_2 = Some(branch_connection);
                return;
            }
            Some(_) => ()
        }
        panic!("Attempted to add an offshoot to a branch with two existing children");
    }
}

impl From<&NewOffshootPriorityItem> for Branch {
    fn from(new_offshoot_priority_item: &NewOffshootPriorityItem) -> Branch { 
        return Branch::new(new_offshoot_priority_item.strength, new_offshoot_priority_item.photoreceptiveness, new_offshoot_priority_item.water_intake, new_offshoot_priority_item.length);
    }
}