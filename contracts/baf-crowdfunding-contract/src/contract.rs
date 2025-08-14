use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

use crate::{
    methods::{
        add_campaign::add_campaign,
        get_campaign::get_campaign,
        get_proof::get_proof,
        initialize::initialize,
        log_proof::log_proof,
        milestone::{create_milestone, get_campaign_milestones, get_milestone},
        proof_milestone::validate_milestone_with_proof,
        withdraw_milestone::withdraw_milestone_funds,
    },
    storage::{
        structs::{campaign::Campaign, milestone::Milestone, proof::Proof},
        types::error::Error,
    },
};

#[contract]
pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {
    pub fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        initialize(&env, admin, token)
    }

    // === CAMPAIGN FUNCTIONS ===
    pub fn create_campaign(
        env: Env,
        campaign_id: String,
        creator: Address,
        title: String,
        description: String,
        goal: i128,
        min_donation: i128,
    ) -> Result<(), Error> {
        add_campaign(
            &env,
            campaign_id,
            creator,
            title,
            description,
            goal,
            min_donation,
        )
    }

    pub fn get_campaign(env: Env, campaign_id: String) -> Result<Campaign, Error> {
        get_campaign(&env, &campaign_id)
    }

    // === MILESTONE FUNCTIONS ===
    pub fn create_milestone(
        env: Env,
        campaign_id: String,
        target_amount: i128,
        description: String,
    ) -> Result<u32, Error> {
        create_milestone(&env, campaign_id, target_amount, description)
    }

    pub fn get_milestone(env: Env, campaign_id: String, sequence: u32) -> Result<Milestone, Error> {
        get_milestone(&env, &campaign_id, sequence)
    }

    pub fn get_campaign_milestones(env: Env, campaign_id: String) -> Result<Vec<Milestone>, Error> {
        get_campaign_milestones(&env, &campaign_id)
    }

    // === PROOF FUNCTIONS ===
    pub fn log_proof(
        env: Env,
        proof_id: String,
        campaign_id: String,
        uri: String,
        description: String,
    ) -> Result<(), Error> {
        log_proof(&env, proof_id, campaign_id, uri, description)
    }

    pub fn get_proof(env: Env, campaign_id: String, proof_id: String) -> Result<Proof, Error> {
        get_proof(&env, &campaign_id, &proof_id)
    }

    pub fn validate_milestone_with_proof(
        env: Env,
        campaign_id: String,
        milestone_sequence: u32,
        proof_id: String,
    ) -> Result<(), Error> {
        validate_milestone_with_proof(&env, campaign_id, milestone_sequence, proof_id)
    }

    // === WITHDRAWAL FUNCTIONS ===
    pub fn withdraw_milestone_funds(
        env: Env,
        campaign_id: String,
        milestone_sequence: u32,
    ) -> Result<i128, Error> {
        withdraw_milestone_funds(&env, campaign_id, milestone_sequence)
    }

    // === LEGACY FUNCTIONS (commented out - need refactoring for String-based IDs) ===
    // pub fn contribute(
    //     env: Env,
    //     contributor: Address,
    //     campaign_address: Address,
    //     amount: i128,
    // ) -> Result<(), Error> {
    //     contribute(&env, contributor, campaign_address, amount)
    // }

    // pub fn withdraw(env: Env, creator: Address) -> Result<(), Error> {
    //     withdraw(&env, creator)
    // }

    // pub fn refund(env: Env, contributor: Address, campaign_address: Address) -> Result<(), Error> {
    //     refund(&env, contributor, campaign_address)
    // }
}
