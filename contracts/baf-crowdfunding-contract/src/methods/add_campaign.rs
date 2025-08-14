use soroban_sdk::{Address, Env, String};

use crate::{
    events,
    storage::{
        campaign::{has_campaign, set_campaign},
        structs::campaign::Campaign,
        types::error::Error,
    },
};

pub fn add_campaign(
    env: &Env,
    campaign_id: String,
    creator: Address,
    title: String,
    description: String,
    goal: i128,
    min_donation: i128,
) -> Result<(), Error> {
    // Verify creator authorization
    creator.require_auth();

    // Validate inputs
    if goal <= 0 {
        return Err(Error::InvalidGoalAmount);
    }

    if min_donation <= 0 || min_donation > goal {
        return Err(Error::InvalidMinDonation);
    }

    // Check if campaign already exists
    if has_campaign(env, &campaign_id) {
        return Err(Error::CampaignAlreadyExists);
    }

    // Create campaign
    let campaign = Campaign {
        id: campaign_id.clone(),
        creator: creator.clone(),
        title,
        description,
        goal,
        min_donation,
        total_raised: 0,
        supporters: 0,
        milestones_count: 0,
        current_milestone: 0,
        withdrawable_amount: 0,
    };

    // Store campaign
    set_campaign(env, &campaign_id, &campaign);

    // Emit event
    events::campaign::add_campaign(env, &creator, &goal);

    Ok(())
}
