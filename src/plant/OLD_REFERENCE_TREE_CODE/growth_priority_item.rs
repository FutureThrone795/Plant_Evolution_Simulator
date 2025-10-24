use crate::plant::branch::{Branch, BranchConnection};
use std::cell::Ref;
use std::cmp::Ordering;

pub struct GrowthPriorityItem<'a> {
    pub priority: f32,

    pub parent_branch: Ref<'a, Branch>,
    pub new_branch_connection: BranchConnection,

    cost: f32
}

impl<'a> GrowthPriorityItem<'a> {
    pub fn new_from_branch(parent_branch: Ref<'a, Branch>, new_branch_connection: BranchConnection, priority: f32) -> GrowthPriorityItem<'a> {
        let cost = new_branch_connection.branch.borrow().calculate_cost();
        
        return GrowthPriorityItem { 
            priority: priority,

            parent_branch: parent_branch, 
            new_branch_connection: new_branch_connection,

            cost: cost
        }
    }

    // https://www.youtube.com/watch?v=JGlla4vGxkY
    pub fn emperor_palpatine(self) {
        self.parent_branch.add_offshoot(self.new_branch_connection);
    }

    pub fn cost(&self) -> f32 {
        return self.cost;
    }
}

impl<'a> PartialEq for GrowthPriorityItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<'a> Eq for GrowthPriorityItem<'a> {}

impl<'a> PartialOrd for GrowthPriorityItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority).map(|o| o.reverse())
    }
}

impl<'a> Ord for GrowthPriorityItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}