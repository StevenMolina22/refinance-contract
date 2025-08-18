use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn add_campaign(env: &Env, creator: &Address, goal: &i128) {
    let topics = (Symbol::new(env, "add_campaign"), creator);
    env.events().publish(topics, goal);
}

pub (crate) fn withdraw(env: &Env, creator: &Address, total_raised: i128) {
    let topics = (Symbol::new(env, "withdraw"), creator);
    env.events().publish(topics, &total_raised);
}