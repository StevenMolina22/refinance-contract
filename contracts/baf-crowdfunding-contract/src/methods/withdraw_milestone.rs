use crate::events;
use crate::methods::token::token_transfer;
use crate::storage;
use crate::storage::types::error::Error;
use soroban_sdk::{Env, String};

/// Withdraw funds up to completed milestone (Creator only)
pub fn withdraw_milestone_funds(
    env: &Env,
    campaign_id: String,
    milestone_sequence: u32,
) -> Result<i128, Error> {
    let mut campaign = storage::campaign::get_campaign(env, &campaign_id)?;

    // Creator authorization
    campaign.creator.require_auth();

    // Validate milestone is completed
    let milestone = storage::milestone::get_milestone(env, &campaign_id, milestone_sequence)?;
    if !milestone.completed {
        return Err(Error::MilestoneNotCompleted);
    }

    // Can only withdraw up to current milestone
    if milestone_sequence > campaign.current_milestone {
        return Err(Error::CannotWithdrawFutureMilestone);
    }

    // Calculate withdrawable amount
    let withdrawable = milestone.target_amount;
    if withdrawable <= 0 {
        return Err(Error::NoFundsToWithdraw);
    }

    // Check if there are actually funds available
    if campaign.withdrawable_amount < withdrawable {
        return Err(Error::NoFundsToWithdraw);
    }

    // Update campaign state
    campaign.withdrawable_amount = 0; // Reset after withdrawal
    storage::campaign::set_campaign(env, &campaign_id, &campaign);

    // Transfer funds to creator
    token_transfer(
        env,
        &env.current_contract_address(),
        &campaign.creator,
        &withdrawable,
    )?;

    // Emit event
    events::milestone::milestone_withdrawal(env, campaign_id, milestone_sequence, withdrawable);

    Ok(withdrawable)
}
