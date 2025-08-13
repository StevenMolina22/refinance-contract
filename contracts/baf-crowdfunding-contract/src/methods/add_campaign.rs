use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::get_admin,
        campaign::{has_campaign, set_campaign},
        structs::campaign::Campaign,
        types::error::Error,
    },
};

pub fn add_campaign(
    env: &Env,
    creator: Address,
    goal: i128,
    min_donation: i128,
) -> Result<(), Error> {
    let current_admin = get_admin(env);

    current_admin.require_auth();

    if has_campaign(env, &creator) {
        return Err(Error::CampaignAlreadyExists);
    }

    let campaign = Campaign {
        goal,
        min_donation,
        total_raised: 0,
        supporters: 0,
        proofs_count: 0,
    };

    set_campaign(&env, &creator, &campaign);
    events::campaign::add_campaign(&env, &creator, &goal);
    Ok(())
}
