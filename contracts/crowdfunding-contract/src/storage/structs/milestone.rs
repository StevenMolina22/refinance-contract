use soroban_sdk::{contracttype, String};

#[derive(Clone)]
#[contracttype]
pub struct Milestone {
    pub campaign_id: String,
    pub sequence: u32,             // 1, 2, 3... (order matters)
    pub target_amount: i128,       // Funding needed to reach this milestone
    pub description: String,       // What this milestone represents
    pub completed: bool,           // Has this milestone been validated?
    pub proof_id: Option<String>,  // Which proof validated this milestone
    pub completed_at: Option<u64>, // When was it completed
}

#[derive(Clone)]
#[contracttype]
pub struct MilestoneKey {
    pub campaign_id: String,
    pub sequence: u32,
}
