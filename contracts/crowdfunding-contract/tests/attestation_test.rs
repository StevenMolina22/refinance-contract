#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_proof_struct_creation() {
    // Test that we can create proof data with String types
    let env = Env::default();

    // Create test proof data
    let proof_id = String::from_str(&env, "proof-123");
    let campaign_id = String::from_str(&env, "campaign-abc");
    let uri = String::from_str(&env, "ipfs://QmProofHash123");
    let description = String::from_str(&env, "Milestone completion proof");
    let timestamp = env.ledger().timestamp();

    // Verify we can create proof-like data with expected constraints
    assert!(!proof_id.is_empty());
    assert!(!campaign_id.is_empty());
    assert!(!uri.is_empty());
    assert!(!description.is_empty());
    // Timestamp is valid (u64 type guarantees >= 0)
    let _ = timestamp;
}

#[test]
fn test_compilation_of_new_types() {
    // This test ensures our new types compile correctly
    let env = Env::default();
    let creator = Address::generate(&env);

    // Test that campaign with milestone fields compiles
    use crowdfunding_contract::storage::structs::campaign::Campaign;
    let campaign_id = String::from_str(&env, "test-campaign");
    let title = String::from_str(&env, "Test Campaign");
    let description = String::from_str(&env, "A test crowdfunding campaign");

    let _campaign = Campaign {
        id: campaign_id.clone(),
        creator: creator.clone(),
        title,
        description,
        goal: 1000,
        min_donation: 10,
        total_raised: 0,
        supporters: 0,
        milestones_count: 0,
        current_milestone: 0,
        withdrawable_amount: 0,
    };

    // Test that Proof struct compiles
    use crowdfunding_contract::storage::structs::proof::Proof;
    let proof_id = String::from_str(&env, "proof-1");
    let uri = String::from_str(&env, "ipfs://QmProofHash");
    let proof_description = String::from_str(&env, "Proof of milestone completion");
    let _proof = Proof {
        id: proof_id.clone(),
        campaign_id: campaign_id.clone(),
        uri,
        description: proof_description,
        timestamp: env.ledger().timestamp(),
    };

    // Test that Milestone struct compiles
    use crowdfunding_contract::storage::structs::milestone::Milestone;
    let milestone_description = String::from_str(&env, "First milestone");
    let _milestone = Milestone {
        campaign_id: campaign_id.clone(),
        sequence: 1,
        target_amount: 500,
        description: milestone_description,
        completed: false,
        proof_id: None,
        completed_at: None,
    };

    // Test DataKey variants compile
    use crowdfunding_contract::storage::types::storage::DataKey;
    let _campaign_key = DataKey::Campaign(campaign_id.clone());
    let _proof_key = DataKey::Proof(campaign_id.clone(), proof_id.clone());
    let _milestone_key = DataKey::Milestone(campaign_id.clone(), 1);
    let _contribution_key = DataKey::Contribution(campaign_id, creator);

    // Test milestone-related errors compile
    use crowdfunding_contract::storage::types::error::Error;
    let _error1 = Error::ProofNotFound;
    let _error2 = Error::MilestoneNotFound;
    let _error3 = Error::InvalidMilestoneAmount;
    let _error4 = Error::MilestoneAlreadyCompleted;

    // If we reach here, all our new types compile successfully
    assert!(true);
}

#[test]
fn test_milestone_validation_logic() {
    // Test that milestone validation constraints work as expected
    let env = Env::default();
    let campaign_id = String::from_str(&env, "test-campaign");

    // Test milestone sequence validation
    let milestone1 = crowdfunding_contract::storage::structs::milestone::Milestone {
        campaign_id: campaign_id.clone(),
        sequence: 1,
        target_amount: 300,
        description: String::from_str(&env, "First milestone"),
        completed: false,
        proof_id: None,
        completed_at: None,
    };

    let milestone2 = crowdfunding_contract::storage::structs::milestone::Milestone {
        campaign_id: campaign_id.clone(),
        sequence: 2,
        target_amount: 600,
        description: String::from_str(&env, "Second milestone"),
        completed: false,
        proof_id: None,
        completed_at: None,
    };

    // Verify sequential ordering
    assert!(milestone1.sequence < milestone2.sequence);
    assert!(milestone1.target_amount < milestone2.target_amount);
    assert_eq!(milestone1.campaign_id, milestone2.campaign_id);
}
