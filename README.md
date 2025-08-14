# ReFinance - Transparent Crowdfunding Platform
## Project Description

ReFinance is a revolutionary crowdfunding platform built on the Stellar blockchain that combines traditional crowdfunding with verifiable transparency through on-chain proof attestation and milestone-based fund disbursement.
**Transparent Disbursement System for Foundations (TDSF)**

Se trata de un sistema avanzado de crowdfunding en Rust que permite a fundaciones lanzar campañas con hitos verificables, aceptar contribuciones, y gestionar retiros condicionados por logros comprobados.

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
* **Milestone-Based Disbursement:** Funds are released incrementally based on verified milestone completion
* **String-Based Campaign IDs:** User-friendly campaign identification for better integration
* **Total Transparency:** All disbursements, proofs, and achievements are recorded on the blockchain, allowing anyone to verify how the funds are used.
* **Automated Accountability:** The system streamlines auditing by requiring proof before releasing funds, eliminating the need for lengthy manual processes.
* **Sequential Milestone Validation:** Ensures milestones are completed in order, preventing fund misuse

---

## Architecture

The platform consists of two main smart contracts:

1. **Crowdfunding Contract** (`baf-crowdfunding-contract`): Core crowdfunding functionality with milestone-based fund management
2. **Milestone NFT Contract** (`milestone-nft-contract`): NFT minting for verified milestones (future integration)

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
rustup target add wasm32v1-none
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
```

2️⃣ Optimizar el contrato para reducir su tamaño en bytes

```bash
# Si tienes rustc 1.85 o superior
   stellar contract optimize --wasm target/wasm32v1-none/release/<contract_name>.wasm
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

## Smart Contracts

### Crowdfunding Contract Functions

#### Campaign Functions
| Función           | Descripción                                                              | Firma                                                                                  |
| ----------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `__constructor`   | Inicializa el contrato con admin y token                                 | `(admin: address, token: address) -> Result<(), Error>`                                |
| `add_campaign` | Crea una campaña con ID único y metadatos                               | `(campaign_id: String, creator: address, title: String, description: String, goal: i128, min_donation: i128) -> Result<(), Error>` |
| `get_campaign`    | Obtiene los datos de una campaña por ID                                 | `(campaign_id: String) -> Result<Campaign, Error>`                               |

#### Milestone Functions
| Función               | Descripción                                                              | Firma                                                                                  |
| --------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `add_milestone`    | Crea un hito para una campaña (solo creador)                           | `(campaign_id: String, target_amount: i128, description: String) -> Result<u32, Error>` |
| `get_milestone`       | Obtiene datos de un hito específico                                     | `(campaign_id: String, sequence: u32) -> Result<Milestone, Error>`                   |
| `get_campaign_milestones` | Obtiene todos los hitos de una campaña                              | `(campaign_id: String) -> Result<Vec<Milestone>, Error>`                             |

#### Proof Functions
| Función               | Descripción                                                              | Firma                                                                                  |
| --------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `add_proof`           | Registra una prueba para una campaña (solo admin)                       | `(proof_id: String, campaign_id: String, uri: String, description: String) -> Result<(), Error>` |
| `get_proof`           | Obtiene los datos de una prueba específica                              | `(campaign_id: String, proof_id: String) -> Result<Proof, Error>`                    |
| `validate_milestone_with_proof` | Valida un hito con prueba (solo admin)                        | `(campaign_id: String, milestone_sequence: u32, proof_id: String) -> Result<(), Error>` |

#### Withdrawal Functions
| Función               | Descripción                                                              | Firma                                                                                  |
| --------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `withdraw_milestone_funds` | Retira fondos hasta el hito completado (solo creador)             | `(campaign_id: String, milestone_sequence: u32) -> Result<i128, Error>`|

---

## Estructuras Principales

