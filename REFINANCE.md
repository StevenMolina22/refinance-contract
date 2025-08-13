This file is a merged representation of the entire codebase, combined into a single document by Repomix.

# File Summary

## Purpose
This file contains a packed representation of the entire repository's contents.
It is designed to be easily consumable by AI systems for analysis, code review,
or other automated processes.

## File Format
The content is organized as follows:
1. This summary section
2. Repository information
3. Directory structure
4. Repository files (if enabled)
5. Multiple file entries, each consisting of:
  a. A header with the file path (## File: path/to/file)
  b. The full contents of the file in a code block

## Usage Guidelines
- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.

## Notes
- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Files are sorted by Git change count (files with more changes are at the bottom)

# Directory Structure
```
contracts/
  baf-crowdfunding-contract/
    src/
      events/
        campaign.rs
        contract.rs
        contribute.rs
        mod.rs
        refund.rs
      methods/
        add_campaign.rs
        contribute.rs
        get_campaign.rs
        initialize.rs
        mod.rs
        refund.rs
        token.rs
        withdraw.rs
      storage/
        structs/
          campaign.rs
          contribution.rs
          mod.rs
        types/
          error.rs
          mod.rs
          storage.rs
        admin.rs
        campaign.rs
        contribution.rs
        mod.rs
        token.rs
      contract.rs
      lib.rs
      TIPS.md
    Cargo.toml
    Makefile
.gitignore
Cargo.toml
README.md
```

# Files

## File: contracts/baf-crowdfunding-contract/src/events/campaign.rs
````rust
use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn add_campaign(env: &Env, creator: &Address, goal: &i128) {
    let topics = (Symbol::new(env, "add_campaign"), creator);
    env.events().publish(topics, goal);
}

pub (crate) fn withdraw(env: &Env, creator: &Address, total_raised: i128) {
    let topics = (Symbol::new(env, "withdraw"), creator);
    env.events().publish(topics, &total_raised);
}
````

## File: contracts/baf-crowdfunding-contract/src/events/contract.rs
````rust
use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn contract_initialized(env: &Env, admin: &Address, token: &Address) {
    let topics = (Symbol::new(env, "contract_initialized"), admin);
    env.events().publish(topics, token);
}
````

## File: contracts/baf-crowdfunding-contract/src/events/contribute.rs
````rust
use soroban_sdk::{Address, Env, Symbol};


pub(crate) fn add_contribute(env: &Env, contributor: &Address, campaign_address: &Address, amount: &i128) {
    let topics = (Symbol::new(env, "add_contribute"), contributor);
    let data = (campaign_address, amount);
    env.events().publish(topics, data);
}
````

## File: contracts/baf-crowdfunding-contract/src/events/mod.rs
````rust
pub mod contract;
pub mod campaign;
pub mod contribute;
pub mod refund;
````

## File: contracts/baf-crowdfunding-contract/src/events/refund.rs
````rust
use soroban_sdk::{Address, Env, Symbol};


pub(crate) fn refund(env: &Env, contributor: &Address, campaign_address: &Address, amount: &i128) {
    let topics = (Symbol::new(env, "refund"), contributor);
    let data = (campaign_address, amount);
    env.events().publish(topics, data);
}
````

## File: contracts/baf-crowdfunding-contract/src/methods/add_campaign.rs
````rust
use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::get_admin, campaign::{has_campaign, set_campaign}, structs::campaign::Campaign, types::error::Error
    },
};

pub fn add_campaign(env: &Env, creator: Address, goal: i128, min_donation: i128) -> Result<(), Error> {
    let current_admin = get_admin(env);

    current_admin.require_auth();

    if has_campaign(env, &creator) {
        return Err(Error::CampaignAlreadyExists);
    }

    let campaign = Campaign {
        goal,
        min_donation,
        total_raised: 0,
        supporters: 0,
    };

    set_campaign(&env, &creator, &campaign);
    events::campaign::add_campaign(&env, &creator, &goal);
    Ok(())
}
````

## File: contracts/baf-crowdfunding-contract/src/methods/contribute.rs
````rust
use soroban_sdk::{Address, Env};
use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, has_campaign, set_campaign}, contribution::set_contribution, types::error::Error
    }
};

