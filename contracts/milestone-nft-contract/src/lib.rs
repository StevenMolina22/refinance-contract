#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contractmeta, contracttype, symbol_short, Address,
    BytesN, Env, String,
};

mod integration;

// Metadata for the contract
contractmeta!(
    key = "Description",
    val = "Milestone NFT Contract for ReFinance Crowdfunding Platform"
);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMetadata {
    /// Unique URI for this milestone NFT
    pub uri: String,
    /// Campaign ID this milestone belongs to
    pub campaign_id: BytesN<32>,
    /// Proof ID associated with this milestone
    pub proof_id: BytesN<32>,
    /// Milestone description
    pub description: String,
    /// Timestamp when milestone was achieved
    pub timestamp: u64,
    /// Whether this milestone proof has been validated
    pub validated: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Contract initialization status
    Initialized,
    /// Contract admin/owner
    Admin,
    /// Collection metadata (name, symbol, base_uri)
    CollectionMetadata,
    /// Token metadata for specific token ID
    TokenMetadata(u32),
    /// Token owner mapping
    TokenOwner(u32),
    /// Owner's token count
    OwnerTokenCount(Address),
    /// Token approvals
    TokenApproval(u32),
    /// Operator approvals for all tokens
    OperatorApproval(Address, Address),
    /// Next token ID to mint
    NextTokenId,
    /// Crowdfunding contract address
    CrowdfundingContract,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionMetadata {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    TokenNotFound = 4,
    NotOwner = 5,
    NotApproved = 6,
    TokenAlreadyExists = 7,
    InvalidCrowdfundingContract = 8,
}

const DAY_IN_LEDGERS: u32 = 17280; // Approximately 24 hours in ledgers (5 second intervals)
const INSTANCE_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS * 30; // 30 days
const INSTANCE_BUMP_AMOUNT: u32 = DAY_IN_LEDGERS * 30; // 30 days

#[contract]
pub struct MilestoneNftContract;

#[contractimpl]
impl MilestoneNftContract {
    /// Initialize the NFT contract
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        base_uri: String,
        crowdfunding_contract: Address,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Initialized) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();

        // Set initialization flag
        env.storage().instance().set(&DataKey::Initialized, &true);

        // Set admin
        env.storage().instance().set(&DataKey::Admin, &admin);

        // Set collection metadata
        let collection_metadata = CollectionMetadata {
            name: name.clone(),
            symbol: symbol.clone(),
            base_uri,
        };
        env.storage()
            .instance()
            .set(&DataKey::CollectionMetadata, &collection_metadata);

        // Set initial token ID counter
        env.storage().instance().set(&DataKey::NextTokenId, &1u32);

        // Set crowdfunding contract address
        env.storage()
            .instance()
            .set(&DataKey::CrowdfundingContract, &crowdfunding_contract);

        // Extend instance lifetime
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Emit initialization event
        env.events()
            .publish((symbol_short!("init"),), (admin, name, symbol));

        Ok(())
    }

    /// Mint a new milestone NFT (only callable by admin or crowdfunding contract)
    pub fn mint_milestone(
        env: Env,
        to: Address,
        uri: String,
        campaign_id: BytesN<32>,
        proof_id: BytesN<32>,
        description: String,
    ) -> Result<u32, Error> {
        Self::require_initialized(&env)?;

        let caller = env.current_contract_address();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        let crowdfunding_contract: Address = env
            .storage()
            .instance()
            .get(&DataKey::CrowdfundingContract)
            .unwrap();

        // Only admin or crowdfunding contract can mint
        if caller != admin && caller != crowdfunding_contract {
            return Err(Error::Unauthorized);
        }

        // Get next token ID
        let token_id: u32 = env
            .storage()
            .instance()
            .get(&DataKey::NextTokenId)
            .unwrap_or(1);

        // Create token metadata
        let metadata = TokenMetadata {
            uri,
            campaign_id: campaign_id.clone(),
            proof_id,
            description,
            timestamp: env.ledger().timestamp(),
            validated: false,
        };

        // Store token metadata
        env.storage()
            .instance()
            .set(&DataKey::TokenMetadata(token_id), &metadata);

        // Set token owner
        env.storage()
            .instance()
            .set(&DataKey::TokenOwner(token_id), &to);

        // Update owner's token count
        let current_count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::OwnerTokenCount(to.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::OwnerTokenCount(to.clone()), &(current_count + 1));

        // Increment next token ID
        env.storage()
            .instance()
            .set(&DataKey::NextTokenId, &(token_id + 1));

        // Extend instance lifetime
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Emit mint event
        env.events().publish(
            (symbol_short!("mint"),),
            (token_id, to.clone(), campaign_id.clone()),
        );

        Ok(token_id)
    }

    /// Validate a milestone (only admin)
    pub fn validate_milestone(env: Env, token_id: u32) -> Result<(), Error> {
        Self::require_initialized(&env)?;

        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut metadata: TokenMetadata = env
            .storage()
            .instance()
            .get(&DataKey::TokenMetadata(token_id))
            .ok_or(Error::TokenNotFound)?;

        metadata.validated = true;
        env.storage()
            .instance()
            .set(&DataKey::TokenMetadata(token_id), &metadata);

        // Extend instance lifetime
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Emit validation event
        env.events().publish((symbol_short!("validate"),), token_id);

        Ok(())
    }

    /// Get token metadata
    pub fn get_token_metadata(env: Env, token_id: u32) -> Result<TokenMetadata, Error> {
        Self::require_initialized(&env)?;

        let metadata: TokenMetadata = env
            .storage()
            .instance()
            .get(&DataKey::TokenMetadata(token_id))
            .ok_or(Error::TokenNotFound)?;

        Ok(metadata)
    }

    /// Get token URI (ERC-721 standard)
    pub fn token_uri(env: Env, token_id: u32) -> Result<String, Error> {
        Self::require_initialized(&env)?;

        let metadata: TokenMetadata = env
            .storage()
            .instance()
            .get(&DataKey::TokenMetadata(token_id))
            .ok_or(Error::TokenNotFound)?;

        Ok(metadata.uri)
    }

    /// Get owner of token
    pub fn owner_of(env: Env, token_id: u32) -> Result<Address, Error> {
        Self::require_initialized(&env)?;

        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenOwner(token_id))
            .ok_or(Error::TokenNotFound)?;

        Ok(owner)
    }

    /// Get balance of owner
    pub fn balance_of(env: Env, owner: Address) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::OwnerTokenCount(owner))
            .unwrap_or(0)
    }

    /// Transfer token (only owner or approved)
    pub fn transfer_from(env: Env, from: Address, to: Address, token_id: u32) -> Result<(), Error> {
        Self::require_initialized(&env)?;

        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenOwner(token_id))
            .ok_or(Error::TokenNotFound)?;

        if owner != from {
            return Err(Error::NotOwner);
        }

        // Check if caller is owner or approved
        let caller = env.current_contract_address();
        let is_approved = if caller == owner {
            true
        } else {
            env.storage()
                .instance()
                .get(&DataKey::TokenApproval(token_id))
                == Some(caller.clone())
                || env
                    .storage()
                    .instance()
                    .get(&DataKey::OperatorApproval(owner.clone(), caller))
                    == Some(true)
        };

        if !is_approved {
            return Err(Error::NotApproved);
        }

        from.require_auth();

        // Update owner
        env.storage()
            .instance()
            .set(&DataKey::TokenOwner(token_id), &to);

        // Update balances
        let from_balance: u32 = env
            .storage()
            .instance()
            .get(&DataKey::OwnerTokenCount(from.clone()))
            .unwrap_or(0);
        let to_balance: u32 = env
            .storage()
            .instance()
            .get(&DataKey::OwnerTokenCount(to.clone()))
            .unwrap_or(0);

        if from_balance > 0 {
            env.storage()
                .instance()
                .set(&DataKey::OwnerTokenCount(from.clone()), &(from_balance - 1));
        }
        env.storage()
            .instance()
            .set(&DataKey::OwnerTokenCount(to.clone()), &(to_balance + 1));

        // Clear approvals
        env.storage()
            .instance()
            .remove(&DataKey::TokenApproval(token_id));

        // Extend instance lifetime
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Emit transfer event
        env.events()
            .publish((symbol_short!("transfer"),), (from, to, token_id));

        Ok(())
    }

    /// Approve address to transfer specific token
    pub fn approve(env: Env, to: Address, token_id: u32) -> Result<(), Error> {
        Self::require_initialized(&env)?;

        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenOwner(token_id))
            .ok_or(Error::TokenNotFound)?;

        owner.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::TokenApproval(token_id), &to);

        // Extend instance lifetime
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Emit approval event
        env.events()
            .publish((symbol_short!("approval"),), (owner, to, token_id));

        Ok(())
    }

    /// Get approved address for token
    pub fn get_approved(env: Env, token_id: u32) -> Result<Option<Address>, Error> {
        Self::require_initialized(&env)?;

        // Verify token exists
        if !env.storage().instance().has(&DataKey::TokenOwner(token_id)) {
            return Err(Error::TokenNotFound);
        }

        let approved = env
            .storage()
            .instance()
            .get(&DataKey::TokenApproval(token_id));
        Ok(approved)
    }

    /// Set approval for all tokens
    pub fn set_approval_for_all(env: Env, operator: Address, approved: bool) -> Result<(), Error> {
        Self::require_initialized(&env)?;

        let owner = env.current_contract_address();
        owner.require_auth();

        env.storage().instance().set(
            &DataKey::OperatorApproval(owner.clone(), operator.clone()),
            &approved,
        );

        // Extend instance lifetime
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Emit approval for all event
        env.events()
            .publish((symbol_short!("appr_all"),), (owner, operator, approved));

        Ok(())
    }

    /// Check if operator is approved for all tokens
    pub fn is_approved_for_all(env: Env, owner: Address, operator: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::OperatorApproval(owner, operator))
            .unwrap_or(false)
    }

    /// Get collection metadata
    pub fn get_collection_metadata(env: Env) -> Result<CollectionMetadata, Error> {
        Self::require_initialized(&env)?;

        let metadata: CollectionMetadata = env
            .storage()
            .instance()
            .get(&DataKey::CollectionMetadata)
            .unwrap();

        Ok(metadata)
    }

    /// Get total supply (next token ID - 1)
    pub fn total_supply(env: Env) -> u32 {
        let next_id: u32 = env
            .storage()
            .instance()
            .get(&DataKey::NextTokenId)
            .unwrap_or(1);
        if next_id > 1 {
            next_id - 1
        } else {
            0
        }
    }

    /// Helper function to check if contract is initialized
    fn require_initialized(env: &Env) -> Result<(), Error> {
        if !env.storage().instance().has(&DataKey::Initialized) {
            return Err(Error::NotInitialized);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_token_metadata_creation() {
        let env = Env::default();

        let metadata = TokenMetadata {
            uri: String::from_str(&env, "https://api.refinance.com/milestone/1"),
            campaign_id: BytesN::from_array(&env, &[1u8; 32]),
            proof_id: BytesN::from_array(&env, &[2u8; 32]),
            description: String::from_str(&env, "First milestone completed"),
            timestamp: 1234567890,
            validated: false,
        };

        assert_eq!(metadata.validated, false);
        assert_eq!(metadata.timestamp, 1234567890);
        assert_eq!(metadata.uri.len(), 37);
        assert_eq!(metadata.description.len(), 25);
    }

    #[test]
    fn test_collection_metadata() {
        let env = Env::default();

        let collection = CollectionMetadata {
            name: String::from_str(&env, "ReFinance Milestones"),
            symbol: String::from_str(&env, "RFM"),
            base_uri: String::from_str(&env, "https://api.refinance.com/metadata/"),
        };

        assert_eq!(collection.name.len(), 20);
        assert_eq!(collection.symbol.len(), 3);
        assert_eq!(collection.base_uri.len(), 35);
    }

    #[test]
    fn test_error_variants() {
        assert_eq!(Error::AlreadyInitialized as u32, 1);
        assert_eq!(Error::NotInitialized as u32, 2);
        assert_eq!(Error::Unauthorized as u32, 3);
        assert_eq!(Error::TokenNotFound as u32, 4);
        assert_eq!(Error::NotOwner as u32, 5);
        assert_eq!(Error::NotApproved as u32, 6);
        assert_eq!(Error::TokenAlreadyExists as u32, 7);
        assert_eq!(Error::InvalidCrowdfundingContract as u32, 8);
    }

    #[test]
    fn test_constants() {
        assert_eq!(DAY_IN_LEDGERS, 17280);
        assert_eq!(INSTANCE_LIFETIME_THRESHOLD, DAY_IN_LEDGERS * 30);
        assert_eq!(INSTANCE_BUMP_AMOUNT, DAY_IN_LEDGERS * 30);
        assert_eq!(INSTANCE_LIFETIME_THRESHOLD, 518400);
        assert_eq!(INSTANCE_BUMP_AMOUNT, 518400);
    }

    #[test]
    fn test_bytes_n_operations() {
        let env = Env::default();

        let campaign_id1 = BytesN::from_array(&env, &[1u8; 32]);
        let campaign_id2 = BytesN::from_array(&env, &[1u8; 32]);
        let campaign_id3 = BytesN::from_array(&env, &[2u8; 32]);

        assert_eq!(campaign_id1, campaign_id2);
        assert_ne!(campaign_id1, campaign_id3);
        assert_eq!(campaign_id1.len(), 32);
        assert_eq!(campaign_id3.len(), 32);
    }

    #[test]
    fn test_data_key_variants() {
        let env = Env::default();

        let _init_key = DataKey::Initialized;
        let _admin_key = DataKey::Admin;
        let _collection_key = DataKey::CollectionMetadata;
        let _token_key = DataKey::TokenMetadata(1);
        let _owner_key = DataKey::TokenOwner(1);
        let _count_key = DataKey::OwnerTokenCount(Address::generate(&env));
        let _approval_key = DataKey::TokenApproval(1);
        let _operator_key =
            DataKey::OperatorApproval(Address::generate(&env), Address::generate(&env));
        let _next_id_key = DataKey::NextTokenId;
        let _cf_key = DataKey::CrowdfundingContract;

        assert!(true);
    }
}
