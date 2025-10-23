pub struct BranchConnection{
    pub branch: Box<Branch>, 
    pub yaw: f32, 
    pub pitch:f32
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
    pub fn calculate_cost(&self) -> f32 {
        return 100.0 + 200.0 * self.length * (self.strength + self.photoreceptiveness + self.water_intake).powf(1.5);
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