# Milestone NFT Contract

A Soroban smart contract that creates NFTs representing milestones in the ReFinance crowdfunding platform. Each NFT represents a validated proof of milestone completion, providing transparency and trust in the crowdfunding process.

## Overview

This contract integrates with the main crowdfunding contract to mint NFTs when milestones are achieved. Each NFT contains:

- **Unique URI**: Custom metadata URI for each milestone
- **Campaign ID**: Links the milestone to its crowdfunding campaign
- **Proof ID**: References the validated proof from the crowdfunding contract
- **Description**: Human-readable milestone description
- **Timestamp**: When the milestone was achieved
- **Validation Status**: Whether the milestone has been validated by admins

## Key Features

- **ERC-721 Compatible**: Standard NFT functionality (transfer, approve, etc.)
- **Custom Metadata**: Rich metadata structure for each milestone
- **Campaign Integration**: Direct connection to crowdfunding campaigns
- **Proof Validation**: Links to on-chain proof system
- **Admin Controls**: Validation and management capabilities
- **Progress Tracking**: Campaign milestone progress monitoring

## Contract Structure

### Data Types

```rust
pub struct TokenMetadata {
    pub uri: String,                // Unique URI for this milestone NFT
    pub campaign_id: BytesN<32>,    // Campaign this milestone belongs to
    pub proof_id: BytesN<32>,       // Associated proof ID
    pub description: String,        // Milestone description
    pub timestamp: u64,             // Achievement timestamp
    pub validated: bool,            // Admin validation status
}

pub struct CollectionMetadata {
    pub name: String,               // Collection name
    pub symbol: String,             // Collection symbol
    pub base_uri: String,           // Base URI for collection
}
```

### Storage Keys

- `Initialized`: Contract initialization status
- `Admin`: Contract administrator address
- `CollectionMetadata`: NFT collection information
- `TokenMetadata(u32)`: Individual token metadata
- `TokenOwner(u32)`: Token ownership mapping
- `CrowdfundingContract`: Authorized crowdfunding contract address

## Core Functions

### Initialization

```rust
pub fn initialize(
    env: Env,
    admin: Address,
    name: String,
    symbol: String,
    base_uri: String,
    crowdfunding_contract: Address,
) -> Result<(), Error>
```

Initializes the NFT contract with collection metadata and admin settings.

### Milestone Management

```rust
pub fn mint_milestone(
    env: Env,
    to: Address,
    uri: String,
    campaign_id: BytesN<32>,
    proof_id: BytesN<32>,
    description: String,
) -> Result<u32, Error>
```

Mints a new milestone NFT. Only callable by admin or authorized crowdfunding contract.

```rust
pub fn validate_milestone(env: Env, token_id: u32) -> Result<(), Error>
```

Validates a milestone (admin only). Sets the `validated` flag to true.

### Query Functions

```rust
pub fn get_token_metadata(env: Env, token_id: u32) -> Result<TokenMetadata, Error>
```

Returns complete metadata for a specific token.

```rust
pub fn get_campaign_milestones(env: Env, campaign_id: BytesN<32>) -> Result<Vec<u32>, Error>
```

Returns all milestone token IDs for a specific campaign.

```rust
pub fn get_campaign_milestone_progress(env: Env, campaign_id: BytesN<32>) -> Result<(u32, u32), Error>
```

Returns (validated_count, total_count) for campaign milestones.

```rust
pub fn proof_has_milestone(env: Env, proof_id: BytesN<32>) -> Result<bool, Error>
```

Checks if a proof has already been minted as an NFT.

### Standard NFT Functions

- `token_uri(token_id)`: Returns the unique URI for a token
- `owner_of(token_id)`: Returns token owner
- `balance_of(owner)`: Returns owner's token count
- `transfer_from(from, to, token_id)`: Transfers token ownership
- `approve(to, token_id)`: Approves address to transfer token
- `set_approval_for_all(operator, approved)`: Sets operator approval for all tokens

## Integration with Crowdfunding Contract

The NFT contract is designed to work seamlessly with the main crowdfunding contract:

1. **Proof Validation**: When a proof is validated in the crowdfunding contract, it can trigger NFT minting
2. **Campaign Tracking**: Each NFT is linked to its campaign for progress monitoring
3. **Trust Building**: NFTs serve as immutable proof of milestone completion
4. **Transparency**: Public visibility of campaign progress through NFT ownership

### Integration Function

```rust
pub fn create_milestone_from_proof(
    env: Env,
    campaign_id: BytesN<32>,
    proof_id: BytesN<32>,
    proof_uri: String,
    proof_description: String,
    recipient: Address,
) -> Result<u32, Error>
```

Called by the crowdfunding contract to create milestone NFTs from validated proofs.

## Deployment

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Soroban CLI
- Stellar network access (testnet/mainnet)

### Build

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Optimize the WASM (optional but recommended)
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/milestone_nft_contract.wasm
```

### Deploy

```bash
# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/milestone_nft_contract.wasm \
  --source alice \
  --network testnet
```

### Initialize

```bash
# Initialize the contract
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --name "ReFinance Milestones" \
  --symbol "RFM" \
  --base_uri "https://api.refinance.com/metadata/" \
  --crowdfunding_contract <CROWDFUNDING_CONTRACT_ADDRESS>
```

## Usage Examples

### Mint a Milestone NFT

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  --network testnet \
  -- mint_milestone \
  --to <RECIPIENT_ADDRESS> \
  --uri "https://api.refinance.com/milestone/campaign1/milestone1" \
  --campaign_id <CAMPAIGN_ID_BYTES> \
  --proof_id <PROOF_ID_BYTES> \
  --description "First milestone: prototype completed"
```

### Validate a Milestone

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  --network testnet \
  -- validate_milestone \
  --token_id 1
```

### Get Campaign Progress

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- get_campaign_milestone_progress \
  --campaign_id <CAMPAIGN_ID_BYTES>
```

### Query Token Metadata

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- get_token_metadata \
  --token_id 1
```

## Security Features

- **Access Control**: Only admin and authorized crowdfunding contract can mint
- **Proof Uniqueness**: Prevents duplicate NFTs for the same proof
- **Validation System**: Two-step process (creation + validation)
- **Immutable Records**: NFT metadata provides permanent milestone records

## Testing

Run the test suite:

```bash
cargo test
```

Key test scenarios:
- Contract initialization
- Milestone minting and validation
- Token transfers and approvals
- Campaign progress tracking
- Unauthorized access prevention

## Error Handling

The contract includes comprehensive error handling:

- `AlreadyInitialized`: Contract already initialized
- `NotInitialized`: Contract not yet initialized
- `Unauthorized`: Caller lacks required permissions
- `TokenNotFound`: Token ID doesn't exist
- `NotOwner`: Caller doesn't own the token
- `NotApproved`: Caller not approved for transfer
- `TokenAlreadyExists`: Attempting to mint duplicate token
- `InvalidCrowdfundingContract`: Invalid crowdfunding contract address

## Future Enhancements

Potential improvements for future versions:

1. **Batch Operations**: Mint multiple milestones in one transaction
2. **Metadata Updates**: Allow updates to milestone descriptions
3. **Milestone Categories**: Different types of milestones (funding, development, etc.)
4. **Staking Integration**: Stake NFTs for governance rights
5. **Royalty System**: Revenue sharing for milestone holders
6. **Cross-Chain Bridge**: Enable milestone NFTs on other networks

## License

This contract is part of the ReFinance crowdfunding platform and follows the project's licensing terms.