pub fn contribute(env: &Env, contributor: Address, campaign_address: Address, amount: i128) -> Result<(), Error> {
    contributor.require_auth();

    if amount < 0 {
        return Err(Error::AmountMustBePositive);
    }

    if !has_campaign(env, &campaign_address) {
        return Err(Error::CampaignNotFound);
    }

    let mut campaign = get_campaign(env, &campaign_address)?;

    if campaign.min_donation > amount {
        return Err(Error::ContributionBelowMinimum);
    }

    if campaign.total_raised + amount > campaign.goal {
        return Err(Error::CampaignGoalExceeded);
    }

    token_transfer(&env, &contributor, &env.current_contract_address(), &amount)?;

    campaign.total_raised += amount;
    campaign.supporters += 1;
    
    set_campaign(env, &campaign_address, &campaign);
    set_contribution(env, &campaign_address, &contributor, amount);
    events::contribute::add_contribute(&env, &contributor, &campaign_address, &amount);

    Ok(())
}
````

## File: contracts/baf-crowdfunding-contract/src/methods/get_campaign.rs
````rust
use soroban_sdk::{Address, Env};

use crate::storage::{
    campaign::get_campaign as read_campaign, structs::campaign::Campaign, types::error::Error
};

pub fn get_campaign(env: &Env, campaign_address: &Address) ->  Result<Campaign, Error> {
    let campaign = read_campaign(env, &campaign_address)?;
    Ok(campaign)
}
````

## File: contracts/baf-crowdfunding-contract/src/methods/initialize.rs
````rust
use soroban_sdk::{Address, Env};

use crate::{
    events,
    storage::{
        admin::{has_admin, set_admin},
        token::set_token,
        types::error::Error,
    },
};

pub fn initialize(env: &Env, admin: Address, token: Address) -> Result<(), Error> {
    if has_admin(env) {
        return Err(Error::ContractInitialized);
    }

    set_admin(&env, &admin);
    set_token(&env, &token);
    events::contract::contract_initialized(&env, &admin, &token);

    Ok(())
}
````

## File: contracts/baf-crowdfunding-contract/src/methods/mod.rs
````rust
pub mod initialize;
pub mod add_campaign;
pub mod contribute;
pub mod token;
pub mod withdraw;
pub mod refund;
pub mod get_campaign;
````

## File: contracts/baf-crowdfunding-contract/src/methods/refund.rs
````rust
use soroban_sdk::{Address, Env};

use crate::{
    events,
    methods::token::token_transfer, storage::{
        campaign::{get_campaign, set_campaign}, contribution::{
            get_contribution, has_contribution, remove_contribution
        }, types::error::Error
    }
};

pub fn refund(env: &Env, contributor: Address, campaign_address: Address) -> Result<(), Error> {
    contributor.require_auth();

    let mut campaign = get_campaign(env, &campaign_address)?;

    if !has_contribution(env, &campaign_address, &contributor) {
        return Err(Error::ContributionNotFound);
    }

    let amount = get_contribution(env, &campaign_address, &contributor);
    token_transfer(&env, &env.current_contract_address(), &contributor, &amount)?;

    campaign.total_raised -= amount;
    campaign.supporters -= 1;

    remove_contribution(env, &campaign_address, &contributor);
    set_campaign(env, &campaign_address, &campaign);
    events::refund::refund(&env, &contributor, &campaign_address, &amount);

    Ok(())
}
````

## File: contracts/baf-crowdfunding-contract/src/methods/token.rs
````rust
use soroban_sdk::{
    token::{self},
    Address, Env,
};

use crate::storage::{token::get_token, types::error::Error};

pub fn token_transfer(env: &Env, from: &Address, to: &Address, amount: &i128) -> Result<(), Error> {
    let token_id = get_token(env);
    let token = token::Client::new(env, &token_id);
    token.transfer(from, to, amount);
    Ok(())
}
````

## File: contracts/baf-crowdfunding-contract/src/methods/withdraw.rs
````rust
use soroban_sdk::{Address, Env};

use crate::{
    events,
    methods::token::token_transfer,
    storage::{
        campaign::{get_campaign, remove_campaign},
        types::error::Error
    }
};

