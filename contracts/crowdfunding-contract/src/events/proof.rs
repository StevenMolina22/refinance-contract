use soroban_sdk::{symbol_short, Env, String};

/// Event emitted when a proof is logged for a campaign
pub(crate) fn proof_logged(env: &Env, campaign_id: &String, proof_id: &String) {
    env.events().publish(
        (symbol_short!("proof"), symbol_short!("logged")),
        (campaign_id.clone(), proof_id.clone()),
    );
}

/// Event emitted when a proof is validated
pub(crate) fn proof_validated(env: &Env, campaign_id: &String, proof_id: &String) {
    env.events().publish(
        (symbol_short!("proof"), symbol_short!("validated")),
        (campaign_id.clone(), proof_id.clone()),
    );
}
