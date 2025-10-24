use crate::plant::branch::Branch;
use std::cmp::Ordering;

pub struct GrowthPriorityItem {
    pub priority: f32,

    pub parent_branch_index: usize,

    pub placement_straightness: f32,
    pub strength: f32,
    pub photoreceptiveness: f32,
    pub water_intake: f32,
    pub length: f32,

    cost: f32
}

impl GrowthPriorityItem {
    pub fn new(parent_branch_index: usize, placement_straightness: f32, strength: f32, photoreceptiveness: f32, water_intake: f32, length: f32, priority: f32) -> GrowthPriorityItem {
        let cost = Branch::calculate_cost_from_individual_parts(strength, photoreceptiveness, water_intake, length);
        
        return GrowthPriorityItem { 
            priority: priority,
            
            parent_branch_index,

            placement_straightness,
            strength,
            photoreceptiveness,
            water_intake,
            length,

            cost: cost
        }
    }

    pub fn cost(&self) -> f32 {
        return self.cost;
    }
}

impl PartialEq for GrowthPriorityItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for GrowthPriorityItem {}

impl PartialOrd for GrowthPriorityItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority).map(|o| o.reverse())
    }
}

impl Ord for GrowthPriorityItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}