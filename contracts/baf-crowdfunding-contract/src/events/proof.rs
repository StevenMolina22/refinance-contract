use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn proof_logged(env: &Env, campaign: &Address, index: &u32) {
    let topics = (Symbol::new(env, "proof_logged"), campaign);
    env.events().publish(topics, index);
}
