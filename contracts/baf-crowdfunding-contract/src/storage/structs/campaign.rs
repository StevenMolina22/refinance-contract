use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub struct Campaign {
    pub goal: i128,
    pub min_donation: i128,
    pub total_raised: i128,
    pub supporters: u32,
    pub proofs_count: u32,
}
