use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Campaign(String),              // String-based campaign ID
    Contribution(String, Address), // (campaign_id, contributor)
    Proof(String, String),         // (campaign_id, proof_id)
    Milestone(String, u32),        // (campaign_id, sequence)
}
