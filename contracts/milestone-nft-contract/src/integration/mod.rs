use soroban_sdk::{Address, BytesN, Env, String};

use crate::{Error, MilestoneNftContract};

impl MilestoneNftContract {
    /// Called by crowdfunding contract when a proof is validated
    /// This creates an NFT milestone for the validated proof
    pub fn create_milestone_from_proof(
        env: Env,
        campaign_id: BytesN<32>,
        proof_id: BytesN<32>,
        _proof_uri: String,
        proof_description: String,
        recipient: Address,
    ) -> Result<u32, Error> {
        // Verify caller is the crowdfunding contract
        let crowdfunding_contract: Address = env
            .storage()
            .instance()
            .get(&crate::DataKey::CrowdfundingContract)
            .ok_or(Error::InvalidCrowdfundingContract)?;

        let caller = env.current_contract_address();
        if caller != crowdfunding_contract {
            return Err(Error::Unauthorized);
        }

        // Create milestone NFT URI based on proof
        let milestone_uri = String::from_str(&env, "https://api.refinance.com/milestone/");

        // Mint the milestone NFT
        Self::mint_milestone(
            env,
            recipient,
            milestone_uri,
            campaign_id,
            proof_id,
            proof_description,
        )
    }

    /// Get all milestones for a specific campaign
    pub fn get_campaign_milestones(
        env: Env,
        campaign_id: BytesN<32>,
    ) -> Result<soroban_sdk::Vec<u32>, Error> {
        Self::require_initialized(&env)?;

        let total_supply = Self::total_supply(env.clone());
        let mut campaign_tokens = soroban_sdk::Vec::new(&env);

        // Iterate through all tokens to find those belonging to this campaign
        for token_id in 1..=total_supply {
            if let Ok(metadata) = Self::get_token_metadata(env.clone(), token_id) {
                if metadata.campaign_id == campaign_id {
                    campaign_tokens.push_back(token_id);
                }
            }
        }

        Ok(campaign_tokens)
    }

    /// Get milestone progress for a campaign (validated vs total)
    pub fn get_campaign_milestone_progress(
        env: Env,
        campaign_id: BytesN<32>,
    ) -> Result<(u32, u32), Error> {
        Self::require_initialized(&env)?;

        let campaign_tokens = Self::get_campaign_milestones(env.clone(), campaign_id)?;
        let total_milestones = campaign_tokens.len();
        let mut validated_milestones = 0u32;

        for i in 0..campaign_tokens.len() {
            if let Some(token_id) = campaign_tokens.get(i) {
                if let Ok(metadata) = Self::get_token_metadata(env.clone(), token_id) {
                    if metadata.validated {
                        validated_milestones += 1;
                    }
                }
            }
        }

        Ok((validated_milestones, total_milestones))
    }

    /// Check if a specific proof has already been minted as an NFT
    pub fn proof_has_milestone(env: Env, proof_id: BytesN<32>) -> Result<bool, Error> {
        Self::require_initialized(&env)?;

        let total_supply = Self::total_supply(env.clone());

        for token_id in 1..=total_supply {
            if let Ok(metadata) = Self::get_token_metadata(env.clone(), token_id) {
                if metadata.proof_id == proof_id {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Update crowdfunding contract address (admin only)
    pub fn update_crowdfunding_contract(env: Env, new_contract: Address) -> Result<(), Error> {
        Self::require_initialized(&env)?;

        let admin: Address = env
            .storage()
            .instance()
            .get(&crate::DataKey::Admin)
            .unwrap();
        admin.require_auth();

        env.storage()
            .instance()
            .set(&crate::DataKey::CrowdfundingContract, &new_contract);

        // Emit event
        env.events()
            .publish((soroban_sdk::symbol_short!("cf_update"),), new_contract);

        Ok(())
    }
}