pub fn withdraw(env: &Env, creator: Address) -> Result<(), Error> {
    creator.require_auth();

    let campaign = get_campaign(env, &creator)?;

    if campaign.total_raised != campaign.goal {
        return Err(Error::CampaignGoalNotReached);
    }

    token_transfer(
        &env,
        &env.current_contract_address(),
        &creator,
        &campaign.total_raised
    )?;

    remove_campaign(env, &creator);
    events::campaign::withdraw(&env, &creator, campaign.total_raised);
    
    Ok(())
}
````

## File: contracts/baf-crowdfunding-contract/src/storage/structs/campaign.rs
````rust
use soroban_sdk::contracttype;


#[derive(Clone)]
#[contracttype]
pub struct Campaign {
    pub goal: i128,
    pub min_donation: i128,
    pub total_raised: i128,
    pub supporters: u32
}
````

## File: contracts/baf-crowdfunding-contract/src/storage/structs/contribution.rs
````rust
use soroban_sdk::contracttype;


#[derive(Clone)]
#[contracttype]
pub struct Contribution {
    pub amount: i128,
}
````

## File: contracts/baf-crowdfunding-contract/src/storage/structs/mod.rs
````rust
pub mod campaign;
pub mod contribution;
````

## File: contracts/baf-crowdfunding-contract/src/storage/types/error.rs
````rust
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    ContractInitialized = 0,
    ContractNotInitialized = 1,
    MathOverflow = 2,
    MathUnderflow = 3,
    CampaignNotFound = 4,
    CampaignGoalExceeded = 5,
    ContributionBelowMinimum = 6,
    AmountMustBePositive = 7,
    CampaignGoalNotReached = 8,
    ContributionNotFound = 9,
    CampaignAlreadyExists = 10,
}
````

## File: contracts/baf-crowdfunding-contract/src/storage/types/mod.rs
````rust
pub mod error;
pub mod storage;
````

## File: contracts/baf-crowdfunding-contract/src/storage/types/storage.rs
````rust
use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Campaign(Address),
    Contribution(Address, Address), // (campaign_address, contributor)
}
````

## File: contracts/baf-crowdfunding-contract/src/storage/admin.rs
````rust
use soroban_sdk::{Address, Env};

use super::types::storage::DataKey;

pub(crate) fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;

    env.storage().instance().has(&key)
}

pub(crate) fn set_admin(env: &Env, admin: &Address) {
    let key = DataKey::Admin;

    env.storage().instance().set(&key, admin);
}

pub(crate) fn get_admin(env: &Env) -> Address {
    let key = DataKey::Admin;

    env.storage().instance().get(&key).unwrap()
}
````

## File: contracts/baf-crowdfunding-contract/src/storage/campaign.rs
````rust
use soroban_sdk::{Address, Env};

use crate::storage::{structs::campaign::Campaign, types::error::Error};

use super::types::storage::DataKey;

pub(crate) fn has_campaign(env: &Env, creator: &Address) -> bool {
    let key = DataKey::Campaign(creator.clone());

    env.storage().instance().has(&key)
}

pub(crate) fn set_campaign(env: &Env, creator: &Address, campaign: &Campaign) {
    let key = DataKey::Campaign(creator.clone());

    env.storage().instance().set(&key, campaign);
}

pub(crate) fn get_campaign(env: &Env, creator: &Address) ->  Result<Campaign, Error> {
    let key = DataKey::Campaign(creator.clone());

    env.storage().instance().get(&key).ok_or(Error::CampaignNotFound)
}

pub(crate) fn remove_campaign(env: &Env, creator: &Address) {
    let key = DataKey::Campaign(creator.clone());

    env.storage().instance().remove(&key);
}
````

## File: contracts/baf-crowdfunding-contract/src/storage/contribution.rs
````rust
use soroban_sdk::{Address, Env};

use super::types::storage::DataKey;

pub(crate) fn has_contribution(env: &Env, campaign_address: &Address, contributor: &Address) -> bool {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().has(&key)
}

pub(crate) fn set_contribution(env: &Env, campaign_address: &Address, contributor: &Address, amount: i128) {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().set(&key, &amount);
}