```rust
#[contracttype]
struct Campaign {
    id: String,                  // Campaign identifier
    creator: Address,
    title: String,               // Campaign title
    description: String,         // Campaign description
    goal: i128,
    min_donation: i128,
    total_raised: i128,
    supporters: u32,

    // Milestone Management
    milestones_count: u32,       // Total milestones for this campaign
    current_milestone: u32,      // Latest completed milestone (0 = none)
    withdrawable_amount: i128,   // Amount available for withdrawal
}

#[contracttype]
struct Milestone {
    campaign_id: String,
    sequence: u32,               // 1, 2, 3... (order matters)
    target_amount: i128,         // Funding needed to reach this milestone
    description: String,         // What this milestone represents
    completed: bool,             // Has this milestone been validated?
    proof_id: Option<String>,    // Which proof validated this milestone
    completed_at: Option<u64>,   // When was it completed
}

#[contracttype]
struct Proof {
    id: String,                  // Proof identifier
    campaign_id: String,         // Which campaign this proof belongs to
    uri: String,                 // IPFS or external URI
    description: String,         // Description of the proof
    timestamp: u64,              // When proof was submitted
}

#[contracttype]
enum DataKey {
    Admin,
    Token,
    Campaign(String),              // String-based campaign ID
    Contribution(String, Address), // (campaign_id, contributor)
    Proof(String, String),         // (campaign_id, proof_id)
    Milestone(String, u32),        // (campaign_id, sequence)
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
  ProofNotFound = 11,
  InvalidGoalAmount = 12,
  InvalidMinDonation = 13,
  InvalidMilestoneAmount = 14,
  MilestoneAmountNotIncreasing = 15,
  MilestoneNotFound = 16,
  MilestoneAlreadyCompleted = 17,
  InsufficientFundsForMilestone = 18,
  MilestoneNotInSequence = 19,
  MilestoneNotCompleted = 20,
  CannotWithdrawFutureMilestone = 21,
  NoFundsToWithdraw = 22,
}
```

---

## Contract Functions from Stellar CLI

### Crowdfunding Contract Commands

### Create Campaign

```bash
        stellar contract deploy \
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm \
        --source admin \
        --network testnet \
        -- add_campaign \
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

### Log Proof (Solo Admin)

```bash
        stellar contract deploy \
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm \
        --source admin \
        --network testnet \
        -- add_proof \
        --campaign <creator_public_key> \
        --uri <proof_uri_64_bytes> \
        --desc <proof_description_128_bytes>
```

### Get Proof

```bash
        stellar contract deploy \
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm \
        --source admin \
        --network testnet \
        -- get_proof \
        --campaign <creator_public_key> \
        --index 0
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

## Milestone-Based Integration Workflow

1. **Campaign Creation**: Foundation creates a crowdfunding campaign with String-based ID
2. **Milestone Setup**: Foundation creates sequential milestones with target amounts
3. **Contribution**: Supporters contribute funds to the campaign
4. **Proof Submission**: Foundation submits proof of milestone completion
5. **Proof Validation**: Admin validates submitted proof and links it to milestone
6. **Sequential Validation**: Milestones must be completed in order (1, 2, 3...)
7. **Fund Release**: Only validated milestones enable incremental fund withdrawal
8. **Transparency**: Public can verify progress through on-chain milestone status

### Example Workflow
```
1. Create campaign "medical-supplies-2024"
2. Create milestones:
   - Milestone 1: $5,000 (Purchase medical equipment)
   - Milestone 2: $8,000 (Staff training completion)
   - Milestone 3: $10,000 (Final implementation)
3. Foundation submits proof-1 with receipts
4. Admin validates proof-1 → Milestone 1 completed
5. Foundation can withdraw $5,000
6. Process repeats for subsequent milestones
```

## Benefits

- **Trust**: Immutable on-chain proof of milestone completion
- **Transparency**: Public verification of campaign progress through String-based IDs
- **Accountability**: Funds released only upon verified sequential milestones
- **Progressive Funding**: Incremental fund release based on demonstrated progress
- **User-Friendly**: String-based campaign identification for better integration
- **Sequential Logic**: Prevents milestone skipping and ensures proper progression
- **Decentralization**: Reduced reliance on centralized oversight

## Conclusion

Este contrato fue desarrollado exclusivamente con fines educativos dentro del contexto del bootcamp, sirviendo como una base práctica para entender los conceptos fundamentales de Soroban y el desarrollo de contratos inteligentes. No está diseñado ni recomendado para ser utilizado en entornos de producción sin antes pasar por una auditoría exhaustiva que garantice su seguridad y robustez. A lo largo del workshop, se profundizará en aspectos clave como la arquitectura del contrato, las mejores prácticas de seguridad y el manejo adecuado de estados, para que los participantes puedan construir soluciones más confiables y escalables.
