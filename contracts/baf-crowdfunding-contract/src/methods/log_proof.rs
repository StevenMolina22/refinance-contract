use crate::{
    events,
    storage::{
        admin::get_admin,
        campaign::{get_campaign, set_campaign},
        structs::proof::Proof,
        types::{error::Error, storage::DataKey},
    },
};
use soroban_sdk::{Address, Env, String};

pub fn log_proof(
    env: &Env,
    campaign_address: Address,
    uri: String,
    description: String,
) -> Result<(), Error> {
    let admin = get_admin(env);
    admin.require_auth();

    let mut campaign = get_campaign(env, &campaign_address)?;
    let proof_index = campaign.proofs_count;
    campaign.proofs_count += 1;

    let proof = Proof {
        uri,
        description,
        timestamp: env.ledger().timestamp(),
    };

    let key = DataKey::Proof(campaign_address.clone(), proof_index);
    env.storage().instance().set(&key, &proof);

    set_campaign(env, &campaign_address, &campaign);

    events::proof::proof_logged(env, &campaign_address, &proof_index);

    Ok(())
}
