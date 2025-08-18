use crate::events;
use crate::storage;
use crate::storage::types::error::Error;
use soroban_sdk::{Env, String};

/// Validate a milestone with proof (Admin only)
pub fn validate_milestone_with_proof(
    env: &Env,
    campaign_id: String,
    milestone_sequence: u32,
    proof_id: String,
) -> Result<(), Error> {
    // Admin authorization
    let admin = storage::admin::get_admin(env);
    admin.require_auth();

    // Get campaign and milestone
    let mut campaign = storage::campaign::get_campaign(env, &campaign_id)?;
    let mut milestone = storage::milestone::get_milestone(env, &campaign_id, milestone_sequence)?;

    // Verify proof exists
    let _proof = storage::proof::get_proof(env, &campaign_id, &proof_id)?;

    // Check if milestone can be completed
    if milestone.completed {
        return Err(Error::MilestoneAlreadyCompleted);
    }

    // Check if campaign has enough funding for this milestone
    if campaign.total_raised < milestone.target_amount {
        return Err(Error::InsufficientFundsForMilestone);
    }

    // Validate sequential completion (can't skip milestones)
    if milestone_sequence != campaign.current_milestone + 1 {
        return Err(Error::MilestoneNotInSequence);
    }

    // Complete milestone
    milestone.completed = true;
    milestone.proof_id = Some(proof_id.clone());
    milestone.completed_at = Some(env.ledger().timestamp());

    // Update campaign
    campaign.current_milestone = milestone_sequence;
    campaign.withdrawable_amount = milestone.target_amount;

    // Store updates
    storage::milestone::set_milestone(env, &campaign_id, milestone_sequence, &milestone);
    storage::campaign::set_campaign(env, &campaign_id, &campaign);

    // Emit event
    events::milestone::milestone_completed(env, campaign_id, milestone_sequence, proof_id);

    Ok(())
}
