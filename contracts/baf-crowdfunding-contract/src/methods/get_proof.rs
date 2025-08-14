use crate::storage::{proof::get_proof as read_proof, structs::proof::Proof, types::error::Error};
use soroban_sdk::{Env, String};

pub fn get_proof(env: &Env, campaign_id: &String, proof_id: &String) -> Result<Proof, Error> {
    read_proof(env, campaign_id, proof_id)
}
