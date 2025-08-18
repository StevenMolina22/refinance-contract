use soroban_sdk::{Address, Env, String, Symbol};

pub(crate) fn refund(env: &Env, contributor: &Address, campaign_id: &String, amount: &i128) {
    let topics = (Symbol::new(env, "refund"), contributor);
    let data = (campaign_id.clone(), amount);
    env.events().publish(topics, data);
}
