# ReFinance - Transparent Crowdfunding Platform
## Project Description

ReFinance is a revolutionary crowdfunding platform built on the Stellar blockchain that combines traditional crowdfunding with verifiable transparency through on-chain proof attestation and NFT milestone representation.
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

## Architecture

The platform consists of two main smart contracts:

1. **Crowdfunding Contract** (`baf-crowdfunding-contract`): Core crowdfunding functionality with proof attestation
2. **Milestone NFT Contract** (`milestone-nft-contract`): NFT minting for verified milestones

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

| Función           | Descripción                                                              | Firma                                                                                  |
| ----------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `__constructor`   | Inicializa el contrato con admin y token                                 | `(admin: address, token: address) -> Result<(), Error>`                                |
| `create_campaign` | Crea una campaña con goal y min_donation                                 | `(creator: address, goal: i128, min_donation: i128) -> Result<(), Error>`              |
| `get_campaign`    | Obtiene los datos de una campaña                                         | `(campaign_address: address) -> Result<Campaign, Error>`                               |
| `contribute`      | Permite a un usuario aportar a una campaña                               | `(contributor: address, campaign_address: address, amount: i128) -> Result<(), Error>` |
| `withdraw`        | Permite al creador retirar fondos si goal fue alcanzado                  | `(creator: address) -> Result<(), Error>`                                              |
| `refund`          | Permite a un contribuyente retirar su aporte si la campaña no tuvo éxito | `(contributor: address, campaign_address: address) -> Result<(), Error>`               |
| `log_proof`       | Registra una prueba de gasto para una campaña (solo admin)               | `(campaign: address, uri: BytesN<64>, desc: BytesN<128>) -> Result<(), Error>`        |
|
| `get_proof`       | Obtiene los datos de una prueba específica por índice                    | `(campaign: address, index: u32) -> Result<Proof, Error>`                             |

---

## Estructuras Principales

```rust
#[contracttype]
struct Campaign {
    goal: i128,
    min_donation: i128,
    supporters: u32,
    total_raised: i128,
    proofs_count: u32,
}

#[contracttype]
struct Contribution {
    amount: i128,
}

struct Proof {
    uri: BytesN<64>,        // URI del documento off-chain (ej: ipfs://<hash>)
    description: BytesN<128>, // Descripción breve de la prueba
    timestamp: u64,         // Timestamp cuando se registró la prueba
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
  ProofNotFound = 11,
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

### Log Proof (Solo Admin)

```bash
        stellar contract deploy \
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm \
        --source admin \
        --network testnet \
        -- log_proof \
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

## Integration Workflow

1. **Campaign Creation**: Foundation creates a crowdfunding campaign
2. **Contribution**: Supporters contribute funds to the campaign
3. **Proof Submission**: Foundation submits proof of milestone completion
4. **Proof Validation**: Admin validates the submitted proof
5. **NFT Minting**: Validated proof triggers automatic NFT creation
6. **Milestone Verification**: NFT serves as immutable proof of achievement
7. **Fund Release**: Validated milestones enable fund withdrawal
8. **Transparency**: Public can verify progress through NFT ownership

## Benefits

- **Trust**: Immutable on-chain proof of milestone completion
- **Transparency**: Public verification of campaign progress
- **Accountability**: Funds released only upon verified milestones
- **Innovation**: NFTs as proof of achievement and trust building
- **Decentralization**: Reduced reliance on centralized oversight

## Conclusion

Este contrato fue desarrollado exclusivamente con fines educativos dentro del contexto del bootcamp, sirviendo como una base práctica para entender los conceptos fundamentales de Soroban y el desarrollo de contratos inteligentes. No está diseñado ni recomendado para ser utilizado en entornos de producción sin antes pasar por una auditoría exhaustiva que garantice su seguridad y robustez. A lo largo del workshop, se profundizará en aspectos clave como la arquitectura del contrato, las mejores prácticas de seguridad y el manejo adecuado de estados, para que los participantes puedan construir soluciones más confiables y escalables.
