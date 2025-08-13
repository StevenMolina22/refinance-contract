use soroban_sdk::{contracttype, BytesN};

// TODO! Add campaign ID
// TODO! Add proof deadline
// TODO! Add valid proof counter (n people who validated it)
#[derive(Clone)]
#[contracttype]
pub struct Proof {
    /// URI pointing to the off-chain proof data (e.g., ipfs://<hash>)
    pub uri: BytesN<64>,
    /// Short description of the proof
    pub description: BytesN<128>,
    /// Ledger timestamp when the proof was logged
    pub timestamp: u64,
}
