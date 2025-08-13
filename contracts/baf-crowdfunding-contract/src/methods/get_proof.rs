use crate::storage::{
    structs::proof::Proof,
    types::{error::Error, storage::DataKey},
};
use soroban_sdk::{Address, Env};

pub fn get_proof(env: &Env, campaign_address: &Address, index: u32) -> Result<Proof, Error> {
    let key = DataKey::Proof(campaign_address.clone(), index);
    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::ProofNotFound)
}
