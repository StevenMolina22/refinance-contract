use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, remove_campaign},
        types::error::Error,
    },
};
use soroban_sdk::{Env, String};

pub fn withdraw(env: &Env, campaign_id: String) -> Result<(), Error> {
    let campaign = get_campaign(env, &campaign_id)?;

    // Authorize the campaign creator
    campaign.creator.require_auth();

    // This logic is for a non-milestone, all-or-nothing campaign.
    if campaign.total_raised < campaign.goal {
        return Err(Error::CampaignGoalNotReached);
    }

    token_transfer(
        env,
        &env.current_contract_address(),
        &campaign.creator,
        &campaign.total_raised,
    )?;

    // The campaign is now complete and can be removed.
    remove_campaign(env, &campaign_id);
    events::campaign::withdraw(env, &campaign.creator, campaign.total_raised);

    Ok(())
}
