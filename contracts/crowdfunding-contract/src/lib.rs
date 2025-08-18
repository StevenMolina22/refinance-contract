#![no_std]

mod contract;
mod events;
mod methods;
pub mod storage;

pub use contract::{CrowdfundingContract, CrowdfundingContractClient};

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_campaign_storage() {
        let env = Env::default();
        let creator = Address::generate(&env);
        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        let campaign_id = String::from_str(&env, "test-campaign");
        let title = String::from_str(&env, "Test Campaign");
        let description = String::from_str(&env, "A test crowdfunding campaign");

        let campaign = storage::structs::campaign::Campaign {
            id: campaign_id.clone(),
            creator: creator.clone(),
            title: title.clone(),
            description: description.clone(),
            goal: 1000,
            min_donation: 10,
            total_raised: 0,
            supporters: 0,
            milestones_count: 0,
            current_milestone: 0,
            withdrawable_amount: 0,
        };

        // Test campaign storage
        let contract_id = env.register(CrowdfundingContract, (admin.clone(), token.clone()));
        env.as_contract(&contract_id, || {
            storage::campaign::set_campaign(&env, &campaign_id, &campaign);
        });
        let retrieved = env.as_contract(&contract_id, || {
            storage::campaign::get_campaign(&env, &campaign_id).unwrap()
        });

        assert_eq!(retrieved.id, campaign_id);
        assert_eq!(retrieved.title, title);
        assert_eq!(retrieved.goal, 1000);
        assert_eq!(retrieved.milestones_count, 0);
    }

    #[test]
    fn test_milestone_storage() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        let campaign_id = String::from_str(&env, "test-campaign");
        let milestone_desc = String::from_str(&env, "First milestone");

        let milestone = storage::structs::milestone::Milestone {
            campaign_id: campaign_id.clone(),
            sequence: 1,
            target_amount: 500,
            description: milestone_desc.clone(),
            completed: false,
            proof_id: None,
            completed_at: None,
        };

        // Test milestone storage
        let contract_id = env.register(CrowdfundingContract, (admin.clone(), token.clone()));
        env.as_contract(&contract_id, || {
            storage::milestone::set_milestone(&env, &campaign_id, 1, &milestone);
        });
        let retrieved = env.as_contract(&contract_id, || {
            storage::milestone::get_milestone(&env, &campaign_id, 1).unwrap()
        });

        assert_eq!(retrieved.campaign_id, campaign_id);
        assert_eq!(retrieved.sequence, 1);
        assert_eq!(retrieved.target_amount, 500);
        assert_eq!(retrieved.completed, false);
    }

    #[test]
    fn test_proof_storage() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);

        let campaign_id = String::from_str(&env, "test-campaign");
        let proof_id = String::from_str(&env, "proof-1");
        let uri = String::from_str(&env, "https://example.com/proof");
        let description = String::from_str(&env, "Test proof");

        let proof = storage::structs::proof::Proof {
            id: proof_id.clone(),
            campaign_id: campaign_id.clone(),
            uri: uri.clone(),
            description: description.clone(),
            timestamp: 1234567890,
        };

        // Test proof storage
        let contract_id = env.register(CrowdfundingContract, (admin.clone(), token.clone()));
        env.as_contract(&contract_id, || {
            storage::proof::set_proof(&env, &campaign_id, &proof_id, &proof);
        });
        let retrieved = env.as_contract(&contract_id, || {
            storage::proof::get_proof(&env, &campaign_id, &proof_id).unwrap()
        });

        assert_eq!(retrieved.id, proof_id);
        assert_eq!(retrieved.campaign_id, campaign_id);
        assert_eq!(retrieved.uri, uri);
        assert_eq!(retrieved.description, description);
    }

    #[test]
    fn test_milestone_creation_logic() {
        let env = Env::default();
        let creator = Address::generate(&env);
        let admin = Address::generate(&env);

        let token = Address::generate(&env);
        let contract_id = env.register(CrowdfundingContract, (admin.clone(), token.clone()));

        let campaign_id = String::from_str(&env, "test-campaign");
        let title = String::from_str(&env, "Test Campaign");
        let description = String::from_str(&env, "A test campaign");

        // Create and store campaign
        let campaign = storage::structs::campaign::Campaign {
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

        env.as_contract(&contract_id, || {
            storage::campaign::set_campaign(&env, &campaign_id, &campaign);
        });

        // Test milestone creation method
        let milestone_desc = String::from_str(&env, "First milestone");
        let target_amount = 500i128;

        // Mock the creator's authorization
        env.mock_all_auths();

        let result = env.as_contract(&contract_id, || {
            methods::milestone::add_milestone(
                &env,
                campaign_id.clone(),
                target_amount,
                milestone_desc,
            )
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Verify milestone was created
        let (milestone, updated_campaign) = env.as_contract(&contract_id, || {
            let milestone = storage::milestone::get_milestone(&env, &campaign_id, 1).unwrap();
            let updated_campaign = storage::campaign::get_campaign(&env, &campaign_id).unwrap();
            (milestone, updated_campaign)
        });
        assert_eq!(milestone.sequence, 1);
        assert_eq!(milestone.target_amount, target_amount);
        assert_eq!(updated_campaign.milestones_count, 1);
    }

    #[test]
    fn test_milestone_validation_errors() {
        let env = Env::default();
        let creator = Address::generate(&env);

        let campaign_id = String::from_str(&env, "test-campaign");
        let title = String::from_str(&env, "Test Campaign");
        let description = String::from_str(&env, "A test campaign");

        // Create campaign with goal of 1000
        let campaign = storage::structs::campaign::Campaign {
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

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let contract_id = env.register(CrowdfundingContract, (admin.clone(), token.clone()));
        env.as_contract(&contract_id, || {
            storage::campaign::set_campaign(&env, &campaign_id, &campaign);
        });

        // Test invalid milestone amount (greater than goal)
        let milestone_desc = String::from_str(&env, "Invalid milestone");
        let invalid_target = 2000i128; // Greater than goal

        // Mock the creator's authorization
        env.mock_all_auths();

        let result = env.as_contract(&contract_id, || {
            methods::milestone::add_milestone(&env, campaign_id, invalid_target, milestone_desc)
        });

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            storage::types::error::Error::InvalidMilestoneAmount
        );
    }

    #[test]
    fn test_proof_logging() {
        let env = Env::default();
        let admin = Address::generate(&env);

        let token = Address::generate(&env);
        let contract_id = env.register(CrowdfundingContract, (admin.clone(), token.clone()));

        let proof_id = String::from_str(&env, "proof-1");
        let campaign_id = String::from_str(&env, "test-campaign");
        let uri = String::from_str(&env, "https://example.com/proof");
        let description = String::from_str(&env, "Proof description");

        // Mock the admin's authorization
        env.mock_all_auths();

        let result = env.as_contract(&contract_id, || {
            methods::add_proof::add_proof(
                &env,
                proof_id.clone(),
                campaign_id.clone(),
                uri.clone(),
                description.clone(),
            )
        });

        assert!(result.is_ok());

        // Verify proof was stored
        let proof = env.as_contract(&contract_id, || {
            storage::proof::get_proof(&env, &campaign_id, &proof_id).unwrap()
        });
        assert_eq!(proof.id, proof_id);
        assert_eq!(proof.campaign_id, campaign_id);
        assert_eq!(proof.uri, uri);
        assert_eq!(proof.description, description);
    }
}
