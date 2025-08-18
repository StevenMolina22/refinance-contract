use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::{has_admin, set_admin},
        token::set_token,
        types::{error::Error, storage::DataKey},
    },
};

pub fn initialize(
    env: &Env,
    admin: Address,
    token: Address,
    nft_contract: Address,
) -> Result<(), Error> {
    if has_admin(env) {
        return Err(Error::ContractInitialized);
    }

    set_admin(&env, &admin);
    set_token(&env, &token);
    env.storage()
        .instance()
        .set(&DataKey::NftContract, &nft_contract);
    events::contract::contract_initialized(&env, &admin, &token);

    Ok(())
}
