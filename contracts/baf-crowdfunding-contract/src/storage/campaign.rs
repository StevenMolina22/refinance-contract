use crate::storage::{
    structs::campaign::Campaign,
    types::{error::Error, storage::DataKey},
};
use soroban_sdk::{Env, String};

pub(crate) fn campaign_key(campaign_id: &String) -> DataKey {
    DataKey::Campaign(campaign_id.clone())
}

pub(crate) fn has_campaign(env: &Env, campaign_id: &String) -> bool {
    let key = campaign_key(campaign_id);
    env.storage().persistent().has(&key)
}

pub(crate) fn set_campaign(env: &Env, campaign_id: &String, campaign: &Campaign) {
    let key = campaign_key(campaign_id);
    env.storage().persistent().set(&key, campaign);
}

pub(crate) fn get_campaign(env: &Env, campaign_id: &String) -> Result<Campaign, Error> {
    let key = campaign_key(campaign_id);
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(Error::CampaignNotFound)
}

pub(crate) fn remove_campaign(env: &Env, campaign_id: &String) {
    let key = campaign_key(campaign_id);
    env.storage().persistent().remove(&key);
}
