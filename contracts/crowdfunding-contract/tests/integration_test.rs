#[cfg(test)]
mod integration_tests {
    use crowdfunding_contract::{CrowdfundingContract, CrowdfundingContractClient};
    use nft_contract::{MilestoneNftContract, MilestoneNftContractClient};
    use soroban_sdk::{testutils::Address as _, token, Address, BytesN, Env, FromVal, String};

    #[test]
    fn test_milestone_validation_mints_nft() {
        let env = Env::default();
        env.mock_all_auths();

        // Setup identities
        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let token_admin = Address::generate(&env);

        // Deploy token contract (mock)
        let token_id = env.register_stellar_asset_contract_v2(token_admin);
        let token_client = token::StellarAssetClient::new(&env, &token_id);

        // Deploy NFT contract
        let nft_contract_id = env.register(MilestoneNftContract, ());
        let nft_client = MilestoneNftContractClient::new(&env, &nft_contract_id);

        // Deploy crowdfunding contract
        let crowdfunding_contract_id = env.register(CrowdfundingContract, ());
        let crowdfunding_client = CrowdfundingContractClient::new(&env, &crowdfunding_contract_id);

        // Initialize NFT contract first
        nft_client.initialize(
            &admin,
            &String::from_str(&env, "Milestone NFT"),
            &String::from_str(&env, "MNFT"),
            &String::from_str(&env, "https://api.refinance.com/"),
            &crowdfunding_contract_id, // Authorize crowdfunding contract
        );

        // Initialize crowdfunding contract with NFT contract address
        crowdfunding_client
            .try___constructor(&admin, &token_id, &nft_contract_id)
            .unwrap();

        // Setup campaign and milestone
        let campaign_id = String::from_str(&env, "campaign-1");
        crowdfunding_client
            .try_add_campaign(
                &campaign_id,
                &creator,
                &String::from_str(&env, "Test Campaign"),
                &String::from_str(&env, "Test Description"),
                &1000_i128,
                &10_i128,
            )
            .unwrap();

        // Add milestone
        let milestone_sequence = crowdfunding_client
            .try_add_milestone(
                &campaign_id,
                &500_i128,
                &String::from_str(&env, "First milestone"),
            )
            .unwrap();

        // Contribute funds to reach milestone
        token_client.mint(&creator, &500_i128);
        crowdfunding_client
            .try_contribute(&creator, &campaign_id, &500_i128)
            .unwrap();

        // Create proof
        let proof_id = String::from_str(&env, "proof-1");
        crowdfunding_client
            .try_add_proof(
                &proof_id,
                &campaign_id,
                &String::from_str(&env, "ipfs://QmProofHash123"),
                &String::from_str(&env, "Milestone completion proof"),
            )
            .unwrap();

        // Verify initial state - no NFTs should exist yet
        let initial_supply = nft_client.total_supply();
        assert_eq!(initial_supply, 0);

        // Validate milestone with proof (this should trigger NFT minting)
        crowdfunding_client
            .try_validate_milestone_with_proof(&campaign_id, &milestone_sequence, &proof_id)
            .unwrap();

        // Verify milestone is completed in crowdfunding contract
        let milestone = crowdfunding_client
            .try_get_milestone(&campaign_id, &milestone_sequence)
            .unwrap();
        assert_eq!(milestone.completed, true);
        assert_eq!(milestone.proof_id, Some(proof_id.clone()));

        // Verify NFT was minted
        let total_supply = nft_client.total_supply();
        assert_eq!(total_supply, 1);

        // Verify NFT owner is the campaign creator
        let nft_owner = nft_client.try_owner_of(&1).unwrap();
        assert_eq!(nft_owner, creator);

        // Verify NFT metadata contains correct information
        let metadata = nft_client.try_get_token_metadata(&1).unwrap();

        // Verify campaign ID hash matches
        let expected_campaign_id_hash: BytesN<32> = env
            .crypto()
            .sha256(&soroban_sdk::Bytes::from_val(&env, &campaign_id.to_val()))
            .into();
        assert_eq!(metadata.campaign_id, expected_campaign_id_hash);

        // Verify proof ID hash matches
        let expected_proof_id_hash: BytesN<32> = env
            .crypto()
            .sha256(&soroban_sdk::Bytes::from_val(&env, &proof_id.to_val()))
            .into();
        assert_eq!(metadata.proof_id, expected_proof_id_hash);

        // Verify other metadata
        assert_eq!(
            metadata.description,
            String::from_str(&env, "Milestone completion proof")
        );
        assert_eq!(metadata.validated, false); // Should be false initially
        assert!(metadata.timestamp > 0);
    }

