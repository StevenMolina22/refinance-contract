# Crowdfunding Contract - Soroban Workshop

**Technical Bootcamp Part I: Soroban & Architecture Fundamentals**

## Introducción

Este proyecto es una demostración práctica para aprender conceptos fundamentales de Soroban y el desarrollo de contratos inteligentes.
Fue creado para el workshop de BAF en el marco del Stellar GIVE Hackathon Argentina 2025.

Se trata de un contrato básico de crowdfunding en Rust que permite a creadores lanzar campañas con metas de recaudación, aceptar contribuciones, y gestionar retiros y reembolsos.

---

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
    stellar contract deploy `
        --wasm target/wasm32v1-none/release/<contract_name>.optimized.wasm `
        --source <contributor_secret_key> `
        --network testnet `
        -- contribute `
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
