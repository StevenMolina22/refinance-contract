use crate::{
    events,
    storage::{admin::get_admin, proof::set_proof, structs::proof::Proof, types::error::Error},
};
use soroban_sdk::{Env, String};

pub fn log_proof(
    env: &Env,
    proof_id: String,
    campaign_id: String,
    uri: String,
    description: String,
) -> Result<(), Error> {
    let admin = get_admin(env);
    admin.require_auth();

    let proof = Proof {
        id: proof_id.clone(),
        campaign_id: campaign_id.clone(),
        uri,
        description,
        timestamp: env.ledger().timestamp(),
    };

    set_proof(env, &campaign_id, &proof_id, &proof);

    events::proof::proof_logged(env, &campaign_id, &proof_id);

    Ok(())
}
