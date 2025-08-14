use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, set_campaign},
        contribution::{get_contribution, has_contribution, remove_contribution},
        types::error::Error,
    },
};
use soroban_sdk::{Address, Env, String};

pub fn refund(env: &Env, contributor: Address, campaign_id: String) -> Result<(), Error> {
    contributor.require_auth();

    if !has_contribution(env, &campaign_id, &contributor) {
        return Err(Error::ContributionNotFound);
    }

    let mut campaign = get_campaign(env, &campaign_id)?;
    let amount = get_contribution(env, &campaign_id, &contributor);

    token_transfer(env, &env.current_contract_address(), &contributor, &amount)?;

    campaign.total_raised -= amount;
    campaign.supporters -= 1;

    remove_contribution(env, &campaign_id, &contributor);
    set_campaign(env, &campaign_id, &campaign);
    events::refund::refund(env, &contributor, &campaign_id, &amount);

    Ok(())
}