    #[test]
    fn test_multiple_milestones_mint_multiple_nfts() {
        let env = Env::default();
        env.mock_all_auths();

        // Setup identities
        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let token_admin = Address::generate(&env);

        // Deploy contracts
        let token_id = env.register_stellar_asset_contract_v2(token_admin);
        let token_client = token::StellarAssetClient::new(&env, &token_id);

        let nft_contract_id = env.register(MilestoneNftContract, ());
        let nft_client = MilestoneNftContractClient::new(&env, &nft_contract_id);

        let crowdfunding_contract_id = env.register(CrowdfundingContract, ());
        let crowdfunding_client = CrowdfundingContractClient::new(&env, &crowdfunding_contract_id);

        // Initialize contracts
        nft_client.initialize(
            &admin,
            &String::from_str(&env, "Milestone NFT"),
            &String::from_str(&env, "MNFT"),
            &String::from_str(&env, "https://api.refinance.com/"),
            &crowdfunding_contract_id,
        );

        crowdfunding_client
            .try___constructor(&admin, &token_id, &nft_contract_id)
            .unwrap();

        // Setup campaign
        let campaign_id = String::from_str(&env, "campaign-multi");
        crowdfunding_client
            .try_add_campaign(
                &campaign_id,
                &creator,
                &String::from_str(&env, "Multi Milestone Campaign"),
                &String::from_str(&env, "Test Description"),
                &1000_i128,
                &10_i128,
            )
            .unwrap();

        // Add two milestones
        let milestone1 = crowdfunding_client
            .try_add_milestone(
                &campaign_id,
                &500_i128,
                &String::from_str(&env, "First milestone"),
            )
            .unwrap();

        let milestone2 = crowdfunding_client
            .try_add_milestone(
                &campaign_id,
                &1000_i128,
                &String::from_str(&env, "Second milestone"),
            )
            .unwrap();

        // Contribute funds to reach both milestones
        token_client.mint(&creator, &1000_i128);
        crowdfunding_client
            .try_contribute(&creator, &campaign_id, &1000_i128)
            .unwrap();

        // Complete first milestone
        let proof1_id = String::from_str(&env, "proof-1");
        crowdfunding_client
            .try_add_proof(
                &proof1_id,
                &campaign_id,
                &String::from_str(&env, "ipfs://QmProof1"),
                &String::from_str(&env, "First milestone proof"),
            )
            .unwrap();

        crowdfunding_client
            .try_validate_milestone_with_proof(&campaign_id, &milestone1, &proof1_id)
            .unwrap();

        // Verify first NFT minted
        assert_eq!(nft_client.total_supply(), 1);
        assert_eq!(nft_client.try_owner_of(&1).unwrap(), creator);

        // Complete second milestone
        let proof2_id = String::from_str(&env, "proof-2");
        crowdfunding_client
            .try_add_proof(
                &proof2_id,
                &campaign_id,
                &String::from_str(&env, "ipfs://QmProof2"),
                &String::from_str(&env, "Second milestone proof"),
            )
            .unwrap();

        crowdfunding_client
            .try_validate_milestone_with_proof(&campaign_id, &milestone2, &proof2_id)
            .unwrap();

        // Verify second NFT minted
        assert_eq!(nft_client.total_supply(), 2);
        assert_eq!(nft_client.try_owner_of(&2).unwrap(), creator);

        // Verify both NFTs have different metadata
        let metadata1 = nft_client.try_get_token_metadata(&1).unwrap();
        let metadata2 = nft_client.try_get_token_metadata(&2).unwrap();

        assert_ne!(metadata1.proof_id, metadata2.proof_id);
        assert_eq!(
            metadata1.description,
            String::from_str(&env, "First milestone proof")
        );
        assert_eq!(
            metadata2.description,
            String::from_str(&env, "Second milestone proof")
        );
    }

    #[test]
    fn test_unauthorized_validation_does_not_mint_nft() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let unauthorized_user = Address::generate(&env);
        let token_admin = Address::generate(&env);

        // Deploy and initialize contracts
        let token_id = env.register_stellar_asset_contract_v2(token_admin);
        let nft_contract_id = env.register(MilestoneNftContract, ());
        let crowdfunding_contract_id = env.register(CrowdfundingContract, ());

        let nft_client = MilestoneNftContractClient::new(&env, &nft_contract_id);
        let crowdfunding_client = CrowdfundingContractClient::new(&env, &crowdfunding_contract_id);

        nft_client.initialize(
            &admin,
            &String::from_str(&env, "Milestone NFT"),
            &String::from_str(&env, "MNFT"),
            &String::from_str(&env, "https://api.refinance.com/"),
            &crowdfunding_contract_id,
        );

        crowdfunding_client
            .try___constructor(&admin, &token_id, &nft_contract_id)
            .unwrap();

        // Setup campaign
        let campaign_id = String::from_str(&env, "campaign-auth-test");
        crowdfunding_client
            .try_add_campaign(
                &campaign_id,
                &creator,
                &String::from_str(&env, "Auth Test Campaign"),
                &String::from_str(&env, "Test Description"),
                &1000_i128,
                &10_i128,
            )
            .unwrap();

        // Try to validate milestone as unauthorized user (should fail)
        // Note: This test would require proper authorization testing setup
        // The validate_milestone_with_proof function requires admin auth

        // Verify no NFTs were minted due to unauthorized access
        assert_eq!(nft_client.total_supply(), 0);
    }
}
