use soroban_sdk::{contracttype, String};

// TODO! Add campaign ID
// TODO! Add proof deadline
// TODO! Add valid proof counter (n people who validated it)
#[derive(Clone)]
#[contracttype]
pub struct Proof {
    /// URI pointing to the off-chain proof data (e.g., ipfs://<hash>)
    pub uri: String,
    /// Short description of the proof
    pub description: String,
    /// Ledger timestamp when the proof was logged
    pub timestamp: u64,
}
