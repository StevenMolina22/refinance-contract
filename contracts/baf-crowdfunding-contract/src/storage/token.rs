use soroban_sdk::{Address, Env};

use super::types::storage::DataKey;

pub fn set_token(env: &Env, token: &Address) {
    let key = DataKey::Token;

    env.storage().instance().set(&key, token);
}

pub fn get_token(env: &Env) -> Address {
    let key = DataKey::Token;

    env.storage().instance().get(&key).unwrap()
}
