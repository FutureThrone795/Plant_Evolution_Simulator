use std::cmp::Ordering;

use crate::plant::branch::Branch;
use crate::plant::Plant;
use crate::plant::branch::BranchConnection;

pub struct NewOffshootPriorityItem {
    pub priority: f32,
        
    // upness, rightness, and forwardness are each in the direction of the branch itself (as opposed to any kind of "world up" or "world right")
    // i.e. high rightness will cause a clockwise spiral from top-down
    pub placement_upness: f32,
    pub placement_rightness: f32,
    pub placement_forwardness: f32,
    pub placement_randomness: f32,

    pub strength: f32,
    pub photoreceptiveness: f32,
    pub water_intake: f32,
    pub length: f32
}

pub struct ModifyBranchPriorityItem {
    pub strength_factor: f32, 
    pub photoreceptiveness_factor: f32, 
    pub water_intake_factor: f32,
    pub length_factor: f32
}

pub enum PriorityItemType {
    NewOffshoot(NewOffshootPriorityItem),
    ModifyBranch(ModifyBranchPriorityItem)
}

pub struct GrowthPriorityItem {
    pub priority: f32,

    pub branch_index: usize,

    pub item: PriorityItemType
}

impl GrowthPriorityItem {
    pub fn new_offshoot_request(branch_index: usize, priority: f32, placement_upness: f32, placement_rightness: f32, placement_forwardness: f32, placement_randomness: f32, strength: f32, photoreceptiveness: f32, water_intake: f32, length: f32) -> GrowthPriorityItem {
        //let cost = Branch::calculate_cost_from_individual_parts(strength, photoreceptiveness, water_intake, length);
        
        return GrowthPriorityItem { 
            priority,

            branch_index,

            item: PriorityItemType::NewOffshoot(
                NewOffshootPriorityItem { 
                    priority,
        
                    placement_upness,
                    placement_rightness,
                    placement_forwardness,
                    placement_randomness,

                    strength,
                    photoreceptiveness,
                    water_intake,
                    length
                }
            )
        }
    }

    pub fn new_modify_branch_request(branch_index: usize, strength_factor: f32, photoreceptiveness_factor: f32, water_intake_factor: f32, length_factor: f32, priority: f32) -> GrowthPriorityItem {
        return GrowthPriorityItem { 
            priority,
            
            branch_index,

            item: PriorityItemType::ModifyBranch( 
                ModifyBranchPriorityItem {
                    strength_factor, 
                    photoreceptiveness_factor, 
                    water_intake_factor, 
                    length_factor 
                }
            ),
        }
    }
}

impl Plant {
    pub fn execute_growth_priority_item(&mut self, growth_priority_item: &GrowthPriorityItem) -> bool {
        match &growth_priority_item.item {
            PriorityItemType::NewOffshoot(item) => {
                let cost = Branch::calculate_cost_from_individual_parts(item.strength, item.photoreceptiveness, item.water_intake, item.length);

                if self.current_energy - cost < self.genome.min_enegy_for_growth {
                    return false;
                }

                let new_index: usize = self.branches.len();
                let new_offshoot = BranchConnection::new(&item, new_index);
                
                self.branches.push(Branch::from(item));
                self.branches[growth_priority_item.branch_index].add_offshoot(new_offshoot);

                self.current_energy -= cost;
            }
            PriorityItemType::ModifyBranch(item) => {
                let new_strength = self.branches[growth_priority_item.branch_index].strength + item.strength_factor;
                let new_photoreceptiveness = item.photoreceptiveness_factor;
                let new_water_intake = self.branches[growth_priority_item.branch_index].water_intake + item.water_intake_factor;
                let new_length = self.branches[growth_priority_item.branch_index].length + item.length_factor;
                
                let current_branch_cost = self.branches[growth_priority_item.branch_index].calculate_cost();
                let new_branch_cost = Branch::calculate_cost_from_individual_parts(new_strength, new_photoreceptiveness, new_water_intake, new_length);
                let cost = new_branch_cost - current_branch_cost;

                if self.current_energy - cost < self.genome.min_enegy_for_growth {
                    return false;
                }

                self.branches[growth_priority_item.branch_index].strength = new_strength;
                self.branches[growth_priority_item.branch_index].photoreceptiveness = new_photoreceptiveness;
                self.branches[growth_priority_item.branch_index].water_intake = new_water_intake;
                self.branches[growth_priority_item.branch_index].length = new_length;
                

                self.current_energy -= cost;
            }
        }

        return true;
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
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for GrowthPriorityItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}