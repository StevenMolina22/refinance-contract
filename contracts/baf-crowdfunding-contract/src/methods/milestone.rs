use crate::events;
use crate::storage::types::error::Error;
use crate::storage::{self, structs::milestone::*};
use soroban_sdk::{Env, String, Vec};

/// Create a new milestone for a campaign (Creator only)
pub fn create_milestone(
    env: &Env,
    campaign_id: String,
    target_amount: i128,
    description: String,
) -> Result<u32, Error> {
    // Verify campaign exists and get it
    let campaign = storage::campaign::get_campaign(env, &campaign_id)?;

    // Verify creator authorization
    campaign.creator.require_auth();

    // Validate target amount
    if target_amount <= 0 || target_amount > campaign.goal {
        return Err(Error::InvalidMilestoneAmount);
    }

    // Get next sequence number
    let sequence = campaign.milestones_count + 1;

    // Validate sequential ordering (each milestone should be higher than previous)
    if sequence > 1 {
        let prev_milestone = get_milestone(env, &campaign_id, sequence - 1)?;
        if target_amount <= prev_milestone.target_amount {
            return Err(Error::MilestoneAmountNotIncreasing);
        }
    }

    // Create milestone
    let milestone = Milestone {
        campaign_id: campaign_id.clone(),
        sequence,
        target_amount,
        description,
        completed: false,
        proof_id: None,
        completed_at: None,
    };

    // Store milestone
    storage::milestone::set_milestone(env, &campaign_id, sequence, &milestone);

    // Update campaign milestone count
    let mut updated_campaign = campaign;
    updated_campaign.milestones_count = sequence;
    storage::campaign::set_campaign(env, &campaign_id, &updated_campaign);

    // Emit event
    events::milestone::milestone_created(env, campaign_id, sequence, target_amount);

    Ok(sequence)
}

/// Get milestone details
pub fn get_milestone(env: &Env, campaign_id: &String, sequence: u32) -> Result<Milestone, Error> {
    storage::milestone::get_milestone(env, campaign_id, sequence)
}

/// Get all milestones for a campaign
pub fn get_campaign_milestones(env: &Env, campaign_id: &String) -> Result<Vec<Milestone>, Error> {
    let campaign = storage::campaign::get_campaign(env, campaign_id)?;
    let mut milestones = Vec::new(env);

    for sequence in 1..=campaign.milestones_count {
        if let Ok(milestone) = storage::milestone::get_milestone(env, campaign_id, sequence) {
            milestones.push_back(milestone);
        }
    }

    Ok(milestones)
}
