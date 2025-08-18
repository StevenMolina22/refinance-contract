use soroban_sdk::{Address, Env};

use super::types::storage::DataKey;

pub fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;

    env.storage().instance().has(&key)
}

pub fn set_admin(env: &Env, admin: &Address) {
    let key = DataKey::Admin;

    env.storage().instance().set(&key, admin);
}

pub fn get_admin(env: &Env) -> Address {
    let key = DataKey::Admin;

    env.storage().instance().get(&key).unwrap()
}
