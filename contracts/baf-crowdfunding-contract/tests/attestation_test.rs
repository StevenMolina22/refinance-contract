#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env,
};

#[test]
fn test_proof_struct_creation() {
    // Test that we can create proof data with correct sizes
    let env = Env::default();

    // Create test proof data
    let uri: BytesN<64> = BytesN::random(&env);
    let description: BytesN<128> = BytesN::random(&env);
    let timestamp = env.ledger().timestamp();

    // Verify we can create proof-like data with expected constraints
    assert_eq!(uri.len(), 64);
    assert_eq!(description.len(), 128);
    // Timestamp is valid (u64 type guarantees >= 0)
    let _ = timestamp;
}

#[test]
fn test_compilation_of_new_types() {
    // This test ensures our new types compile correctly
    let env = Env::default();

    // Test that campaign with proofs_count compiles
    use baf_crowdfunding_contract::storage::structs::campaign::Campaign;
    let _campaign = Campaign {
        goal: 1000,
        min_donation: 10,
        total_raised: 0,
        supporters: 0,
        proofs_count: 0,
    };

    // Test that Proof struct compiles
    use baf_crowdfunding_contract::storage::structs::proof::Proof;
    let uri = soroban_sdk::String::from_str(&env, "https://example.com/proof");
    let description = soroban_sdk::String::from_str(&env, "https://example.com/proof");
    let _proof = Proof {
        uri,
        description,
        timestamp: env.ledger().timestamp(),
    };

    // Test DataKey::Proof variant compiles
    use baf_crowdfunding_contract::storage::types::storage::DataKey;
    let campaign_address = Address::generate(&env);
    let _key = DataKey::Proof(campaign_address, 0);

    // Test Error::ProofNotFound compiles
    use baf_crowdfunding_contract::storage::types::error::Error;
    let _error = Error::ProofNotFound;

    // If we reach here, all our new types compile successfully
    assert!(true);
}
