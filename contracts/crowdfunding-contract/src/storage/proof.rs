use crate::storage::{
    structs::proof::Proof,
    types::{error::Error, storage::DataKey},
};
use soroban_sdk::{Env, String};

pub(crate) fn proof_key(campaign_id: &String, proof_id: &String) -> DataKey {
    DataKey::Proof(campaign_id.clone(), proof_id.clone())
}

pub(crate) fn set_proof(env: &Env, campaign_id: &String, proof_id: &String, proof: &Proof) {
    let key = proof_key(campaign_id, proof_id);
    env.storage().persistent().set(&key, proof);
}

pub(crate) fn get_proof(
    env: &Env,
    campaign_id: &String,
    proof_id: &String,
) -> Result<Proof, Error> {
    let key = proof_key(campaign_id, proof_id);
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(Error::ProofNotFound)
}

pub(crate) fn has_proof(env: &Env, campaign_id: &String, proof_id: &String) -> bool {
    let key = proof_key(campaign_id, proof_id);
    env.storage().persistent().has(&key)
}

pub(crate) fn remove_proof(env: &Env, campaign_id: &String, proof_id: &String) {
    let key = proof_key(campaign_id, proof_id);
    env.storage().persistent().remove(&key);
}