pub(crate) fn get_contribution(env: &Env, campaign_address: &Address, contributor: &Address) -> i128 {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().get(&key).unwrap()
}

pub(crate) fn remove_contribution(env: &Env, campaign_address: &Address, contributor: &Address) {
    let key = DataKey::Contribution(campaign_address.clone(), contributor.clone());

    env.storage().instance().remove(&key);
}
````

## File: contracts/baf-crowdfunding-contract/src/storage/mod.rs
````rust
pub mod admin;
pub mod types;
pub mod token;
pub mod structs;
pub mod campaign;
pub mod contribution;
````

## File: contracts/baf-crowdfunding-contract/src/storage/token.rs
````rust
use soroban_sdk::{Address, Env};

use super::types::storage::DataKey;

pub(crate) fn set_token(env: &Env, token: &Address) {
    let key = DataKey::Token;

    env.storage().instance().set(&key, token);
}

pub(crate) fn get_token(env: &Env) -> Address {
    let key = DataKey::Token;

    env.storage().instance().get(&key).unwrap()
}
````

## File: contracts/baf-crowdfunding-contract/src/contract.rs
````rust
use soroban_sdk::{contract, contractimpl, Env, Address};

use crate::{
    methods::{
        add_campaign::add_campaign,
        contribute::contribute,
        get_campaign::get_campaign,
        initialize::initialize,
        refund::refund,
        withdraw::withdraw
    },
    storage::{
        structs::campaign::Campaign,
        types::error::Error
    },
};

#[contract]
pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {
    pub fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        initialize(&env, admin, token)
    }

    pub fn create_campaign(env: Env, creator: Address, goal: i128, min_donation: i128) -> Result<(), Error> {
        add_campaign(&env, creator, goal, min_donation)
    }

    pub fn get_campaign(env: Env, campaign_address: Address) -> Result<Campaign, Error> {
        get_campaign(&env, &campaign_address)
    }

    pub fn contribute(env: Env, contributor: Address, campaign_address: Address, amount: i128) -> Result<(), Error> {
        contribute(&env, contributor, campaign_address, amount)
    }

    pub fn withdraw(env: Env, creator: Address) -> Result<(), Error> {
        withdraw(&env, creator)
    }

    pub fn refund(env: Env, contributor: Address, campaign_address: Address) -> Result<(), Error> {
        refund(&env, contributor, campaign_address)
    }
}
````

## File: contracts/baf-crowdfunding-contract/src/lib.rs
````rust
#![no_std]

mod contract;
mod events;
mod methods;
mod storage;

pub use contract::CrowdfundingContract;
````

## File: contracts/baf-crowdfunding-contract/src/TIPS.md
````markdown
**Tips**:
- Initialize the contract before deployment for more security (just as it is done in the project)
- Always test the contract
- run scout-audit
```sh
    # https://github.com/CoinFabrik/scout-audit
    cargo scout
```
````

## File: contracts/baf-crowdfunding-contract/Cargo.toml
````toml
[package]
name = "baf-crowdfunding-contract"
version = "0.0.0"
edition = "2021"
publish = false

[lib]
crate-type = ["lib", "cdylib"]
doctest = false

