use crate::{
    events,
    storage::{
        admin::get_admin,
        campaign::{get_campaign, set_campaign},
        structs::proof::Proof,
        types::{error::Error, storage::DataKey},
    },
};
use soroban_sdk::{Address, BytesN, Env};

pub fn log_proof(
    env: &Env,
    campaign_address: Address,
    uri: BytesN<64>,
    description: BytesN<128>,
) -> Result<(), Error> {
    // 1. Only the contract admin can log a proof.
    let admin = get_admin(env);
    admin.require_auth();

    // 2. Get the campaign and increment its proof counter.
    let mut campaign = get_campaign(env, &campaign_address)?;
    let proof_index = campaign.proofs_count;
    campaign.proofs_count += 1;

    // 3. Create the proof instance.
    let proof = Proof {
        uri,
        description,
        timestamp: env.ledger().timestamp(),
    };

    // 4. Store the proof using a key derived from the campaign and the new index.
    let key = DataKey::Proof(campaign_address.clone(), proof_index);
    env.storage().instance().set(&key, &proof);

    // 5. Save the updated campaign state (with the new proof count).
    set_campaign(env, &campaign_address, &campaign);

    // 6. Emit an event for off-chain services.
    events::proof::proof_logged(env, &campaign_address, &proof_index);

    Ok(())
}
