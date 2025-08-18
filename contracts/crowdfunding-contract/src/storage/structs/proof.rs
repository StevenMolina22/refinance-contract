use soroban_sdk::{contracttype, String};

#[derive(Clone)]
#[contracttype]
pub struct Proof {
    pub id: String,          // Proof identifier
    pub campaign_id: String, // Which campaign this proof belongs to
    pub uri: String,         // IPFS or external URI
    pub description: String, // Description of the proof
    pub timestamp: u64,      // When proof was submitted
}