[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
````

## File: contracts/baf-crowdfunding-contract/Makefile
````
default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
````

## File: Cargo.toml
````toml
[workspace]
resolver = "2"
members = [
  "contracts/*",
]

[workspace.dependencies]
soroban-sdk = "22.0.0"

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true

# For more information about this profile see https://soroban.stellar.org/docs/basic-tutorials/logging#cargotoml-profile
[profile.release-with-logs]
inherits = "release"
debug-assertions = true
````

## File: .gitignore
````
# Rust's output directory
target

# Local settings
.soroban
.stellar
temp
repomix.config.json
.repomixignore
.env
````

## File: README.md
````markdown
# ReFinance
## Project Description
**Transparent Disbursement System for Foundations (TDSF)**

Se trata de un contrato básico de crowdfunding en Rust que permite a fundaciones lanzar campañas con metas de recaudación, aceptar contribuciones, y gestionar retiros y reembolsos.

### Problem Description
A persistent challenge for non-profit organizations is the lack of transparency in fund management. Donors often have no visibility into how their contributions are spent, which can lead to a decline in trust and, potentially, a reduction in donations. Foundations, in turn, struggle to efficiently and credibly demonstrate their impact and accountability.

### Our Solution: Verifiable Transparency with Blockchain
We propose a fund management system that uses blockchain technology to create a **transparent and conditional disbursement flow**. Instead of transferring all funds to foundations at once, our platform disburses money in stages, contingent on the fulfillment of predefined objectives.

The process works as follows:
1.  **Defining Milestones and Objectives:** Foundations and donors collaborate to establish a detailed project plan with clear, verifiable milestones and goals.
2.  **Uploading Immutable Evidence:** As the foundation meets each milestone (e.g., purchasing materials or completing a project phase), it uploads proof (invoices, photos, videos, etc.) to the blockchain. This evidence, once registered, is immutable and publicly accessible.
3.  **NFTs as Proof of Impact:** Each completed and verified milestone generates a **unique NFT**. These NFTs are not just collectibles; they are **immutable records of the foundation's social impact**. They serve as a verifiable portfolio of achievements that foundations can use to attract future donors and demonstrate their effectiveness.
4.  **Conditional Disbursements:** Once the evidence is registered on the blockchain and verified (either through an automated process or a governance mechanism), the next tranche of funds is automatically released to the foundation.

---

### Key Features
* **Total Transparency:** All disbursements, proofs, and achievements are recorded on the blockchain, allowing anyone to verify how the funds are used.
* **Automated Accountability:** The system streamlines auditing by requiring proof before releasing funds, eliminating the need for lengthy manual processes.
* **Tokenization of Impact:** NFTs are not only proof but also an innovative way for foundations to showcase and celebrate their accomplishments, creating a verifiable history of their work.

---

### Future Vision
Our long-term goal is to take transparency one step further. We aim to integrate the system so that disbursements are made **directly to suppliers** (e.g., the materials provider or catering service), completely eliminating the possibility of fund diversion. This would ensure that every donated dollar directly translates into a good or service for the final beneficiary.

### Hackathon Relevance
This project directly addresses the theme of transparency and trust in the social sector, using decentralized technologies to solve a critical problem. It's a practical solution that demonstrates the potential of blockchain to generate real and verifiable social impact.

## Setup

### Rust Toolchain

Descarga e instala Rust siguiendo la guía oficial:
https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup

### Target

Luego instala el target WASM según tu versión de Rustc:

```bash
# Si tienes rustc 1.85 o superior
rustup target add wasm32v1-none

# Si tienes rustc menor a 1.85
rustup target add wasm32-unknown-unknown
```

### Instalar Stellar CLI

```bash
cargo install --locked stellar-cli@23.0.0
```

---

## Extensiones para VS Code

1️⃣ Even Better TOML
2️⃣ CodeLLDB (debugging paso a paso)
3️⃣ Rust Analyzer (soporte para Rust)

---

## Comandos básicos para crear y desplegar el contrato

### Deploy en Testnet:

🔑 Generar Keypair para las pruebas

```bash
stellar keys generate --global alice --network testnet --fund
```

📌 Pasos para el deploy:
1️⃣ Compilar el contrato y generar el archivo .wasm

```bash
# Si tienes rustc 1.85 o superior
  cargo build --target wasm32v1-none --release

# Si tienes rustc menor a 1.85
  cargo build --target wasm32-unknown-unknown --release
```

2️⃣ Optimizar el contrato para reducir su tamaño en bytes

```bash
# Si tienes rustc 1.85 o superior
   stellar contract optimize --wasm target/wasm32v1-none/release/<contract_name>.wasm

# Si tienes rustc menor a 1.85
 stellar contract optimize --wasm target/wasm32-unknown-unknown/release/<contract_name>.wasm
```

1️⃣ Generar Admin Keypair para las pruebas

```bash
stellar keys generate --global admin --network testnet --fund
```

2️⃣ Obtener el token address de XLM para usar en el contrato

```bash
stellar contract asset id --asset native --network testnet
```

_Nota: devuelve `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`_

4️⃣ Obtener el admin public key

```bash
stellar keys address admin
```

_Nota: devuelve `GDXAECCYWYW2QKQDTGVQUTC6CQEQR3REC3PKZKXOP76PJJ6V3FRYXCO3`_

5️⃣ Deployar el contrato en la Testnet y obtener el contract ID

```bash
        stellar contract deploy \
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm \
        --source admin \
        --network testnet \
        -- \
        --admin <admin_public_key>
        --token <token_address>
```

_Nota: devuelve `CBAH4Z5CNELXMN7PVW2SAAB6QVOID34SAQAFHJF7Q7JUNACRQEJX66MB`_

---

## Funciones del Contrato

| Función           | Descripción                                                              | Firma                                                                                  |
| ----------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `__constructor`   | Inicializa el contrato con admin y token                                 | `(admin: address, token: address) -> Result<(), Error>`                                |
| `create_campaign` | Crea una campaña con goal y min_donation                                 | `(creator: address, goal: i128, min_donation: i128) -> Result<(), Error>`              |
| `get_campaign`    | Obtiene los datos de una campaña                                         | `(campaign_address: address) -> Result<Campaign, Error>`                               |
| `contribute`      | Permite a un usuario aportar a una campaña                               | `(contributor: address, campaign_address: address, amount: i128) -> Result<(), Error>` |
| `withdraw`        | Permite al creador retirar fondos si goal fue alcanzado                  | `(creator: address) -> Result<(), Error>`                                              |
| `refund`          | Permite a un contribuyente retirar su aporte si la campaña no tuvo éxito | `(contributor: address, campaign_address: address) -> Result<(), Error>`               |

---

## Estructuras Principales

```rust
#[contracttype]
struct Campaign {
  goal: i128,
  min_donation: i128,
  supporters: u32,
  total_raised: i128,
}

#[contracttype]
struct Contribution {
  amount: i128,
}

#[contracttype]
enum DataKey {
  Admin(),
  Token(),
  Campaign(address),
  Contribution(address, address),
}

#[contracterror]
enum Errors {
  ContractInitialized = 0,
  ContractNotInitialized = 1,
  MathOverflow = 2,
  MathUnderflow = 3,
  CampaignNotFound = 4,
  CampaignGoalExceeded = 5,
  ContributionBelowMinimum = 6,
  AmountMustBePositive = 7,
  CampaignGoalNotReached = 8,
  ContributionNotFound = 9,
  CampaignAlreadyExists = 10,
}
```

---

## Funciones del contrato desde el Stellar CLI

### Create Campaign

```bash
        stellar contract deploy \
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm \
        --source admin \
        --network testnet \
        -- create_campaign \
        --creator <creator_public_key>
        --goal 100000000
```

### Get Campaign

```bash
        stellar contract deploy \
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm \
        --source admin \
        --network testnet \
        -- get_campaign \
        --campaign_address <creator_public_key>
```

### Add Contribution

```bash
        stellar contract deploy \
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm \
        --source <contributor_secret_key> \
        --network testnet \
        -- contribute \
        --contributor <contributor_public_key>
        --campaign_address <creator_public_key>
        --amount 100000000
```

---

## Nota:

| XLM     | Stroops       | Explicación                             |
| ------- | ------------- | --------------------------------------- |
| 1 XLM   | 10,000,000    | 1 XLM equivale a 10 millones de stroops |
| 5 XLM   | 50,000,000    | 5 XLM en stroops                        |
| 10 XLM  | 100,000,000   | 10 XLM en stroops                       |
| 100 XLM | 1,000,000,000 | 100 XLM en stroops                      |

---

## Conclusion

Este contrato fue desarrollado exclusivamente con fines educativos dentro del contexto del bootcamp, sirviendo como una base práctica para entender los conceptos fundamentales de Soroban y el desarrollo de contratos inteligentes. No está diseñado ni recomendado para ser utilizado en entornos de producción sin antes pasar por una auditoría exhaustiva que garantice su seguridad y robustez. A lo largo del workshop, se profundizará en aspectos clave como la arquitectura del contrato, las mejores prácticas de seguridad y el manejo adecuado de estados, para que los participantes puedan construir soluciones más confiables y escalables.
````
